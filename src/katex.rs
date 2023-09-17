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
                // TODO next param contains binary operators as well
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
            None => unreachable!(),
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
                    // A
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
                    "amalg" | "coprod" => text.push_str("product.co"),
                    "And" => text.push_str("\\&"),
                    "approxeq" => text.push_str("approx.eq"),
                    "approxcolon" => text.push_str("approx:"),
                    "approxcoloncolon" => text.push_str("approx::"),
                    "arcctg" => text.push_str("#math.op(\"arcctg\")"),
                    "arctg" => text.push_str("#math.op(\"arctg\")"),
                    "argmax" => text.push_str("arg max"),
                    "argmin" => text.push_str("arg min"),
                    "ast" => text.push('*'),
                    "asymp" => text.push('≍'),
                    // B
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
                        text.push_str("#box(stroke: 0.5pt)[$");
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
                    // C
                    "cancel" => {
                        text.push_str("cancel(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "Cap" | "doublecap" => text.push_str("sect.double"),
                    "cap" => text.push_str("sect"),
                    "cdot" | "cdotp" | "centerdot" => text.push_str("dot.op"),
                    "cdots" | "dots" | "dotsb" | "dotsc" | "dotsi" | "dotsm" => {
                        text.push_str("dots.h.c")
                    }
                    "check" => {
                        text.push_str("caron(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "circ" => text.push_str("compose"),
                    "circeq" => text.push('≗'),
                    "circlearrowleft" => text.push_str("arrow.ccw"),
                    "circlearrowright" => text.push_str("arrow.cw"),
                    "circledast" => text.push_str("ast.circle"),
                    "circledcirc" => text.push_str("circle.nested"),
                    "circleddash" => text.push_str("dash.circle"),
                    "circledR" => text.push('®'),
                    "circledS" => text.push('Ⓢ'),
                    "clubs" | "clubsuit" => text.push_str("suit.club"),
                    "cnums" => text.push_str("CC"),
                    "Colonapprox" => text.push_str("::approx"),
                    "colonapprox" => text.push_str(":approx"),
                    "coloncolon" => text.push_str("::"),
                    "coloncolonapprox" => text.push_str("::approx"),
                    "coloncolonequals" | "Coloneqq" => text.push_str("::="),
                    "coloncolonminus" | "Coloneq" => text.push_str("\"::-\""),
                    "coloncolonsim" | "Colonsim" => text.push_str("\"::~\""),
                    "coloneq" | "colonminus" => text.push_str("\":-\""),
                    "colonequals" | "coloneqq" => text.push_str(":="),
                    "colonsim" => text.push_str("\":~\""),
                    "colorbox" => {
                        // expect both text input
                        text.push_str("#text(fill: ");
                        text.push_str(&latex_color_to_typst(scanner.next_param().unwrap()));
                        text.push_str(")[");
                        text.push_str(&scanner.next_param().unwrap());
                        text.push_str("]");
                    }
                    "complexes" => text.push_str("CC"),
                    "cong" => text.push_str("tilde.equiv"),
                    "cosec" => text.push_str("#math.op(\"cosec\")"),
                    "cotg" => text.push_str("#math.op(\"cotg\")"),
                    "cth" => text.push_str("#math.op(\"cth\")"),
                    "Cup" | "doublecup" => text.push_str("union.double"),
                    "cup" => text.push_str("union"),
                    "curlyeqprec" => text.push_str("eq.prec"),
                    "curlyeqsucc" => text.push_str("eq.succ"),
                    "curlyvee" => text.push_str("or.curly"),
                    "curlywedge" => text.push_str("and.curly"),
                    "curvearrowleft" => text.push_str("arrow.ccw.half"),
                    "curvearrowright" => text.push_str("arrow.cw.half"),
                    // D
                    "dag" => text.push_str("dagger"),
                    "Dagger" | "ddag" | "ddagger" => text.push_str("dagger.double"),
                    "daleth" => text.push_str("ℸ"),
                    "Darr" | "dArr" | "Downarrow" => text.push_str("arrow.b.double"),
                    "darr" | "downarrow" => text.push_str("arrow.b"),
                    "dashleftarrow" => text.push_str("arrow.l.dash"),
                    "dashrightarrow" => text.push_str("arrow.r.dash"),
                    "dashv" => text.push_str("tack.l"),
                    "dbinom" => {
                        text.push_str("dbinom(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push_str(", ");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "dbcolon" => text.push_str("::"),
                    "ddot" => {
                        text.push_str("dot.double(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push_str(")");
                    }
                    "ddots" => text.push_str("dots.down"),
                    "digaamma" => text.push('ϝ'),
                    "dfrac" => {
                        text.push_str("frac(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push_str(", ");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "diagdown" => text.push('╲'),
                    "diagup" => text.push('╱'),
                    "Diamond" => text.push_str("lozenge.stroked"),
                    "diamond" => text.push_str("diamond.stroked.small"),
                    "diamonds" | "diamondsuit" => text.push('♢'),
                    "displaystyle" => {
                        text.push_str("display(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "divideontimes" => text.push_str("times.div"),
                    "dot" => {
                        text.push_str("dot(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "Doteq" | "doteqdot" => text.push('≑'),
                    "doteq" => text.push('≐'),
                    "dotplus" => text.push_str("plus.dot"),
                    "dotso" => text.push_str("dots.h"),
                    "doublebarwedge" => text.push('⩞'),
                    "downdownarrows" => text.push_str("arrow.bb"),
                    "downharpoonleft" => text.push_str("harpoon.bl"),
                    "downharpoonright" => text.push_str("harpoon.br"),
                    // E
                    "ell" => text.push_str("cal(l)"),
                    "empty" | "emptyset" => text.push_str("empty"),
                    "enspace" => text.push_str("space.en"),
                    "epsilon" => text.push_str("epsilon.alt"),
                    "eqcirc" => text.push('≖'),
                    "Eqcolon" => text.push_str("\"-::\""),
                    "eqcolon" => text.push_str("\"-:\""),
                    "Eqqcolon" | "equalscoloncolon" => text.push_str("\"=::\""),
                    "eqqcolon" | "equalscolon" => text.push_str("=:"),
                    "eqsim" => text.push_str("eq.tilde"),
                    "eqslantgtr" => text.push('⪖'),
                    "eqslantless" => text.push('⪕'),
                    "eth" => text.push('ð'),
                    "exist" => text.push_str("exists"),
                    // F
                    "fallingdotseq" => text.push('≒'),
                    "fbox" => {
                        // expect text input
                        text.push_str("#box[");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push_str("]");
                    }
                    "fcolorbox" => {
                        // expect text input
                        text.push_str("#box(stroke: ");
                        text.push_str(&latex_color_to_typst(scanner.next_param().unwrap()));
                        text.push_str(", fill: ");
                        text.push_str(&latex_color_to_typst(scanner.next_param().unwrap()));
                        text.push_str(")[");
                        text.push_str(&scanner.next_param().unwrap());
                        text.push_str("]");
                    }
                    "Finv" => text.push('Ⅎ'),
                    "flat" => text.push('♭'),
                    "frac" => {
                        text.push_str("frac(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push_str(", ");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "frak" => {
                        text.push_str("frak(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "frown" => text.push('⌢'),
                    // G
                    "Game" => text.push('⅁'),
                    "ge" | "geq" => text.push_str(">="),
                    "geqq" => text.push_str("ge.equiv"),
                    "geqslant" => text.push_str("gt.eq.slant"),
                    "gets" => text.push_str("arrow.l"),
                    "gg" => text.push_str(">>"),
                    "ggg" | "gggtr" => text.push_str(">>>"),
                    "gnapprox" => text.push('⪊'),
                    "gneq" => text.push('⪈'),
                    "gneqq" => text.push_str("gt.nequiv"),
                    "gnsim" => text.push_str("gt.ntilde"),
                    "grave" => {
                        text.push_str("grave(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "gt" => text.push('>'),
                    "gtapprox" => text.push('⪆'),
                    "gtreqless" => text.push_str("gt.eq.lt"),
                    "gtreqqless" => text.push('⪌'),
                    "gtrless" => text.push_str("gt.lt"),
                    "gtrsim" => text.push_str("gt.tilde"),
                    "gvertneqq" => text.push_str("gt.nequiv"),
                    // H
                    "H" => {
                        text.push_str("acute.double(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "Harr" | "hArr" => text.push_str("<=>"),
                    "harr" => text.push_str("<->"),
                    "hat" => {
                        text.push_str("hat(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "hbar" | "hslash" => text.push_str("planck.reduce"),
                    "hearts" | "heartsuit" => text.push('♡'),
                    "hookleftarrow" => text.push_str("arrow.l.hook"),
                    "hookrightarrow" => text.push_str("arrow.r.hook"),
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

fn latex_color_to_typst(color: String) -> String {
    if color.chars().next().unwrap() == '#' {
        format!("rgb(\"{}\")", color)
    } else {
        color
    }
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

    #[test]
    fn color_test() {
        println!("{}", latex_color_to_typst("#00ff00".to_string()));
        println!("{}", latex_color_to_typst("red".to_string()));
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
