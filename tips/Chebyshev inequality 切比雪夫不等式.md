tags:: tips, math, math-probability

$$
P(|X - \mu| \ge b) \le \frac{Var(X)}{b^2}
$$

where $\mu$ is the mean of `X`.

### Proof:

Apply [[Markov inequality 马尔可夫不等式]] to the variables $Y=(X - \mu)^2$

$$
P(|X - \mu| \ge b) = P((X - \mu)^2 \ge b^2) \le \frac{E(X - \mu)^2}{b^2} = \frac{Var(X)}{b^2}
$$


### Applications:

- $P(|X - \mu| \ge k \sigma ) \le \frac{1}{k^2}$

- 与平均相差2个标准差以上的值，数目不多于1/4
- 与平均相差3个标准差以上的值，数目不多于1/9
- ...


- https://en.wikipedia.org/wiki/Chebyshev%27s_inequality
