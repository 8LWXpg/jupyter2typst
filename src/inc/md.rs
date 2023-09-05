use markdown::{mdast::Node, to_mdast, ParseOptions};
use sha1::{Digest, Sha1};
use std::{fs::File, io::Write};
use url::Url;

pub fn md_to_typst(md: Vec<&str>, img_path: &str) -> String {
    // TODO https://github.com/wooorm/markdown-rs
    let tree = to_mdast(&md.join(""), &ParseOptions::gfm()).unwrap();

    // write tree to debug file
    let mut file = File::create("debug.txt").unwrap();
    file.write_all(format!("{:#?}", tree).as_bytes()).unwrap();

    ast_parse(tree, img_path)
}

fn ast_parse(node: Node, img_path: &str) -> String {
    let mut context = String::new();
    match node {
        Node::BlockQuote(node) => {
            context.push_str("#block_quote[\n");
            for child in node.children {
                context.push_str(&ast_parse(child, img_path));
            }
            context.push_str("]\n");
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
        Node::Definition(_) => {
            // TODO wtf is definition
        }
        Node::Delete(node) => {
            context.push_str("#strike[");
            for child in node.children {
                context.push_str(&ast_parse(child, img_path));
            }
            context.push_str("]");
        }
        Node::Emphasis(node) => {
            context.push_str("#emph[");
            for child in node.children {
                context.push_str(&ast_parse(child, img_path));
            }
            context.push_str("]");
        }
        Node::FootnoteDefinition(_) => {
            // TODO footnote
        }
        Node::FootnoteReference(_) => {
            // TODO footnote
        }
        Node::Heading(node) => {
            context.push_str(&format!("{} ", "=".repeat(node.depth as usize)));
            for child in node.children {
                context.push_str(&ast_parse(child, img_path));
            }
            context.push_str("\n")
        }
        Node::Html(node) => {
            context.push_str(&html_to_typst(&node.value));
        }
        Node::Image(node) => match Url::parse(&node.url) {
            Ok(url) => {
                context.push_str(&format!("#image({})", download_image(url, img_path)));
            }
            Err(_) => {
                context.push_str(&format!("#image({})", node.url));
            }
        },
        Node::InlineCode(node) => {
            context.push_str(&format!("`{}`", node.value));
        }
        Node::InlineMath(node) => {
            context.push_str(&latex_to_typst(&node.value));
        }
        Node::Link(node) => {
            context.push_str("#link[");
            for child in node.children {
                context.push_str(&ast_parse(child, img_path));
            }
            context.push_str("]");
        }
        Node::List(node) => {
            // TODO List
        }
        Node::ListItem(node) => {
            // TODO List
        }
        Node::Math(node) => context.push_str(&latex_to_typst(&node.value)),
        Node::Paragraph(node) => {
            for child in node.children {
                context.push_str(&ast_parse(child, img_path));
            }
            context.push_str("\n");
        }
        Node::Root(node) => {
            for child in node.children {
                context.push_str(&ast_parse(child, img_path));
            }
        }
        Node::Strong(node) => {
            context.push_str("*");
            for child in node.children {
                context.push_str(&ast_parse(child, img_path));
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
                context.push_str(&ast_parse(child, img_path));
            }
            context.push_str(")\n");
        }
        Node::TableCell(node) => {
            context.push_str("  [");
            for child in node.children {
                context.push_str(&ast_parse(child, img_path));
            }
            context.push_str("],\n");
        }
        Node::TableRow(node) => {
            for child in node.children {
                context.push_str(&ast_parse(child, img_path));
            }
        }
        Node::Text(node) => {
            context.push_str(&escape(&node.value));
        }
        _ => {}
    }
    return context;
}

pub fn sha1(s: &str) -> String {
    let mut sha1 = Sha1::new();
    sha1.update(s);
    sha1.finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

fn download_image(url: Url, img_path: &str) -> String {
    // TODO download image
    let mut path = img_path.to_string();
    path.push_str(&sha1(url.as_str()));
    path.push_str(".png");
    path
}

fn escape(s: &str) -> String {
    // TODO escape special characters
    s.to_string()
}

fn html_to_typst(html: &str) -> String {
    // TODO html to typst
    html.to_string()
}

fn latex_to_typst(latex: &str) -> String {
    // TODO latex to typst
    latex.to_string()
}
