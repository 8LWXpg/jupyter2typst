use regex::Regex;

struct match_pattern {
    heading: Regex,
    ordered_list: Regex,
    bold_italic: Regex,
    bold: Regex,
    italic: Regex,
    code: Regex,
    link: Regex,
    image: Regex,
    line: Regex,
}

impl match_pattern {
    fn new() -> Self {
        Self {
            heading: Regex::new(r"^#{1,6} ").unwrap(),
            ordered_list: Regex::new(r"^(\d+\. )").unwrap(),
            bold_italic: (),
            bold: Regex::new(r"\*\*").unwrap(),
            italic: (),
            code: (),
            link: (),
            image: (),
            line: (),
        }
    }
}

pub fn md_to_typst(md: Vec<&str>) -> String {
    let mut ret = String::new();
    // TODO convert basic markdown to typst in test4
    // TODO download url image
    // TODO escape latex by `$$`
    // TODO escape html by `<` and `>`
    let mut code_block = false;
    for line in md {
        let converted = match line {
            s if s.starts_with("###### ") => format!("====== {0}\n<{0}>", &s[7..]),
            s if s.starts_with("##### ") => format!("===== {0}\n<{0}>", &s[6..]),
            s if s.starts_with("#### ") => format!("==== {0}\n<{0}>", &s[5..]),
            s if s.starts_with("### ") => format!("=== {0}\n<{0}>", &s[4..]),
            s if s.starts_with("## ") => format!("== {0}\n<{0}>", &s[3..]),
            s if s.starts_with("# ") => format!("= {0}\n<{0}>", &s[2..]),
            s if s.starts_with("```") => format!("```{0}", &s[3..]),
            // TODO make blockquote function in typst
            s if s.starts_with("> ") => format!("#block_quote[\n{0}\n]", &s[2..]),
            s if s.starts_with("- ") => s.to_string(),
            _ => line.to_string(),
        };
    }

    ret
}
