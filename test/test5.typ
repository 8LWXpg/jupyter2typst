#import "template.typ": *
#show: template

#block[
\<font color\=\"red\"\>\<H1\>LATEX INFO\<\/H1\>\<\/font\>
]
#block[
In new Jupyter notebooks I have been working you can just type latex in markdown cells and it works.
]
#block[
Example of new way on next line, but won\'t work in old canopy I have on home computer.
(kept old one so far because lets me edit directly in cnaopy whereas new Canopy goes to browser and is a bit annoying in opening and closing.)
To see it work, easiest way is to upload to tmpnb.org and use there.
]
#block[
$ $$
nabla times arrow(bold(B)) -space.sixth frac(1, c)space.sixth frac(diff arrow(bold(E)), diff t) & = frac(4pi, c)arrow(bold(j)) \
nabla dot.op arrow(bold(E)) & = 4 pi rho \
nabla times arrow(bold(E))space.sixth +space.sixth frac(1, c)space.sixth frac(diff arrow(bold(B)), diff t) & = arrow(bold(0)) \
nabla dot.op arrow(bold(B)) & = 0
$$ $]
#block[
*Easy Equation writing examples*
$c = sqrt(a^2 + b^2)$
Logarithmic growth of a population of cells can be described mathematically as
$N = N_o e^l n 2(t\/t_2)$
(from page 177 of Methods in Yeast Genetics, 205 Edition)
*See #link("http://meta.math.stackexchange.com/questions/5020/mathjax-basic-tutorial-and-quick-reference")[here] for an awesome reference for MathJax*
]
#block[
\"and use single \$ (rather than double \$\$) to keep the equation in\-line. stackoverflow.com\/q\/19412644\/1224255\" \- from #link("http://stackoverflow.com/questions/13208286/how-to-write-latex-in-ipython-notebook")[http:\/\/stackoverflow.com\/questions\/13208286\/how\-to\-write\-latex\-in\-ipython\-notebook]  (\<\-\-\-this itself was tricky to write and I had to use minrk\'s April 18th answer at #link("https://github.com/ipython/ipython/issues/3197/")[https:\/\/github.com\/ipython\/ipython\/issues\/3197\/] as a basis
]
#block[
Based on #link("https://tex.stackexchange.com/questions/158897/how-do-i-get-a-hyphen-in-mathrm")[here] I figured out (probably again) how to add hyphen when in math mode in Jupyter notebooks and not have it loolike a minus sign.
$frac(m i t o space p u r i f i c a t i o n space R N A#[-]S e q space d a t a, t o t a l space c e l l space R N A#[-]s e q space d a t a)$
VS.
$frac(o b s_a - o b s_b, e x p_a - e x p_b)$
]
#block[
#code-block("#from JupyterLab demo notebook November 2, 2016
from IPython.display import Latex
Latex('''The mass-energy equivalence is described by the famous equation
 
$$E=mc^2$$
 
discovered in 1905 by Albert Einstein. 
In natural units ($c$ = 1), the formula expresses the identity
 
\\\\begin{equation}
E=m
\\\\end{equation}''')"
, lang: "python", count: 15)
]
#block[
#result-block("<IPython.core.display.Latex object>")
]
#block[
#code-block("%%latex
\\begin{aligned}
\\nabla \\times \\vec{\\mathbf{B}} -\\, \\frac1c\\, \\frac{\\partial\\vec{\\mathbf{E}}}{\\partial t} & = \\frac{4\\pi}{c}\\vec{\\mathbf{j}} \\\\
\\nabla \\cdot \\vec{\\mathbf{E}} & = 4 \\pi \\rho \\\\
\\nabla \\times \\vec{\\mathbf{E}}\\, +\\, \\frac1c\\, \\frac{\\partial\\vec{\\mathbf{B}}}{\\partial t} & = \\vec{\\mathbf{0}} \\\\
\\nabla \\cdot \\vec{\\mathbf{B}} & = 0
\\end{aligned}"
, lang: "python", count: 3)
]
#block[
#result-block("<IPython.core.display.Latex object>")
]
#block[
#code-block("%%latex

$$
\\frac{1}{3}\\
$$

$$
\\frac{obs}{expe}\\
$$

$$ Ai(z) =
	\\frac13\\sqrt{z}\\left[
	I_{-1/3}(\\zeta)
	-I_{1/3}(\\zeta) \\right]
	=
	\\pi^{-1}\\sqrt{z/3}K_{1/3}(\\zeta)
$$

{\\bf 10.4.15}
$$ Ai(-z) =
	\\frac13\\sqrt{z}
	\\left[
	J_{1/3}(\\zeta) +
	J_{-1/3}(\\zeta) \\right]
	=
	\\frac12 \\sqrt{z/3} \\left[
	e^{\\pi i/6} H_{1/3}^{(1)}(\\zeta)
	+ e^{-\\pi i/6}H_{1/3}^{(2)}(\\zeta)
	\\right]
$$
"
, lang: "python", count: 11)
]
#block[
#result-block("<IPython.core.display.Latex object>")
]
