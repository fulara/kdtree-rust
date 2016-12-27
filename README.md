# kdtree-rust [![Build Status](https://travis-ci.org/fulara/kdtree-rust.svg?branch=develop)](https://travis-ci.org/fulara/kdtree-rust)
kdtree implementation for rust.

Implementation uses sliding midpoint variation of the tree. [More Info here](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.74.210&rep=rep1&type=pdf)

##Benchmark
`cargo bench` using travis :)
```
running 3 tests
test bench_creating_1000_000_node_tree          ... bench: 275,155,622 ns/iter (+/- 32,713,321)
test bench_creating_1000_node_tree              ... bench:     121,314 ns/iter (+/- 1,977)
test bench_single_loop_times_for_1000_node_tree ... bench:         162 ns/iter (+/- 76)
test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured
```

~275ms to create a 1000_000 node tree. << this bench is now disabled.  
~120us to create a 1000 node tree.  
160ns to query the tree.  

##License
The Unlicense
