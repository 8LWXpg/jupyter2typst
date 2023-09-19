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
                // pick up '\operatorname*' specifically
                if ret == "operatorname" && c == '*' {
                    ret.push(c);
                    self.cursor += 1;
                }
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
                let word = scanner.next_word();
                match word.as_str() {
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
                    "bowtie" | "Join" => text.push('⋈'),
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
                        text.push_str(")[$upright(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push_str(")$]");
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
                    "dotso" | "ldots" | "mathellipsis" => text.push_str("..."),
                    "doublebarwedge" => text.push('⩞'),
                    "downdownarrows" => text.push_str("arrows.bb"),
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
                        text.push_str("#box[$upright(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push_str(")$]");
                    }
                    "fcolorbox" => {
                        // expect text input
                        text.push_str("#box(stroke: ");
                        text.push_str(&latex_color_to_typst(scanner.next_param().unwrap()));
                        text.push_str(", fill: ");
                        text.push_str(&latex_color_to_typst(scanner.next_param().unwrap()));
                        text.push_str(")[$upright(");
                        text.push_str(&scanner.next_param().unwrap());
                        text.push_str(")$]");
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
                    "gets" | "larr" | "leftarrow" => text.push_str("<-"),
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
                    // H
                    "H" => {
                        text.push_str("acute.double(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "Harr" | "hArr" | "Leftrightarrow" | "Lrarr" | "lrArr" => text.push_str("<=>"),
                    "harr" | "leftrightarrow" | "lrarr" => text.push_str("<->"),
                    "hat" => {
                        text.push_str("hat(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "hbar" | "hslash" => text.push_str("planck.reduce"),
                    "hearts" | "heartsuit" => text.push('♡'),
                    "hookleftarrow" => text.push_str("arrow.l.hook"),
                    "hookrightarrow" => text.push_str("arrow.r.hook"),
                    // I
                    "i" | "imath" => text.push_str("dotless.i"),
                    "iff" | "Longleftrightarrow" => text.push_str("<==>"),
                    "iiint" => text.push_str("integral.triple"),
                    "iint" => text.push_str("integral.double"),
                    "image" => text.push_str("Im"),
                    "impliedby" | "Longleftarrow" => text.push_str("<=="),
                    "implies" => text.push_str("==>"),
                    "infin" | "infty" => text.push_str("infinity"),
                    "injlim" => text.push_str("#math.op(\"inj\u{2009}lim\", limits: true)"),
                    "int" | "intop" => text.push_str("integral"),
                    "intercal" => text.push('⊺'),
                    "isin" => text.push_str("in"),
                    // JK
                    "j" | "jmath" => text.push_str("dotless.j"),
                    "KaTeX" | "LaTeX" => text.push_str(&format!("\"{}\"", word)),
                    // L
                    "lang" | "langle" => text.push('⟨'),
                    "Larr" | "lArr" | "Leftarrow" => text.push_str("arrow.l.double"),
                    "lBrace" => text.push('⦃'),
                    "lbrace" => text.push('{'),
                    "lbrack" => text.push('['),
                    "lceil" => text.push('⌈'),
                    "ldotp" => text.push('.'),
                    "le" | "leq" => text.push_str("<="),
                    "leadsto" => text.push_str("arrow.r.squiggly"),
                    "leftarrowtail" => text.push_str("<-<"),
                    "leftharpoondown" => text.push_str("harpoon.lb"),
                    "leftharpoonup" => text.push_str("harpoon.lt"),
                    "leftleftarrows" => text.push_str("arrows.ll"),
                    "leftrightarrows" => text.push_str("arrows.lr"),
                    "leftrightharpoons" => text.push_str("harpoons.ltrb"),
                    "leftrightsquigarrow" => text.push_str("arrow.l.r.wave"),
                    "leftthreetimes" => text.push_str("times.three.l"),
                    "leqq" => text.push_str("lt.equiv"),
                    "leqslant" => text.push_str("lt.eq.slant"),
                    "lessapprox" => text.push('⪅'),
                    "lessdot" => text.push_str("lt.dot"),
                    "lesseqgtr" => text.push_str("lt.eq.gt"),
                    "lesseqqgtr" => text.push('⪋'),
                    "lessgtr" => text.push_str("lt.gt"),
                    "lesssim" => text.push_str("lt.tilde"),
                    "lfloor" => text.push('⌊'),
                    "lgroup" => text.push('⟮'),
                    "lhd" => text.push_str("ld.tri"),
                    "ll" => text.push_str("<<"),
                    "llbracket" => text.push_str("bracket.l.double"),
                    "llcorner" => text.push('⌞'),
                    "Lleftarrow" => text.push_str("arrow.l.triple"),
                    "lll" | "llless" => text.push_str("<<<"),
                    "lnapprox" => text.push('⪉'),
                    "lneq" => text.push('⪇'),
                    "lneqq" => text.push_str("lt.nequiv"),
                    "lnot" => text.push_str("not"),
                    "lnsim" => text.push_str("lt.ntilde"),
                    "longleftarrow" => text.push_str("<--"),
                    "longleftrightarrow" => text.push_str("<-->"),
                    "longmapsto" => text.push_str("arrow.r.long.bar"),
                    "Longrightarrow" => text.push_str("==>"),
                    "longrightarrow" => text.push_str("-->"),
                    "looparrowleft" => text.push_str("arrow.l.loop"),
                    "looparrowright" => text.push_str("arrow.r.loop"),
                    "lor" => text.push_str("or"),
                    "lozenge" => text.push_str("lozenge.stroked"),
                    "lparen" => text.push('('),
                    "lrcorner" => text.push('⌟'),
                    "lq" => text.push_str("quote.l.single"),
                    "Lsh" => text.push('↰'),
                    "lt" => text.push('<'),
                    "ltimes" => text.push_str("times.l"),
                    "lVert" => text.push_str("parallel"),
                    "lvert" => text.push_str("divides"),
                    // M
                    "mapsto" => text.push_str("arrow.r.bar"),
                    "mathbb" => {
                        text.push_str("bb(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "mathbf" => {
                        text.push_str("bold(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "mathcal" => {
                        text.push_str("cal(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "mathfrak" => {
                        text.push_str("frak(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "mathit" => {
                        text.push_str("italic(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "mathnormal" | "mathop" => {
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                    }
                    "mathring" => {
                        text.push_str("circle(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "mathrm" => {
                        text.push_str("upright(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "mathsf" => {
                        text.push_str("sans(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "mathsterling" => text.push_str("pound"),
                    "measuredangle" => text.push_str("angle.arc"),
                    "medspace" => text.push_str("#h(2em/9)"),
                    "mho" => text.push_str("ohm.inv"),
                    "mid" => text.push('|'),
                    "minuscolon" => text.push_str("\"-:\""),
                    "minuscoloncolon" => text.push_str("\"-::\""),
                    "minuso" => text.push('⊖'),
                    "models" => text.push_str("tack.r.double"),
                    "mp" => text.push_str("minus.plus"),
                    // N
                    "N" | "natnums" => text.push_str("NN"),
                    "natural" => text.push('♮'),
                    "negmedspace" => text.push_str("#h(-2em/9)"),
                    "ncong" => text.push_str("tilde.equiv.not"),
                    "ne" | "neq" => text.push_str("!="),
                    "nearrow" => text.push_str("arrow.tr"),
                    "neg" => text.push_str("not"),
                    "negthickspace" => text.push_str("#h(-5em/18)"),
                    "negthinspace" => text.push_str("#h(-1em/6)"),
                    "nexist" => text.push_str("exists.not"),
                    "ngeq" => text.push_str("gt.eq.not"),
                    "ngtr" => text.push_str("gt.not"),
                    "ni" | "owns" => text.push_str("in.rev"),
                    "nLeftarrow" => text.push_str("arrow.l.double.not"),
                    "nleftarrow" => text.push_str("arrow.l.not"),
                    "nLeftrightarrow" => text.push_str("arrow.l.r.double.not"),
                    "nleftrightarrow" => text.push_str("arrow.l.r.not"),
                    "nleq" => text.push_str("lt.eq.not"),
                    "nless" => text.push_str("lt.not"),
                    "nmid" => text.push_str("divides.not"),
                    "nobreakspace" => text.push_str("space.nobreak"),
                    "notin" => text.push_str("in.not"),
                    "notni" => text.push_str("in.rev.not"),
                    "notparallel" => text.push_str("parallel.not"),
                    "nprec" => text.push_str("prec.not"),
                    "npreceq" => text.push_str("prec.eq.not"),
                    "nRightarrow" => text.push_str("arrow.r.double.not"),
                    "nrightarrow" => text.push_str("arrow.r.not"),
                    "nsim" => text.push_str("tilde.not"),
                    "nsubseteq" | "nsupseteq" => text.push_str("subset.eq.not"),
                    "nsucc" => text.push_str("succ.not"),
                    "nsucceq" => text.push_str("succ.eq.not"),
                    "ntriangleleft" => text.push_str("lt.tri.not"),
                    "ntrianglelefteq" => text.push_str("lt.tri.eq.not"),
                    "ntriangleright" => text.push_str("gt.tri.not"),
                    "ntrianglerighteq" => text.push_str("gt.tri.eq.not"),
                    "nVDash" => text.push('⊯'),
                    "nVdash" => text.push('⊮'),
                    "nvDash" => text.push_str("tack.r.double.not"),
                    "nvdash" => text.push_str("tack.r.not"),
                    "nwarrow" => text.push_str("arrow.tl"),
                    // O
                    "O" => text.push('Ø'),
                    "o" => text.push('ø'),
                    "odot" => text.push_str("dot.circle"),
                    "OE" => text.push('Œ'),
                    "oe" => text.push('œ'),
                    "oiiint" => text.push_str("integral.vol"),
                    "oiint" => text.push_str("integral.surf"),
                    "oint" => text.push_str("integral.cont"),
                    "ominus" => text.push_str("minus.circle"),
                    "operatorname" => {
                        text.push_str("#math.op(\"");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push_str("\")");
                    }
                    "operatorname*" | "operatornamewithlimits" => {
                        text.push_str("#math.op(\"");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push_str("\", limits: true)");
                    }
                    "oplus" => text.push_str("plus.circle"),
                    "origof" => text.push('⊶'),
                    "oslash" => text.push('⊘'),
                    "otimes" => text.push_str("times.circle"),
                    "overbrace" => {
                        text.push_str("overbrace(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push_str(", ");
                        // except '^' here
                        if scanner.next().is_some_and(|c| c != '^') {
                            panic!("expected '^' after $1 in overbrace");
                        }
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "overgroup" => {
                        text.push_str("accent(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push_str(", turtle.t)");
                    }
                    "overleftarrow" => {
                        text.push_str("arrow.l(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "overline" => {
                        text.push_str("overline(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
                    "overrightarrow" => {
                        text.push_str("arrow.r(");
                        text.push_str(&latex_to_typst(scanner.next_param().unwrap()));
                        text.push(')');
                    }
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

    #[test]
    fn test_parse_typ4() {
        println!(
            "{:?}",
            latex_to_typst("\\overbrace{x+⋯+x}^{n\\text{ times}}".to_string())
        );
    }
}
