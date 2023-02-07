tags:: tips, math, math-combinatorics

- https://zh.wikipedia.org/wiki/%E9%94%99%E6%8E%92%E9%97%AE%E9%A2%98
- https://en.wikipedia.org/wiki/Derangement

设 $D_n$ 为 n 个数字不在自己位置上的排列总数.

`D_2` = 1
```
21
```

`D_3` = 2
```
231
312
```

`D_4` = 9
```
2143 -> 21..
2341 -> 231.
2413 -> 231.
3142 -> 312.
3412 -> 3.1.
3421 -> 312.
4123 -> 312.
4312 -> 231.
4321 -> .32.
```

一个错排将最后一个数字i与最大数字M交换:

- (case-1) 对应一个$D_{n-1}$的错排(如果最大数字M不在位置i上)

```
num:   ... M ... i
index:       i
```

- (case-2) 或者对应一个$D_{n-2}$的错排(如果最大数字M在位置i上)

```
num:   ... M ... i
index:     i
```

∴ $D_n = (n-1) D_{n-1} + (n-1) D_{n-2}$

设 $F_n = D_n -nD_{n-1}$, 则 $F_n = -F_{n-1}$, 且$F_2 = 1$,

∴ $F_n = (-1)^n$

∴ $D_n = n D_{n-1} + (-1)^n$

∴ $D_n = n! \sum_{i=0}^n \frac{(-1)^i}{i!} \approx \frac{n!}{e}$
