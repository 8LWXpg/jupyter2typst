use std::{fs::File, io::Write};

use markdown::{mdast::Node, to_mdast, ParseOptions};

pub fn md_to_typst(md: Vec<&str>) -> String {
    // TODO https://github.com/wooorm/markdown-rs
    let tree = to_mdast(md.join("").as_str(), &ParseOptions::gfm()).unwrap();

    // write tree to debug file
    let mut file = File::create("debug.txt").unwrap();
    file.write_all(format!("{:#?}", tree).as_bytes()).unwrap();

    return ast_parse(tree);
}

fn ast_parse(node: Node) -> String {
    let mut context = String::new();
    match node {
        Node::BlockQuote(node) => {
            context.push_str("#block_quote[\n");
            for child in node.children {
                context.push_str(&ast_parse(child));
            }
        }
        Node::Break(_) => {
            context.push_str("\n");
        }
        Node::Code(node) => {
            context.push_str(&format!(
                "```{}\n",
                node.lang.unwrap_or_else(|| "".to_string())
            ));
            context.push_str(node.value.as_str());
            context.push_str("\n```\n");
        }
        Node::Definition(_) => {
            // TODO knowing wtf is definition
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
        _ => panic!("Unknown node type"),
    }
    return context;
}
