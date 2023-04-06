# `sequencer`

Finding holes in positive integer sequences.

## Example

Given the sequence

```
1 2 3 4 _ 6 7
```

the next number to be inserted should be `5`, since it is currently missing.

## Strategies

`sequencer` provides two strategies:

- `sequential_search` iterates from start to end
- `divide_and_conquer` determines in which half of the sequence a hole may exist, thefore runs in `O(log(n))`

## Rough Benchmark

```
+------------+-----------+----------------+----------------------+----------------+
| Length     | Took CC   | Comparisons CC | Took SS              | Comparisons SS |
+------------+-----------+----------------+----------------------+----------------+
|         10 |     667ns |              6 |                583ns |              6 |
+------------+-----------+----------------+----------------------+----------------+
|        100 |     379ns |             14 |            1us 546ns |             74 |
+------------+-----------+----------------+----------------------+----------------+
|       1000 |     526ns |             20 |           13us 416ns |            496 |
+------------+-----------+----------------+----------------------+----------------+
|      10000 |     628ns |             24 |          106us 941ns |           5954 |
+------------+-----------+----------------+----------------------+----------------+
|     100000 |  1us 51ns |             29 |      1ms 552us 127ns |          86832 |
+------------+-----------+----------------+----------------------+----------------+
|    1000000 | 1us 638ns |             36 |     18ms 918us 656ns |         974965 |
+------------+-----------+----------------+----------------------+----------------+
|   10000000 | 3us 971ns |             40 |    114ms 250us 489ns |        5999446 |
+------------+-----------+----------------+----------------------+----------------+
|  100000000 | 6us 221ns |             42 |    820ms 671us 776ns |       43292435 |
+------------+-----------+----------------+----------------------+----------------+
| 1000000000 | 6us 357ns |             48 | 8s 250ms 816us 317ns |      428581316 |
+------------+-----------+----------------+----------------------+----------------+
```
