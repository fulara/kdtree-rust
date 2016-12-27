# kdtree-rust [![Build Status](https://travis-ci.org/fulara/kdtree-rust.svg?branch=develop)](https://travis-ci.org/fulara/kdtree-rust)
kdtree implementation for rust.

Implementation uses sliding midpoint variation of the tree. [More Info here](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.74.210&rep=rep1&type=pdf)

##Benchmark
`cargo bench` using travis :)
```
running 2 tests
test bench_creating_1000_node_tree              ... bench:     121,792 ns/iter (+/- 7,904)
test bench_single_loop_times_for_1000_node_tree ... bench:         161 ns/iter (+/- 88)
test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured
```

~120us to create a 1000 node tree.
160ns to query the tree.

##License
The Unlicense
