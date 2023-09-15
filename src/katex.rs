use image::Rgb;

#[derive(Debug)]
enum Node {
    Begin(Begin),
    Box(Box),
    Def(Def),
    Large(Large),
    Overlap(Overlap),
    Text(Text),
    MultiParam(MultiParam),
}

#[derive(Debug)]
struct Begin {
    pub children: Vec<Node>,
    pub kind: String,
}

#[derive(Debug)]
struct Box {
    pub children: Vec<Node>,
    pub foreground: Option<Rgb<u8>>,
    pub background: Option<Rgb<u8>>,
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
    pub value: String,
    pub color: Option<Rgb<u8>>,
}

#[derive(Debug)]
struct MultiParam {
    pub params: Vec<Node>,
    pub children: Vec<Node>,
}

#[derive(Debug)]
struct Scanner {
    cursor: usize,
    characters: Vec<char>,
}

impl Scanner {
    pub fn new(text: String) -> Self {
        Self {
            cursor: 0,
            characters: text.chars().collect(),
        }
    }

    /// Returns character at the cursor without advancing the cursor.
    pub fn peek(&self) -> Option<char> {
        self.characters.get(self.cursor).copied()
    }

    /// Returns the next word (ascii alphabet only) in the scanner..
    pub fn next_word(&mut self) -> Option<String> {
        // TODO decide skipping non-alphabetic characters before starting to collect a word or not
        let mut ret = String::new();
        while let Some(c) = self.next() {
            if !c.is_ascii_alphabetic() {
                self.cursor -= 1;
                break;
            }
            ret.push(c);
        }
        Some(ret).filter(|s| !s.is_empty())
    }

    /// Returns the next LaTeX parameter in the scanner.
    pub fn next_param(&mut self) -> Option<String> {
        let mut ret = String::new();

        // trim whitespace
        while let Some(c) = self.next() {
            if !c.is_whitespace() {
                self.cursor -= 1;
                break;
            }
        }

        // check if next character is '\\', '{', or any other character
        match self.next() {
            Some('\\') => {
                ret.push('\\');
                match self.next_word() {
                    Some(word) => ret.push_str(&word),
                    None => ret.push(self.next().unwrap()),
                }
            }
            Some('{') => {
                let mut depth = 0;
                while let Some(c) = self.next() {
                    match c {
                        '{' => depth += 1,
                        '}' => {
                            if depth == 0 {
                                break;
                            }
                            depth -= 1;
                        }
                        _ => {}
                    }
                    ret.push(c);
                }
            }
            Some(c) => ret.push(c),
            None => {}
        }
        Some(ret).filter(|s| !s.is_empty())
    }
}

impl Iterator for Scanner {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.characters.get(self.cursor).copied();
        self.cursor += 1;
        item
    }
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
    let mut ast = Vec::new();

    let mut scanner = Scanner::new(latex);
    let mut text = String::new();
    let mut color: Option<Rgb<u8>> = None;
    while let Some(c) = scanner.next() {
        match c {
            '\\' => {
                let word = scanner.next_word().unwrap();
                match word.as_str() {
                    // TODO escape characters
                    "" => match c {
                        '!' => text.push_str("#h(-1em/6)"),
                        '\'' => {
                            text.push_str("acute(");
                        }
                        ' ' => text.push_str("space"),
                        '(' | ')' => {}
                        '"' => text.push_str("dot.double("),
                        ',' => text.push_str("space.sixth "),
                        '.' => text.push_str("dot("),
                        ':' => text.push_str("#h(2em/9)"),
                        ';' => text.push_str("#h(5em/18)"),
                        '`' => text.push_str("grave("),
                        _ => text.push(c),
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }

    /* const COLLECT_END: &[char] = &[
        '!', '#', '%', '&', '\'', '"', ',', '.', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']',
        '^', '_', '`', '{', '|', '}', '~', '⟨', '⟩', ' ', '\n',
    ];
    const ESCAPE: &[char] = &['{', '}', ' ', '\\', '#', '%', '&'];
    // collect end: SYMBOLS, ESCAPE

    let mut cur = String::new();
    let mut collect = false;
    let mut close_param = false;
    let mut last = '0';
    for c in latex.chars() {
        match c {
            c if COLLECT_END.contains(&c) => {
                if collect {
                    // TODO giant match
                    match cur.as_str() {
                    }
                    // TODO push c
                } else {
                    match c {
                        '\\' => collect = true,
                        '0'..='9' => {
                            if !COLLECT_END.contains(&last) {
                                text.push(' ');
                            }
                            text.push(c);
                        }
                        '{' => text.push('('),
                        '}' => text.push(')'),
                        _ => text.push(c),
                    }
                }
            }
            '%' => text.push_str("//"),
            _ => {
                if collect {
                    cur.push(c);
                } else {
                    if !COLLECT_END.contains(&last) {
                        text.push(' ');
                    }
                    text.push(c);
                }
            }
        }
        last = c;
    }
    if text.len() > 0 {
        ast.push(Node::Text(Text {
            value: text,
            color: color,
        }));
    } */

    ast
}

fn ast_to_typst(node: Node) -> String {
    // TODO ast to typst
    let mut typ = String::new();

    match node {
        Node::Text(text) => typ.push_str(&text.value),
        _ => {}
    }

    typ
}

#[cfg(test)]
mod scanner_tests {
    use super::*;

    #[test]
    fn next_word_test() {
        let mut scanner = Scanner::new("\n\\frac\t\\land=3aa".to_string());
        while let Some(c) = scanner.next() {
            match c {
                '\\' => {
                    let word = scanner.next_word().unwrap();
                    println!("{}", word);
                }
                _ => {}
            }
        }
    }

    #[test]
    fn next_param_test() {
        let mut scanner = Scanner::new("\n\t\\land\\%=3aa\\\\".to_string());
        while let Some(c) = scanner.next_param() {
            println!("{}", c);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_typ() {
        println!("{:?}", latex_to_ast("N = N_oe^{ln2(t/t_2)}".to_string()));
    }
}
