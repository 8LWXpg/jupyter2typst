use std::{borrow::Cow, iter::Peekable, str::Chars};

use crate::typ;
use itertools::Itertools;
use regex::Regex;

const BINARY_OPERATORS: &[char] = &['_', '^'];

#[derive(Debug)]
pub struct ScannerError {
	message: String,
	characters: String,
}

impl ScannerError {
	fn new(message: String, scanner: Scanner) -> Self {
		Self {
			message,
			characters: scanner.collect(),
		}
	}
}

impl std::fmt::Display for ScannerError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{} in {}", self.message, self.characters)
	}
}

#[derive(Debug, Clone)]
/// A simple one way scanner is enough of most KeTeX parsing
struct Scanner<'a>(Peekable<Chars<'a>>);

impl<'a> Scanner<'a> {
	pub fn new(text: &'a str) -> Self {
		Self(text.chars().peekable())
	}

	/// Returns character at the cursor without advancing the cursor.
	pub fn peek(&mut self) -> Option<&char> {
		self.0.peek()
	}

	/// Returns the next word (ASCII alphabet only) in the scanner.
	pub fn next_word(&mut self) -> String {
		let mut ret: String = self.0.peeking_take_while(|&c| c.is_ascii_alphabetic()).collect();
		// pick up '\operatorname*' specifically
		if ret == "operatorname" {
			if let Some('*') = self.peek() {
				self.next(); // Consume '*'
				ret.push('*');
				return ret;
			}
		}
		ret
	}

	/// Returns the next LaTeX parameter in the scanner.
	pub fn next_param(&mut self) -> Result<String, ScannerError> {
		let mut ret = String::new();

		// trim whitespace
		let next = self.by_ref().find(|&c| !c.is_whitespace());

		// check if next character is '\\', '{', or any other character
		match next {
			Some('\\') => {
				ret.push('\\');
				match self.next_word().as_str() {
					"" => ret.push(self.next().ok_or_else(|| {
						ScannerError::new("Expected a character after '\\'".to_string(), self.clone())
					})?),
					word => ret += word,
				}
				// process training binary operators
				while let Some(&c) = self.peek() {
					if c.is_whitespace() {
						self.next();
					} else if BINARY_OPERATORS.contains(&c) {
						self.next();
						ret.push(c);
						ret.push_str(&self.next_param()?);
					} else {
						break;
					}
				}
			}
			Some('{') => {
				let mut depth = 0;
				ret.extend(self.by_ref().take_while(|&c| match c {
					'{' => {
						depth += 1;
						true
					}
					'}' if depth == 0 => false,
					'}' => {
						depth -= 1;
						true
					}
					_ => true,
				}));
			}
			Some(c) => ret.push(c),
			None => return Err(ScannerError::new("Unexpected end of input".to_string(), self.clone())),
		}
		Ok(ret)
	}

	/// Returns the next LaTeX optional parameter in the scanner
	/// returns empty string if there's no optional parameter
	pub fn next_param_optional(&mut self) -> String {
		let mut ret = String::new();

		// trim whitespace
		let _ = self.by_ref().0.peeking_take_while(|&c| !c.is_whitespace());

		// check if next character is '\\', '{', or any other character
		if let Some('[') = self.peek() {
			self.next();
			ret.extend(self.take_while(|&c| c != ']'));
		}
		ret
	}

	/// Return characters until one of the characters in `chars` is found.
	/// The ending character is consumed
	pub fn until_chars(&mut self, chars: &str) -> String {
		self.take_while(|&c| chars.contains(c)).collect()
	}

	/// Return characters until one of the characters **not** in `chars` is found.
	/// The ending character is consumed
	pub fn until_chars_not(&mut self, chars: &str) -> String {
		self.take_while(|&c| !chars.contains(c)).collect()
	}

	/// Return characters until match the input string.
	/// The ending string is consumed
	pub fn until_string(&mut self, string: &str) -> String {
		let mut ret = String::new();
		for c in self.by_ref() {
			ret.push(c);
			if ret.ends_with(string) {
				ret.truncate(ret.len() - string.len());
				break;
			}
		}
		ret
	}
}

impl Iterator for Scanner<'_> {
	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next()
	}
}

macro_rules! matrix {
	// no alignment in matrix currently
	($scanner:expr, $param:expr, $delim:expr) => {{
		format!(
			"mat(delim: {}, {})",
			$delim,
			matrix_to_typst($scanner.until_string(&format!("\\end{{{}}}", $param)))?
		)
	}};
	($scanner:expr, $param:expr) => {{
		format!(
			"mat({})",
			matrix_to_typst($scanner.until_string(&format!("\\end{{{}}}", $param)))?
		)
	}};
}

macro_rules! matrix_opt {
	// no alignment in matrix currently
	($scanner:expr, $param:expr, $delim:expr) => {{
		$scanner.next_param_optional();
		format!(
			"mat(delim: {}, {})",
			$delim,
			matrix_to_typst($scanner.until_string(&format!("\\end{{{}}}", $param)))?
		)
	}};
	($scanner:expr, $param:expr) => {{
		$scanner.next_param_optional();
		format!(
			"mat({})",
			matrix_to_typst($scanner.until_string(&format!("\\end{{{}}}", $param)))?
		)
	}};
}

macro_rules! single {
	($scanner:expr, $fn:expr) => {{
		format!("{}({})", $fn, latex_to_typst($scanner.next_param()?.into())?)
	}};
	($scanner:expr, $fn:expr, $params:expr) => {{
		format!(
			"{}({}, {})",
			$fn,
			$params,
			latex_to_typst($scanner.next_param()?.into())?
		)
	}};
}

macro_rules! double {
	($scanner:expr, $fn:expr) => {{
		format!(
			"{}({}, {})",
			$fn,
			latex_to_typst($scanner.next_param()?.into())?,
			latex_to_typst($scanner.next_param()?.into())?
		)
	}};
	($scanner:expr, $fn1:expr, $fn2:expr) => {{
		format!(
			"{}({}({}, {}))",
			$fn1,
			$fn2,
			latex_to_typst($scanner.next_param()?.into())?,
			latex_to_typst($scanner.next_param()?.into())?
		)
	}};
}

macro_rules! accent {
	($scanner:expr, $accent:expr) => {{
		format!(
			"accent({}, {})",
			latex_to_typst($scanner.next_param()?.into())?,
			$accent
		)
	}};
}

