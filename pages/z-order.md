- https://docs.delta.io/latest/optimizations-oss.html#data-skipping
	- Z-Ordering is **not idempotent**. Everytime the Z-Ordering is executed it will try to create a new clustering of data in all files (new and existing files that were part of previous Z-Ordering) in a partition.
	- Z-Ordering aims to produce evenly-balanced data files with respect to the number of tuples, but not necessarily data size on disk. The two measures are most often correlated, but there can be situations when that is not the case, leading to skew in optimize task times.
	  
	  For example, if you ZORDER BY *date* and your most recent records are all much wider (for example longer arrays or string values) than the ones in the past, it is expected that the OPTIMIZE job’s task durations will be skewed, as well as the resulting file sizes. This is, however, only a problem for the OPTIMIZE command itself; it should not have any negative impact on subsequent queries.
	- delta.io:
- https://en.wikipedia.org/wiki/Z-order_curve
-
-
- not idempotent:
-