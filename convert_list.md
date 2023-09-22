# Convert List

full list in [KaTeX](https://katex.org/docs/support_table)

## TODOs

- scripting
- begin
- lr
- binary
- affect all
- no alternative
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
| `\arraystretch`     | TODO#begin           |
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
| `\bf`                   | TODO#affect all             |
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
| `\cal`              | TODO#affect all        |
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
| `\coloncolonsim`    | `::tilde.op`           |
| `\Coloneq`          | `"::−"`                |
| `\coloneq`          | `":−"`                 |
| `\colonequals`      | `:=`                   |
| `\Coloneqq`         | `::=`                  |
| `\coloneqq`         | `:=`                   |
| `\colonminus`       | `":−"`                 |
| `\Colonsim`         | `::tilde.op`           |
| `\colonsim`         | `:tilde.op`            |
| `\color`            | TODO#affect all        |
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

| LaTeX               | Typst                    |
| ------------------- | ------------------------ |
| `\dag`              | `dagger`                 |
| `\Dagger`           | `dagger.double`          |
| `\dagger`           | `dagger`                 |
| `\daleth`           | `ℸ`                      |
| `\Darr`             | `arrow.b.double`         |
| `\dArr`             | `arrow.b.double`         |
| `\darr`             | `arrow.b`                |
| `\dashleftarrow`    | `arrow.l.dash`           |
| `\dashrightarrow`   | `arrow.r.dash`           |
| `\dashv`            | `tack.l`                 |
| `\dbinom`           | `display(binom($1, $2))` |
| `\dbcolon`          | `::`                     |
| `\ddag`             | `dagger.double`          |
| `\ddagger`          | `dagger.double`          |
| `\ddot`             | `dot.double($1)`         |
| `\ddots`            | `dots.down`              |
| `\def`              | TODO#scripting           |
| `\deg`              | `deg`                    |
| `\degree`           | `degree`                 |
| `\Delta`            | `Delta`                  |
| `\delta`            | `delta`                  |
| `\det`              | `det`                    |
| `\digamma`          | `ϝ`                      |
| `\dfrac`            | `display(frac($1, $2))`  |
| `\diagdown`         | `╲`                      |
| `\diagup`           | `╱`                      |
| `\Diamond`          | `lozenge.stroked`        |
| `\diamond`          | `diamond.stroked.small`  |
| `\diamonds`         | `♢`                      |
| `\diamondsuit`      | `♢`                      |
| `\dim`              | `dim`                    |
| `\displaystyle`     | `display($1)`            |
| `\div`              | `div`                    |
| `\divideontimes`    | `times.div`              |
| `\dot`              | `dot($1)`                |
| `\Doteq`            | `≑`                      |
| `\doteq`            | `≐`                      |
| `\doteqdot`         | `≑`                      |
| `\dotplus`          | `plus.dot`               |
| `\dots`             | `dots.h.c`               |
| `\dotsb`            | `dots.h.c`               |
| `\dotsc`            | `dots.h.c`               |
| `\dotsi`            | `dots.h.c`               |
| `\dotsm`            | `dots.h.c`               |
| `\dotso`            | `...`                    |
| `\doublebarwedge`   | `⩞`                      |
| `\doublecap`        | `sect.double`            |
| `\doublecup`        | `union.double`           |
| `\Downarrow`        | `arrow.b.double`         |
| `\downarrow`        | `arrow.b`                |
| `\downdownarrows`   | `arrows.bb`              |
| `\downharpoonleft`  | `harpoon.bl`             |
| `\downharpoonright` | `harpoon.br`             |

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
| `\Eqcolon`          | `"−::"`        |
| `\eqcolon`          | `dash.colon`   |
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
| `\footnotesize`  | TODO#affect all                  |
| `\forall`        | `forall`                         |
| `\frac`          | `frac($1, $2)`                   |
| `\frak`          | `frak($1)`                       |
| `\frown`         | `⌢`                              |
| `\futurelet`     | TODO#scripting                   |

## G

| LaTeX        | Typst                |
| ------------ | -------------------- |
| `\Game`      | `⅁`                  |
| `\Gamma`     | `Gamma`              |
| `\gamma`     | `gamma`              |
| `\gcd`       | `gcd`                |
| `\ge`        | `>=`                 |
| `\genfrac`   | TODO#not sure        |
| `\geq`       | `>=`                 |
| `\geqq`      | `ge.equiv`           |
| `\geqslant`  | `gt.eq.slant`        |
| `\gets`      | `arrow.l`            |
| `\gg`        | `>>`                 |
| `\ggg`       | `>>>`                |
| `\gggtr`     | `>>>`                |
| `\gimel`     | `gimel`              |
| `\global`    | TODO#not sure        |
| `\gnapprox`  | `⪊`                  |
| `\gneq`      | `⪈`                  |
| `\gneqq`     | `gt.nequiv`          |
| `\gnsim`     | `gt.ntilde`          |
| `\grave`     | `grave($1)`          |
| `\gt`        | `>`                  |
| `gtapprox`   | `⪆`                  |
| `gtreqless`  | `gt.eq.lt`           |
| `gtreqqless` | `⪌`                  |
| `gtrless`    | `gt.lt`              |
| `gtrsim`     | `gt.tilde`           |
| `gvertneqq`  | not found in unicode |

## H

| LaTeX                         | Typst                  |
| ----------------------------- | ---------------------- |
| `\H`                          | `acute.double($1)`     |
| `\Harr`                       | `<=>`                  |
| `\hArr`                       | `<=>`                  |
| `\harr`                       | `<->`                  |
| `\hat`                        | `hat($1)`              |
| `\hbar`                       | `planck.reduce`        |
| `\hbox`                       | TODO#not sure          |
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
| `\huge`                       | TODO#affect all        |
| `\Huge`                       | TODO#affect all        |

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
| `\it`              | TODO#affect all                            |

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

| LaTeX                     | Typst                |
| ------------------------- | -------------------- |
| `\Lambda`                 | `Lambda`             |
| `\lambda`                 | `lambda`             |
| `\land`                   | `and`                |
| `\lang`                   | `angle.l`            |
| `\langle`                 | `angle.l`            |
| `\Larr`                   | `arrow.l.double`     |
| `\lArr`                   | `arrow.l.double`     |
| `\larr`                   | `<-`                 |
| `\large` and its variants | TODO#affect all      |
| `\LaTeX`                  | `"LaTeX"`            |
| `\lBrace`                 | `⦃`                  |
| `\lbrace`                 | `{`                  |
| `\lbrack`                 | `[`                  |
| `\lceil`                  | `⌈`                  |
| `\ldotp`                  | `.`                  |
| `\ldots`                  | `...`                |
| `\le`                     | `<=`                 |
| `\leadsto`                | `arrow.r.squiggly`   |
| `\left`                   | TODO#lr              |
| `\leftarrow`              | `<-`                 |
| `\Leftarrow`              | `arrow.l.double`     |
| `\leftarrowtail`          | `<-<`                |
| `\leftharpoondown`        | `harpoon.lb`         |
| `\leftharpoonup`          | `harpoon.lt`         |
| `\leftleftarrows`         | `arrows.ll`          |
| `\Leftrightarrow`         | `<=>`                |
| `\leftrightarrow`         | `<->`                |
| `\leftrightarrows`        | `arrows.lr`          |
| `\leftrightharpoons`      | `harpoons.ltrb`      |
| `\leftrightsquigarrow`    | `arrow.l.r.wave`     |
| `\leftthreetimes`         | `times.three.l`      |
| `\leq`                    | `<=`                 |
| `\leqq`                   | `lt.equiv`           |
| `\leqslant`               | `lt.eq.slant`        |
| `\lessapprox`             | `⪅`                  |
| `\lessdot`                | `lt.dot`             |
| `\lesseqgtr`              | `lt.eq.gt`           |
| `\lesseqqgtr`             | `⪋`                  |
| `\lessgtr`                | `lt.gt`              |
| `\lesssim`                | `lt.tilde`           |
| `\let`                    | TODO#scripting       |
| `\lfloor`                 | `⌊`                  |
| `\lg`                     | `lg`                 |
| `\lgroup`                 | `⟮`                  |
| `\lhd`                    | `lt.tri`             |
| `\lim`                    | `lim`                |
| `\liminf`                 | `liminf`             |
| `\limits`                 | TODO#not sure        |
| `\limsup`                 | `limsup`             |
| `\ll`                     | `<<`                 |
| `\llap`                   | TODO#spacing         |
| `\llbracket`              | `bracket.l.double`   |
| `\llcorner`               | `⌞`                  |
| `\Lleftarrow`             | `arrow.l.triple`     |
| `\lll`                    | `<<<`                |
| `\llless`                 | `<<<`                |
| `\ln`                     | `ln`                 |
| `\lnapprox`               | `⪉`                  |
| `\lneq`                   | `⪇`                  |
| `\lneqq`                  | `lt.nequiv`          |
| `\lnot`                   | `not`                |
| `\lnsim`                  | `lt.ntilde`          |
| `\log`                    | `log`                |
| `\long`                   | TODO#lr              |
| `\Longleftarrow`          | `<==`                |
| `\longleftarrow`          | `<--`                |
| `\Longleftrightarrow`     | `<==>`               |
| `\longleftrightarrow`     | `<-->`               |
| `\longmapsto`             | `arrow.r.long.bar`   |
| `\Longrightarrow`         | `==>`                |
| `\longrightarrow`         | `-->`                |
| `\looparrowleft`          | `arrow.l.loop`       |
| `\looparrowright`         | `arrow.r.loop`       |
| `\lor`                    | `or`                 |
| `\lozenge`                | `lozenge.stroked`    |
| `\lparen`                 | `(`                  |
| `\Lrarr`                  | `<=>`                |
| `\lrArr`                  | `<=>`                |
| `\lrarr`                  | `<->`                |
| `\lrcorner`               | `⌟`                  |
| `\lq`                     | `quote.l.single`     |
| `\Lsh`                    | `↰`                  |
| `\lt`                     | `<`                  |
| `\ltimes`                 | `times.l`            |
| `\lVert`                  | `parallel`           |
| `\lvert`                  | `divides`            |
| `\lvertneqq`              | not found in unicode |

## M

| LaTeX              | Typst           |
| ------------------ | --------------- |
| `\maltese`         | `maltese`       |
| `\mapsto`          | `arrow.r.bar`   |
| `\mathbb`          | `bb($1)`        |
| `\mathbf`          | `bold($1)`      |
| `\mathbin`         | TODO#spacing    |
| `\mathcal`         | `cal($1)`       |
| `\mathchoise`      | TODO#spacing    |
| `\mathclap`        | TODO#spacing    |
| `\mathclose`       | TODO#lr         |
| `\mathellipsis`    | `...`           |
| `\mathfrak`        | `frak($1)`      |
| `\mathinner`       | TODO#spacing    |
| `\mathit`          | `italic($1)`    |
| `\mathllap`        | TODO#spacing    |
| `\mathnormal`      | ommited         |
| `\mathop`          | ommited         |
| `\mathopen`        | TODO#lr         |
| `\mathord`         | TODO#spacing    |
| `\mathpunct`       | TODO#spacing    |
| `\mathrel`         | TODO#spacing    |
| `\mathrlap`        | TODO#spacing    |
| `\mathring`        | `circle($1)`    |
| `\mathrm`          | `upright($1)`   |
| `\mathscr`         | TODO#font       |
| `\mathsf`          | `sans($1)`      |
| `\mathsterling`    | `pound`         |
| `\mathstrut`       | TODO#spacing    |
| `\mathtt`          | `mono($1)`      |
| `\max`             | `max`           |
| `\measuredangle`   | `angle.arc`     |
| `\medspace`        | `#h(2em/9)`     |
| `\mho`             | `ohm.inv`       |
| `\mid`             | `\|`            |
| `\middle`          | TODO#lr         |
| `\minuscolon`      | `dash.colon`    |
| `\minuscoloncolon` | `"−::"`         |
| `\minuso`          | `⦵`             |
| `\mkren`           | TODO#spacing    |
| `\mod`             | `mod`           |
| `\models`          | `tack.r.double` |
| `\mp`              | `minus.plus`    |
| `\mskip`           | TODO#spacing    |
| `\Mu`              | `Mu`            |
| `\mu`              | `mu`            |
| `\multimap`        | `multimap`      |

## N

| LaTeX               | Typst                  |
| ------------------- | ---------------------- |
| `\N`                | `NN`                   |
| `\nabla`            | `nabla`                |
| `\natnums`          | `NN`                   |
| `\natural`          | `♮`                    |
| `\negmedspace`      | `#h(-2em/9)`           |
| `\ncong`            | `tilde.equiv.not`      |
| `\ne`               | `!=`                   |
| `\nearrow`          | `arrow.tr`             |
| `\neg`              | `not`                  |
| `\negthickspace`    | `#h(-5em/18)`          |
| `\negthinmedspace`  | `#h(-1em/6)`           |
| `\neq`              | `!=`                   |
| `\newcommand`       | TODO#scripting         |
| `\newline`          | `\ `                   |
| `nexist`            | `exists.not`           |
| `\ngeq`             | `gt.eq.not`            |
| `\ngeqq`            | not found in unicode   |
| `\ngeqslant`        | not found in unicode   |
| `\ngtr`             | `gt.not`               |
| `\ni`               | `in.rev`               |
| `\nLeftarrow`       | `arrow.l.double.not`   |
| `\nleftarrow`       | `arrow.l.not`          |
| `\nLeftrightarrow`  | `arrow.l.r.double.not` |
| `\nleftrightarrow`  | `arrow.l.r.not`        |
| `\nleq`             | `lt.eq.not`            |
| `\nleqq`            | not found in unicode   |
| `\nleqslant`        | not found in unicode   |
| `\nless`            | `lt.not`               |
| `\nmid`             | `divides.not`          |
| `\nobreak`          | TODO#spacing           |
| `\nobreakspace`     | `space.nobreak`        |
| `\noexpand`         | TODO#scripting         |
| `\nolimits`         | TODO#not sure          |
| `\nonumber`         | TODO#begin             |
| `\normalsize`       | TODO#affect all        |
| `\notin`            | `in.not`               |
| `\notni`            | `in.rev.not`           |
| `\nparallel`        | `parallel.not`         |
| `\nprec`            | `prec.not`             |
| `\npreceq`          | `prec.eq.not`          |
| `\nRightarrow`      | `arrow.r.double.not`   |
| `\nrightarrow`      | `arrow.r.not`          |
| `\nshortmid`        | not found in unicode   |
| `\nshortparallel`   | not found in unicode   |
| `\nsim`             | `tilde.not`            |
| `\nsubseteq`        | `subset.eq.not`        |
| `\nsubseteqq`       | not found in unicode   |
| `\nsucc`            | `succ.not`             |
| `\nsucceq`          | `succ.eq.not`          |
| `\nsupseteq`        | `supset.eq.not`        |
| `\nsupseteqq`       | not found in unicode   |
| `\ntriangleleft`    | `lt.tri.not`           |
| `\ntrianglelefteq`  | `lt.tri.eq.not`        |
| `\ntriangleright`   | `gt.tri.not`           |
| `\ntrianglerighteq` | `gt.tri.eq.not`        |
| `\Nu`               | `Nu`                   |
| `\nu`               | `nu`                   |
| `\nVDash`           | `⊯`                    |
| `\nVdash`           | `⊮`                    |
| `\nvDash`           | `tack.r.double.not`    |
| `\nvdash`           | `tack.r.not`           |
| `\nwarrow`          | `arrow.tl`             |

## O

| LaTeX                     | Typst                               |
| ------------------------- | ----------------------------------- |
| `\O`                      | `Ø`                                 |
| `\o`                      | `ø`                                 |
| `\odot`                   | `dot.circle`                        |
| `\OE`                     | `Œ`                                 |
| `\oe`                     | `œ`                                 |
| `\oiiint`                 | `integral.vol`                      |
| `\oiint`                  | `integral.surf`                     |
| `\oint`                   | `integral.cont`                     |
| `\Omega`                  | `Omega`                             |
| `\omega`                  | `omega`                             |
| `\Omicron`                | `Omicron`                           |
| `\omicron`                | `omicron`                           |
| `\ominus`                 | `minus.circle`                      |
| `\operatorname`           | `#math.op("$1")`                    |
| `\operatorname*`          | `#math.op("$1", limits: true)`      |
| `\operatornamewithlimits` | `#math.op("$1", limits: true)`      |
| `\oplus`                  | `plus.circle`                       |
| `\origof`                 | `⊶`                                 |
| `\oslash`                 | `⊘`                                 |
| `\otimes`                 | `times.circle`                      |
| `\over`                   | TODO#binary                         |
| `\overbrace`              | `overbrace($1)` `overbrace($1, $2)` |
| `\overgroup`              | `accent($1, turtle.t)`              |
| `\overleftarrow`          | `arrow.l($1)`                       |
| `\overleftharpoon`        | TODO#no alternative                 |
| `\overleftrightarrow`     | TODO#no alternative                 |
| `\overline`               | `overline($1)`                      |
| `\overlinesegment`        | TODO#no alternative                 |
| `\Overrightarrow`         | TODO#no alternative                 |
| `\overrightarrow`         | `arrow.r($1)`                       |
| `\overrightharpoon`       | TODO#no alternative                 |
| `\overset`                | TODO#not sure                       |
| `\owns`                   | `in.rev`                            |

## P

| LaTeX             | Typst                                       |
| ----------------- | ------------------------------------------- |
| `\P`              | `pilcrow`                                   |
| `\parallel`       | `parallel`                                  |
| `\partial`        | `diff`                                      |
| `\perp`           | `bot`                                       |
| `\phantom`        | TODO#spacing                                |
| `\phase`          | TODO#no alternative                         |
| `\Phi`            | `Phi`                                       |
| `\phi`            | `phi.alt`                                   |
| `\Pi`             | `Pi`                                        |
| `\pi`             | `pi`                                        |
| `\pitchfork`      | `⋔`                                         |
| `\plim`           | `#math.op("plim", limits: true)`            |
| `\plusmn`         | `plus.minus`                                |
| `\pm`             | `plus.minus`                                |
| `\pmb`            | `bold($1)` maybe works                      |
| `\pmod`           | `mod`                                       |
| `\pod`            | TODO#spacing                                |
| `\pounds`         | `pound`                                     |
| `\Pr`             | `Pr`                                        |
| `\prec`           | `prec`                                      |
| `\precapprox`     | `prec.approx`                               |
| `\preccurlyeq`    | `prec.eq`                                   |
| `\preceq`         | `⪯`                                         |
| `\precnapprox`    | `prec.napprox`                              |
| `\precneqq`       | `prec.nequiv`                               |
| `\precnsim`       | `prec.ntilde`                               |
| `\precsim`        | `prec.tilde`                                |
| `\prime`          | `prime`                                     |
| `\prod`           | `product`                                   |
| `\projlim`        | `#math.op("proj\u{2009}lim", limits: true)` |
| `\propto`         | `prop`                                      |
| `\providecommand` | TODO#scripting                              |
| `\Psi`            | `Psi`                                       |
| `\psi`            | `psi`                                       |
| `\pu`             | not supported in ipynb                      |

## QR

| LaTeX                | Typst              |
| -------------------- | ------------------ |
| `\qquad`             | `#h(2em)`          |
| `\quad`              | `space.quad`       |
| `\R`                 | `RR`               |
| `\r`                 | `circle($1)`       |
| `\raisebox`          | TODO#not sure      |
| `\rang`              | `angle.r`          |
| `\rangle`            | `angle.r`          |
| `\Rarr`              | `=>`               |
| `\rArr`              | `=>`               |
| `\rarr`              | `->`               |
| `\ratio`             | `:`                |
| `\rBrace`            | `⦄`                |
| `\rbrace`            | `}`                |
| `\rbrack`            | `]`                |
| `\rceil`             | `⌉`                |
| `\Re`                | `Re`               |
| `\real`              | `Re`               |
| `\Reals`             | `RR`               |
| `\reals`             | `RR`               |
| `\renewcommand`      | TODO#scripting     |
| `\restriction`       | `harpoon.tr`       |
| `\rfloor`            | `⌋`                |
| `\rgroup`            | `turtle.r`         |
| `\rhd`               | `gt.tri`           |
| `\Rho`               | `Rho`              |
| `\rho`               | `rho`              |
| `\right`             | TODO#lr            |
| `\Rightarrow`        | `=>`               |
| `\rightarrow`        | `->`               |
| `\rightarrowtail`    | `>->`              |
| `\rightharpoondown`  | `harpoon.rb`       |
| `\rightharpoonup`    | `harpoon.rt`       |
| `\rightleftarrows`   | `arrows.rl`        |
| `\rightleftharpoons` | `harpoons.rtlb`    |
| `\rightrightarrows`  | `arrows.rr`        |
| `\rightsquigarrow`   | `arrow.r.squiggly` |
| `\rightthreetimes`   | `times.three.r`    |
| `\risingdotseq`      | `≓`                |
| `\rlap`              | TODO#spacing       |
| `\rm`                | TODO#affect all    |
| `\rmoustache`        | `⎱`                |
| `\rparen`            | `)`                |
| `\rq`                | `'`                |
| `rrbracket`          | `bracket.r.double` |
| `\Rrightarrow`       | `arrow.r.triple`   |
| `\Rsh`               | `↱`                |
| `\rtimes`            | `times.r`          |
| `\rule`              | TODO#not sure      |
| `\rVert`             | `parallel`         |
| `\rvert`             | `divides`          |

## S

| LaTeX                | Typst                     |
| -------------------- | ------------------------- |
| `\S`                 | `section`                 |
| `\scriptscriptstyle` | TODO#affect all           |
| `\scriptsize`        | TODO#affect all           |
| `\scriptstyle`       | TODO#affect all           |
| `\sdot`              | `dot.op`                  |
| `\searrow`           | `arrow.br`                |
| `\sec`               | `sec`                     |
| `\sect`              | `section`                 |
| `\Set`               | `{$1}`                    |
| `\set`               | `{$1}`                    |
| `\setminus`          | `without`                 |
| `\sf`                | TODO#affect all           |
| `sharp`              | `♯`                       |
| `\shortmid`          | TODO#no alternative       |
| `\shortparallel`     | TODO#no alternative       |
| `\Sigma`             | `Sigma`                   |
| `\sigma`             | `sigma`                   |
| `\sim`               | `tilde.op`                |
| `\simcolon`          | `tilde.op:`               |
| `\simcoloncolon`     | `tilde.op::`              |
| `\simeq`             | `tilde.eq`                |
| `\sin`               | `sin`                     |
| `\sinh`              | `sinh`                    |
| `\sixptsize`         | TODO#affect all           |
| `\sh`                | `#math.op("sh")`          |
| `\small`             | TODO#affect all           |
| `\smallint`          | TODO#not sure             |
| `\smallsetminus`     | TODO#not sure             |
| `\smallsmile`        | `⌣`                       |
| `\sout`              | TODO#not sure             |
| `\space`             | `space`                   |
| `\spades`            | `suit.spade`              |
| `\spadesuit`         | `suit.spade`              |
| `\sphericalangle`    | `angle.spheric`           |
| `\sqcap`             | `sect.sq`                 |
| `\sqcup`             | `union.sq`                |
| `\square`            | `square.stroked`          |
| `\sqrt`              | `sqrt($1)` `root($1, $2)` |
| `\sqsubset`          | `subset.sq`               |
| `\sqsubseteq`        | `subset.eq.sq`            |
| `\sqsupset`          | `supset.sq`               |
| `\sqsupseteq`        | `supset.eq.sq`            |
| `\ss`                | `ß`                       |
| `\stackrel`          | TODO#not sure             |
| `\star`              | `star.op`                 |
| `\sub`               | `subset`                  |
| `\sube`              | `subset.eq`               |
| `\Subset`            | `subset.double`           |
| `\subset`            | `subset`                  |
| `\subseteq`          | `subset.eq`               |
| `\subseteqq`         | `⫅`                       |
| `\subsetneq`         | `subset.neq`              |
| `\subsetneqq`        | `⫋`                       |
| `\substack`          | TODO#not sure             |
| `\succ`              | `succ`                    |
| `\succapprox`        | `succ.approx`             |
| `\succcurlyeq`       | `succ.eq`                 |
| `\succeq`            | `⪰`                       |
| `\succnapprox`       | `succ.napprox`            |
| `\succneqq`          | `succ.nequiv`             |
| `\succnsim`          | `succ.ntilde`             |
| `\sum`               | `sum`                     |
| `\sup`               | `sup`                     |
| `\supe`              | `supset.eq`               |
| `\Supset`            | `supset.double`           |
| `\supset`            | `supset`                  |
| `\supseteq`          | `supset.eq`               |
| `\supseteqq`         | `⫆`                       |
| `\supsetneq`         | `supset.neq`              |
| `\supsetneqq`        | `⫌`                       |
| `\surd`              | `√`                       |
| `\swarrow`           | `arrow.bl`                |

## T

*some command is text mode only*

| LaTeX                | Typst                   |
| -------------------- | ----------------------- |
| `\tag`               | TODO#not sure           |
| `\tag*`              | TODO#not sure           |
| `\tan`               | `tan`                   |
| `\tanh`              | `tanh`                  |
| `\Tau`               | `Tau`                   |
| `\tau`               | `tau`                   |
| `\tbinom`            | `inline(binom($1, $2))` |
| `\TeX`               | `"TeX"`                 |
| `\text`              | `#[$1]`                 |
| `\textasciitilde`    | `~`                     |
| `\textasciicircum`   | `\^`                    |
| `\textbackslash`     | `\\`                    |
| `\textbar`           | `\|`                    |
| `\textbardbl`        | `‖`                     |
| `\textbf`            | `bold(#[$1])`           |
| `\textbraceleft`     | `{`                     |
| `\textbraceright`    | `}`                     |
| `\textcircled`       | TODO#not sure           |
| `\textcolor`         | `text(fill: $1)[$2]`    |
| `\textdagger`        | `#sym.dagger`           |
| `\textdaggerdbl`     | `#sym.dagger.double`    |
| `\textdegree`        | `#sym.degree`           |
| `\textdollarsign`    | `\$`                    |
| `\textellipsis`      | `...`                   |
| `\textemdash`        | `---`                   |
| `\textendash`        | `--`                    |
| `\textgreater`       | `#sym.gt`               |
| `\textit`            | `italic(#[$1])`         |
| `\textless`          | `#sym.lt`               |
| `\textmd`            | `#[$1]`                 |
| `\textnormal`        | `#[$1]`                 |
| `\textquotedblleft`  | `#sym.quote.l.double`   |
| `\textquotedblright` | `#sym.quote.r.double`   |
| `\textquoteleft`     | `#sym.quote.l.single`   |
| `\textquoteright`    | `#sym.quote.r.single`   |
| `\textregistered`    | `®`                     |
| `\textrm`            | `#[$1]`                 |
| `\textsf`            | `sans(#[$1])`           |
| `\textsterling`      | `#sym.pound`            |
| `\textsyle`          | `inline($1)`            |
| `\texttt`            | `mono(#[$1])`           |
| `\textunderscore`    | `\_`                    |
| `\textup`            | `#[$1]`                 |
| `\tfrac`             | `inline(frac($1, $2))`  |
| `\tg`                | `tg`                    |
| `\th`                | `#math.op("th")`        |
| `\therefore`         | `therefore`             |
| `\Theta`             | `Theta`                 |
| `\theta`             | `theta`                 |
| `\thetasym`          | `theta.alt`             |
| `\thickapprox`       | `bold(approx)`          |
| `\thicksim`          | `bold(tilde)`           |
| `\thickspace`        | `#h(5em/18)`            |
| `\thinspace`         | `space.sixth`           |
| `\tilde`             | `tilde($1)`             |
| `\times`             | `times`                 |
| `\tiny`              | TODO#affect all         |
| `\to`                | `->`                    |
| `\top`               | `top`                   |
| `\triangle`          | `triangle.stroked.t`    |
| `\triangledown`      | `triangle.stroked.b`    |
| `\triangleleft`      | `triangle.stroked.l`    |
| `\trianglelefteq`    | `lt.tri.eq`             |
| `\triangleq`         | `eq.delta`              |
| `\triangleright`     | `triangle.stroked.r`    |
| `\trianglerighteq`   | `gt.tri.eq`             |
| `\tt`                | TODO#affect all         |
| `\twoheadleftarrow`  | `<<-`                   |
| `\twoheadrightarrow` | `->>`                   |

## U

| LaTeX                  | Typst                                 |
| ---------------------- | ------------------------------------- |
| `\u`                   | `breve($1)`                           |
| `\Uarr`                | `arrow.t.double`                      |
| `\uArr`                | `arrow.t.double`                      |
| `\uarr`                | `arrow.t`                             |
| `\ulcorner`            | `⌜`                                   |
| `\underbar`            | `underline($1)`                       |
| `\underbrace`          | `underbrace($1)` `underbrace($1, $2)` |
| `\undergroup`          | `accent($1, turtle.b)`                |
| `\underleftarrow`      | TODO#no alternative                   |
| `\underleftrightarrow` | TODO#no alternative                   |
| `\underline`           | `underline($1)`                       |
| `\underlinesegment`    | TODO#no alternative                   |
| `\underrightarrow`     | TODO#no alternative                   |
| `\underset`            | TODO#not sure                         |
| `\unlhd`               | `lt.tri.eq`                           |
| `\unrhd`               | `gt.tri.eq`                           |
| `\Uparrow`             | `arrow.t.double`                      |
| `\uparrow`             | `arrow.t`                             |
| `\Updownarrow`         | `arrow.t.b.double`                    |
| `\updownarrow`         | `arrow.t.b`                           |
| `\upharpoonleft`       | `harpoon.tl`                          |
| `\upharpoonright`      | `harpoon.tr`                          |
| `\uplus`               | `union.plus`                          |
| `\Upsilon`             | `Upsilon`                             |
| `\upsilon`             | `upsilon`                             |
| `\upuparrows`          | `arrows.tt`                           |
| `\urcorner`            | `⌝`                                   |
| `\url`                 | not supported in ipynb                |
| `\utilde`              | TODO#no alternative                   |

## V

| LaTeX               | Typst                |
| ------------------- | -------------------- |
| `\v`                | `caron($1)`          |
| `\varDelta`         | `italic(Delta)`      |
| `\varepsilon`       | `italic(epsilon)`    |
| `\varGamma`         | `italic(Gamma)`      |
| `\varinjlim`        | TODO#no alternative  |
| `\varkappa`         | `italic(kappa)`      |
| `\varLambda`        | `italic(Lambda)`     |
| `\varliminf`        | TODO#no alternative  |
| `\varlimsup`        | TODO#no alternative  |
| `\varnothing`       | `italic(nothing)`    |
| `\varOmega`         | `italic(Omega)`      |
| `\varPhi`           | `italic(Phi)`        |
| `\varphi`           | `italic(phi)`        |
| `\varPi`            | `italic(Pi)`         |
| `\varpi`            | `italic(pi.alt)`     |
| `\varprojlim`       | TODO#no alternative  |
| `\varpropto`        | `prop`               |
| `\varPsi`           | `italic(Psi)`        |
| `\varrho`           | `italic(rho.alt)`    |
| `\varSigma`         | `italic(Sigma)`      |
| `\varsigma`         | `italic(sigma.alt)`  |
| `\varsubsetneq`     | `subset.neq`         |
| `\varsubsetneqq`    | `⫋`                  |
| `\varsupsetneq`     | `supset.neq`         |
| `\varsupsetneqq`    | `⫌`                  |
| `\varTheta`         | `italic(Theta)`      |
| `\vartheta`         | `italic(theta)`      |
| `\vartriangle`      | `triangle.stroked.t` |
| `\vartriangleleft`  | `lt.tri`             |
| `\vartriangleright` | `gt.tri`             |
| `\varUpsilon`       | `italic(Upsilon)`    |
| `\varXi`            | `italic(Xi)`         |
| `\vcentcolon`       | `:`                  |
| `\vcenter`          | TODO#not sure        |
| `\Vdash`            | `⊩`                  |
| `\vDash`            | `tack.r.double`      |
| `\vdash`            | `tack.r`             |
| `\vdots`            | `dots.v`             |
| `\vec`              | `arrow($1)`          |
| `\vee`              | `or`                 |
| `\veebar`           | `⊻`                  |
| `\verb`             | TODO#not sure        |
| `\Vert`             | `parallel`           |
| `\vert`             | `divides`            |
| `\vphantom`         | TODO#spacing         |
| `\Vvdash`           | `⊪`                  |

## W

| LaTeX        | Typst       |
| ------------ | ----------- |
| `\wedge`     | `and`       |
| `\weierp`    | `℘`         |
| `\widecheck` | `caron($1)` |
| `\widehat`   | `hat($1)`   |
| `\widetilde` | `tilde($1)` |
| `\wp`        | `℘`         |
| `\wr`        | `wreath`    |

## X

| LaTeX                 | Typst                               |
| --------------------- | ----------------------------------- |
| `\xcancel`            | `cancel(cross: #true, $1)`          |
| `\xdef`               | TODO#scripting                      |
| `\Xi`                 | `Xi`                                |
| `\xi`                 | `xi`                                |
| `\xhookleftarrow`     | `xarrow(sym: arrow.l.hook, $1)`     |
| `\xhookrightarrow`    | `xarrow(sym: arrow.r.hook, $1)`     |
| `\xLeftarrow`         | `xarrow(sym: arrow.l.double, $1)`   |
| `\xleftarrow`         | `xarrow(sym: arrow.l, $1)`          |
| `\xleftharpoondown`   | `xarrow(sym: harpoon.lb, $1)`       |
| `\xleftharpoonup`     | `xarrow(sym: harpoon.lt, $1)`       |
| `\xLeftrightarrow`    | `xarrow(sym: arrow.l.r.double, $1)` |
| `\xleftrightarrow`    | `xarrow(sym: arrow.l.r, $1)`        |
| `\xleftrightharpoons` | `xarrow(sym: harpoons.ltrb, $1)`    |
| `\xlongequal`         | `xarrow(sym: eq, $1)`               |
| `\xmapsto`            | `xarrow(sym: arrow.r.bar, $1)`      |
| `\xRightarrow`        | `xarrow(sym: arrow.r.double, $1)`   |
| `\xrightarrow`        | `xarrow(sym: arrow.r, $1)`          |
| `\xrightharpoondown`  | `xarrow(sym: harpoon.rb, $1)`       |
| `\xrightharpoonup`    | `xarrow(sym: harpoon.rt, $1)`       |
| `\xrightleftharpoons` | `xarrow(sym: harpoons.rtlb, $1)`    |
| `\xtofrom`            | `xarrow(sym: arrow.l.r, $1)`        |
| `\xtwoheadleftarrow`  | `xarrow(sym: arrow.l.twohead, $1)`  |
| `\xtwoheadrightarrow` | `xarrow(sym: arrow.r.twohead, $1)`  |

## YZ

| LaTeX   | Typst  |
| ------- | ------ |
| `\yen`  | `yen`  |
| `\Z`    | `ZZ`   |
| `\Zeta` | `Zeta` |
| `\zeta` | `zeta` |