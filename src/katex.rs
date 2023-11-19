use regex::Regex;
#[path = "typ.rs"]
mod typ;

#[derive(Debug, Clone)]
struct Scanner {
    cursor: usize,
    characters: Vec<char>,
}

#[derive(Debug)]
pub struct ScannerError {
    message: String,
    cursor: usize,
    characters: String,
}

impl ScannerError {
    fn new(message: String, scanner: Scanner) -> Self {
        Self {
            message,
            cursor: scanner.cursor,
            characters: scanner.characters.iter().collect(),
        }
    }
}

impl std::fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} at position {} in {}",
            self.message, self.cursor, self.characters
        )
    }
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

    /// Returns the next word (ascii alphabet only) in the scanner.
    pub fn next_word(&mut self) -> String {
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
    pub fn next_param(&mut self) -> Result<String, ScannerError> {
        let mut ret = String::new();
        const BINARY_OPERATORS: &[char] = &['_', '^'];

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
                    "" => ret.push(self.next().ok_or(ScannerError::new(
                        "Expected a character after '\\'".to_string(),
                        self.clone(),
                    ))?),
                    word => ret.push_str(&word),
                }
                loop {
                    match self.next() {
                        Some(c) if c.is_whitespace() => {}
                        Some(c) if BINARY_OPERATORS.contains(&c) => {
                            ret.push(c);
                            ret.push_str(&self.next_param()?);
                        }
                        _ => {
                            self.cursor -= 1;
                            break;
                        }
                    }
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
            None => {
                return Err(ScannerError::new(
                    "Unexpected end of input".to_string(),
                    self.clone(),
                ))
            }
        }
        Ok(ret)
    }

    /// Returns the next LaTeX optional parameter in the scanner
    /// returns empty string if there's no optional parameter
    pub fn next_param_optional(&mut self) -> String {
        let mut ret = String::new();

        // trim whitespace
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.cursor += 1;
        }

        // check if next character is '\\', '{', or any other character
        match self.peek() {
            Some('[') => {
                self.cursor += 1;
                while let Some(c) = self.next() {
                    match c {
                        ']' => break,
                        _ => ret.push(c),
                    }
                }
            }
            _ => return "".to_string(),
        }
        ret
    }

    /// Return characters until one of the characters in `chars` is found.
    /// The ending character is consumed
    pub fn until_chars(&mut self, chars: &str) -> String {
        let mut ret = String::new();
        while let Some(c) = self.next() {
            if chars.contains(c) {
                break;
            }
            ret.push(c);
        }
        ret
    }

    /// Return characters until one of the characters **not** in `chars` is found.
    /// The ending character is consumed
    pub fn until_chars_not(&mut self, chars: &str) -> String {
        let mut ret = String::new();
        while let Some(c) = self.next() {
            if !chars.contains(c) {
                break;
            }
            ret.push(c);
        }
        ret
    }

    /// Return characters until match the input string.
    /// The ending string is consumed
    pub fn until_string(&mut self, string: String) -> String {
        let mut ret = String::new();
        while let Some(c) = self.next() {
            ret.push(c);
            if ret.ends_with(&string) {
                ret.truncate(ret.len() - string.len());
                break;
            }
        }
        ret
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

macro_rules! matrix {
    // no alignment in matrix currently
    ($scanner:expr, $param:expr, $delim:expr) => {{
        $scanner.next_param_optional();
        format!(
            "mat(delim: {}, {})",
            $delim,
            matrix_to_typst($scanner.until_string(format!("\\end{{{}}}", $param)))
        )
    }};
    ($scanner:expr, $param:expr) => {{
        $scanner.next_param_optional();
        format!(
            "mat({})",
            matrix_to_typst($scanner.until_string(format!("\\end{{{}}}", $param)))
        )
    }};
}

macro_rules! single {
    ($scanner:expr, $fn:expr) => {{
        format!(
            "{}({})",
            $fn,
            latex_to_typst($scanner.next_param().unwrap())
        )
    }};
    ($scanner:expr, $fn:expr, $params:expr) => {{
        format!(
            "{}({}{})",
            $fn,
            $params,
            latex_to_typst($scanner.next_param().unwrap())
        )
    }};
}

macro_rules! double {
    ($scanner:expr, $fn:expr) => {{
        format!(
            "{}({}, {})",
            $fn,
            latex_to_typst($scanner.next_param().unwrap()),
            latex_to_typst($scanner.next_param().unwrap())
        )
    }};
    ($scanner:expr, $fn1:expr, $fn2:expr) => {{
        format!(
            "{}({}({}, {}))",
            $fn1,
            $fn2,
            latex_to_typst($scanner.next_param().unwrap()),
            latex_to_typst($scanner.next_param().unwrap())
        )
    }};
}

macro_rules! accent {
    ($scanner:expr, $accent:expr) => {{
        format!(
            "accent({}, {})",
            latex_to_typst($scanner.next_param().unwrap()),
            $accent
        )
    }};
}

pub fn latex_to_typst(latex: String) -> String {
    let mut scanner = Scanner::new(latex);
    let mut text = String::new();
    while let Some(c) = scanner.next() {
        let push = match c {
            '\\' => match scanner.next_word().as_str() {
                // same one goes to default
                "" => {
                    let c = scanner.next().unwrap();
                    match c {
                        '\'' | '"' | '.' | '`' | '=' | '~' | '^' => {
                            let func = match c {
                                '\'' => "acute".to_owned(),
                                '"' => "dot.double".to_owned(),
                                '.' => "dot".to_owned(),
                                '`' => "grave".to_owned(),
                                '=' => "macron".to_owned(),
                                '~' => "tilde".to_owned(),
                                '^' => "hat".to_owned(),
                                _ => unreachable!(),
                            };
                            single!(scanner, func)
                        }
                        // escape characters in Typst
                        '_' | '&' | '#' => format!("\\{}", c),
                        '!' => "#h(-1em/6)".to_owned(),
                        ' ' => "space".to_owned(),
                        '(' | ')' => "".to_owned(),
                        ',' => "space.sixth".to_owned(),
                        ':' | '>' => "space.med".to_owned(),
                        ';' => "#h(5em/18)".to_owned(),
                        '|' => "||".to_owned(),
                        '\\' => "\\".to_owned(),
                        _ => format!("{}", c),
                    }
                }
                // A
                "AA" => "circle(A)".to_owned(),
                "aa" => "circle(a)".to_owned(),
                "acute" => single!(scanner, "acute"),
                "AE" => "Æ".to_owned(),
                "ae" => "æ".to_owned(),
                "alefsym" => "alef".to_owned(),
                "amalg" | "coprod" => "product.co".to_owned(),
                "And" => "\\&".to_owned(),
                "approxeq" => "approx.eq".to_owned(),
                "approxcolon" => "approx:".to_owned(),
                "approxcoloncolon" => "approx::".to_owned(),
                "arcctg" => "#math.op(\"arcctg\")".to_owned(),
                "arctg" => "#math.op(\"arctg\")".to_owned(),
                "argmax" => "arg max".to_owned(),
                "argmin" => "arg min".to_owned(),
                "ast" => "*".to_owned(),
                "asymp" => "≍".to_owned(),
                // B
                "backepsilon" => "in.rev.small".to_owned(),
                "backprime" => "prime.rev".to_owned(),
                "backsim" => "tilde.rev".to_owned(),
                "backsimeq" => "tilde.eq.rev".to_owned(),
                "backslash" => "\\\\".to_owned(),
                "bar" => single!(scanner, "macron"),
                "barwedge" => "⊼".to_owned(),
                "Bbb" => single!(scanner, "bb"),
                "Bbbk" => "bb(k)".to_owned(),
                "bcancel" => single!(scanner, "cancel", "inverted: #true, "),
                "begin" => {
                    // skip numbering because there's a issue for numbering equation separately in Typst
                    let param = scanner.next_param().unwrap();
                    match param.as_str() {
                        "align" | "align*" | "aligned" | "equation" | "equation*" | "gather" | "gathered" | "split" => {
                            format!(
                                "$${}$$",
                                latex_to_typst(scanner.until_string(format!("\\end{{{}}}", param)))
                            )
                        },
                        "alignat" | "alignat*" | "alignedat" => {
                            scanner.next_param().unwrap();
                            format!(
                                "$${}$$",
                                latex_to_typst(scanner.until_string(format!("\\end{{{}}}", param)))
                            )
                        }
                        "array" | "darray" => {
                            let _ = scanner.next_param();
                            format!(
                                "mat(delim: #none, {})",
                                matrix_to_typst(scanner.until_string(format!("\\end{{{}}}", param)))
                            )
                        }
                        "Bmatrix" | "Bmatrix*" => matrix!(scanner, param, "\"{\""),
                        "bmatrix" | "bmatrix*" => matrix!(scanner, param, "\"[\""),
                        "cases" | "dcases" => format!(
                            "cases({})",    
                            scanner.until_string(format!("\\end{{{}}}", param))
                                .split("\\\\")
                                .map(|s| {
                                    latex_to_typst(s.to_string())
                                })
                                .collect::<Vec<String>>()
                                .join(",")
                        ),
                        "CD" => {
                            // TODO: begin{CD}
                            format!(
                                "CD({})",
                                scanner.until_string(format!("\\end{{{}}}", param))
                            )
                        }
                        "matrix" | "matrix*" => matrix!(scanner, param, "#none"),
                        "pmatrix" | "pmatrix*" => matrix!(scanner, param, "\"(\""),
                        "rcases" => format!(
                            "cases(reverse: #true, {})",    
                            scanner.until_string(format!("\\end{{{}}}", param))
                                .split("\\\\")
                                .map(|s| {
                                    latex_to_typst(s.to_string())
                                })
                                .collect::<Vec<String>>()
                                .join(",")
                        ),
                        "smallmatrix" => format!(
                            "inline(mat(delim: #none, {}))",
                            matrix_to_typst(scanner.until_string(format!("\\end{{{}}}", param)))
                        ),
                        "Vmatrix" | "Vmatrix*" => matrix!(scanner, param, "\"||\""),
                        "vmatrix" | "vmatrix*" => matrix!(scanner, param, "\"|\""),
                        _ => unreachable!()
                    }
                }
                "between" => "≬".to_owned(),
                "bigcap" => "sect.big".to_owned(),
                "bigcirc" => "circle.stroked.big".to_owned(),
                "bigcup" => "union.big".to_owned(),
                "bigdot" => "dot.circle.big".to_owned(),
                "bigoplus" => "plus.circle.big".to_owned(),
                "bigotimes" => "times.circle.big".to_owned(),
                "bigsqcup" => "union.sq.big".to_owned(),
                "bigstar" => "star.stroked".to_owned(),
                "bigtriangledown" => "triangle.stroked.b".to_owned(),
                "bigtriangleup" | "vartriangle" | "triangle" => "triangle.stroked.t".to_owned(),
                "biguplus" => "union.plus.big".to_owned(),
                "bigvee" => "or.big".to_owned(),
                "bigwedge" => "and.big".to_owned(),
                "binom" => double!(scanner, "binom"),
                "blacklozenge" => "lozen
                ge.filled".to_owned(),
                "blacksquare" => "square.filled".to_owned(),
                "blacktriangle" => "triangle.filled.t".to_owned(),
                "blacktriangledown" => "triangle.filled.b".to_owned(),
                "blacktriangleleft" => "triangle.filled.l".to_owned(),
                "blacktriangleright" => "triangle.filled.r".to_owned(),
                "bm" | "bold" | "boldsymbol" => single!(scanner, "bold"),
                "bmod" | "pmod" => "mod".to_owned(),
                "bowtie" | "Join" => "⋈".to_owned(),
                "Box" => "square.stroked".to_owned(),
                "boxdot" => "dot.square".to_owned(),
                "boxed" => format!(
                    "#box(inset: (left: 3pt, right: 3pt), outset: (top: 3pt, bottom: 3pt), stroke: 0.5pt)[${}$]",
                    latex_to_typst(scanner.next_param().unwrap())
                ),
                "boxminus" => "minus.square".to_owned(),
                "boxplus" => "plus.square".to_owned(),
                "boxtimes" => "times.square".to_owned(),
                "Bra" | "bra" => format!("lr(angle.l {} |)", latex_to_typst(scanner.next_param().unwrap())),
                "breve" | "u" => single!(scanner, "breve"),
                "bull" | "bullet" => "circle.filled.small".to_owned(),
                "Bumpeq" => "≎".to_owned(),
                "bumpeq" => "≏".to_owned(),
                // C
                "cancel" => single!(scanner, "cancel"),
                "Cap" | "doublecap" => "sect.double".to_owned(),
                "cap" => "sect".to_owned(),
                "cdot" | "cdotp" | "centerdot" | "sdot" => "dot.op".to_owned(),
                "cfrac" => double!(scanner, "display", "frac"),
                "char" => {
                    let code = match scanner.peek().unwrap() {
                        '"' => {
                            scanner.cursor += 1;
                            scanner.until_chars_not("0123456789abcdefABCDEF")
                        }
                        '\''=>{
                            scanner.cursor += 1;
                            format!(
                                "{:x}",
                                u32::from_str_radix(&scanner.until_chars_not("01234567"), 8).unwrap()
                            )
                        }
                        _ => format!(
                            "{:x}",
                            u32::from_str_radix(&scanner.until_chars_not("0123456789"), 10).unwrap()
                        )
                    };
                    scanner.cursor -= 1;
                    format!("\\u{{{}}}", code)
                }
                ,
                "cdots" | "dots" | "dotsb" | "dotsc" | "dotsi" | "dotsm" => "dots.h.c".to_owned(),
                "check" | "V" | "widecheck" => single!(scanner, "caron"),
                "circ" => "compose".to_owned(),
                "circeq" => "≗".to_owned(),
                "circlearrowleft" => "arrow.ccw".to_owned(),
                "circlearrowright" => "arrow.cw".to_owned(),
                "circledast" => "ast.circle".to_owned(),
                "circledcirc" => "circle.nested".to_owned(),
                "circleddash" => "dash.circle".to_owned(),
                "circledR" => "®".to_owned(),
                "circledS" => "Ⓢ".to_owned(),
                "clubs" | "clubsuit" => "suit.club".to_owned(),
                "cnums" => "CC".to_owned(),
                "Colonapprox" => "::approx".to_owned(),
                "colonapprox" => ":approx".to_owned(),
                "coloncolon" => "::".to_owned(),
                "coloncolonapprox" => "::approx".to_owned(),
                "coloncolonequals" | "Coloneqq" => "::=".to_owned(),
                "coloncolonminus" | "Coloneq" => "\"::−\"".to_owned(),
                "coloncolonsim" | "Colonsim" => "::tilde.op".to_owned(),
                "coloneq" | "colonminus" => "\":−\"".to_owned(),
                "colonequals" | "coloneqq" => ":=".to_owned(),
                "colonsim" => ":tilde.op".to_owned(),
                "colorbox" => format!(
                    "#box(inset: (left: 3pt, right: 3pt), outset: (top: 3pt, bottom: 3pt), fill: {})[{}]",
                    color_to_typst(scanner.next_param().unwrap()),
                    text_to_typst(scanner.next_param().unwrap())
                ),
                "complexes" => "CC".to_owned(),
                "cong" => "tilde.equiv".to_owned(),
                "cosec" => "#math.op(\"cosec\")".to_owned(),
                "cotg" => "#math.op(\"cotg\")".to_owned(),
                "cth" => "#math.op(\"cth\")".to_owned(),
                "Cup" | "doublecup" => "union.double".to_owned(),
                "cup" => "union".to_owned(),
                "curlyeqprec" => "eq.prec".to_owned(),
                "curlyeqsucc" => "eq.succ".to_owned(),
                "curlyvee" => "or.curly".to_owned(),
                "curlywedge" => "and.curly".to_owned(),
                "curvearrowleft" => "arrow.ccw.half".to_owned(),
                "curvearrowright" => "arrow.cw.half".to_owned(),
                // D
                "dag" => "dagger".to_owned(),
                "Dagger" | "ddag" | "ddagger" => "dagger.double".to_owned(),
                "daleth" => "ℸ".to_owned(),
                "Darr" | "dArr" | "Downarrow" => "arrow.b.double".to_owned(),
                "darr" | "downarrow" => "arrow.b".to_owned(),
                "dashleftarrow" => "arrow.l.dash".to_owned(),
                "dashrightarrow" => "arrow.r.dash".to_owned(),
                "dashv" => "tack.l".to_owned(),
                "dbinom" => double!(scanner, "display", "binom"),
                "dbcolon" => "::".to_owned(),
                "ddot" => single!(scanner, "dot.double"),
                "ddots" => "dots.down".to_owned(),
                "digaamma" => "ϝ".to_owned(),
                "dfrac" => double!(scanner, "display", "frac"),
                "diagdown" => "╲".to_owned(),
                "diagup" => "╱".to_owned(),
                "Diamond" => "lozenge.stroked".to_owned(),
                "diamond" => "diamond.stroked.small".to_owned(),
                "diamonds" | "diamondsuit" => "♢".to_owned(),
                "displaystyle" => single!(scanner, "display"),
                "divideontimes" => "times.div".to_owned(),
                "dot" => single!(scanner, "dot"),
                "Doteq" | "doteqdot" => "≑".to_owned(),
                "doteq" => "≐".to_owned(),
                "dotplus" => "plus.dot".to_owned(),
                "dotso" | "ldots" | "mathellipsis" => "...".to_owned(),
                "doublebarwedge" => "⩞".to_owned(),
                "downdownarrows" => "arrows.bb".to_owned(),
                "downharpoonleft" => "harpoon.bl".to_owned(),
                "downharpoonright" => "harpoon.br".to_owned(),
                // E
                "ell" => "cal(l)".to_owned(),
                "empty" | "emptyset" => "empty".to_owned(),
                "enspace" => "space.en".to_owned(),
                "epsilon" => "epsilon.alt".to_owned(),
                "eqcirc" => "≖".to_owned(),
                "Eqcolon" | "minuscoloncolon" => "\"−::\"".to_owned(),
                "eqcolon" | "minuscolon" => "dash.colon".to_owned(),
                "Eqqcolon" | "equalscoloncolon" => "\"=::\"".to_owned(),
                "eqqcolon" | "equalscolon" => "=:".to_owned(),
                "eqsim" => "eq.tilde".to_owned(),
                "eqslantgtr" => "⪖".to_owned(),
                "eqslantless" => "⪕".to_owned(),
                "eth" => "ð".to_owned(),
                "exist" => "exists".to_owned(),
                // F
                "fallingdotseq" => "≒".to_owned(),
                "fbox" => format!(
                    "#box(inset: (left: 3pt, right: 3pt), outset: (top: 3pt, bottom: 3pt))[$upright({})$]",
                    text_to_typst(scanner.next_param().unwrap())
                ),
                "fcolorbox" => format!(
                    "#box(inset: (left: 3pt, right: 3pt), outset: (top: 3pt, bottom: 3pt))(stroke: {}, fill: {})[$upright({})$]",
                    color_to_typst(scanner.next_param().unwrap()),
                    color_to_typst(scanner.next_param().unwrap()),
                    text_to_typst(scanner.next_param().unwrap())
                ),
                "Finv" => "Ⅎ".to_owned(),
                "flat" => "♭".to_owned(),
                "frac" => double!(scanner, "frac"),
                "frak" | "mathfrak" => single!(scanner, "frak"),
                "frown" => "⌢".to_owned(),
                // G
                "Game" => "⅁".to_owned(),
                "ge" | "geq" => ">=".to_owned(),
                "geqq" => "ge.equiv".to_owned(),
                "geqslant" => "gt.eq.slant".to_owned(),
                "gets" | "larr" | "leftarrow" => "<-".to_owned(),
                "gg" => ">>".to_owned(),
                "ggg" | "gggtr" => ">>>".to_owned(),
                "gnapprox" => "⪊".to_owned(),
                "gneq" => "⪈".to_owned(),
                "gneqq" => "gt.nequiv".to_owned(),
                "gnsim" => "gt.ntilde".to_owned(),
                "grave" => single!(scanner, "grave"),
                "gt" => ">".to_owned(),
                "gtapprox" => "⪆".to_owned(),
                "gtreqless" => "gt.eq.lt".to_owned(),
                "gtreqqless" => "⪌".to_owned(),
                "gtrless" => "gt.lt".to_owned(),
                "gtrsim" => "gt.tilde".to_owned(),
                // H
                "H" => single!(scanner, "acute.double"),
                "Harr" | "hArr" | "Leftrightarrow" | "Lrarr" | "lrArr" => "<=>".to_owned(),
                "harr" | "leftrightarrow" | "lrarr" => "<->".to_owned(),
                "hat" | "widehat" => single!(scanner, "hat"),
                "hbar" | "hslash" => "planck.reduce".to_owned(),
                "hbox" | "mathnormal" | "mathop" => latex_to_typst(scanner.next_param().unwrap()),
                "hearts" | "heartsuit" => "♡".to_owned(),
                "hookleftarrow" => "arrow.l.hook".to_owned(),
                "hookrightarrow" => "arrow.r.hook".to_owned(),
                "hphantom" => format!("#box(height: 0pt, hide[${}$])", latex_to_typst(scanner.next_param().unwrap())),
                "hspace" | "mskip" => single!(scanner, "#h"),
                // I
                "i" | "imath" => "dotless.i".to_owned(),
                "iff" | "Longleftrightarrow" => "<==>".to_owned(),
                "iiint" => "integral.triple".to_owned(),
                "iint" => "integral.double".to_owned(),
                "image" => "Im".to_owned(),
                "impliedby" | "Longleftarrow" => "<==".to_owned(),
                "implies" => "==>".to_owned(),
                "infin" | "infty" => "infinity".to_owned(),
                "injlim" => "#math.op(\"inj\\u{{2009}}lim\", limits: true)".to_owned(),
                "int" | "intop" => "integral".to_owned(),
                "intercal" => "⊺".to_owned(),
                "isin" => "in".to_owned(),
                // JK
                "j" | "jmath" => "dotless.j".to_owned(),
                "KaTeX" => "\"KaTeX\"".to_owned(),
                "Ket" | "ket" => format!("lr(| {} angle.r)", latex_to_typst(scanner.next_param().unwrap())),
                // L
                "lang" | "langle" => "angle.l".to_owned(),
                "Larr" | "lArr" | "Leftarrow" => "arrow.l.double".to_owned(),
                "LaTeX" => "\"LaTeX\"".to_owned(),
                "lBrace" => "⦃".to_owned(),
                "lbrace" => "{".to_owned(),
                "lbrack" => "[".to_owned(),
                "lceil" => "⌈".to_owned(),
                "ldotp" => ".".to_owned(),
                "le" | "leq" => "<=".to_owned(),
                "leadsto" => "arrow.r.squiggly".to_owned(),
                "lfloor" => "⌊".to_owned(),
                "lgroup" => "turtle.l".to_owned(),
                "lhd" | "vartriangleleft" => "lt.tri".to_owned(),
                "limits" | "nolimits" => "".to_owned(),
                "ll" => "<<".to_owned(),
                "llbracket" => "bracket.l.double".to_owned(),
                "llcorner" => "⌞".to_owned(),
                "Lleftarrow" => "arrow.l.triple".to_owned(),
                "lll" | "llless" => "<<<".to_owned(),
                "lnapprox" => "⪉".to_owned(),
                "lneq" => "⪇".to_owned(),
                "lneqq" => "lt.nequiv".to_owned(),
                "lnot" => "not".to_owned(),
                "lnsim" => "lt.ntilde".to_owned(),
                "longleftarrow" => "<--".to_owned(),
                "longleftrightarrow" => "<-->".to_owned(),
                "longmapsto" => "arrow.r.long.bar".to_owned(),
                "Longrightarrow" => "==>".to_owned(),
                "longrightarrow" => "-->".to_owned(),
                "looparrowleft" => "arrow.l.loop".to_owned(),
                "looparrowright" => "arrow.r.loop".to_owned(),
                "lor" | "vee" => "or".to_owned(),
                "lozenge" => "lozenge.stroked".to_owned(),
                "lparen" => "(".to_owned(),
                "lrcorner" => "⌟".to_owned(),
                "lq" => "quote.l.single".to_owned(),
                "Lsh" => "↰".to_owned(),
                "lt" => "<".to_owned(),
                "ltimes" => "times.l".to_owned(),
                "lVert" | "rVert" | "Vert" => "parallel".to_owned(),
                "lvert" | "rvert" | "vert" => "divides".to_owned(),
                // M
                "mapsto" => "arrow.r.bar".to_owned(),
                "mathbb" => single!(scanner, "bb"),
                "mathbf" => single!(scanner, "bold"),
                "mathbin" => format!("#math.op(\"{}\")", typ::escape_string(scanner.next_param().unwrap())),
                "mathcal" => single!(scanner, "cal"),
                "mathclap" => single!(scanner, "#box", "width: 0pt, "),
                "mathclose" => format!("#h(0pt) {}", latex_to_typst(scanner.next_param().unwrap())),
                "mathit" => single!(scanner, "italic"),
                "mathopen" => format!("{} #h(0pt)", latex_to_typst(scanner.next_param().unwrap())),
                "mathring" | "r" => single!(scanner, "circle"),
                "mathrm" => single!(scanner, "upright"),
                "mathsf" => single!(scanner, "sans"),
                "mathsterling" => "pound".to_owned(),
                "mathstrut" => "#hide(box(width: 0pt, \")\"))".to_owned(),
                "measuredangle" => "angle.arc".to_owned(),
                "medspace" => "space.med".to_owned(),
                "mho" => "ohm.inv".to_owned(),
                "mid" => "|".to_owned(),
                "minuso" => "⊖".to_owned(),
                "models" | "vDash" => "tack.r.double".to_owned(),
                "mp" => "minus.plus".to_owned(),
                // N
                "N" | "natnums" => "NN".to_owned(),
                "natural" => "♮".to_owned(),
                "negmedspace" => "#h(-2em/9)".to_owned(),
                "ncong" => "tilde.equiv.not".to_owned(),
                "ne" | "neq" => "!=".to_owned(),
                "nearrow" => "arrow.tr".to_owned(),
                "neg" => "not".to_owned(),
                "negthickspace" => "#h(-5em/18)".to_owned(),
                "negthinspace" => "#h(-1em/6)".to_owned(),
                "nexist" => "exists.not".to_owned(),
                "ngeq" => "gt.eq.not".to_owned(),
                "ngtr" => "gt.not".to_owned(),
                "ni" | "owns" => "in.rev".to_owned(),
                "nLeftarrow" => "arrow.l.double.not".to_owned(),
                "nleftarrow" => "arrow.l.not".to_owned(),
                "nLeftrightarrow" => "arrow.l.r.double.not".to_owned(),
                "nleftrightarrow" => "arrow.l.r.not".to_owned(),
                "nleq" => "lt.eq.not".to_owned(),
                "nless" => "lt.not".to_owned(),
                "nmid" => "divides.not".to_owned(),
                "nobreakspace" => "space.nobreak".to_owned(),
                "notin" => "in.not".to_owned(),
                "notni" => "in.rev.not".to_owned(),
                "notparallel" => "parallel.not".to_owned(),
                "nprec" => "prec.not".to_owned(),
                "npreceq" => "prec.eq.not".to_owned(),
                "nRightarrow" => "arrow.r.double.not".to_owned(),
                "nrightarrow" => "arrow.r.not".to_owned(),
                "nsim" => "tilde.not".to_owned(),
                "nsubseteq" | "nsupseteq" => "subset.eq.not".to_owned(),
                "nsucc" => "succ.not".to_owned(),
                "nsucceq" => "succ.eq.not".to_owned(),
                "ntriangleleft" => "lt.tri.not".to_owned(),
                "ntrianglelefteq" => "lt.tri.eq.not".to_owned(),
                "ntriangleright" => "gt.tri.not".to_owned(),
                "ntrianglerighteq" => "gt.tri.eq.not".to_owned(),
                "nVDash" => "⊯".to_owned(),
                "nVdash" => "⊮".to_owned(),
                "nvDash" => "tack.r.double.not".to_owned(),
                "nvdash" => "tack.r.not".to_owned(),
                "nwarrow" => "arrow.tl".to_owned(),
                // O
                "O" => "Ø".to_owned(),
                "o" => "ø".to_owned(),
                "odot" => "dot.circle".to_owned(),
                "OE" => "Œ".to_owned(),
                "oe" => "œ".to_owned(),
                "oiiint" => "integral.vol".to_owned(),
                "oiint" => "integral.surf".to_owned(),
                "oint" => "integral.cont".to_owned(),
                "ominus" => "minus.circle".to_owned(),
                "operatorname" => format!("#math.op(\"{}\")", typ::escape_string(scanner.next_param().unwrap())),
                "operatorname*" | "operatornamewithlimits" => format!(
                    "#math.op(\"{}\", limits: true)",
                    typ::escape_string(scanner.next_param().unwrap())
                ),
                "oplus" => "plus.circle".to_owned(),
                "origof" => "⊶".to_owned(),
                "oslash" => "⊘".to_owned(),
                "otimes" => "times.circle".to_owned(),
                "overbrace" => {
                    let param1 = latex_to_typst(scanner.next_param().unwrap());
                    match scanner.peek() {
                        Some('^') => {
                            scanner.cursor += 1;
                            format!(
                                "overbrace({}, {})",
                                param1,
                                latex_to_typst(scanner.next_param().unwrap())
                            )
                        }
                        _ => format!("overbrace({})", param1),
                    }
                }
                "overgroup" => accent!(scanner, "\\u{{0311}}"),
                "overleftarrow" => single!(scanner, "arrow.l"),
                "overleftharpoon" => accent!(scanner, "harpoon.lt"),
                "overleftrightarrow" => accent!(scanner, "arrow.l.r"),
                "overline" => single!(scanner, "overline"),
                "overlinesegment" => accent!(scanner, "\\u{{20e9}}"),
                "overrightarrow" | "vec" => single!(scanner, "arrow"),
                "overrightharpoon" => accent!(scanner, "harpoon.rt"),
                // P
                "P" => "pilcrow".to_owned(),
                "partial" => "diff".to_owned(),
                "perp" => "bot".to_owned(),
                "phantom" => format!("#hide[${}$]", latex_to_typst(scanner.next_param().unwrap())),
                "phi" => "phi.alt".to_owned(),
                "pitchfork" => "⋔".to_owned(),
                "plim" => "#math.op(\"plim\", limits: true)".to_owned(),
                "plusmn" | "pm" => "plus.minus".to_owned(),
                "pounds" => "pound".to_owned(),
                "precapprox" => "prec.approx".to_owned(),
                "preccurlyeq" => "prec.eq".to_owned(),
                "preceq" => "⪯".to_owned(),
                "precnapprox" => "prec.napprox".to_owned(),
                "precneqq" => "prec.nequiv".to_owned(),
                "precnsim" => "prec.ntilde".to_owned(),
                "precsim" => "prec.tilde".to_owned(),
                "prime" | "rq" => "'".to_owned(),
                "prod" => "product".to_owned(),
                "projlim" => "#math.op(\"proj\\u{{2009}}lim\", limits: true)".to_owned(),
                "propto" | "varpropto" => "prop".to_owned(),
                // QR
                "qquad" => "#h(2em)".to_owned(),
                "quad" => "space.quad".to_owned(),
                "R" => "RR".to_owned(),
                "raisebox" => format!(
                    "#text(baseline: -{})[{}]",
                    scanner.next_param().unwrap(),
                    latex_to_typst(scanner.next_param().unwrap())
                ),
                "rang" | "rangle" => "angle.r".to_owned(),
                "Rarr" | "rArr" | "Rightarrow" => "=>".to_owned(),
                "rarr" | "rightarrow" | "to" => "->".to_owned(),
                "ratio" => ":".to_owned(),
                "rBrace" => "⦄".to_owned(),
                "rbrace" => "}".to_owned(),
                "rbrack" => "]".to_owned(),
                "rceil" => "⌉".to_owned(),
                "Reals" | "reals" => "RR".to_owned(),
                "restriction" => "harpoon.tr".to_owned(),
                "rfloor" => "⌋".to_owned(),
                "rgroup" => "turtle.r".to_owned(),
                "rhd" | "vartriangleright" => "gt.tri".to_owned(),
                "rightarrowtail" => ">->".to_owned(),
                "rightharpoondown" => "harpoon.rb".to_owned(),
                "rightharpoonup" => "harpoon.rt".to_owned(),
                "rightleftarrows" => "arrows.rl".to_owned(),
                "rightleftharpoons" => "harpoons.rtlb".to_owned(),
                "rightrightarrows" => "arrows.rr".to_owned(),
                "rightsquigarrow" => "arrow.r.squiggly".to_owned(),
                "rightthreetimes" => "times.three.r".to_owned(),
                "risingdotseq" => "≓".to_owned(),
                "rmoustache" => "⎱".to_owned(),
                "rparen" => ")".to_owned(),
                "rrbracket" => "bracket.r.double".to_owned(),
                "Rrightarrow" => "arrow.r.triple".to_owned(),
                "Rsh" => "↱".to_owned(),
                "rtimes" => "times.r".to_owned(),
                "rule" => {
                    match scanner.next_param_optional().as_str() {
                        "" => format!(
                            "#box(fill: black, width: {}, height: {})",
                            scanner.next_param().unwrap(),
                            scanner.next_param().unwrap()
                        ),
                        p => format!(
                            "#box(inset: (bottom: {}), box(fill: black, width: {}, height: {}))",
                            p,
                            scanner.next_param().unwrap(),
                            scanner.next_param().unwrap(),
                        ),
                    }
                }
                // S
                "S" | "sect" => "section".to_owned(),
                "searrow" => "arrow.br".to_owned(),
                "Set" | "set" => format!("{{{}}}", latex_to_typst(scanner.next_param().unwrap())),
                "setminus" | "smallsetminus" => "without".to_owned(),
                "sharp" => "♯".to_owned(),
                "sim" => "tilde.op".to_owned(),
                "simcolon" => "tilde.op:".to_owned(),
                "simcoloncolon" => "tilde.op::".to_owned(),
                "simeq" => "tilde.eq".to_owned(),
                "sh" => "#math.op(\"sh\")".to_owned(),
                "smallint" => "inline(integral)".to_owned(),
                "smallsmile" => "⌣".to_owned(),
                "sout" => single!(scanner, "cancel", "angle: #90deg, "),
                "spades" | "spadesuit" => "suit.spade".to_owned(),
                "sphericalangle" => "angle.spheric".to_owned(),
                "sqcap" => "sect.sq".to_owned(),
                "sqcup" => "union.sq".to_owned(),
                "square" => "square.stroked".to_owned(),
                "sqrt" => {
                    let p = scanner.next_param_optional();
                    if p != "" {
                        format!(
                            "root({}, {})",
                            latex_to_typst(p),
                            latex_to_typst(scanner.next_param().unwrap())
                        )
                    } else {
                        single!(scanner, "sqrt")
                    }
                }
                "sqsubset" => "subset.sq".to_owned(),
                "sqsubseteq" => "subset.eq.sq".to_owned(),
                "sqsupset" => "superset.sq".to_owned(),
                "sqsupseteq" => "superset.eq.sq".to_owned(),
                "ss" => "ß".to_owned(),
                "star" => "star.op".to_owned(),
                "sub" => "subset".to_owned(),
                "sube" | "subseteq" => "subset.eq".to_owned(),
                "Subset" => "subset.double".to_owned(),
                "subseteqq" => "⫅".to_owned(),
                "subsetneq" | "varsubsetneq" => "subset.neq".to_owned(),
                "subsetneqq" | "varsubsetneqq" => "⫋".to_owned(),
                "succapprox" => "succ.approx".to_owned(),
                "succcurlyeq" => "succ.eq".to_owned(),
                "succeq" => "⪰".to_owned(),
                "succnapprox" => "succ.napprox".to_owned(),
                "succneqq" => "succ.nequiv".to_owned(),
                "succnsim" => "succ.ntilde".to_owned(),
                "supe" | "supseteq" => "supset.eq".to_owned(),
                "Supset" => "superset.double".to_owned(),
                "supseteqq" => "⫆".to_owned(),
                "supsetneq" | "varsupsetneq" => "superset.neq".to_owned(),
                "supsetneqq" | "varsupsetneqq" => "⫌".to_owned(),
                "surd" => "√".to_owned(),
                "swarrow" => "arrow.bl".to_owned(),
                // T
                "tbinom" => double!(scanner, "inline", "binom"),
                "TeX" => "\"TeX\"".to_owned(),
                "text" | "textmd" | "textnormal" | "textrm" | "textup" => {
                    format!("#[{}]", text_to_typst(scanner.next_param().unwrap()))
                }
                "textbf" => format!("bold(#[{}])", text_to_typst(scanner.next_param().unwrap())),
                "textcolor" => format!(
                    "#text(fill: {})[{}]",
                    color_to_typst(scanner.next_param().unwrap()),
                    text_to_typst(scanner.next_param().unwrap())
                ),
                "textit" => format!("italic(#[{}])", text_to_typst(scanner.next_param().unwrap())),
                "textsf" => format!("sans(#[{}])", text_to_typst(scanner.next_param().unwrap())),
                "textstyle" => format!("inline({})", latex_to_typst(scanner.next_param().unwrap())),
                "texttt" => format!("mono(#[{}])", text_to_typst(scanner.next_param().unwrap())),
                "tfrac" => double!(scanner, "inline", "frac"),
                "th" => "#math.op(\"th\")".to_owned(),
                "thetasym" => "theta.alt".to_owned(),
                "thickapprox" => "bold(approx)".to_owned(),
                "thicksim" => "bold(tilde)".to_owned(),
                "thickspace" => "#h(5em/18)".to_owned(),
                "thinspace" => "space.sixth".to_owned(),
                "tilde" | "widetilde" => single!(scanner, "tilde"),
                "triangledown" => "triangle.stroked.b".to_owned(),
                "triangleleft" => "triangle.stroked.l".to_owned(),
                "trianglelefteq" => "lt.tri.eq".to_owned(),
                "triangleq" => "eq.delta".to_owned(),
                "triangleright" => "triangle.stroked.r".to_owned(),
                "trianglerighteq" => "gt.tri.eq".to_owned(),
                "twoheadleftarrow" => "<<-".to_owned(),
                "twoheadrightarrow" => "->>".to_owned(),
                // U
                "Uarr" | "uArr" | "Uparrow" => "arrow.t.double".to_owned(),
                "uarr" | "uparrow" => "arrow.t".to_owned(),
                "ulcorner" => "⌜".to_owned(),
                "underbar" | "underline" => single!(scanner, "underline"),
                "underbrace" => {
                    let param1 = latex_to_typst(scanner.next_param().unwrap());
                    match scanner.peek() {
                        Some('_') => {
                            scanner.cursor += 1;
                            format!(
                                "underbrace({}, {})",
                                param1,
                                latex_to_typst(scanner.next_param().unwrap())
                            )
                        }
                        _ => format!("underbrace({})", param1),
                    }
                }
                "undergroup" => accent!(scanner, "\\u{{032e}}"),
                "underleftrightarrow" => accent!(scanner, "\\u{{034d}}"),
                "unlhd" => "lt.tri.eq".to_owned(),
                "unrhd" => "gt.tri.eq".to_owned(),
                "Updownarrow" => "arrow.t.b.double".to_owned(),
                "updownarrow" => "arrow.t.b".to_owned(),
                "upharpoonleft" => "harpoon.tl".to_owned(),
                "upharpoonright" => "harpoon.tr".to_owned(),
                "uplus" => "union.plus".to_owned(),
                "upuparrows" => "arrows.tt".to_owned(),
                "urcorner" => "⌝".to_owned(),
                // V
                "varDelta" => "italic(Delta)".to_owned(),
                "varepsilon" => "italic(epsilon)".to_owned(),
                "varGamma" => "italic(Gamma)".to_owned(),
                "varkappa" => "italic(kappa)".to_owned(),
                "varnothing" => "italic(nothing)".to_owned(),
                "varOmega" => "italic(Omega)".to_owned(),
                "varPhi" => "italic(Phi)".to_owned(),
                "varphi" => "italic(phi)".to_owned(),
                "varPi" => "italic(Pi)".to_owned(),
                "varpi" => "italic(pi.alt)".to_owned(),
                "varPsi" => "italic(Psi)".to_owned(),
                "varrho" => "italic(rho.alt)".to_owned(),
                "varSigma" => "italic(Sigma)".to_owned(),
                "varsigma" => "italic(sigma.alt)".to_owned(),
                "varTheta" => "italic(Theta)".to_owned(),
                "vartheta" => "italic(theta.alt)".to_owned(),
                "varUpsilon" => "italic(Upsilon)".to_owned(),
                "varXi" => "italic(Xi)".to_owned(),
                "vcentcolon" => ":".to_owned(),
                "Vdash" => "⊩".to_owned(),
                "vdash" => "tack.r".to_owned(),
                "vdots" => "dots.v".to_owned(),
                "veebar" => "⊻".to_owned(),
                "vphantom" => format!("#box(width: 0pt, hide[${}$])", latex_to_typst(scanner.next_param().unwrap())),
                "Vvdash" => "⊪".to_owned(),
                // W
                "wedge" | "land" => "and".to_owned(),
                "weierp" | "wp" => "℘".to_owned(),
                "wr" => "wreath".to_owned(),
                // X
                "xcancel" => single!(scanner, "cancel", "cross: #true, "),
                "xhookleftarrow" => single!(scanner, "xarrow", "sym: arrow.l.hook, "),
                "xhookrightarrow" => single!(scanner, "xarrow", "sym: arrow.r.hook, "),
                "xLeftarrow" => single!(scanner, "xarrow", "sym: arrow.l.double, "),
                "xleftarrow" => single!(scanner, "xarrow", "sym: arrow.l, "),
                "xleftharpoondown" => single!(scanner, "xarrow", "sym: harpoon.lb, "),
                "xleftharpoonup" => single!(scanner, "xarrow", "sym: harpoon.lt, "),
                "xLeftrightarrow" => single!(scanner, "xarrow", "sym: arrow.l.r.double, "),
                "xleftrightarrow" => single!(scanner, "xarrow", "sym: arrow.l.r, "),
                "xleftrightharpoons" => single!(scanner, "xarrow", "sym: harpoons.ltrb, "),
                "xlongequal" => single!(scanner, "xarrow", "sym: equal, "),
                "xmapsto" => single!(scanner, "xarrow", "sym: arrow.r.bar, "),
                "xRightarrow" => single!(scanner, "xarrow", "sym: arrow.r.double, "),
                "xrightarrow" => single!(scanner, "xarrow", "sym: arrow.r, "),
                "xrightharpoondown" => single!(scanner, "xarrow", "sym: harpoon.rb, "),
                "xrightharpoonup" => single!(scanner, "xarrow", "sym: harpoon.rt, "),
                "xrightleftharpoons" => single!(scanner, "xarrow", "sym: harpoons.rtlb, "),
                "xtwoheadleftarrow" => single!(scanner, "xarrow", "sym: arrow.l.twohead, "),
                "xtwoheadrightarrow" => single!(scanner, "xarrow", "sym: arrow.r.twohead, "),
                // YZ
                "Z" => "ZZ".to_owned(),
                word => word.to_owned(),
            }
            '%' => format!("//{}\n", scanner.until_chars("\n")),
            '~' => "space.nobreak".to_owned(),
            '/' | '"' => format!("\\{}", c),
            _ => match c {
                '{' | '}' => "".to_string(),
                _ => c.to_string(),
            },
        };
        // insert space if current and next character is a alphabetic character
        if let Some(first) = push.chars().next() {
            if let Some(prev) = text.chars().last() {
                if prev.is_alphabetic() && (first.is_alphabetic() || first.is_ascii_digit()) {
                    text.push(' ');
                }
            }
        }
        text.push_str(&push);
    }

    text
}

fn color_to_typst(color: String) -> String {
    if color.chars().next().unwrap() == '#' {
        format!("rgb(\"{}\")", color)
    } else {
        color
    }
}

pub fn text_to_typst(text: String) -> String {
    let mut scanner = Scanner::new(text);
    let mut ret = String::new();
    while let Some(c) = scanner.next() {
        let push = match c {
            '\\' => match scanner.next_word().as_str() {
                "textasciitilde" => "~".to_owned(),
                "textasciicircum" => "\\^".to_owned(),
                "textbackslash" => "\\\\".to_owned(),
                "textbar" => "|".to_owned(),
                "textbardbl" => "‖".to_owned(),
                "textbraceleft" => "{".to_owned(),
                "textbraceright" => "}".to_owned(),
                "textdagger" => "#sym.dagger".to_owned(),
                "textdaggerdbl" => "#sym.dagger.double".to_owned(),
                "textdegree" => "#sym.degree".to_owned(),
                "textdollar" => "\\$".to_owned(),
                "textellipsis" => "...".to_owned(),
                "textemdash" => "---".to_owned(),
                "textendash" => "--".to_owned(),
                "textgreater" => "#sym.gt".to_owned(),
                "textless" => "#sym.lt".to_owned(),
                "textquotedblleft" => "#sym.quote.l.double".to_owned(),
                "textquotedblright" => "#sym.quote.r.double".to_owned(),
                "textquoteleft" => "#sym.quote.l.single".to_owned(),
                "textquoteright" => "#sym.quote.r.single".to_owned(),
                "textregistered" => "®".to_owned(),
                "textsterling" => "#sym.pound".to_owned(),
                "textunderscore" => "\\_".to_owned(),
                word => word.to_owned(),
            },
            '$' => {
                let mut math = String::new();
                while let Some(c) = scanner.next() {
                    if c == '$' {
                        break;
                    }
                    math.push(c);
                }
                format!("${}$", latex_to_typst(math))
            }
            _ => c.to_string(),
        };
        ret.push_str(&push);
    }
    ret
}

/// split with '\\', then split with '&' (not '\&'), finally process element by element
///
/// for example:
/// ```
/// a& b\\
/// c& d\\
/// ```
///
/// will be converted to:
/// ```
/// a, b;
/// c, d
/// ```
fn matrix_to_typst(content: String) -> String {
    Regex::new(r"\\\\|\\cr")
        .unwrap()
        .split(&content)
        .map(|row| {
            let mut s = row
                .split('&')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let mut i = 0;
            while i < s.len() {
                if s[i].ends_with('\\') {
                    let temp = s[i + 1].clone();
                    s[i].push('&');
                    s[i].push_str(&temp);
                    s.remove(i + 1);
                }
                i += 1;
            }
            s.iter()
                .map(|col| latex_to_typst(col.to_string()))
                .collect::<Vec<String>>()
                .join(",")
        })
        .collect::<Vec<String>>()
        .join(";")
}

#[cfg(test)]
mod function_tests {
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
        while let Ok(c) = scanner.next_param() {
            println!("{}", c);
        }
    }

    #[test]
    fn color_test() {
        println!("{}", color_to_typst("#00ff00".to_string()));
        println!("{}", color_to_typst("red".to_string()));
    }

    #[test]
    fn matrix_test1() {
        println!("{}", matrix_to_typst("a& b\\\\\nc& d".to_string()));
    }

    #[test]
    fn matrix_test2() {
        println!("{}", matrix_to_typst("a& b\\cr\nc& d".to_string()));
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_scanner_error() {
    //     let scanner = Scanner::new("Hello, World!".to_string());
    //     let error = ScannerError::new("test".to_string(), scanner);
    //     let result: Result<String, ScannerError> = Err(error);

    //     result.unwrap();
    // }

    #[test]
    fn test_parse_typ() {
        println!(
            "{:?}",
            latex_to_typst("\\bcancel{N = N_oe^{ln2(t/t_2)}}".to_string())
        );
    }

    #[test]
    fn test_parse_typ2() {
        println!("{:?}", latex_to_typst("hello".to_string()));
    }

    #[test]
    fn test_parse_typ3() {
        println!("{:?}", latex_to_typst("\\binom {asdf} {aas}".to_string()));
    }

    #[test]
    fn test_parse_typ4() {
        println!(
            "{:?}",
            latex_to_typst("\\overbrace{x+⋯+x}^{n\\text{ times$\\int$}}".to_string())
        );
    }

    #[test]
    fn test_parse_typ5() {
        println!("{:?}", latex_to_typst("\\dot\\sum _ 0 ^n".to_string()));
    }

    #[test]
    fn test_parse_typ6() {
        println!(
            "{:?}",
            latex_to_typst("\\frac54 = 1\\tfrac   {1}   {4}\\\\".to_string())
        );
    }

    #[test]
    fn test_parse_typ7() {
        println!(
            "{:?}",
            latex_to_typst("h\\raisebox{2pt}{$\\psi ighe$}r".to_string())
        );
    }

    #[test]
    fn matrix1() {
        println!(
            "{:?}",
            latex_to_typst(
                "\\begin{array}{cc}
a& b\\\\
c& d
\\end{array}"
                    .to_string()
            )
        );
    }
}
