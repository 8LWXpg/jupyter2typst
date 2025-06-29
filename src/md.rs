use itertools::Itertools;
use markdown::{Constructs, ParseOptions, mdast::Node, to_mdast};
use reqwest::blocking;
use sha1::{Digest, Sha1};
use std::borrow::Cow;
use std::fmt::Write as _;
use std::sync::{LazyLock, Mutex};
use std::{collections::HashMap, fs::File, io::Write};
use url::Url;

use crate::IMG_PATH;
use crate::{katex, typ};

static FOOTNOTE_DEFINITIONS: LazyLock<Mutex<HashMap<String, String>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
/// <name in attachments, file path>
static ATTACHMENTS: LazyLock<Mutex<HashMap<String, String>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

/// Convert Markdown to Typst.
///
/// # Arguments
///
/// - `md` (`&str`) - Markdown string
/// - `attachments` (`HashMap<String, String>`) - Peprocessed attachments with <name, file_path>
pub fn md_to_typst(md: &str, attachments: HashMap<String, String>) -> String {
	let tree = to_mdast(
		md,
		&ParseOptions {
			constructs: Constructs {
				math_flow: true,
				math_text: true,
				..Constructs::gfm()
			},
			..Default::default()
		},
	)
	.unwrap();

	// Write tree to debug file
	// let mut file = File::create("debug.txt").unwrap();
	// file.write_all(format!("{:#?}", tree).as_bytes()).unwrap();
	{
		let mut w_fd = FOOTNOTE_DEFINITIONS.lock().unwrap();
		*w_fd = footnote_grep(&tree);

		let mut w_a = ATTACHMENTS.lock().unwrap();
		*w_a = attachments;
	}
	ast_parse(&tree).to_string()
}

macro_rules! parse_children {
	($node:expr) => {
		$node.children.iter().map(ast_parse).join("")
	};
}

fn ast_parse(node: &Node) -> Cow<'_, str> {
	match node {
		Node::Blockquote(node) => format!(
			"#block-quote[\n  {}\n]\n",
			node.children
				.iter()
				.map(|child| ast_parse(child).trim_end_matches('\n').replace('\n', "\n  "))
				.join("\n  ")
		)
		.into(),
		Node::Break(_) => "\n".into(),
		Node::Code(node) => format!("```{}\n{}\n```\n", node.lang.as_deref().unwrap_or_default(), node.value).into(),
		Node::Delete(node) => format!("#strike[{}]", parse_children!(node)).into(),
		Node::Emphasis(node) => format!("#emph[{}]", parse_children!(node)).into(),
		Node::FootnoteDefinition(_) => "".into(),
		Node::FootnoteReference(node) => {
			let id = &node.identifier;
			if let Some(link) = FOOTNOTE_DEFINITIONS.lock().unwrap().get(id) {
				format!("#link(\"{link}\")[^{id}]")
			} else {
				format!("[^{id}]")
			}
			.into()
		}
		Node::Heading(node) => format!("{} {}\n\n", "=".repeat(node.depth as usize), parse_children!(node)).into(),
		Node::Html(node) => html_to_typst(&node.value).into(),
		Node::Image(node) => match Url::parse(&node.url) {
			Ok(url) => match url.scheme() {
				"http" | "https" => format!("#image(\"{}\")", download_image(url)).into(),
				_ => {
					let name = node.url.strip_prefix("attachment:").unwrap();
					let a = ATTACHMENTS.lock().unwrap();
					if let Some(file_path) = a.get(name) {
						format!("#image(\"{file_path}\")").into()
					} else {
						"".into()
					}
				}
			},
			Err(_) => {
				let name = node.url.strip_prefix("attachment:").unwrap();
				if let Some(file_path) = ATTACHMENTS.lock().unwrap().get(name) {
					format!("#image(\"{file_path}\")").into()
				} else {
					"".into()
				}
			}
		},
		Node::InlineCode(node) => format!("`{}`", node.value).into(),
		Node::InlineMath(node) => format!("${}$", katex::latex_to_typst((&node.value).into()).unwrap()).into(),
		Node::Link(node) => format!("#link(\"{}\")[{}]", node.url, parse_children!(node)).into(),
		Node::List(node) => format!(
			"{}\n",
			node.children
				.iter()
				.map(|child| {
					let mut ret = format!(
						"{} {}\n",
						if node.ordered { '+' } else { '-' },
						ast_parse(child).trim_end_matches('\n').replace('\n', "\n  ")
					);
					if node.spread {
						ret.push('\n');
					}
					ret
				})
				.join("")
		)
		.into(),
		Node::ListItem(node) => parse_children!(node).into(),
		Node::Math(node) => format!("$ {} $\n", katex::latex_to_typst((&node.value).into()).unwrap()).into(),
		Node::Paragraph(node) => format!("{}\n", parse_children!(node)).into(),
		Node::Root(node) => parse_children!(node).into(),
		Node::Strong(node) => format!("*{}*", parse_children!(node)).into(),
		Node::Table(node) => format!(
			"#table(\n  columns: {},\n  align: ({}),\n  table.header(\n    {}  ),\n{})\n\n",
			node.align.len(),
			node.align
				.iter()
				.map(|a| {
					match a {
						markdown::mdast::AlignKind::Left => "left",
						markdown::mdast::AlignKind::Center => "center",
						markdown::mdast::AlignKind::Right => "right",
						markdown::mdast::AlignKind::None => "auto",
					}
				})
				.collect::<Vec<_>>()
				.join(", "),
			ast_parse(&node.children[0]),
			node.children.iter().skip(1).map(|child| ast_parse(child)).join("")
		)
		.into(),
		Node::TableCell(node) => format!("[{}]", parse_children!(node)).into(),
		Node::TableRow(node) => {
			format!("  {},\n", node.children.iter().map(|child| ast_parse(child)).join(", ")).into()
		}
		Node::Text(node) => typ::escape_content(&node.value).into(),
		Node::ThematicBreak(_) => "#line(length: 100%)\n".into(),
		_ => unreachable!(),
	}
}

