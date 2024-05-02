https://en.wikipedia.org/w/index.php?title=Formal_derivativ

- ## Definition
  
  Fix a ring $R$ (not necessarily commutative) and let $A=R[x]$ be the ring of polynomials over $R$. (If $R$ is not commutative, this is the [free algebra](https://en.wikipedia.org/wiki/Free_algebra) over a single indeterminate variable.)
  
  Then the formal derivative is an operation on elements of $A$, where if
  
  $f(x)\,=\,a_{n}x^{n}+\cdots +a_{1}x+a_{0},$
  
  then its formal derivative is
  
  $f'(x)\,=\,Df(x)=na_{n}x^{n-1}+\cdots +ia_{i}x^{i-1}+\cdots +a_{1}.$
  
  
  In the above definition, for any nonnegative integer $i$ and $r\in R$, $ir$ is defined as usual in a ring: $ir=\underbrace {r+r+\cdots +r} _{i{\text{ times}}}$ (with $ir=0$ if $i=0$).[[1\]](https://en.wikipedia.org/wiki/Formal_derivative#cite_note-1)
  
  This definition also works even if $R$ does not have a [multiplicative identity](https://en.wikipedia.org/wiki/Identity_element) (that is, $R$ is a [rng](https://en.wikipedia.org/wiki/Rng_(algebra))).
-
- ### Alternative axiomatic definition[[edit](https://en.wikipedia.org/w/index.php?title=Formal_derivative&action=edit&section=2)]
  
  One may also define the formal derivative axiomatically as the map $(\ast )^{\prime }\colon R[x]\to R[x]$ satisfying the following properties.
  
  1. $r'=0$ for all $r\in R\subset R[x].$
  2. The normalization axiom, $x'=1.$
  3. The map commutes with the addition operation in the polynomial ring, $(a+b)'=a'+b'.$
  4. The map satisfies Leibniz's law with respect to the polynomial ring's multiplication operation, $(a\cdot b)'=a'\cdot b+a\cdot b'.$
  
  One may prove that this axiomatic definition yields a well-defined map respecting all of the usual ring axioms.
  
  The formula above (i.e. the definition of the formal derivative when the coefficient ring is commutative) is a direct consequence of the aforementioned axioms:
  
  ${\begin{aligned}\left(\sum _{i}a_{i}x^{i}\right)'&=\sum _{i}\left(a_{i}x^{i}\right)'\\&=\sum _{i}\left((a_{i})'x^{i}+a_{i}\left(x^{i}\right)'\right)\\&=\sum _{i}\left(0x^{i}+a_{i}\left(\sum _{j=1}^{i}x^{j-1}(x')x^{i-j}\right)\right)\\&=\sum _{i}\sum _{j=1}^{i}a_{i}x^{i-1}\\&=\sum _{i}ia_{i}x^{i-1}.\end{aligned}}$