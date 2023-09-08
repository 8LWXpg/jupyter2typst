use markdown::{mdast::Node, to_mdast, ParseOptions};
use reqwest::blocking;
use sha1::{Digest, Sha1};
use std::{collections::HashMap, fs::File, io::Write};
use url::Url;

use crate::IMG_PATH;

use once_cell::sync::OnceCell;

static FOOTNOTE_DEFINITIONS: OnceCell<HashMap<String, String>> = OnceCell::new();

pub fn md_to_typst(md: Vec<&str>) -> String {
    let tree = to_mdast(&md.join(""), &ParseOptions::gfm()).unwrap();

    // write tree to debug file
    let mut file = File::create("debug.txt").unwrap();
    file.write_all(format!("{:#?}", tree).as_bytes()).unwrap();

    FOOTNOTE_DEFINITIONS
        .set(footnote_grep(tree.clone()))
        .unwrap();
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
            if let Some(link) = FOOTNOTE_DEFINITIONS.get().unwrap().get(id) {
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
            context.push_str(&format!(
                "\n<{}>\n\n",
                &escape(&item, true).replace(" ", "-").to_lowercase()
            ));
        }
        Node::Html(node) => {
            context.push_str(&html_to_typst(&node.value));
        }
        Node::Image(node) => match Url::parse(&node.url) {
            Ok(url) => {
                context.push_str(&format!("#image(\"{}\")", download_image(url)));
            }
            Err(_) => {
                context.push_str(&format!("#image(\"{}\")", node.url));
            }
        },
        Node::InlineCode(node) => {
            context.push_str(&format!("`{}`", node.value));
        }
        Node::InlineMath(node) => {
            context.push_str(&latex_to_typst(&node.value));
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
            }
            context.push_str("\n");
        }
        Node::ListItem(node) => {
            for child in node.children {
                context.push_str(&ast_parse(child));
            }
            if node.spread {
                println!("spread");
                context.push_str("\n");
            }
        }
        Node::Math(node) => context.push_str(&latex_to_typst(&node.value)),
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
            context.push_str(&escape(&node.value, false));
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
    let img_bytes = blocking::get(url.as_str()).unwrap().bytes().unwrap();
    let path = format!("{}/{}.png", IMG_PATH.get().unwrap(), sha1(url.as_str()));
    let mut file = File::create(&path).unwrap();
    file.write_all(&img_bytes).unwrap();
    path
}

fn escape(s: &str, remove: bool) -> String {
    // https://typst.app/docs/reference/syntax/#markup
    const ESCAPE: &[char] = &[
        '*', '_', '`', '<', '>', '@', '=', '-', '+', '/', '$', '\\', '\'', '"', '~', '#',
    ];

    let mut result = String::new();
    for c in s.chars() {
        if ESCAPE.contains(&c) {
            if remove {
                continue;
            } else {
                result.push('\\');
            }
        }
        result.push(c);
    }
    result
}

pub fn html_to_typst(html: &str) -> String {
    // TODO html to typst
    html.to_string()
}

pub fn latex_to_typst(latex: &str) -> String {
    // TODO latex to typst
    latex.to_string()
}
