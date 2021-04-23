/// Implements a TreeNode in Binary Search Tree
/// This is a recursive data structure and left
/// and right refers to sub trees.
///
/// Tree is an entry point for the root node. It's much simpler
/// to create a tree form sorted unique i32.
///
/// # Example
/// Implements binary search tree with traversal (inorder)
///
/// ```rust
/// use bst::TreeNode;
///
/// let root = TreeNode::build(vec![1,2,3,4,5,6,7,8,9]);
///
/// root.inorder()
/// ```
use std::fmt::Debug;
use std::cmp::Ord;

pub struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>
}

impl<T: Debug + Ord + Copy> TreeNode<T> {
    // data vec must be sorted
    pub fn build(mut data: Vec<T>) -> TreeNode<T> {
        data.sort();
        let n = data.len() as isize;
        let root = TreeNode::build_recursive(&data[0..], 0, n-1);

        match root {
            None => { panic!("Empty node"); },
            Some(r) => { *r }
        }
    }

    pub fn build_recursive(data: &[T], start: isize, end: isize) -> Option<Box<TreeNode<T>>> {

        if start > end {
            return None;
        };

        let mid = (start + end) / 2;

        let root = TreeNode {
            val: data[mid as usize],
            left: TreeNode::build_recursive(&data, start, mid-1),
            right: TreeNode::build_recursive(&data, mid + 1, end)
        };
        Some(Box::new(root))
    }

    pub fn inorder(&self) {
        match self.left {
            None => {},
            Some(ref node) => {
                node.inorder();
            }
        };
        println!("{:?}", self.val);
        match self.right {
            None => {},
            Some(ref node) => {
                node.inorder();
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::TreeNode;
    #[test]
    fn build() {
        let root = TreeNode::build(vec![10,11,5,4,1,2,3,9,8,7,6]);
        root.inorder();
        assert_eq!(root.val, 6);
    }
}
