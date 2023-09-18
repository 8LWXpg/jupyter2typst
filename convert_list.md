# Convert List

full list in [KaTeX](https://katex.org/docs/support_table)

## TODOs

- scripting
- begin
- lr
- binary
- not sure

## Symbols

| LaTeX    | Typst            |
| -------- | ---------------- |
| `!`      | `!`              |
| `\!`     | `#h(-1em/6)`     |
| `#`      | TODO#scripting   |
| `\#`     | `\#`             |
| `%`      | `//`             |
| `\%`     | `%`              |
| `&`      | TODO#begin       |
| `\&`     | `\&`             |
| `'`      | `'`              |
| `\'`     | `acute($1)`      |
| `(`      | `(`              |
| `)`      | `)`              |
| `\(…\)`  | TODO#not sure    |
| `\`      | `space`          |
| `\"`     | `dot.double($1)` |
| `\$`     | `$`              |
| `\,`     | `space.sixth`    |
| `\:`     | `#h(2em/9)`      |
| `\;`     | `#h(5em/18)`     |
| `_`      | `_`              |
| `\_`     | `\_`             |
| `` \` `` | `grave($1)`      |
| `<`      | `<`              |
| `\=`     | `macron($1)`     |
| `>`      | `>`              |
| `\>`     | `#h(2em/9)`      |
| `[`      | `[`              |
| `]`      | `]`              |
| `{}`     | TODO#not sure    |
| `\{`     | `{`              |
| `\}`     | `}`              |
| `\|`     | `\|`             |
| `\\|`    | `\|\|`           |
| `~`      | `space.nobreak`  |
| `\~`     | `tilde($1)`      |
| `^`      | `^`              |
| `\^`     | `hat($1)`        |

## A

| LaTeX               | Typst                |
| ------------------- | -------------------- |
| `\AA`               | `circle(A)`          |
| `\aa`               | `circle(a)`          |
| `\above`            | TODO#binary          |
| `\acute`            | `acute($1)`          |
| `\AE`               | `Æ`                  |
| `\ae`               | `æ`                  |
| `\alef`             | `alef`               |
| `\alefsym`          | `alef`               |
| `\aleph`            | `aleph`              |
| `\allowbreak`       | TODO#not sure        |
| `\Alpha`            | `Alpha`              |
| `\alpha`            | `alpha`              |
| `\amalg`            | `product.co`         |
| `\And`              | `\&`                 |
| `\angl`             | TODO#not sure        |
| `\angln`            | TODO#not sure        |
| `\angle`            | `angle`              |
| `\approx`           | `approx`             |
| `\approxeq`         | `approx.eq`          |
| `\approxcolon`      | `approx:`            |
| `\approxcoloncolon` | `approx::`           |
| `\arccos`           | `arccos`             |
| `\arcctg`           | `#math.op("arcctg")` |
| `\arcsin`           | `arcsin`             |
| `\arctan`           | `arctan`             |
| `\arg`              | `arg`                |
| `\argmax`           | `arg max`            |
| `\argmin`           | `arg min`            |
| `\arraystretch`     | TODO                 |
| `\ast`              | `*`                  |
| `\asymp`            | `≍`                  |
| `\atop`             | TODO#binary          |

## B

| LaTeX                   | Typst                       |
| ----------------------- | --------------------------- |
| `\backepsilon`          | `in.rev.small`              |
| `\backprime`            | `prime.rev`                 |
| `\backsim`              | `tilde.rev`                 |
| `\backsimeq`            | `tilde.eq.rev`              |
| `\backslash`            | `\\`                        |
| `\bar`                  | `macron($1)`                |
| `\barwedge`             | `⊼`                         |
| `\Bbb`                  | `bb($1)`                    |
| `\bcancel`              | `cancel(inverted: #true)`   |
| `\begin`                | TODO#begin                  |
| `\begingroup`           | TODO#begin                  |
| `Beta`                  | `Beta`                      |
| `\beta`                 | `beta`                      |
| `\beth`                 | `beth`                      |
| `\between`              | `≬`                         |
| `\bf`                   | TODO#font                   |
| `\big` and its variants | TODO#font                   |
| `\bigcap`               | `sect.big`                  |
| `\bigcirc`              | `circle.stroked.big`        |
| `\bigcup`               | `union.big`                 |
| `\bigdot`               | `dot.circle.big`            |
| `\bigplus`              | `plus.circle.big`           |
| `\bigtimes`             | `times.circle.big`          |
| `\bigsqcup`             | `union.square.big`          |
| `\bigstar`              | `star.stroked`              |
| `\bigtriangledown`      | `triangle.stroked.b`        |
| `\bigtriangleup`        | `triangle.stroked.t`        |
| `\biguplus`             | `union.plus.big`            |
| `\bigvee`               | `or.big`                    |
| `\bigwedge`             | `and.big`                   |
| `\binom`                | `binom($1, $2)`             |
| `\blacklozenge`         | `lozenge.filled`            |
| `\blacksquare`          | `square.filled`             |
| `\blacktriangle`        | `triangle.filled.t`         |
| `\blacktriangledown`    | `triangle.filled.b`         |
| `\blacktriangleleft`    | `triangle.filled.l`         |
| `\blacktriangleright`   | `triangle.filled.r`         |
| `\bm`                   | `bold($1)`                  |
| `\bmod`                 | `mod`                       |
| `\bold`                 | `bold($1)`                  |
| `\boldsymbol`           | `bold($1)`                  |
| `\bot`                  | `bot`                       |
| `\bowtie`               | `⋈`                         |
| `\Box`                  | `square.stroked`            |
| `\boxdot`               | `dot.square`                |
| `\boxed`                | `#box(stroke: 0.5pt)[$$1$]` |
| `\boxminus`             | `minus.square`              |
| `\boxplus`              | `plus.square`               |
| `\boxtimes`             | `times.square`              |
| `\bra` and its variants | TODO#lr                     |
| `\brace`                | TODO#binary                 |
| `\brack`                | TODO#binary                 |
| `\breve`                | `breve($1)`                 |
| `\bull`                 | `circle.filled.small`       |
| `\bullet`               | `circle.filled.small`       |
| `\Bumpeq`               | `≎`                         |
| `\bumpeq`               | `≏`                         |

## C

| LaTeX               | Typst                  |
| ------------------- | ---------------------- |
| `\cal`              | TODO#font              |
| `\cancel`           | `cancel($1)`           |
| `\Cap`              | `sect.double`          |
| `\cap`              | `sect`                 |
| `\cdot`             | `dot.op`               |
| `\cdotp`            | `dot.op`               |
| `\cdots`            | `dots.h.c`             |
| `\ce`               | not supported in ipynb |
| `\centerdot`        | `dot.op`               |
| `\cfrac`            | TODO#not sure          |
| `\char`             | TODO#not sure          |
| `\check`            | `caron($1)`            |
| `\ch`               | not supported in ipynb |
| `\Chi`              | `Chi`                  |
| `\chi`              | `chi`                  |
| `\choose`           | TODO#binary            |
| `\circ`             | `compose`              |
| `\circeq`           | `≗`                    |
| `\circlearrowleft`  | `arrow.ccw`            |
| `\circlearrowright` | `arrow.cw`             |
| `\circledast`       | `ast.circle`           |
| `\circledcirc`      | `circle.nested`        |
| `\circleddash`      | `dash.circle`          |
| `\circledR`         | `®`                    |
| `\circledS`         | `Ⓢ`                    |
| `\clubs`            | `suit.club`            |
| `\clubsuit`         | `suit.club`            |
| `\cnums`            | `CC`                   |
| `\colon`            | `colon`                |
| `\Colonapprox`      | `::approx`             |
| `\colonapprox`      | `:approx`              |
| `\coloncolon`       | `::`                   |
| `\coloncolonapprox` | `::approx`             |
| `\coloncolonequals` | `::=`                  |
| `\coloncolonminus`  | `"::−"`                |
| `\coloncolonsim`    | `"::~"`                |
| `\Coloneq`          | `"::−"`                |
| `\coloneq`          | `":-"`                 |
| `\colonequals`      | `:=`                   |
| `\Coloneqq`         | `::=`                  |
| `\coloneqq`         | `:=`                   |
| `\colonminus`       | `":−"`                 |
| `\Colonsim`         | `"::~"`                |
| `\colonsim`         | `":~"`                 |
| `\color`            | TODO#font              |
| `\colorbox`         | `#box(fill: $1)[$2]`   |
| `\complement`       | `complement`           |
| `\Complex`          | `CC`                   |
| `\cong`             | `tilde.equiv`          |
| `\coprod`           | `product.co`           |
| `\copyright`        | `copyright`            |
| `\cos`              | `cos`                  |
| `\cosec`            | `#math.op("cosec")`    |
| `\cosh`             | `cosh`                 |
| `\cot`              | `cot`                  |
| `\cotg`             | `#math.op("cotg")`     |
| `\coth`             | `coth`                 |
| `\cr`               | TODO#begin             |
| `\csc`              | `csc`                  |
| `\ctg`              | `ctg`                  |
| `\cth`              | `#math.op("cth")`      |
| `\Cup`              | `union.double`         |
| `\cup`              | `union`                |
| `\curlyeqprec`      | `eq.prec`              |
| `\curlyeqsucc`      | `eq.succ`              |
| `\curlyvee`         | `or.curly`             |
| `\curlywedge`       | `and.curly`            |
| `\curvearrowleft`   | `arrow.ccw.half`       |
| `\curvearrowright`  | `arrow.cw.half`        |

## D

| LaTeX               | Typst                   |
| ------------------- | ----------------------- |
| `\dag`              | `dagger`                |
| `\Dagger`           | `dagger.double`         |
| `\dagger`           | `dagger`                |
| `\daleth`           | `ℸ`                     |
| `\Darr`             | `arrow.b.double`        |
| `\dArr`             | `arrow.b.double`        |
| `\darr`             | `arrow.b`               |
| `\dashleftarrow`    | `arrow.l.dash`          |
| `\dashrightarrow`   | `arrow.r.dash`          |
| `\dashv`            | `tack.l`                |
| `\dbinom`           | `binom($1, $2)`         |
| `\dbcolon`          | `::`                    |
| `\ddag`             | `dagger.double`         |
| `\ddagger`          | `dagger.double`         |
| `\ddot`             | `dot.double($1)`        |
| `\ddots`            | `dots.down`             |
| `\def`              | TODO#scripting          |
| `\deg`              | `deg`                   |
| `\degree`           | `degree`                |
| `\Delta`            | `Delta`                 |
| `\delta`            | `delta`                 |
| `\det`              | `det`                   |
| `\digamma`          | `ϝ`                     |
| `\dfrac`            | `frac($1, $2)`          |
| `\diagdown`         | `╲`                     |
| `\diagup`           | `╱`                     |
| `\Diamond`          | `lozenge.stroked`       |
| `\diamond`          | `diamond.stroked.small` |
| `\diamonds`         | `♢`                     |
| `\diamondsuit`      | `♢`                     |
| `\dim`              | `dim`                   |
| `\displaystyle`     | `display($1)`           |
| `\div`              | `div`                   |
| `\divideontimes`    | `times.div`             |
| `\dot`              | `dot($1)`               |
| `\Doteq`            | `≑`                     |
| `\doteq`            | `≐`                     |
| `\doteqdot`         | `≑`                     |
| `\dotplus`          | `plus.dot`              |
| `\dots`             | `dots.h.c`              |
| `\dotsb`            | `dots.h.c`              |
| `\dotsc`            | `dots.h.c`              |
| `\dotsi`            | `dots.h.c`              |
| `\dotsm`            | `dots.h.c`              |
| `\dotso`            | `dots.h`                |
| `\doublebarwedge`   | `⩞`                     |
| `\doublecap`        | `sect.double`           |
| `\doublecup`        | `union.double`          |
| `\Downarrow`        | `arrow.b.double`        |
| `\downarrow`        | `arrow.b`               |
| `\downdownarrows`   | `arrows.bb`             |
| `\downharpoonleft`  | `harpoon.bl`            |
| `\downharpoonright` | `harpoon.br`            |

## E

| LaTeX               | Typst          |
| ------------------- | -------------- |
| `\edef`             | TODO#scripting |
| `\ell`              | `ell`          |
| `\empty`            | `emptyset`     |
| `\emptyset`         | `emptyset`     |
| `\end`              | TODO#begin     |
| `\endgroup`         | TODO#not sure  |
| `\enspace`          | `space.en`     |
| `\Epsilon`          | `Epsilon`      |
| `\epsilon`          | `epsilon.alt`  |
| `\eqcirc`           | `≖`            |
| `\Eqcolon`          | `"-::"`        |
| `\eqcolon`          | `"-:"`         |
| `\Eqqcolon`         | `"=::"`        |
| `\eqqcolon`         | `=:`           |
| `\eqsim`            | `minus.tilde`  |
| `\eqslantgtr`       | `⪖`            |
| `\eqslantless`      | `⪕`            |
| `\equalscolon`      | `=:`           |
| `\equalscoloncolon` | `"=::"`        |
| `\equiv`            | `equiv`        |
| `\Eta`              | `Eta`          |
| `\eta`              | `eta`          |
| `\eth`              | `ð`            |
| `\exist`            | `exists`       |
| `\exists`           | `exists`       |
| `\exp`              | `exp`          |
| `\expandafter`      | TODO#scripting |

## F

| LaTeX            | Typst                            |
| ---------------- | -------------------------------- |
| `\fallingdotseq` | `≒`                              |
| `\fbox`          | `#box(stroke: 0.5pt)[$1]`        |
| `\fcolorbox`     | `#box(stroke: $1, fill: $2)[$3]` |
| `\Finv`          | `Ⅎ`                              |
| `\flat`          | `♭`                              |
| `\footnotesize`  | TODO#font                        |
| `\forall`        | `forall`                         |
| `\frac`          | `frac($1, $2)`                   |
| `\frak`          | `frak($1)`                       |
| `\frown`         | `⌢`                              |
| `\futurelet`     | TODO#scripting                   |

## G

| LaTeX        | Typst                     |
| ------------ | ------------------------- |
| `\Game`      | `⅁`                       |
| `\Gamma`     | `Gamma`                   |
| `\gamma`     | `gamma`                   |
| `\gcd`       | `gcd`                     |
| `\ge`        | `>=`                      |
| `\genfrac`   | TODO#not sure             |
| `\geq`       | `>=`                      |
| `\geqq`      | `ge.equiv`                |
| `\geqslant`  | `gt.eq.slant`             |
| `\gets`      | `arrow.l`                 |
| `\gg`        | `>>`                      |
| `\ggg`       | `>>>`                     |
| `\gggtr`     | `>>>`                     |
| `\gimel`     | `gimel`                   |
| `\global`    | TODO#not sure             |
| `\gnapprox`  | `⪊`                       |
| `\gneq`      | `⪈`                       |
| `\gneqq`     | `gt.nequiv`               |
| `\gnsim`     | `gt.ntilde`               |
| `\grave`     | `grave($1)`               |
| `\gt`        | `>`                       |
| `gtapprox`   | `⪆`                       |
| `gtreqless`  | `gt.eq.lt`                |
| `gtreqqless` | `⪌`                       |
| `gtrless`    | `gt.lt`                   |
| `gtrsim`     | `gt.tilde`                |
| `gvertneqq`  | not even found in unicode |

## H

| LaTeX                         | Typst                  |
| ----------------------------- | ---------------------- |
| `\H`                          | `acute.double($1)`     |
| `\Harr`                       | `<=>`                  |
| `\hArr`                       | `<=>`                  |
| `\harr`                       | `<->`                  |
| `\hat`                        | `hat($1)`              |
| `\hbar`                       | `planck.reduce`        |
| `hbox`                        | TODO#not sure          |
| `\hdashline`                  | TODO#begin             |
| `\hearts`                     | `♡`                    |
| `\heartsuit`                  | `♡`                    |
| `\hline`                      | TODO#begin             |
| `\hom`                        | `hom`                  |
| `\hookleftarrow`              | `arrow.l.hook`         |
| `\hookrightarrow`             | `arrow.r.hook`         |
| `\hphantom`                   | TODO#spacing           |
| `\href`                       | not supported in ipynb |
| `\hskip`                      | TODO#spacing           |
| `\hslash`                     | `planck.reduce`        |
| `\hspace`                     | TODO#spacing           |
| `\htmlClass` and its variants | not supported in ipynb |
| `\huge`                       | TODO#font              |
| `\Huge`                       | TODO#font              |

## I

| LaTeX              | Typst                                      |
| ------------------ | ------------------------------------------ |
| `\i`               | `dotless.i`                                |
| `\iff`             | `<==>`                                     |
| `\iiint`           | `integral.triple`                          |
| `\iint`            | `integral.double`                          |
| `\Im`              | `Im`                                       |
| `\image`           | `Im`                                       |
| `\imageof`         | `⊷`                                        |
| `\imath`           | `dotless.i`                                |
| `\impliedby`       | `<==`                                      |
| `\implies`         | `==>`                                      |
| `\in`              | `in`                                       |
| `\includegraphics` | not supported in ipynb                     |
| `\inf`             | `inf`                                      |
| `\infin`           | `infinity`                                 |
| `\infty`           | `infinity`                                 |
| `\injlim`          | `#math.op("inj\u{2009}lim", limits: true)` |
| `\int`             | `integral`                                 |
| `\intercal`        | `⊺`                                        |
| `\intop`           | `integral`                                 |
| `\Iota`            | `Iota`                                     |
| `\iota`            | `iota`                                     |
| `\isin`            | `in`                                       |
| `\it`              | TODO#font                                  |

## JK

| LaTeX    | Typst        |
| -------- | ------------ |
| `\j`     | `dotless.j`  |
| `\jmath` | `dotless.j`  |
| `\Join`  | `⋈`          |
| `\Kappa` | `Kappa`      |
| `\kappa` | `kappa`      |
| `\KaTeX` | `"KaTeX"`    |
| `\ker`   | `ker`        |
| `\kern`  | TODO#spacing |
| `\Ket`   | TODO#lr      |
| `\ket`   | TODO#lr      |

## L

| LaTeX                     | Typst                     |
| ------------------------- | ------------------------- |
| `\Lambda`                 | `Lambda`                  |
| `\lambda`                 | `lambda`                  |
| `\land`                   | `and`                     |
| `\lang`                   | `⟨`                       |
| `\langle`                 | `⟨`                       |
| `\Larr`                   | `arrow.l.double`          |
| `\lArr`                   | `arrow.l.double`          |
| `\larr`                   | `<-`                      |
| `\large` and its variants | TODO#font                 |
| `\LaTeX`                  | `"LaTeX"`                 |
| `\lBrace`                 | `⦃`                       |
| `\lbrace`                 | `{`                       |
| `\lbrack`                 | `[`                       |
| `\lceil`                  | `⌈`                       |
| `\ldotp`                  | `.`                       |
| `\ldots`                  | `dots.h`                  |
| `\le`                     | `<=`                      |
| `\leadsto`                | `arrow.r.squiggly`        |
| `\left`                   | TODO#lr                   |
| `\leftarrow`              | `<-`                      |
| `\Leftarrow`              | `arrow.l.double`          |
| `\leftarrowtail`          | `<-<`                     |
| `\leftharpoondown`        | `harpoon.lb`              |
| `\leftharpoonup`          | `harpoon.lt`              |
| `\leftleftarrows`         | `arrows.ll`               |
| `\Leftrightarrow`         | `<=>`                     |
| `\leftrightarrow`         | `<->`                     |
| `\leftrightarrows`        | `arrows.lr`               |
| `\leftrightharpoons`      | `harpoons.ltrb`           |
| `\leftrightsquigarrow`    | `arrow.l.r.wave`          |
| `\leftthreetimes`         | `times.three.l`           |
| `\leq`                    | `<=`                      |
| `\leqq`                   | `lt.equiv`                |
| `\leqslant`               | `lt.eq.slant`             |
| `\lessapprox`             | `⪅`                       |
| `\lessdot`                | `lt.dot`                  |
| `\lesseqgtr`              | `lt.eq.gt`                |
| `\lesseqqgtr`             | `⪋`                       |
| `\lessgtr`                | `lt.gt`                   |
| `\lesssim`                | `lt.tilde`                |
| `\let`                    | TODO#scripting            |
| `\lfloor`                 | `⌊`                       |
| `\lg`                     | `lg`                      |
| `\lgroup`                 | `⟮`                       |
| `\lhd`                    | `ld.tri`                  |
| `\lim`                    | `lim`                     |
| `\liminf`                 | `liminf`                  |
| `\limits`                 | TODO#not sure             |
| `\limsup`                 | `limsup`                  |
| `\ll`                     | `<<`                      |
| `\llap`                   | TODO#spacing              |
| `\llbracket`              | `bracket.l.double`        |
| `\llcorner`               | `⌞`                       |
| `\Lleftarrow`             | `arrow.l.triple`          |
| `\lll`                    | `<<<`                     |
| `\llless`                 | `<<<`                     |
| `\ln`                     | `ln`                      |
| `\lnapprox`               | `⪉`                       |
| `\lneq`                   | `⪇`                       |
| `\lneqq`                  | `lt.nequiv`               |
| `\lnot`                   | `not`                     |
| `\lnsim`                  | `lt.ntilde`               |
| `\log`                    | `log`                     |
| `\long`                   | TODO#lr                   |
| `\Longleftarrow`          | `<==`                     |
| `\longleftarrow`          | `<--`                     |
| `\Longleftrightarrow`     | `<==>`                    |
| `\longleftrightarrow`     | `<-->`                    |
| `\longmapsto`             | `arrow.r.long.bar`        |
| `\Longrightarrow`         | `==>`                     |
| `\longrightarrow`         | `-->`                     |
| `\looparrowleft`          | `arrow.l.loop`            |
| `\looparrowright`         | `arrow.r.loop`            |
| `\lor`                    | `or`                      |
| `\lozenge`                | `lozenge.stroked`         |
| `\lparen`                 | `(`                       |
| `\Lrarr`                  | `<=>`                     |
| `\lrArr`                  | `<=>`                     |
| `\lrarr`                  | `<->`                     |
| `\lrcorner`               | `⌟`                       |
| `\lq`                     | `quote.l.single`          |
| `\Lsh`                    | `↰`                       |
| `\lt`                     | `<`                       |
| `\ltimes`                 | `times.l`                 |
| `\lVert`                  | `parallel`                |
| `\lvert`                  | `divides`                 |
| `\lvertneqq`              | not even found in unicode |

## M

| LaTeX | Typst |
| ----- | ----- |
