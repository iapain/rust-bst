# Binary Search Tree

Implements a Binary Search Tree in Rust. This is a recursive data structure and left and right refers to sub trees.

# Installation
In your cargo dependencies add following
```
ds-bst = "*"
```

```rust
use ds_bst::BinarySearchTree;

let mut node = BinarySearchTree:new(5);
node.insert(1);
node.insert(2);
node.insert(10);
// or

let mut root = BinarySearchTree::from(vec![1,2,3,4,5,6,7,8,9]);
root.insert(10);
let ordered: Vec<_> = root.inorder();
let min = root.find_min();
let max = root.find_max();
```
