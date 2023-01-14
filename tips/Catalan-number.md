tags:: math, math-combinatorics

[Catalan number](https://en.wikipedia.org/wiki/Catalan_number) satisfies **recurrence relation**:

$$
C_0 = 1
\quad
\text{and}
\quad
C_{n+1} = \sum_{i=0}^{n} C_i C_{n-i}
\tag{reccurrence}
$$

And its general term formula is( ${2n \choose n}$ is [binomial-coefficient](https://en.wikipedia.org/wiki/Binomial_coefficient) ):

$$
C_n = \frac{1}{n+1} {2n \choose n}
$$

## Proofs

- [Catalan-number proof: generating function](./Catalan-number%20proof:%20generating%20function.md)


## Application

- The count of different [binary-tree](https://en.wikipedia.org/wiki/Binary_tree) of exactly `n` nodes is $C_n$:
    [n节点的binary-tree的个数](./n节点的binary-tree的个数.md)