pub fn latex_to_typst(latex: Cow<str>) -> Result<Cow<str>, ScannerError> {
	let mut scanner = Scanner::new(&latex);
	let mut text = String::new();
	while let Some(c) = scanner.next() {
		let push: Cow<str> = match c {
			'\\' => match scanner.next_word().as_str() {
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
							single!(scanner, func).into()
						}
						// escape characters in Typst
						'_' | '&' | '#' => format!("\\{}", c).into(),
						'!' => "#h(-1em/6)".into(),
						' ' => "space".into(),
						'(' | ')' => "".into(),
						',' => "space.sixth".into(),
						':' | '>' => "space.med".into(),
						';' => "#h(5em/18)".into(),
						'|' => "||".into(),
						'\\' => "\\".into(),
						_ => format!("{}", c).into(),
					}
				}
				// A
				"AA" => "circle(A)".into(),
				"aa" => "circle(a)".into(),
				"acute" => single!(scanner, "acute").into(),
				"AE" => "Æ".into(),
				"ae" => "æ".into(),
				"alefsym" => "alef".into(),
				"amalg" | "coprod" => "product.co".into(),
				"And" => "\\&".into(),
				"approxeq" => "approx.eq".into(),
				"approxcolon" => "approx:".into(),
				"approxcoloncolon" => "approx::".into(),
				"arcctg" => "#math.op(\"arcctg\")".into(),
				"arctg" => "#math.op(\"arctg\")".into(),
				"argmax" => "arg max".into(),
				"argmin" => "arg min".into(),
				"ast" => "*".into(),
				"asymp" => "≍".into(),
				// B
				"backepsilon" => "in.rev.small".into(),
				"backprime" => "prime.rev".into(),
				"backsim" => "tilde.rev".into(),
				"backsimeq" => "tilde.eq.rev".into(),
				"backslash" => "\\\\".into(),
				"bar" => single!(scanner, "macron").into(),
				"barwedge" => "⊼".into(),
				"Bbb" => single!(scanner, "bb").into(),
				"Bbbk" => "bb(k)".into(),
				"bcancel" => single!(scanner, "cancel", "inverted: #true").into(),
				"begin" => {
					// Skip numbering because there's an issue for numbering equation separately in Typst
					let param = scanner.next_param()?;
					match param.as_str() {
						"align" | "align*" | "aligned" | "equation" | "equation*" | "gather" | "gather*"
						| "gathered" | "split" => format!(
							"$${}$$",
							latex_to_typst(scanner.until_string(&format!("\\end{{{}}}", param)).into())?
						)
						.into(),
						"alignat" | "alignat*" | "alignedat" => {
							scanner.next_param()?;
							format!(
								"$${}$$",
								latex_to_typst(scanner.until_string(&format!("\\end{{{}}}", param)).into())?
							)
							.into()
						}
						"array" | "darray" => {
							let _ = scanner.next_param();
							format!(
								"mat(delim: #none, {})",
								matrix_to_typst(scanner.until_string(&format!("\\end{{{}}}", param)))?
							)
							.into()
						}
						"Bmatrix" => matrix!(scanner, param, "\"{\"").into(),
						"Bmatrix*" => matrix_opt!(scanner, param, "\"{\"").into(),
						"bmatrix" => matrix!(scanner, param, "\"[\"").into(),
						"bmatrix*" => matrix_opt!(scanner, param, "\"[\"").into(),
						"cases" | "dcases" => format!(
							"cases({})",
							scanner
								.until_string(&format!("\\end{{{}}}", param))
								.split("\\\\")
								.map(|s| latex_to_typst(s.into()))
								.collect::<Result<Vec<_>, _>>()?
								.join(",")
						)
						.into(),
						"CD" => {
							// TODO: begin{CD}
							format!("CD({})", scanner.until_string(&format!("\\end{{{}}}", param))).into()
						}
						"matrix" => matrix!(scanner, param, "#none").into(),
						"matrix*" => matrix_opt!(scanner, param, "#none").into(),
						"pmatrix" => matrix!(scanner, param, "\"(\"").into(),
						"pmatrix*" => matrix_opt!(scanner, param, "\"(\"").into(),
						"rcases" => format!(
							"cases(reverse: #true, {})",
							scanner
								.until_string(&format!("\\end{{{}}}", param))
								.split("\\\\")
								.map(|s| latex_to_typst(s.into()))
								.collect::<Result<Vec<_>, _>>()?
								.join(",")
						)
						.into(),
						"smallmatrix" => format!(
							"inline(mat(delim: #none, {}))",
							matrix_to_typst(scanner.until_string(&format!("\\end{{{}}}", param)))?
						)
						.into(),
						"Vmatrix" => matrix!(scanner, param, "\"||\"").into(),
						"Vmatrix*" => matrix_opt!(scanner, param, "\"||\"").into(),
						"vmatrix" => matrix!(scanner, param, "\"|\"").into(),
						"vmatrix*" => matrix_opt!(scanner, param, "\"|\"").into(),
						_ => unreachable!(),
					}
				}
				"between" => "≬".into(),
				"bigcap" => "sect.big".into(),
				"bigcirc" => "circle.stroked.big".into(),
				"bigcup" => "union.big".into(),
				"bigdot" => "dot.circle.big".into(),
				"bigoplus" => "plus.circle.big".into(),
				"bigotimes" => "times.circle.big".into(),
				"bigsqcup" => "union.sq.big".into(),
				"bigstar" => "star.stroked".into(),
				"bigtriangledown" => "triangle.stroked.b".into(),
				"bigtriangleup" | "vartriangle" | "triangle" => "triangle.stroked.t".into(),
				"biguplus" => "union.plus.big".into(),
				"bigvee" => "or.big".into(),
				"bigwedge" => "and.big".into(),
				"binom" => double!(scanner, "binom").into(),
				"blacklozenge" => "lozenge.filled".into(),
				"blacksquare" => "square.filled".into(),
				"blacktriangle" => "triangle.filled.t".into(),
				"blacktriangledown" => "triangle.filled.b".into(),
				"blacktriangleleft" => "triangle.filled.l".into(),
				"blacktriangleright" => "triangle.filled.r".into(),
				"bm" | "bold" | "boldsymbol" => single!(scanner, "bold").into(),
				"bmod" | "pmod" => "mod".into(),
				"bowtie" | "Join" => "⋈".into(),
				"Box" => "square.stroked".into(),
				"boxdot" => "dot.square".into(),
				"boxed" => format!(
					"#box(inset: (left: 3pt, right: 3pt), outset: (top: 3pt, bottom: 3pt), stroke: 0.5pt)[${}$]",
					latex_to_typst(scanner.next_param()?.into())?
				)
				.into(),
				"boxminus" => "minus.square".into(),
				"boxplus" => "plus.square".into(),
				"boxtimes" => "times.square".into(),
				"Bra" | "bra" => format!("lr(angle.l {} |)", latex_to_typst(scanner.next_param()?.into())?).into(),
				"Braket" | "braket" => format!(
					"lr(angle.l {} angle.r)",
					latex_to_typst(scanner.next_param()?.into())?.replace('|', "mid(|)")
				)
				.into(),
				"breve" | "u" => single!(scanner, "breve").into(),
				"bull" | "bullet" => "circle.filled.small".into(),
				"Bumpeq" => "≎".into(),
				"bumpeq" => "≏".into(),
				// C
				"cancel" => single!(scanner, "cancel").into(),
				"Cap" | "doublecap" => "sect.double".into(),
				"cap" => "sect".into(),
				"cdot" | "cdotp" | "centerdot" | "sdot" => "dot.op".into(),
				"cfrac" => double!(scanner, "display", "frac").into(),
				"char" => {
					let code = match scanner.peek().unwrap() {
						'"' => {
							scanner.next();
							scanner.until_chars_not("0123456789abcdefABCDEF")
						}
						'\'' => {
							scanner.next();
							format!(
								"{:x}",
								u32::from_str_radix(&scanner.until_chars_not("01234567"), 8).unwrap()
							)
						}
						_ => format!("{:x}", scanner.until_chars_not("0123456789").parse::<u32>().unwrap()),
					};
					// scanner.cursor -= 1;
					format!("\\u{{{}}}", code).into()
				}
				"cdots" | "dots" | "dotsb" | "dotsc" | "dotsi" | "dotsm" => "dots.h.c".into(),
				"check" | "V" | "widecheck" => single!(scanner, "caron").into(),
				"circ" => "compose".into(),
				"circeq" => "≗".into(),
				"circlearrowleft" => "arrow.ccw".into(),
				"circlearrowright" => "arrow.cw".into(),
				"circledast" => "ast.circle".into(),
				"circledcirc" => "circle.nested".into(),
				"circleddash" => "dash.circle".into(),
				"circledR" => "®".into(),
				"circledS" => "Ⓢ".into(),
				"clubs" | "clubsuit" => "suit.club".into(),
				"cnums" => "CC".into(),
				"Colonapprox" => "::approx".into(),
				"colonapprox" => ":approx".into(),
				"coloncolon" => "::".into(),
				"coloncolonapprox" => "::approx".into(),
				"coloncolonequals" | "Coloneqq" => "::=".into(),
				"coloncolonminus" | "Coloneq" => "\"::−\"".into(),
				"coloncolonsim" | "Colonsim" => "::tilde.op".into(),
				"coloneq" | "colonminus" => "\":−\"".into(),
				"colonequals" | "coloneqq" => ":=".into(),
				"colonsim" => ":tilde.op".into(),
				"colorbox" => format!(
					"#box(inset: (left: 3pt, right: 3pt), outset: (top: 3pt, bottom: 3pt), fill: {})[{}]",
					color_to_typst(&scanner.next_param()?),
					text_to_typst(&scanner.next_param()?)?
				)
				.into(),
				"complexes" => "CC".into(),
				"cong" => "tilde.equiv".into(),
				"cosec" => "#math.op(\"cosec\")".into(),
				"cotg" => "#math.op(\"cotg\")".into(),
				"cth" => "#math.op(\"cth\")".into(),
				"Cup" | "doublecup" => "union.double".into(),
				"cup" => "union".into(),
				"curlyeqprec" => "eq.prec".into(),
				"curlyeqsucc" => "eq.succ".into(),
				"curlyvee" => "or.curly".into(),
				"curlywedge" => "and.curly".into(),
				"curvearrowleft" => "arrow.ccw.half".into(),
				"curvearrowright" => "arrow.cw.half".into(),
				// D
				"dag" => "dagger".into(),
				"Dagger" | "ddag" | "ddagger" => "dagger.double".into(),
				"daleth" => "ℸ".into(),
				"Darr" | "dArr" | "Downarrow" => "arrow.b.double".into(),
				"darr" | "downarrow" => "arrow.b".into(),
				"dashleftarrow" => "arrow.l.dash".into(),
				"dashrightarrow" => "arrow.r.dash".into(),
				"dashv" => "tack.l".into(),
				"dbinom" => double!(scanner, "display", "binom").into(),
				"dbcolon" => "::".into(),
				"ddot" => single!(scanner, "dot.double").into(),
				"ddots" => "dots.down".into(),
				"digaamma" => "ϝ".into(),
				"dfrac" => double!(scanner, "display", "frac").into(),
				"diagdown" => "╲".into(),
				"diagup" => "╱".into(),
				"Diamond" => "lozenge.stroked".into(),
				"diamond" => "diamond.stroked.small".into(),
				"diamonds" | "diamondsuit" => "♢".into(),
				"displaystyle" => single!(scanner, "display").into(),
				"divideontimes" => "times.div".into(),
				"dot" => single!(scanner, "dot").into(),
				"Doteq" | "doteqdot" => "≑".into(),
				"doteq" => "≐".into(),
				"dotplus" => "plus.dot".into(),
				"dotso" | "ldots" | "mathellipsis" => "...".into(),
				"doublebarwedge" => "⩞".into(),
				"downdownarrows" => "arrows.bb".into(),
				"downharpoonleft" => "harpoon.bl".into(),
				"downharpoonright" => "harpoon.br".into(),
				// E
				"ell" => "cal(l)".into(),
				"empty" | "emptyset" => "empty".into(),
				"enspace" => "space.en".into(),
				"epsilon" => "epsilon.alt".into(),
				"eqcirc" => "≖".into(),
				"Eqcolon" | "minuscoloncolon" => "\"−::\"".into(),
				"eqcolon" | "minuscolon" => "dash.colon".into(),
				"Eqqcolon" | "equalscoloncolon" => "\"=::\"".into(),
				"eqqcolon" | "equalscolon" => "=:".into(),
				"eqsim" => "eq.tilde".into(),
				"eqslantgtr" => "⪖".into(),
				"eqslantless" => "⪕".into(),
				"eth" => "ð".into(),
				"exist" => "exists".into(),
				// F
				"fallingdotseq" => "≒".into(),
				"fbox" => format!(
					"#box(inset: (left: 3pt, right: 3pt), outset: (top: 3pt, bottom: 3pt))[$upright({})$]",
					text_to_typst(&scanner.next_param()?)?
				)
				.into(),
				"fcolorbox" => format!(
					"#box(inset: (left: 3pt, right: 3pt), outset: (top: 3pt, bottom: 3pt))(stroke: {}, fill: {})[$upright({})$]",
					color_to_typst(&scanner.next_param()?),
					color_to_typst(&scanner.next_param()?),
					text_to_typst(&scanner.next_param()?)?
				)
				.into(),
				"Finv" => "Ⅎ".into(),
				"flat" => "♭".into(),
				"frac" => double!(scanner, "frac").into(),
				"frak" | "mathfrak" => single!(scanner, "frak").into(),
				"frown" => "⌢".into(),
				// G
				"Game" => "⅁".into(),
				"ge" | "geq" => ">=".into(),
				"geqq" => "ge.equiv".into(),
				"geqslant" => "gt.eq.slant".into(),
				"gets" | "larr" | "leftarrow" => "<-".into(),
				"gg" => ">>".into(),
				"ggg" | "gggtr" => ">>>".into(),
				"gnapprox" => "⪊".into(),
				"gneq" => "⪈".into(),
				"gneqq" => "gt.nequiv".into(),
				"gnsim" => "gt.ntilde".into(),
				"grave" => single!(scanner, "grave").into(),
				"gt" => ">".into(),
				"gtapprox" => "⪆".into(),
				"gtreqless" => "gt.eq.lt".into(),
				"gtreqqless" => "⪌".into(),
				"gtrless" => "gt.lt".into(),
				"gtrsim" => "gt.tilde".into(),
				// H
				"H" => single!(scanner, "acute.double").into(),
				"Harr" | "hArr" | "Leftrightarrow" | "Lrarr" | "lrArr" => "<=>".into(),
				"harr" | "leftrightarrow" | "lrarr" => "<->".into(),
				"hat" | "widehat" => single!(scanner, "hat").into(),
				"hbar" | "hslash" => "planck.reduce".into(),
				"hbox" | "mathnormal" | "mathop" => latex_to_typst(scanner.next_param()?.into())?,
				"hearts" | "heartsuit" => "♡".into(),
				"hookleftarrow" => "arrow.l.hook".into(),
				"hookrightarrow" => "arrow.r.hook".into(),
				"hphantom" => format!(
					"#box(height: 0pt, hide[${}$])",
					latex_to_typst(scanner.next_param()?.into())?
				)
				.into(),
				"hspace" | "mskip" => single!(scanner, "#h").into(),
				// I
				"i" | "imath" => "dotless.i".into(),
				"iff" | "Longleftrightarrow" => "<==>".into(),
				"iiint" => "integral.triple".into(),
				"iint" => "integral.double".into(),
				"image" => "Im".into(),
				"impliedby" | "Longleftarrow" => "<==".into(),
				"implies" => "==>".into(),
				"infin" | "infty" => "infinity".into(),
				"injlim" => "#math.op(\"inj\\u{{2009}}lim\", limits: true)".into(),
				"int" | "intop" => "integral".into(),
				"intercal" => "⊺".into(),
				"isin" => "in".into(),
				// JK
				"j" | "jmath" => "dotless.j".into(),
				"KaTeX" => "\"KaTeX\"".into(),
				"Ket" | "ket" => format!("lr(| {} angle.r)", latex_to_typst(scanner.next_param()?.into())?).into(),
				// L
				"lang" | "langle" => "angle.l".into(),
				"Larr" | "lArr" | "Leftarrow" => "arrow.l.double".into(),
				"LaTeX" => "\"LaTeX\"".into(),
				"lBrace" => "⦃".into(),
				"lbrace" => "{".into(),
				"lbrack" => "[".into(),
				"lceil" => "⌈".into(),
				"ldotp" => ".".into(),
				"le" | "leq" => "<=".into(),
				"leadsto" => "arrow.r.squiggly".into(),
				"left" => format!(
					"lr({}{}{})",
					scanner.next_param()?,
					latex_to_typst(scanner.until_string("\\right").into())?,
					scanner.next_param()?,
				)
				.into(),
				"lfloor" => "⌊".into(),
				"lgroup" => "turtle.l".into(),
				"lhd" | "vartriangleleft" => "lt.tri".into(),
				"limits" | "nolimits" => "".into(),
				"ll" => "<<".into(),
				"llbracket" => "bracket.l.double".into(),
				"llcorner" => "⌞".into(),
				"Lleftarrow" => "arrow.l.triple".into(),
				"lll" | "llless" => "<<<".into(),
				"lnapprox" => "⪉".into(),
				"lneq" => "⪇".into(),
				"lneqq" => "lt.nequiv".into(),
				"lnot" => "not".into(),
				"lnsim" => "lt.ntilde".into(),
				"longleftarrow" => "<--".into(),
				"longleftrightarrow" => "<-->".into(),
				"longmapsto" => "arrow.r.long.bar".into(),
				"Longrightarrow" => "==>".into(),
				"longrightarrow" => "-->".into(),
				"looparrowleft" => "arrow.l.loop".into(),
				"looparrowright" => "arrow.r.loop".into(),
				"lor" | "vee" => "or".into(),
				"lozenge" => "lozenge.stroked".into(),
				"lparen" => "(".into(),
				"lrcorner" => "⌟".into(),
				"lq" => "quote.l.single".into(),
				"Lsh" => "↰".into(),
				"lt" => "<".into(),
				"ltimes" => "times.l".into(),
				"lVert" | "rVert" | "Vert" => "parallel".into(),
				"lvert" | "rvert" | "vert" => "divides".into(),
				// M
				"mapsto" => "arrow.r.bar".into(),
				"mathbb" => single!(scanner, "bb").into(),
				"mathbf" => single!(scanner, "bold").into(),
				"mathbin" => format!("#math.op(\"{}\")", typ::escape_string(&scanner.next_param()?)).into(),
				"mathcal" => single!(scanner, "cal").into(),
				"mathclap" => single!(scanner, "#box", "width: 0pt").into(),
				"mathclose" => format!("#h(0pt) {}", latex_to_typst(scanner.next_param()?.into())?).into(),
				"mathit" => single!(scanner, "italic").into(),
				"mathopen" => format!("{} #h(0pt)", latex_to_typst(scanner.next_param()?.into())?).into(),
				"mathring" | "r" => single!(scanner, "circle").into(),
				"mathrm" => single!(scanner, "upright").into(),
				"mathsf" => single!(scanner, "sans").into(),
				"mathsterling" => "pound".into(),
				"mathstrut" => "#hide(box(width: 0pt, \")\"))".into(),
				"measuredangle" => "angle.arc".into(),
				"medspace" => "space.med".into(),
				"mho" => "ohm.inv".into(),
				"mid" => "|".into(),
				"middle" => single!(scanner, "mid").into(),
				"minuso" => "⊖".into(),
				"models" | "vDash" => "tack.r.double".into(),
				"mp" => "minus.plus".into(),
				// N
				"N" | "natnums" => "NN".into(),
				"natural" => "♮".into(),
				"negmedspace" => "#h(-2em/9)".into(),
				"ncong" => "tilde.equiv.not".into(),
				"ne" | "neq" => "!=".into(),
				"nearrow" => "arrow.tr".into(),
				"neg" => "not".into(),
				"negthickspace" => "#h(-5em/18)".into(),
				"negthinspace" => "#h(-1em/6)".into(),
				"nexist" => "exists.not".into(),
				"ngeq" => "gt.eq.not".into(),
				"ngtr" => "gt.not".into(),
				"ni" | "owns" => "in.rev".into(),
				"nLeftarrow" => "arrow.l.double.not".into(),
				"nleftarrow" => "arrow.l.not".into(),
				"nLeftrightarrow" => "arrow.l.r.double.not".into(),
				"nleftrightarrow" => "arrow.l.r.not".into(),
				"nleq" => "lt.eq.not".into(),
				"nless" => "lt.not".into(),
				"nmid" => "divides.not".into(),
				"nobreakspace" => "space.nobreak".into(),
				"notin" => "in.not".into(),
				"notni" => "in.rev.not".into(),
				"notparallel" => "parallel.not".into(),
				"nprec" => "prec.not".into(),
				"npreceq" => "prec.eq.not".into(),
				"nRightarrow" => "arrow.r.double.not".into(),
				"nrightarrow" => "arrow.r.not".into(),
				"nsim" => "tilde.not".into(),
				"nsubseteq" | "nsupseteq" => "subset.eq.not".into(),
				"nsucc" => "succ.not".into(),
				"nsucceq" => "succ.eq.not".into(),
				"ntriangleleft" => "lt.tri.not".into(),
				"ntrianglelefteq" => "lt.tri.eq.not".into(),
				"ntriangleright" => "gt.tri.not".into(),
				"ntrianglerighteq" => "gt.tri.eq.not".into(),
				"nVDash" => "⊯".into(),
				"nVdash" => "⊮".into(),
				"nvDash" => "tack.r.double.not".into(),
				"nvdash" => "tack.r.not".into(),
				"nwarrow" => "arrow.tl".into(),
				// O
				"O" => "Ø".into(),
				"o" => "ø".into(),
				"odot" => "dot.circle".into(),
				"OE" => "Œ".into(),
				"oe" => "œ".into(),
				"oiiint" => "integral.vol".into(),
				"oiint" => "integral.surf".into(),
				"oint" => "integral.cont".into(),
				"ominus" => "minus.circle".into(),
				"operatorname" => format!("#math.op(\"{}\")", typ::escape_string(&scanner.next_param()?)).into(),
				"operatorname*" | "operatornamewithlimits" => format!(
					"#math.op(\"{}\", limits: true)",
					typ::escape_string(&scanner.next_param()?)
				)
				.into(),
				"oplus" => "plus.circle".into(),
				"origof" => "⊶".into(),
				"oslash" => "⊘".into(),
				"otimes" => "times.circle".into(),
				"overbrace" => {
					let param1 = latex_to_typst(scanner.next_param()?.into())?;
					match scanner.peek() {
						Some('^') => {
							scanner.next();
							format!(
								"overbrace({}, {})",
								param1,
								latex_to_typst(scanner.next_param()?.into())?
							)
							.into()
						}
						_ => format!("overbrace({})", param1).into(),
					}
				}
				"overgroup" => accent!(scanner, "\\u{{0311}}").into(),
				"overleftarrow" => single!(scanner, "arrow.l").into(),
				"overleftharpoon" => accent!(scanner, "harpoon.lt").into(),
				"overleftrightarrow" => accent!(scanner, "arrow.l.r").into(),
				"overline" => single!(scanner, "overline").into(),
				"overlinesegment" => accent!(scanner, "\\u{{20e9}}").into(),
				"overrightarrow" | "vec" => single!(scanner, "arrow").into(),
				"overrightharpoon" => accent!(scanner, "harpoon.rt").into(),
				// P
				"P" => "pilcrow".into(),
				"partial" => "diff".into(),
				"perp" => "bot".into(),
				"phantom" => format!("#hide[${}$]", latex_to_typst(scanner.next_param()?.into())?).into(),
				"phi" => "phi.alt".into(),
				"pitchfork" => "⋔".into(),
				"plim" => "#math.op(\"plim\", limits: true)".into(),
				"plusmn" | "pm" => "plus.minus".into(),
				"pounds" => "pound".into(),
				"precapprox" => "prec.approx".into(),
				"preccurlyeq" => "prec.eq".into(),
				"preceq" => "⪯".into(),
				"precnapprox" => "prec.napprox".into(),
				"precneqq" => "prec.nequiv".into(),
				"precnsim" => "prec.ntilde".into(),
				"precsim" => "prec.tilde".into(),
				"prime" | "rq" => "'".into(),
				"prod" => "product".into(),
				"projlim" => "#math.op(\"proj\\u{{2009}}lim\", limits: true)".into(),
				"propto" | "varpropto" => "prop".into(),
				// QR
				"qquad" => "#h(2em)".into(),
				"quad" => "space.quad".into(),
				"R" => "RR".into(),
				"raisebox" => format!(
					"#text(baseline: -{})[{}]",
					scanner.next_param()?,
					latex_to_typst(scanner.next_param()?.into())?
				)
				.into(),
				"rang" | "rangle" => "angle.r".into(),
				"Rarr" | "rArr" | "Rightarrow" => "=>".into(),
				"rarr" | "rightarrow" | "to" => "->".into(),
				"ratio" => ":".into(),
				"rBrace" => "⦄".into(),
				"rbrace" => "}".into(),
				"rbrack" => "]".into(),
				"rceil" => "⌉".into(),
				"Reals" | "reals" => "RR".into(),
				"restriction" => "harpoon.tr".into(),
				"rfloor" => "⌋".into(),
				"rgroup" => "turtle.r".into(),
				"rhd" | "vartriangleright" => "gt.tri".into(),
				"rightarrowtail" => ">->".into(),
				"rightharpoondown" => "harpoon.rb".into(),
				"rightharpoonup" => "harpoon.rt".into(),
				"rightleftarrows" => "arrows.rl".into(),
				"rightleftharpoons" => "harpoons.rtlb".into(),
				"rightrightarrows" => "arrows.rr".into(),
				"rightsquigarrow" => "arrow.r.squiggly".into(),
				"rightthreetimes" => "times.three.r".into(),
				"risingdotseq" => "≓".into(),
				"rmoustache" => "⎱".into(),
				"rparen" => ")".into(),
				"rrbracket" => "bracket.r.double".into(),
				"Rrightarrow" => "arrow.r.triple".into(),
				"Rsh" => "↱".into(),
				"rtimes" => "times.r".into(),
				"rule" => match scanner.next_param_optional().as_str() {
					"" => format!(
						"#box(fill: black, width: {}, height: {})",
						scanner.next_param()?,
						scanner.next_param()?
					)
					.into(),
					p => format!(
						"#box(inset: (bottom: {}), box(fill: black, width: {}, height: {}))",
						p,
						scanner.next_param()?,
						scanner.next_param()?,
					)
					.into(),
				},
				// S
				"S" | "sect" => "section".into(),
				"searrow" => "arrow.br".into(),
				"Set" | "set" => format!("{{{}}}", latex_to_typst(scanner.next_param()?.into())?).into(),
				"setminus" | "smallsetminus" => "without".into(),
				"sharp" => "♯".into(),
				"sim" => "tilde.op".into(),
				"simcolon" => "tilde.op:".into(),
				"simcoloncolon" => "tilde.op::".into(),
				"simeq" => "tilde.eq".into(),
				"sh" => "#math.op(\"sh\")".into(),
				"smallint" => "inline(integral)".into(),
				"smallsmile" => "⌣".into(),
				"sout" => single!(scanner, "cancel", "angle: #90deg").into(),
				"spades" | "spadesuit" => "suit.spade".into(),
				"sphericalangle" => "angle.spheric".into(),
				"sqcap" => "sect.sq".into(),
				"sqcup" => "union.sq".into(),
				"square" => "square.stroked".into(),
				"sqrt" => {
					let p = scanner.next_param_optional();
					match p.as_str() {
						"" => single!(scanner, "sqrt").into(),
						_ => format!(
							"root({}, {})",
							latex_to_typst(p.into())?,
							latex_to_typst(scanner.next_param()?.into())?
						)
						.into(),
					}
				}
				"sqsubset" => "subset.sq".into(),
				"sqsubseteq" => "subset.eq.sq".into(),
				"sqsupset" => "superset.sq".into(),
				"sqsupseteq" => "superset.eq.sq".into(),
				"ss" => "ß".into(),
				"star" => "star.op".into(),
				"sub" => "subset".into(),
				"sube" | "subseteq" => "subset.eq".into(),
				"Subset" => "subset.double".into(),
				"subseteqq" => "⫅".into(),
				"subsetneq" | "varsubsetneq" => "subset.neq".into(),
				"subsetneqq" | "varsubsetneqq" => "⫋".into(),
				"succapprox" => "succ.approx".into(),
				"succcurlyeq" => "succ.eq".into(),
				"succeq" => "⪰".into(),
				"succnapprox" => "succ.napprox".into(),
				"succneqq" => "succ.nequiv".into(),
				"succnsim" => "succ.ntilde".into(),
				"supe" | "supseteq" => "supset.eq".into(),
				"Supset" => "superset.double".into(),
				"supseteqq" => "⫆".into(),
				"supsetneq" | "varsupsetneq" => "superset.neq".into(),
				"supsetneqq" | "varsupsetneqq" => "⫌".into(),
				"surd" => "√".into(),
				"swarrow" => "arrow.bl".into(),
				// T
				"tbinom" => double!(scanner, "inline", "binom").into(),
				"TeX" => "\"TeX\"".into(),
				"text" | "textmd" | "textnormal" | "textrm" | "textup" => {
					format!("#[{}]", text_to_typst(&scanner.next_param()?)?).into()
				}
				"textbf" => format!("bold(#[{}])", text_to_typst(&scanner.next_param()?)?).into(),
				"textcolor" => format!(
					"#text(fill: {})[{}]",
					color_to_typst(&scanner.next_param()?),
					text_to_typst(&scanner.next_param()?)?
				)
				.into(),
				"textit" => format!("italic(#[{}])", text_to_typst(&scanner.next_param()?)?).into(),
				"textsf" => format!("sans(#[{}])", text_to_typst(&scanner.next_param()?)?).into(),
				"textstyle" => format!("inline({})", latex_to_typst(scanner.next_param()?.into())?).into(),
				"texttt" => format!("mono(#[{}])", text_to_typst(&scanner.next_param()?)?).into(),
				"tfrac" => double!(scanner, "inline", "frac").into(),
				"th" => "#math.op(\"th\")".into(),
				"thetasym" => "theta.alt".into(),
				"thickapprox" => "bold(approx)".into(),
				"thicksim" => "bold(tilde)".into(),
				"thickspace" => "#h(5em/18)".into(),
				"thinspace" => "space.sixth".into(),
				"tilde" | "widetilde" => single!(scanner, "tilde").into(),
				"triangledown" => "triangle.stroked.b".into(),
				"triangleleft" => "triangle.stroked.l".into(),
				"trianglelefteq" => "lt.tri.eq".into(),
				"triangleq" => "eq.delta".into(),
				"triangleright" => "triangle.stroked.r".into(),
				"trianglerighteq" => "gt.tri.eq".into(),
				"twoheadleftarrow" => "<<-".into(),
				"twoheadrightarrow" => "->>".into(),
				// U
				"Uarr" | "uArr" | "Uparrow" => "arrow.t.double".into(),
				"uarr" | "uparrow" => "arrow.t".into(),
				"ulcorner" => "⌜".into(),
				"underbar" | "underline" => single!(scanner, "underline").into(),
				"underbrace" => {
					let param1 = latex_to_typst(scanner.next_param()?.into())?;
					match scanner.peek() {
						Some('_') => {
							scanner.next();
							format!(
								"underbrace({}, {})",
								param1,
								latex_to_typst(scanner.next_param()?.into())?
							)
							.into()
						}
						_ => format!("underbrace({})", param1).into(),
					}
				}
				"undergroup" => accent!(scanner, "\\u{{032e}}").into(),
				"underleftrightarrow" => accent!(scanner, "\\u{{034d}}").into(),
				"unlhd" => "lt.tri.eq".into(),
				"unrhd" => "gt.tri.eq".into(),
				"Updownarrow" => "arrow.t.b.double".into(),
				"updownarrow" => "arrow.t.b".into(),
				"upharpoonleft" => "harpoon.tl".into(),
				"upharpoonright" => "harpoon.tr".into(),
				"uplus" => "union.plus".into(),
				"upuparrows" => "arrows.tt".into(),
				"urcorner" => "⌝".into(),
				// V
				"varDelta" => "italic(Delta)".into(),
				"varepsilon" => "italic(epsilon)".into(),
				"varGamma" => "italic(Gamma)".into(),
				"varkappa" => "italic(kappa)".into(),
				"varnothing" => "italic(nothing)".into(),
				"varOmega" => "italic(Omega)".into(),
				"varPhi" => "italic(Phi)".into(),
				"varphi" => "italic(phi)".into(),
				"varPi" => "italic(Pi)".into(),
				"varpi" => "italic(pi.alt)".into(),
				"varPsi" => "italic(Psi)".into(),
				"varrho" => "italic(rho.alt)".into(),
				"varSigma" => "italic(Sigma)".into(),
				"varsigma" => "italic(sigma.alt)".into(),
				"varTheta" => "italic(Theta)".into(),
				"vartheta" => "italic(theta.alt)".into(),
				"varUpsilon" => "italic(Upsilon)".into(),
				"varXi" => "italic(Xi)".into(),
				"vcentcolon" => ":".into(),
				"Vdash" => "⊩".into(),
				"vdash" => "tack.r".into(),
				"vdots" => "dots.v".into(),
				"veebar" => "⊻".into(),
				"vphantom" => format!(
					"#box(width: 0pt, hide[${}$])",
					latex_to_typst(scanner.next_param()?.into())?
				)
				.into(),
				"Vvdash" => "⊪".into(),
				// W
				"wedge" | "land" => "and".into(),
				"weierp" | "wp" => "℘".into(),
				"wr" => "wreath".into(),
				// X
				"xcancel" => single!(scanner, "cancel", "cross: #true").into(),
				"xhookleftarrow" => single!(scanner, "xarrow", "sym: arrow.l.hook").into(),
				"xhookrightarrow" => single!(scanner, "xarrow", "sym: arrow.r.hook").into(),
				"xLeftarrow" => single!(scanner, "xarrow", "sym: arrow.l.double").into(),
				"xleftarrow" => single!(scanner, "xarrow", "sym: arrow.l").into(),
				"xleftharpoondown" => single!(scanner, "xarrow", "sym: harpoon.lb").into(),
				"xleftharpoonup" => single!(scanner, "xarrow", "sym: harpoon.lt").into(),
				"xLeftrightarrow" => single!(scanner, "xarrow", "sym: arrow.l.r.double").into(),
				"xleftrightarrow" => single!(scanner, "xarrow", "sym: arrow.l.r").into(),
				"xleftrightharpoons" => single!(scanner, "xarrow", "sym: harpoons.ltrb").into(),
				"xlongequal" => single!(scanner, "xarrow", "sym: equal").into(),
				"xmapsto" => single!(scanner, "xarrow", "sym: arrow.r.bar").into(),
				"xRightarrow" => single!(scanner, "xarrow", "sym: arrow.r.double").into(),
				"xrightarrow" => single!(scanner, "xarrow", "sym: arrow.r").into(),
				"xrightharpoondown" => single!(scanner, "xarrow", "sym: harpoon.rb").into(),
				"xrightharpoonup" => single!(scanner, "xarrow", "sym: harpoon.rt").into(),
				"xrightleftharpoons" => single!(scanner, "xarrow", "sym: harpoons.rtlb").into(),
				"xtwoheadleftarrow" => single!(scanner, "xarrow", "sym: arrow.l.twohead").into(),
				"xtwoheadrightarrow" => single!(scanner, "xarrow", "sym: arrow.r.twohead").into(),
				// YZ
				"Z" => "ZZ".into(),
				word => Cow::Owned(word.to_string()),
			},
			_ if BINARY_OPERATORS.contains(&c) => match scanner.peek() {
				Some('{') => format!("{}({})", c, latex_to_typst(scanner.next_param()?.into())?).into(),
				Some(&next) => {
					scanner.next();
					format!("{}{}", c, next).into()
				}
				_ => unreachable!(),
			},
			'%' => format!("//{}\n", scanner.until_chars("\n")).into(),
			'~' => "space.nobreak".into(),
			'/' | '"' => format!("\\{}", c).into(),
			'{' | '}' => "".into(),
			_ => c.to_string().into(),
		};
		// Insert space if current and next character is an alphabetic character
		if let Some(first) = push.chars().next() {
			if let Some(prev) = text.chars().last() {
				if prev.is_alphabetic() && (first.is_alphabetic() || first.is_ascii_digit()) {
					text.push(' ');
				}
			}
		}
		text += &push;
	}

	Ok(text.into())
}

