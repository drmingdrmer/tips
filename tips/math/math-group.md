### 半群
S×S→S。若·满足结合律，即：
∀x,y,z∈S，有(x·y)·z=x·(y·z)
- ### 幺半群
  是一个带有二元运算 `*`: M × M → M 的集合 M ，其符合下列公理：
  结合律：对任何在 M 内的a、b、c ， `(a·b)·c = a·(b·c)` 。
  单位元：存在一在 M 内的元素e，使得任一于 M 内的 a 都会符合 a*e = e*a = a 。
  
  **幺半群** 也可以说是带有单位元的**半群**
- ### order 阶数 (群)
- 一个群的阶是指其势，即其元素的个数；
- 一个群内的一个元素a之阶（有时称为周期）是指会使得am = e的最小正整数m（其中的e为这个群的单位元素，且am为a的m次幂）。若没有此数存在，则称a有无限阶。有限群的所有元素有有限阶。
  
  一个群G的阶被标记为ord(G)或|G|，而一个元素的阶则标记为ord(a)或|a|。
  [Order](https://en.wikipedia.org/wiki/Order_(group_theory))
- ### normal subgroup 正规子群
  https://en.wikipedia.org/wiki/Normal_subgroup
  
  群G的子群N是正规子群，如果它在共轭变换下不变；就是说对于每个N中元素n和每个G中的元素g，元素gng−1仍在N中。我们写为
  
  $$
  N \triangleleft G, \Leftrightarrow \forall n \in {N}, \forall g \in G, gng^{-1} \in N
  $$
  
  下列条件等价于子群 N 在 G 中是正规子群。其中任何一个都可以用作定义：
  
  对于 G 中的所有g，$gNg^{-1} ⊆ N$。
  对于 G 中的所有g，$gNg^[−1] = N$。
  N 在 G 中的左陪集的集合和右陪集的集合是一致的。
  对于 G 中的所有g，gN = Ng。
  N 是 G 的若干共轭类的并集。
  存在以 N 为核的 G 的群同态。
- ### Index of a subgroup
  Formally, the index of H in G is defined as the number of cosets of H in G.
  
  The index of H in G is usually denoted |G : H| or [G : H] or (G:H)
  
  If G and H are finite groups, then the index of H in G is equal to the quotient of the orders of the two groups:
  $$
  |G:H| = \frac{|G|}{|H|}.
  $$
- ### quotient group
  Let N be a normal subgroup of a group G.
  We define the set G/N to be the set of all left cosets of N in G, i.e., G/N = { aN : a ∈ G }.
  Define an operation on G/N as follows.
  For each aN and bN in G/N, the product of aN and bN is (aN)(bN).
  This defines an operation on G/N, because we have the following equalities of subsets of G:
  
  (aN)(bN) = a(Nb)N = a(bN)N = (ab)NN = (ab)N