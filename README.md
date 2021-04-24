### Rust implementation of a Binary Search Tree

Implements a Binary Search Tree Node. This is a recursive data structure and left and right refers to sub trees.

```rust
use bst::TreeNode;

let mut root = TreeNode::build(vec![1,2,3,4,5,6,7,8,9]);
root.insert(10);
let ordered: Vec<_> = root.inorder();
let min = root.find_min();
let max = root.find_max();
```