fn color_to_typst(color: &str) -> Cow<str> {
	if color.starts_with('#') {
		format!("rgb(\"{}\")", color).into()
	} else {
		color.into()
	}
}

pub fn text_to_typst(text: &str) -> Result<String, ScannerError> {
	let mut scanner = Scanner::new(text);
	let mut ret = String::new();
	while let Some(c) = scanner.next() {
		let push: Cow<str> = match c {
			'\\' => match scanner.next_word().as_str() {
				"textasciitilde" => "~".into(),
				"textasciicircum" => "\\^".into(),
				"textbackslash" => "\\\\".into(),
				"textbar" => "|".into(),
				"textbardbl" => "‖".into(),
				"textbraceleft" => "{".into(),
				"textbraceright" => "}".into(),
				"textdagger" => "#sym.dagger".into(),
				"textdaggerdbl" => "#sym.dagger.double".into(),
				"textdegree" => "#sym.degree".into(),
				"textdollar" => "\\$".into(),
				"textellipsis" => "...".into(),
				"textemdash" => "---".into(),
				"textendash" => "--".into(),
				"textgreater" => "#sym.gt".into(),
				"textless" => "#sym.lt".into(),
				"textquotedblleft" => "#sym.quote.l.double".into(),
				"textquotedblright" => "#sym.quote.r.double".into(),
				"textquoteleft" => "#sym.quote.l.single".into(),
				"textquoteright" => "#sym.quote.r.single".into(),
				"textregistered" => "®".into(),
				"textsterling" => "#sym.pound".into(),
				"textunderscore" => "\\_".into(),
				word => word.to_owned().into(),
			},
			'$' => {
				let mut math = String::new();
				for c in scanner.by_ref() {
					if c == '$' {
						break;
					}
					math.push(c);
				}
				format!("${}$", latex_to_typst(math.into())?).into()
			}
			_ => c.to_string().into(),
		};
		ret += &push;
	}
	Ok(ret)
}

