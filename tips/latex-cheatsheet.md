tags:: cheatsheet, latex

- $\forall x \in [-\infty,\infty]$
- $\forall x \in \mathbb{R}$
- $A \in B$ element of
- $A \sub B \subset C \subseteq D$ subset
- $\overline{A}$ complement(set), overline
- $A \cdot B$ dot product
- $A \times B$ times multiply
- $\det A$ determinant of matrix A
- $A \lessgtr B$ less and greater
- $A \prec B \preceq C$ precede or equal
- $A \succ B \succeq C$ succeed or equal
- $A \centernot\implies B$, $A \;\not\!\!\!\implies B$ not imply
- $A \implies B$ implies
- $A \leftrightarrow B$
- $A \iff B$ iff, if and only if
- $A \equiv B$ equivalent
- $A \neq B$ not equal
- $p \approx q$ $p \thickapprox q$ `\approx` and `\thickapprox`
- $\vec{A}$ vector
- $V^\dag$ dagger, complex vector transpose: transpose + conjugate
- $A \land B$ , $A \and B$ logic and
- $A \cap B$ cap, intersection
- $A \cup B$ cup, union
- $A \setminus B$ setminus diff
- $A \ge B$ greater than or equal
- $A \perp B$ perpendicular 垂直的, 直角的
- $n \choose k$, $\binom{n}{k}$  combination number,  binomial coefficient
- $\sum_{n=1}^{\infty}$ `\sum_{n=1}^{\infty}` sum 求和
- $$
  \begin{cases}
  x       & = v1 \\
  x+y+z   & = v4 \\
  x+2y+4z & = v5
  \end{cases}
  $$
  equation system `\begin{case}` `&` for align
- **Spacing**
	- $A \quad B$ `\quad` quadratone, 空格
	- $A \; B$ thick space
	- $A \: B$ medium space
	- $A \, B$ thin space
	- $A \!\! B$ : `\!`: insert a negative thin space
- **Other**
	- $\because A \therefore B$ `\because` and `\therefore`
	- $A \text{and} B$ `\text{foo}` add literal text 任意文字
	-
	- $$f(x) = 1 \tag{2.34} \label{foo}$$
	  $$ \eqref{name} $$
	  `\tag{text}`: Manually add equation number 加标号
	  `\label{name}` add reference label 加引用标记
	  `\eqref{name}` 引用一个label
- **Set**
	- $\{x | p(x)\}$ $\{x : p(x)\}$ $\{x ; p(x)\}$
	- $\{x \in A | p(x) \}$
-
-
- Create a new symbol with macro:
  https://tex.stackexchange.com/questions/499668/how-to-create-a-succeeds-or-precedes-symbol
- Reference:
	- https://en.wikipedia.org/wiki/List_of_logic_symbols
	- https://www.geeksforgeeks.org/equality-and-inference-symbols-in-latex/