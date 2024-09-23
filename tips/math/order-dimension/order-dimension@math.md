- **Order dimension** Dimension of Partially Ordered Set
  https://en.wikipedia.org/wiki/Order_dimension
	- The dimension of a poset P is the least integer t for which there exists a family
	  ${\mathcal {R}}=(<_{1},\dots ,<_{t})$
	  of linear extensions of P so that, for every x and y in P, x precedes y in P if and only if it precedes y in all of the linear extensions, if any such t exists. That is,
	  $P=\bigcap {\mathcal {R}}=\bigcap _{i=1}^{t}<_{i}.$
	  An alternative definition of order dimension is the minimal number of total orders such that P embeds into their product with componentwise ordering i.e. $x\leq y$ if and only if $x_{i}\leq y_{i}$ for all i
- **Linear extension**
  a linear extension of a partial order is a total order (or linear order) that is compatible with the partial order.
  Given any partial orders $\leq$ and $\leq ^{*}$ on a set $X,$ $\leq ^{*}$ is a linear extension of $\leq$ exactly when
	- $\leq ^{*}$ is a total order, and
	  For every $x,y\in X,$ if $x\leq y,$ then $x\leq ^{*}y.$
	- **狄尔沃斯定理**
	  https://zh.wikipedia.org/zh-hans/狄尔沃斯定理
	  Sperner 定理和 Dilworth 定理表明： 一个偏序集合的序维度至少等于其最大反链的大小。
-
-
- **Example of dimensions of infinite set**
	- [[infinite-poset-2d@math@order-dimension]]
	- [[infinite-poset-3d@math@order-dimension]]
	- [[infinite-poset-4d@math@order-dimension]] #read
-
- 标准例(Standard Example)或称为Sn是序理论中一个重要的概念,用于证明偏序集合序维度的下限。其主要特征如下:
  https://en.wikipedia.org/wiki/Order_dimension#Example
  https://en.wikipedia.org/wiki/Crown_graph #read
	- 组成:
		- 2n个元素:a1, a2, ..., an 和 b1, b2, ..., bn
	- 偏序关系:
		- 对每个i,ai < bi
		- 当i ≠ j时,ai与bj不可比较
	- 序维度为 *n*：
		- Sn的序维度恰好为n
		- 在标准例中，任何 𝑛 个元素的反链（元素两两不可比较）都存在，因而需要至少 𝑛  个线性扩展来表示其偏序关系。#read
-
-
- **Reference**
	- https://en.wikipedia.org/wiki/Order_theory