/// split with `\\`, then split with `&` (not `\&`), finally process element by element
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
fn matrix_to_typst(content: String) -> Result<String, ScannerError> {
	Ok(Regex::new(r"\\\\|\\cr")
		.unwrap()
		.split(&content)
		.map(|row| {
			let mut s = row.split('&').map(|s| s.into()).collect::<Vec<String>>();
			let mut i = 0;
			while i < s.len() {
				if s[i].ends_with('\\') {
					let temp = s[i + 1].clone();
					s[i].push('&');
					s[i] += &temp;
					s.remove(i + 1);
				}
				i += 1;
			}
			Ok(s.iter()
				.map(|col| latex_to_typst(col.into()))
				.collect::<Result<Vec<_>, _>>()?
				.join(","))
		})
		.collect::<Result<Vec<String>, _>>()?
		.join(";"))
}

#[cfg(test)]
mod function_tests {
	use super::*;

	#[test]
	fn next_word_test() {
		let mut scanner = Scanner::new("\n\\frac\t\\land\\abc");
		let mut count = 0;
		let assert = ["frac", "land", "abc"];
		while let Some(c) = scanner.next() {
			if c == '\\' {
				let word = scanner.next_word();
				// println!("{}", word);
				assert_eq!(word, assert[count]);
				count += 1;
			}
		}
	}

