use std::{fs::File, io::Write};

use markdown::{to_mdast, ParseOptions};

pub fn md_to_typst(md: Vec<&str>) -> String {
    // TODO https://github.com/wooorm/markdown-rs
    let tree = to_mdast(md.join("").as_str(), &ParseOptions::default()).unwrap();
    // write tree to debug file
    let mut file = File::create("debug.txt").unwrap();
    file.write_all(format!("{:#?}", tree).as_bytes()).unwrap();

    String::from("TODO")
}
