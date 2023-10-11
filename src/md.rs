use image;
use markdown::{mdast::Node, to_mdast, Constructs, ParseOptions};
use reqwest::blocking;
use sha1::{Digest, Sha1};
use std::sync::RwLock;
use std::{collections::HashMap, fs::File, io::Write};
use url::Url;

use crate::IMG_PATH;
mod katex;
mod typ;

use once_cell::sync::Lazy;

static FOOTNOTE_DEFINITIONS: Lazy<RwLock<HashMap<String, String>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
/// <name in attachments, file path>
static ATTACHMENTS: Lazy<RwLock<HashMap<String, String>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

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
        Node::BlockQuote(node) => {
            context.push_str("#block-quote[\n");
            for child in node.children {
                let mut item = ast_parse(child);
                item = format!("  {}\n", item.trim_end_matches("\n").replace("\n", "\n  "));
                context.push_str(&item);
            }
            context.push_str("]\n\n");
        }
        Node::Break(_) => {
            context.push_str("\n");
        }
        Node::Code(node) => {
            context.push_str(&format!(
                "```{}\n",
                node.lang.unwrap_or_else(|| "".to_string())
            ));
            context.push_str(&node.value);
            context.push_str("\n```\n");
        }
        Node::Delete(node) => {
            context.push_str("#strike[");
            for child in node.children {
                context.push_str(&ast_parse(child));
            }
            context.push_str("]");
        }
        Node::Emphasis(node) => {
            context.push_str("#emph[");
            for child in node.children {
                context.push_str(&ast_parse(child));
            }
            context.push_str("]");
        }
        Node::FootnoteDefinition(_) => {
            // do nothing
        }
        Node::FootnoteReference(node) => {
            let id = node.identifier.as_str();
            if let Some(link) = FOOTNOTE_DEFINITIONS.read().unwrap().to_owned().get(id) {
                context.push_str(&format!("#link(\"{}\")[^{}]", link, id))
            } else {
                context.push_str(&format!("[^{}]", id))
            }
        }
        Node::Heading(node) => {
            let mut item = String::new();
            context.push_str(&format!("{} ", "=".repeat(node.depth as usize)));
            for child in node.children {
                item = ast_parse(child);
            }
            context.push_str(&item);
            context.push_str("\n\n");
        }
        Node::Html(node) => {
            context.push_str(&typ::escape_content(html_to_typst(&node.value)));
        }
        Node::Image(node) => match Url::parse(&node.url) {
            Ok(url) => match url.scheme() {
                "http" | "https" => {
                    context.push_str(&format!("#image(\"{}\")", download_image(url)))
                }
                _ => {
                    let name = node.url.strip_prefix("attachment:").unwrap();
                    let a = ATTACHMENTS.read().unwrap();
                    if let Some(file_path) = a.to_owned().get(name) {
                        context.push_str(&format!("#image(\"{}\")", file_path))
                    };
                }
            },
            Err(_) => {
                let name = node.url.strip_prefix("attachment:").unwrap();
                if let Some(file_path) = ATTACHMENTS.read().unwrap().to_owned().get(name) {
                    context.push_str(&format!("#image(\"{}\")", file_path))
                };
            }
        },
        Node::InlineCode(node) => {
            context.push_str(&format!("`{}`", node.value));
        }
        Node::InlineMath(node) => {
            // println!("{}\n", node.value);
            context.push_str(&format!("${}$", katex::latex_to_typst(node.value)));
        }
        Node::Link(node) => {
            context.push_str(&format!("#link(\"{}\")[", node.url));
            for child in node.children {
                context.push_str(&ast_parse(child));
            }
            context.push_str("]");
        }
        Node::List(node) => {
            for child in node.children {
                context.push_str(if node.ordered { "+ " } else { "- " });
                let mut item = ast_parse(child);
                item = item.trim_end_matches("\n").replace("\n", "\n  ") + "\n";
                context.push_str(&item);
                if node.spread {
                    context.push_str("\n");
                }
            }
            context.push_str("\n");
        }
        Node::ListItem(node) => {
            for child in node.children {
                context.push_str(&ast_parse(child));
            }
        }
        Node::Math(node) => {
            // println!("{}\n", node.value);
            context.push_str(&format!("$ {} $", katex::latex_to_typst(node.value)))
        }
        Node::Paragraph(node) => {
            for child in node.children {
                context.push_str(&ast_parse(child));
            }
            context.push_str("\n");
        }
        Node::Root(node) => {
            for child in node.children {
                context.push_str(&ast_parse(child));
            }
        }
        Node::Strong(node) => {
            context.push_str("*");
            for child in node.children {
                context.push_str(&ast_parse(child));
            }
            context.push_str("*");
        }
        Node::Table(node) => {
            context.push_str("#table(\n");
            context.push_str(&format!("  columns: {},\n", node.align.len()));
            context.push_str(&format!(
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
            ));
            for child in node.children {
                context.push_str(&ast_parse(child));
            }
            context.push_str(")\n\n");
        }
        Node::TableCell(node) => {
            context.push_str("[");
            for child in node.children {
                context.push_str(&ast_parse(child));
            }
            context.push_str("], ");
        }
        Node::TableRow(node) => {
            context.push_str("  ");
            for child in node.children {
                context.push_str(&ast_parse(child));
            }
            context.pop();
            context.push_str("\n");
        }
        Node::Text(node) => {
            context.push_str(&typ::escape_content(node.value));
        }
        Node::ThematicBreak(_) => {
            context.push_str("#line(length: 100%)\n");
        }
        _ => {
            println!("unhandled node: {:?}", node);
        }
    }
    return context;
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
                context.push_str(&footnote_def_parse(child));
            }
        }
        Node::Text(node) => context.push_str(&node.value),
        Node::Link(node) => context.push_str(&node.url),
        _ => println!("footnote_def_parse unhandled node: {:?}\n", node),
    }
    context
}

pub fn sha1(s: &str) -> String {
    let mut sha1 = Sha1::new();
    sha1.update(s);
    sha1.finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

fn download_image(url: Url) -> String {
    println!("Downloading image from {}", url);
    let response = match blocking::get(url.as_str()) {
        Ok(r) => r,
        Err(_) => {
            println!("download image failed: {}", url);
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
                println!("not supported image format: {}", c);
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
                        println!("not supported image format: {:?}", format);
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
}
