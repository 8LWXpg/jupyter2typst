use image::Rgb;

#[derive(Debug)]
enum Node {
    Begin(Begin),
    Box(Box),
    Color(Color),
    Def(Def),
    Large(Large),
    Overlap(Overlap),
    Text(Text),
    MultiParam(MultiParam),
    Root(Root),
}

#[derive(Debug)]
struct Begin {
    pub children: Vec<Node>,
    pub kind: String,
}

#[derive(Debug)]
struct Box {
    pub children: Vec<Node>,
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

#[derive(Debug)]
struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug)]
struct Def {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
struct Large {
    pub children: Vec<Node>,
}

#[derive(Debug)]
struct Overlap {
    pub children: Vec<Node>,
}

#[derive(Debug)]
struct Text {
    pub text: String,
}

#[derive(Debug)]
struct MultiParam {
    pub children: Vec<Node>,
}

#[derive(Debug)]
struct Root {
    pub children: Vec<Node>,
}

pub fn latex_to_typst(latex: String) -> String {
    // TODO latex to typst
    let mut typ = String::from("$\n");

    let ast = latex_to_ast(latex);
    for node in ast {
        typ.push_str(&ast_to_typst(node));
    }

    typ.push_str("\n$\n");
    typ
}

fn latex_to_ast(latex: String) -> Vec<Node> {
    const SYMBOLS: &[char] = &[
        '\'', '(', ')', '[', ']', '\\', '^', '_', '|', '&', '*', '+', '-', '/', ':', '<', '=', '>',
        '⟨', '⟩', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    ];
    const ESCAPE: &[char] = &['{', '}', ' ', '\\'];
    let mut ast = Vec::new();

    let mut text = String::new();
    let mut cur = String::new();
    let mut collect = false;
    let mut last = '0';
    for c in latex.chars() {
        match c {
            '\\' => {
                if collect {
                    collect = false;
                    text.push_str("\\");
                } else {
                    collect = true;
                    ast.push(Node::Text(Text { text: text }));
                    text = String::new();
                }
            }
            ' ' => {
                if collect {
                    collect = false;
                    // TODO giant list of commands matching
                    match cur {
                        _ => ast.push(Node::Text(Text { text: cur })),
                    }
                    cur = String::new();
                } else {
                    text.push(' ');
                }
            }
            '{' => {
                if collect {
                    collect = false;
                    // TODO giant list of commands matching
                    match cur {
                        _ => ast.push(Node::Text(Text { text: cur })),
                    }
                    cur = String::new();
                } else {
                    text.push('(');
                }
            }
            '}' => {
                if collect {
                    text.push(c);
                } else {
                    text.push(')');
                }
            }
            '0'..='9' => {
                if collect {
                    cur.push(c);
                } else {
                    if !SYMBOLS.contains(&last) && !ESCAPE.contains(&last) {
                        text.push(' ');
                    }
                    text.push(c);
                }
            }
            c if SYMBOLS.contains(&c) => {
                if collect {
                    cur.push(c)
                } else {
                    text.push(c)
                }
            }
            _ => {
                if collect {
                    cur.push(c);
                } else {
                    if !SYMBOLS.contains(&last) && !ESCAPE.contains(&last) {
                        text.push(' ');
                    }
                    text.push(c);
                }
            }
        }
        last = c;
    }
    if text.len() > 0 {
        ast.push(Node::Text(Text { text: text }));
    }

    ast
}

fn ast_to_typst(node: Node) -> String {
    let mut typ = String::new();

    match node {
        Node::Text(text) => typ.push_str(&text.text),
        _ => {}
    }

    typ
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_typ() {
        println!("{:?}", latex_to_ast("N = N_oe^{ln2(t/t_2)}".to_string()));
    }
}
