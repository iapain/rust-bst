/// Implements a Binary Search Tree Node.
/// This is a recursive data structure and left
/// and right refers to sub trees.
///
/// Tree is an entry point for the root node. It's much simpler
/// to create a tree form a vector.
///
/// # Example
/// Implements binary search tree with traversal (inorder)
///
/// ```rust
/// use bst::TreeNode;
///
/// let mut root = TreeNode::build(vec![1,2,3,4,5,6,7,8,9]);
/// root.insert(10);
/// let ordered: Vec<_> = root.inorder();
/// ```
pub struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>
}

impl<T: PartialOrd + Copy> TreeNode<T> {
    /// Delegates tree building to `TreeNode::build_recursive()`
    /// This sorts vector input and pass splice to tree builder.
    pub fn build(mut data: Vec<T>) -> TreeNode<T> {
        data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        let n = data.len() as isize;
        let root = TreeNode::build_recursive(&data[0..], 0, n-1);

        match root {
            None => { panic!("Empty node"); },
            Some(r) => { *r }
        }
    }

    /// Recursively builds tree maintaining BST properties.
    /// Uses `O(n)` time.
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

    /// Inorder traverse tree which yields elements in sorted order.
    /// Uses `O(n)` time.
    /// TODO: Implement Iterator
    pub fn inorder(&self) -> Vec<T>{
        let mut ret: Vec<T> = Vec::new();

        match self.left {
            None => {},
            Some(ref node) => {
                let v = node.inorder();
                ret.extend(v);
            }
        };
        ret.push(self.val);
        match self.right {
            None => {},
            Some(ref node) => {
                let v = node.inorder();
                ret.extend(v);
            }
        }
        ret
    }

    /// Inserts an element in a tree.
    /// Uses `O(n)` time.
    pub fn insert(&mut self, val: T) {
        if self.val > val {
            match self.left {
                None => self.left = Some(Box::new(TreeNode {val: val, left: None, right: None})),
                Some(ref mut n) => n.insert(val)
            }
        } else {
            match self.right {
                None => self.right = Some(Box::new(TreeNode {val: val, left: None, right: None})),
                Some(ref mut n) => n.insert(val)
            }
        }
    }


    /// Checks if element exists in a tree.
    /// Uses `O(n)` time.
    pub fn exists(&self, val: T) -> bool {
        if self.val == val {
            return true;
        }
        if self.val > val {
            return match self.left {
                None => false,
                Some(ref n) => n.exists(val)
            };
        }
        if self.val < val {
            return match self.right {
                None => false,
                Some(ref n) => n.exists(val)
            };
        }
        false
    }

    /// Finds minimum element in a tree.
    /// Uses `O(n)` time.
    pub fn find_min(&self) -> T {
        match self.left {
            None => self.val,
            Some(ref n) => n.find_min()
        }
    }

    /// Finds maximum element in a tree.
    /// Uses `O(n)` time.
    pub fn find_max(&self) -> T {
        match self.right {
            None => self.val,
            Some(ref n) => n.find_max()
        }
    }
}



#[cfg(test)]
mod tests {
    use super::TreeNode;
    #[test]
    fn build() {
        let mut root = TreeNode::build(vec![10,11,5,4,1,2,3,9,8,7,6]);
        assert_eq!(root.val, 6);
        root.insert(12);
        assert_eq!(root.exists(12), true);
        assert_eq!(root.exists(13), false);
        assert_eq!(root.exists(1), true);
        assert_eq!(root.find_min(), 1);
        assert_eq!(root.find_max(), 12);
        let sorted: Vec<_> = root.inorder();
        assert_eq!(sorted, vec![1,2,3,4,5,6,7,8,9,10,11,12]);
    }
    #[test]
    fn even() {
        let root = TreeNode::build(vec![3,4,2,1]);
        assert_eq!(root.val, 2);
    }
    #[test]
    fn float() {
        let mut root = TreeNode::build(vec![1.1, 1.0, 1.5, 1.9, 1.7]);
        assert_eq!(root.val, 1.5);
        root.insert(1.8);
        assert_eq!(root.exists(1.8), true);
        assert_eq!(root.find_max(), 1.9);
    }
}
