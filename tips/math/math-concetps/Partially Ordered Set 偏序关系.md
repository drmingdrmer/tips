- https://en.wikipedia.org/wiki/Partially_ordered_set
- ## Definition(poset)
- a ≤ a (reflexivity: every element is related to itself).
- if a ≤ b and b ≤ a, then a = b (antisymmetry: two distinct elements cannot be related in both directions).
- if a ≤ b and b ≤ c, then a ≤ c (transitivity: if a first element is related to a second element, and, in turn, that element is related to a third element, then the first element is related to the third element).
- ## Extrema
- ### Greatest / Least Element
  
  g in P is a greatest element:
  if for every element a in P, a ≤ g
- ### Maximal / Minimal Elements
  
  g in P is a maximal element:
  if there is no element a in P such that a > g
- ### Upper / Lower bounds
  
  For a subset A of P, an element x in P is an upper bound of A:
  if a ≤ x, for each element a in A
- ### Supermum / Infimum
- Supermum: Least Upper Bound / Join
- Infimum: Greatest Lower Bound / Meet
  
  An upper bound b of S is called a supremum of S:
  if for all upper bounds z of S in P, z ≥ b.
  
  > a poset may have many minimal upper bounds without having a least upper bound
- # Preorder 预序关系
  https://en.wikipedia.org/wiki/Preorder
- ## Definition
  
  ≤ is called a ***preorder\*** or ***quasiorder\*** if it is:
  [reflexive](https://en.wikipedia.org/wiki/Reflexive_relation) and [transitive](https://en.wikipedia.org/wiki/Transitive_relation); that is, if it satisfies:
  
  1. [Reflexivity](https://en.wikipedia.org/wiki/Reflexive_relation): a≤a for all a∈P, and
  2. [Transitivity](https://en.wikipedia.org/wiki/Transitive_relation): if a≤b and b≤c then a≤c for all a,b,c∈P.
  
  But not [antisymmetric](https://en.wikipedia.org/wiki/Antisymmetric_relation).
  
  If it is also anti-symmetric it is Partial order.
- ### Example
  
  [Hasse diagram](https://en.wikipedia.org/wiki/Hasse_diagram) of the preorder *x R y* defined by *x*[//](https://en.wikipedia.org/wiki/Integer_division)4≤*y*[//](https://en.wikipedia.org/wiki/Integer_division)4 on the [natural numbers](https://en.wikipedia.org/wiki/Natural_numbers). Due to the cycles, *R* is not anti-symmetric. If all numbers in a cycle are considered equivalent, a partial, even linear, order[[1\]](https://en.wikipedia.org/wiki/Preorder#cite_note-1) is obtained. See first example below.
  
  ```
                  ...
                  ^
                  |
            9 --> 10
            ^     |
            |     v
            8 <-- 11
            ^
            |
      5 --> 6
      ^     |
      |     v
      4 <-- 7
      ^
      |
  1 --> 2
  ^     |
  |     v
  0 <-- 3
  ```