	#[test]
	fn next_param_test() {
		let mut scanner = Scanner::new("\n\t\\land\\%=3aa\\\\");
		let mut count = 0;
		let assert = ["\\land", "\\%", "=", "3", "a", "a", "\\\\"];
		while let Ok(c) = scanner.next_param() {
			// println!("{}", c);
			assert_eq!(c, assert[count]);
			count += 1;
		}
	}

	#[test]
	fn color_test() {
		assert_eq!(color_to_typst("#00ff00"), "rgb(\"#00ff00\")");
		// println!("{}", color_to_typst("#00ff00".to_string()));
		assert_eq!(color_to_typst("red"), "red");
		// println!("{}", color_to_typst("red".to_string()));
	}

	#[test]
	fn matrix_test1() {
		assert_eq!(matrix_to_typst("a& b\\\\\nc& d".to_string()).unwrap(), "a, b;\nc, d")
	}

	#[test]
	fn matrix_test2() {
		assert_eq!(matrix_to_typst("a& b\\cr\nc& d".to_string()).unwrap(), "a, b;\nc, d")
	}
}
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_typ() {
		assert_eq!(
			latex_to_typst("\\bcancel{N = N_oe^{ln2(t/t_2)}}".into()).unwrap(),
			"cancel(inverted: #true, N = N_o e^(l n 2(t\\/t_2)))"
		);
		assert_eq!(latex_to_typst("hello".into()).unwrap(), "h e l l o");
		assert_eq!(
			latex_to_typst("\\binom {asdf}  {aas}".into()).unwrap(),
			"binom(a s d f, a a s)"
		);
		assert_eq!(
			latex_to_typst("\\overbrace{x+⋯+x}^{n\\text{ times$\\int$}}".into()).unwrap(),
			"overbrace(x+⋯+x, n#[ times$integral$])"
		);
		assert_eq!(latex_to_typst("\\dot\\sum _ 0 ^n".into()).unwrap(), "dot(sum_0^n)");
		assert_eq!(
			latex_to_typst("\\frac54 = 1\\tfrac   {1}   {4}\\\\".into()).unwrap(),
			"frac(5, 4) = 1inline(frac(1, 4))\\"
		);
		assert_eq!(
			latex_to_typst("h\\raisebox{2pt}{$\\psi ighe$}r".into()).unwrap(),
			"h#text(baseline: -2pt)[$psi i g h e$]r"
		);
		assert_eq!(
			latex_to_typst("\\left(\\frac{3}{2}\\middle| B\\right)".into()).unwrap(),
			"lr((frac(3, 2)mid(|) B))"
		)
	}

	#[test]
	fn matrix() {
		assert_eq!(
			latex_to_typst("\\begin{array}{cc}\na& b\\\\\nc& d\n\\end{array}".into()).unwrap(),
			"mat(delim: #none, \na, b;\nc, d\n)"
		);
		assert_eq!(
			latex_to_typst("\\begin{matrix}\na& b\\\\\nc& d\n\\end{matrix}".into()).unwrap(),
			"mat(delim: #none, \na, b;\nc, d\n)"
		);
	}
}
