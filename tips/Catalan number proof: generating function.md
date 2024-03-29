tags:: tips, math, math-combinatorics, proof

[generating function](https://en.wikipedia.org/wiki/Generating_function) :

$$
g(x) = \sum_{i=0}^{\infty} C_i x^i
$$

From recurrence relation

$$
C_{n+1} = \sum_{i=0}^{n} C_i C_{n-i}
$$

We have:

$$
g = 1 + x {g(x)}^2
$$


Solve this quadratic equation of `c` it yields tow solution:

$$
g(x) = \frac{1+\sqrt{1-4x}}{2x}
\quad
\text{or}
\quad
g(x) = \frac{1-\sqrt{1-4x}}{2x}
$$

We choose the second one because it gives: $C_0 = \lim_{x \to 0} g(x) = 1$

The square root term is expanded as following, where $\frac{1}{2} \choose n$ is [Newton's generalized binomial theorem](https://en.wikipedia.org/wiki/Binomial_theorem#Newton's_generalized_binomial_theorem).

$$
\sqrt{1+y} = 1
+{\frac{1}{2} \choose 1} y
+{\frac{1}{2} \choose 2} y^2
+{\frac{1}{2} \choose 3} y^3
+...
$$


So we have:

$$
\sqrt{1-4x} = \sum_{i=0}{\frac{1}{2} \choose i} (-1) ^ i 4^i x^i \tag{exp}
$$

And we already know(where `!!` is [double factorial](https://en.wikipedia.org/wiki/Double_factorial)):

$$
{\frac{1}{2} \choose i} =
\frac{
\frac{1}{2}
\frac{-1}{2}
\frac{-3}{2}
\frac{-5}{2}
...
\frac{1-2i+2}{2} }{ i! }
= 2^{-i}
(-1)^{i+1}
(2i-3)!!
\frac{ 1 } { i! }
\tag{gen-bin}
$$

And we already know:

$$
(2n)! = 2^n n! (2n-1)!!
\tag{even-fac}
$$

From (gen-bin) and (even-fac), we have:

$$
{\frac{1}{2} \choose i} = 4^{-i} (-1)^{i+1} { 2i \choose i } \frac{1}{2i-1}
\tag{gen-bin-2}
$$

From (exp) and (gen-bin-2) we have:

$$
\sqrt{1-4x}
= - \sum_{i=0}\frac{1}{2i-1} { 2i \choose i} x^i
\tag{sqrt-series}
$$


Therefore, with (sqrt-series) we have:


$$
g(x) = \frac{1-\sqrt{1-4x}}{2x} = \frac{1}{2} \sum_{i=1} \frac{1}{2i-1} {2i \choose i} x^{i-1}
$$

Let j = i - 1:

$$
g(x) = \sum_{j=0} \frac{1}{j+1} {2j \choose j} x^i
$$

Finally, the coefficient of $x^i$:

$$
C_n = \frac{1}{n+1} {2n \choose n}
$$
