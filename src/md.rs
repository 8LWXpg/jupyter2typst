use markdown::{mdast::Node, to_mdast, Constructs, ParseOptions};
use reqwest::blocking;
use sha1::{Digest, Sha1};
use std::fmt::Write as _;
use std::sync::{LazyLock, RwLock};
use std::{collections::HashMap, fs::File, io::Write};
use url::Url;

use crate::IMG_PATH;
use crate::{katex, typ};

static FOOTNOTE_DEFINITIONS: LazyLock<RwLock<HashMap<String, String>>> =
	LazyLock::new(|| RwLock::new(HashMap::new()));
/// <name in attachments, file path>
static ATTACHMENTS: LazyLock<RwLock<HashMap<String, String>>> =
	LazyLock::new(|| RwLock::new(HashMap::new()));

pub fn md_to_typst(md: Vec<&str>, attachments: HashMap<String, String>) -> String {
	let tree = to_mdast(
		&md.join(""),
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

	// write tree to debug file
	// let mut file = File::create("debug.txt").unwrap();
	// file.write_all(format!("{:#?}", tree).as_bytes()).unwrap();
	{
		let mut w_fd = FOOTNOTE_DEFINITIONS.write().unwrap();
		*w_fd = footnote_grep(tree.clone());

		let mut w_a = ATTACHMENTS.write().unwrap();
		*w_a = attachments;
	}
	ast_parse(tree)
}

fn ast_parse(node: Node) -> String {
	let mut context = String::new();

	match node {
		Node::Blockquote(node) => {
			context += "#block-quote[\n";
			for child in node.children {
				context += &format!(
					"  {}\n",
					ast_parse(child)
						.trim_end_matches('\n')
						.replace('\n', "\n  ")
				);
			}
			context += "]\n";
		}
		Node::Break(_) => context.push('\n'),
		Node::Code(node) => {
			context += &format!("```{}\n", node.lang.unwrap_or_else(|| "".to_string()));
			context += &node.value;
			context += "\n```\n";
		}
		Node::Delete(node) => {
			context += "#strike[";
			for child in node.children {
				context += &ast_parse(child);
			}
			context.push(']');
		}
		Node::Emphasis(node) => {
			context += "#emph[";
			for child in node.children {
				context += &ast_parse(child);
			}
			context.push(']');
		}
		Node::FootnoteDefinition(_) => {
			// do nothing
		}
		Node::FootnoteReference(node) => {
			let id = node.identifier.as_str();
			if let Some(link) = FOOTNOTE_DEFINITIONS.read().unwrap().to_owned().get(id) {
				context += &format!("#link(\"{}\")[^{}]", link, id)
			} else {
				context += &format!("[^{}]", id)
			}
		}
		Node::Heading(node) => {
			let mut item = String::new();
			context += &format!("{} ", "=".repeat(node.depth as usize));
			for child in node.children {
				item = ast_parse(child);
			}
			context += &item;
			context += "\n\n";
		}
		Node::Html(node) => context += &typ::escape_content(html_to_typst(&node.value)),
		Node::Image(node) => match Url::parse(&node.url) {
			Ok(url) => match url.scheme() {
				"http" | "https" => context += &format!("#image(\"{}\")", download_image(url)),
				_ => {
					let name = node.url.strip_prefix("attachment:").unwrap();
					let a = ATTACHMENTS.read().unwrap();
					if let Some(file_path) = a.to_owned().get(name) {
						context += &format!("#image(\"{}\")", file_path)
					};
				}
			},
			Err(_) => {
				let name = node.url.strip_prefix("attachment:").unwrap();
				if let Some(file_path) = ATTACHMENTS.read().unwrap().to_owned().get(name) {
					context += &format!("#image(\"{}\")", file_path)
				};
			}
		},
		Node::InlineCode(node) => context += &format!("`{}`", node.value),
		Node::InlineMath(node) => {
			context += &format!("${}$", katex::latex_to_typst(node.value.into()).unwrap())
		}
		Node::Link(node) => {
			context += &format!("#link(\"{}\")[", node.url);
			for child in node.children {
				context += &ast_parse(child);
			}
			context.push(']');
		}
		Node::List(node) => {
			for child in node.children {
				context += if node.ordered { "+ " } else { "- " };
				let mut item = ast_parse(child)
					.trim_end_matches('\n')
					.replace('\n', "\n  ");
				item.push('\n');
				context += &item;
				if node.spread {
					context.push('\n');
				}
			}
			context.push('\n');
		}
		Node::ListItem(node) => {
			for child in node.children {
				context += &ast_parse(child);
			}
		}
		Node::Math(node) => {
			// println!("{}\n", node.value);
			context += &format!(
				"$ {} $\n",
				katex::latex_to_typst(node.value.into()).unwrap()
			)
		}
		Node::Paragraph(node) => {
			for child in node.children {
				context += &ast_parse(child);
			}
			context.push('\n');
		}
		Node::Root(node) => {
			for child in node.children {
				context += &ast_parse(child);
			}
		}
		Node::Strong(node) => {
			context.push('*');
			for child in node.children {
				context += &ast_parse(child);
			}
			context.push('*');
		}
		Node::Table(node) => {
			context += "#table(\n";
			context += &format!("  columns: {},\n", node.align.len());
			context += &format!(
				"  align: ({}),\n",
				node.align
					.iter()
					.map(|a| {
						match a {
							markdown::mdast::AlignKind::Left => "left".to_string(),
							markdown::mdast::AlignKind::Center => "center".to_string(),
							markdown::mdast::AlignKind::Right => "right".to_string(),
							markdown::mdast::AlignKind::None => "auto".to_string(),
						}
					})
					.collect::<Vec<String>>()
					.join(", ")
			);
			let mut children = node.children;
			context += "  table.header(\n";
			context += "    ";
			context += &ast_parse(children.remove(0));
			context += "  ),\n";
			for child in children {
				context += &ast_parse(child);
			}
			context += ")\n\n";
		}
		Node::TableCell(node) => {
			context.push('[');
			for child in node.children {
				context += &ast_parse(child);
			}
			context += "], ";
		}
		Node::TableRow(node) => {
			context += "  ";
			for child in node.children {
				context += &ast_parse(child);
			}
			context.pop();
			context.push('\n');
		}
		Node::Text(node) => {
			context += &typ::escape_content(node.value);
		}
		Node::ThematicBreak(_) => {
			context += "#line(length: 100%)\n";
		}
		_ => unreachable!(),
	}

	context
}

fn footnote_grep(node: Node) -> HashMap<String, String> {
	let mut definitions: HashMap<String, String> = HashMap::new();
	match node {
		Node::FootnoteDefinition(node) => {
			let mut item = String::new();
			for child in node.children {
				item = footnote_def_parse(child);
			}
			definitions.insert(node.identifier.clone(), item);
		}
		_ => {
			if let Some(children) = node.children() {
				for child in children {
					let mut d = footnote_grep(child.clone());
					definitions.extend(d.drain());
				}
			}
		}
	}
	definitions
}

fn footnote_def_parse(node: Node) -> String {
	let mut context = String::new();
	match node {
		Node::Paragraph(node) => {
			for child in node.children {
				context += &footnote_def_parse(child);
			}
		}
		Node::Text(node) => context += &node.value,
		Node::Link(node) => context += &node.url,
		_ => println!("footnote_def_parse unhandled node: {:?}\n", node),
	}
	context
}

pub fn sha1(s: &str) -> String {
	let mut sha1 = Sha1::new();
	sha1.update(s);
	sha1.finalize().iter().fold(String::new(), |mut output, p| {
		write!(output, "{:02x}", p).unwrap();
		output
	})
}

fn download_image(url: Url) -> String {
	println!("Downloading image from {}", url);
	let response = match blocking::get(url.as_str()) {
		Ok(r) => r,
		Err(e) => {
			println!("Download image at {} failed: {}", url, e);
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
				println!("Unsupported image format: {}", c);
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
						println!("Unsupported image format: {:?}", format);
						return url.as_str().to_string();
					}
				},
				Err(e) => {
					eprintln!("{}", e);
					return url.as_str().to_string();
				}
			}
		}
	};

	let path = format!(
		"{}/{}.{}",
		IMG_PATH.get().unwrap(),
		sha1(url.as_str()),
		content_type,
	);
	let mut file = File::create(&path).unwrap();
	file.write_all(&img_bytes).unwrap();
	println!("Downloaded image to {}", path);
	path
}

pub fn html_to_typst(html: &str) -> String {
	// TODO html to typst
	html.to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

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
			md_to_typst(vec![table], HashMap::new()),
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