fn footnote_grep(node: &Node) -> HashMap<String, String> {
	let mut definitions: HashMap<String, String> = HashMap::new();
	match node {
		Node::FootnoteDefinition(node) => {
			let mut item = String::new();
			for child in &node.children {
				item = footnote_def_parse(child);
			}
			definitions.insert(node.identifier.clone(), item);
		}
		_ => {
			if let Some(children) = node.children() {
				for child in children {
					let mut d = footnote_grep(child);
					definitions.extend(d.drain());
				}
			}
		}
	}
	definitions
}

fn footnote_def_parse(node: &Node) -> String {
	match node {
		Node::Paragraph(node) => node.children.iter().map(footnote_def_parse).join(""),
		Node::Text(node) => node.value.to_owned(),
		Node::Link(node) => node.url.to_owned(),
		_ => unreachable!("footnote_def_parse unhandled node: {:?}\n", node),
	}
}

pub fn sha1(s: &str) -> String {
	let mut sha1 = Sha1::new();
	sha1.update(s);
	sha1.finalize().iter().fold(String::new(), |mut output, p| {
		write!(output, "{p:02x}").unwrap();
		output
	})
}

fn download_image(url: Url) -> String {
	println!("Downloading image from {url}");
	let response = match blocking::get(url.as_str()) {
		Ok(r) => r,
		Err(e) => {
			println!("Download image at {url} failed: {e}");
			return url.as_str().to_string();
		}
	};

	let headers = response.headers().clone();
	let img_bytes = response.bytes().unwrap();
	let content_type = match headers.get(reqwest::header::CONTENT_TYPE) {
		Some(content_type) => match content_type.to_str().unwrap() {
			"image/png" => "png",
			"image/jpeg" => "jpg",
			"image/gif" => "gif",
			"image/svg+xml" => "svg",
			c => {
				println!("Unsupported image format: {c}");
				return url.as_str().to_string();
			}
		},
		None => {
			// guess image format, not containing svg
			match image::guess_format(&img_bytes) {
				Ok(format) => match format {
					image::ImageFormat::Png => "png",
					image::ImageFormat::Jpeg => "jpg",
					image::ImageFormat::Gif => "gif",
					_ => {
						println!("Unsupported image format: {format:?}");
						return url.as_str().to_string();
					}
				},
				Err(e) => {
					eprintln!("{e}");
					return url.as_str().to_string();
				}
			}
		}
	};

	let path = format!("{}/{}.{}", IMG_PATH.get().unwrap(), sha1(url.as_str()), content_type,);
	let mut file = File::create(&path).unwrap();
	file.write_all(&img_bytes).unwrap();
	println!("Downloaded image to {path}");
	path
}

pub fn html_to_typst(html: &str) -> String {
	// TODO html to typst
	typ::escape_content(html)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_heading_math() {
		let md = "## heading $math$";
		assert_eq!(md_to_typst(md, HashMap::new()), "== heading $m a t h$\n\n")
	}

	#[test]
	fn test_mdast() {
		let md = "$$\n\\LaTeX\n$$\n$\\KaTeX$";
		println!(
			"{:#?}",
			to_mdast(
				md,
				&ParseOptions {
					constructs: Constructs {
						math_flow: true,
						math_text: true,
						..Constructs::gfm()
					},
					..Default::default()
				}
			)
			.unwrap()
		);
	}

	#[test]
	fn test_table() {
		let table = "| Syntax test | Description | Test | XXX | XXX |
| ----------- | ----------- | ---- | --- | --- |
| Header      | Title       | AAAA | XXX | XXX |
| Header      | Title       | AAAA | XXX | XXX |
| Paragraph   | Text        | BBBB | XXX | XXX |";
		// println!("{}", md_to_typst(vec![table], HashMap::new()));
		assert_eq!(
			md_to_typst(table, HashMap::new()),
			"#table(
  columns: 5,
  align: (auto, auto, auto, auto, auto),
  table.header(
      [Syntax test], [Description], [Test], [XXX], [XXX],
  ),
  [Header], [Title], [AAAA], [XXX], [XXX],
  [Header], [Title], [AAAA], [XXX], [XXX],
  [Paragraph], [Text], [BBBB], [XXX], [XXX],
)

"
			.to_string(),
		)
	}
}
