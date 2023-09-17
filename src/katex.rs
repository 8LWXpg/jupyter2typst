// use image::Rgb;

// #[derive(Debug)]
// enum Node {
//     Begin(Begin),
//     Box(Box),
//     Def(Def),
//     Large(Large),
//     Overlap(Overlap),
//     Text(Text),
//     MultiParam(MultiParam),
// }

// /// \begin
// #[derive(Debug)]
// struct Begin {
//     pub children: Vec<Node>,
//     pub kind: String,
// }

// /// \colorbox or \fcolorbox
// #[derive(Debug)]
// struct Box {
//     pub children: Vec<Node>,
//     pub foreground: Option<Rgb<u8>>,
//     pub background: Option<Rgb<u8>>,
// }

// /// \color or \textcolor
// #[derive(Debug)]
// struct Color {
//     pub children: Vec<Node>,
//     pub color: Rgb<u8>,
// }

// #[derive(Debug)]
// struct Def {
//     pub name: String,
//     pub value: String,
// }

// #[derive(Debug)]
// struct Large {
//     pub children: Vec<Node>,
// }

// #[derive(Debug)]
// struct Overlap {
//     pub children: Vec<Node>,
// }

// #[derive(Debug)]
// struct Text {
//     pub value: String,
// }

// #[derive(Debug)]
// struct MultiParam {
//     pub params: Vec<Node>,
//     pub children: Vec<Node>,
// }

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
    pub fn next_word(&mut self) -> String {
        // TODO decide skipping non-alphabetic characters before starting to collect a word or not
        let mut ret = String::new();
        while let Some(c) = self.peek() {
            if !c.is_ascii_alphabetic() {
                break;
            }
            self.cursor += 1;
            ret.push(c);
        }
        ret
    }

    /// Returns the next LaTeX parameter in the scanner.
    pub fn next_param(&mut self) -> Option<String> {
        let mut ret = String::new();

        // trim whitespace
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.cursor += 1;
        }

        // check if next character is '\\', '{', or any other character
        match self.next() {
            Some('\\') => {
                ret.push('\\');
                match self.next_word().as_str() {
                    "" => ret.push(self.next().unwrap()),
                    word => ret.push_str(&word),
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

// pub fn latex_to_typst(latex: String) -> String {
//     // TODO latex to typst
//     let mut typ = String::from("$\n");

//     let ast = latex_to_ast(latex);
//     for node in ast {
//         typ.push_str(&ast_to_typst(node));
//     }

//     typ.push_str("\n$\n");
//     typ
// }

pub fn latex_to_typst(latex: String) -> String {
    let mut scanner = Scanner::new(latex);
    let mut text = String::new();
    while let Some(c) = scanner.next() {
        match c {
            '\\' => {
                // TODO giant match of all LaTeX commands
                match scanner.next_word().as_str() {
                    // same one goes to default
                    "" => {
                        let c = scanner.next().unwrap();
                        match c {
                            '\'' | '"' | '.' | '`' | '=' | '~' | '^' => {
                                let func = match c {
                                    '\'' => "acute",
                                    '"' => "dot.double",
                                    '.' => "dot",
                                    '`' => "grave",
                                    '=' => "macron",
                                    '~' => "tilde",
                                    '^' => "hat",
                                    _ => unreachable!(),
                                };
                                text.push_str(&format!("{}(", func));
                                text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                                text.push(')');
                            }
                            // escape characters in Typst
                            '_' | '&' | '#' => {
                                text.push('\\');
                                text.push(c);
                            }
                            '!' => text.push_str("#h(-1em/6)"),
                            ' ' => text.push_str("space"),
                            '(' | ')' => {}
                            ',' => text.push_str("space.sixth"),
                            ':' | '>' => text.push_str("#h(2em/9)"),
                            ';' => text.push_str("#h(5em/18)"),
                            '|' => text.push_str("||"),
                            '\\' => text.push_str("\\"),
                            _ => text.push(c),
                        }
                    }
                    "AA" => text.push_str("circle(A)"),
                    "aa" => text.push_str("circle(a)"),
                    "acute" => {
                        text.push_str("acute(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "AE" => text.push('Æ'),
                    "ae" => text.push('æ'),
                    "alefsym" => text.push_str("alef"),
                    "amalg" => text.push_str("product.co"),
                    "And" => text.push_str("\\&"),
                    "approxeq" => text.push_str("approx.eq"),
                    "approxcolon" => text.push_str("approx colon"),
                    "approxcoloncolon" => text.push_str("approx colon colon"),
                    "arcctg" => text.push_str("#math.op(\"arcctg\")"),
                    "arctg" => text.push_str("#math.op(\"arctg\")"),
                    "argmax" => text.push_str("arg max"),
                    "argmin" => text.push_str("arg min"),
                    "ast" => text.push('*'),
                    "asymp" => text.push('≍'),
                    "backepsilon" => text.push_str("in.rev.small"),
                    "backprime" => text.push_str("prime.rev"),
                    "backsim" => text.push_str("tilde.rev"),
                    "backsimeq" => text.push_str("tilde.eq.rev"),
                    "backslash" => text.push_str("\\\\"),
                    "bar" => {
                        text.push_str("macron(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "barwedge" => text.push('⊼'),
                    "Bbb" => {
                        text.push_str("bb(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "Bbbk" => text.push_str("bb(k)"),
                    "bcancel" => {
                        text.push_str("cancel(inverted: #true, ");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "begin" => {
                        // TODO
                    }
                    "between" => text.push('≬'),
                    "bf" => {
                        // TODO
                    }
                    "bigcap" => text.push_str("sect.big"),
                    "bigcirc" => text.push_str("circle.stroked.big"),
                    "bigcup" => text.push_str("union.big"),
                    "bigdot" => text.push_str("dot.circle.big"),
                    "bigoplus" => text.push_str("plus.circle.big"),
                    "bigotimes" => text.push_str("times.circle.big"),
                    "bigsqcup" => text.push_str("union.sq.big"),
                    "bigstar" => text.push_str("star.stroked"),
                    "bigtriangledown" => text.push_str("triangle.stroked.b"),
                    "bigtriangleup" => text.push_str("triangle.stroked.t"),
                    "biguplus" => text.push_str("union.plus.big"),
                    "bigvee" => text.push_str("or.big"),
                    "bigwedge" => text.push_str("and.big"),
                    "binom" => {
                        text.push_str("binom(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push_str(", ");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "blacklozenge" => text.push_str("lozenge.filled"),
                    "blacksquare" => text.push_str("square.filled"),
                    "blacktriangle" => text.push_str("triangle.filled.t"),
                    "blacktriangledown" => text.push_str("triangle.filled.b"),
                    "blacktriangleleft" => text.push_str("triangle.filled.l"),
                    "blacktriangleright" => text.push_str("triangle.filled.r"),
                    "bm" | "bold" | "boldsymbol" => {
                        // TODO seperate bold
                        text.push_str("bold(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "bmod" => text.push_str("mod"),
                    "bowtie" => text.push('⋈'),
                    "Box" => text.push_str("square.stroked"),
                    "boxdot" => text.push_str("dot.square"),
                    "boxed" => {
                        text.push_str("#box[$");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push_str("$]");
                    }
                    "boxminus" => text.push_str("minus.square"),
                    "boxplus" => text.push_str("plus.square"),
                    "boxtimes" => text.push_str("times.square"),
                    "breve" => {
                        text.push_str("breve(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "bull" | "bullet" => text.push_str("circle.filled.small"),
                    "Bumpeq" => text.push('≎'),
                    "bumpeq" => text.push('≏'),
                    word => text.push_str(word),
                }
            }
            '%' => text.push_str("//"),
            '~' => text.push_str("space.nobreak"),
            '/' | '"' => text.push_str(&format!("\\{}", c)),
            _ => {
                // TODO one to one mapping of LaTeX characters to Typst characters
                let c = match c {
                    '{' => '(',
                    '}' => ')',
                    _ => c,
                };
                // insert space if current and next character is a alphabetic character
                if let Some(prev) = text.chars().last() {
                    if prev.is_alphabetic() && (c.is_alphabetic() || c.is_ascii_digit()) {
                        text.push(' ');
                    }
                }
                text.push(c);
            }
        }
    }

    text
}

// fn ast_to_typst(node: Node) -> String {
//     // TODO ast to typst
//     let mut typ = String::new();

//     match node {
//         Node::Text(text) => typ.push_str(&text.value),
//         _ => {}
//     }

//     typ
// }

#[cfg(test)]
mod scanner_tests {
    use super::*;

    #[test]
    fn next_word_test() {
        let mut scanner = Scanner::new("\n\\frac\t\\land=3aa".to_string());
        while let Some(c) = scanner.next() {
            match c {
                '\\' => {
                    let word = scanner.next_word();
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
        println!("{:?}", latex_to_typst("N = N_oe^{ln2(t/t_2)}".to_string()));
    }

    #[test]
    fn test_parse_typ2() {
        println!("{:?}", latex_to_typst("\\^a".to_string()));
    }

    #[test]
    fn test_parse_typ3() {
        println!("{:?}", latex_to_typst("\\binom {asdf} {aas}".to_string()));
    }
}
