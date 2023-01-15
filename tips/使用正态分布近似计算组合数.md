title:: 使用正态分布近似计算组合数
tags:: tips, math, binomial-distribution, normal-distribution, approximation


[二项式分布](https://zh.wikipedia.org/wiki/%E4%BA%8C%E9%A0%85%E5%BC%8F%E5%88%86%E5%B8%83)可以近似为[正态分布](https://zh.wikipedia.org/wiki/%E6%AD%A3%E6%80%81%E5%88%86%E5%B8%83), 所以组合数 `C(n, m)` 也可以用正态来近似计算,
我用这种方法在工程设计阶段做快速的数值估算, 挺方便的:

例如要计算12 盘 [RAID-6](https://zh.wikipedia.org/wiki/RAID) 的数据丢失风险(2块冗余盘所以掉3盘会丢数据:`C(12, 3)`),
或设计hash table时估算一个bucket中刚好存储了`i`个key的概率: $p(i) = {n \choose i} ( \frac{1}{b} )^{ i } ( 1 - \frac{1}{b} )^{n-i}$(`n`是key总数, `b`是bucket总数)

![foo](../assets/approx-combination/binomial-normal-margin.png)

```python
import math

def facto(n):
    if n == 1:
        return 1
    return n*facto(n-1)

def combi(n, m):
    return facto(n) / facto(m) / facto(n-m)

def approx_combi(n, m):
    u = n * .5
    sig = (n*.5*.5) ** 0.5

    power = - (m-u) ** 2 / 2 / sig / sig
    return 1.0 / (sig * ((2*math.pi)**.5)) * (math.e ** power) * (2**n)

n, m = 12, 3
print("Approximate combination number C({}, {}) by normal distribution:".format(n, m))
print("accurate, approximated:")
print(combi(n, m), '%.3f' % approx_combi(n, m))
```

[approx-combination.py](../assets/approx-combination/approx-combination.py)

Output:

```
Approximate combination number C(12, 3) by normal distribution:

accurate, approximated:
220.0     210.508
```
