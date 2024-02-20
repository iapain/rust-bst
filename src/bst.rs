/// Implements a [Binary Search Tree](https://en.wikipedia.org/wiki/Binary_search_tree).
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
/// use ds_bst::BinarySearchTree;
///
/// let mut root = BinarySearchTree::from(vec![1,2,3,4,5,6,7,8,9]);
/// let ordered: Vec<_> = root.inorder();
/// let maximum = root.find_max();
/// let height = root.height();
///
/// root.insert(10);
/// root.insert(11);
/// root.insert(10); // will be ingnored
/// ```
///
/// It also supports both consumable and non-cosumable iterator
/// which returns values inorder.
///
/// ```rust
/// use ds_bst::BinarySearchTree;
/// let root = BinarySearchTree::from(vec![1,2,3,4,5,6,7,8,9]);
/// for value in &root {
///     // It will print values in-order traversal
///     println!("{}", *value);
/// }
/// ```
use std::mem::swap;
use std::cmp::{max};

#[derive(Debug)]
pub struct BinarySearchTree<T> {
    val: T,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>
}

impl<T: PartialOrd + Copy + std::fmt::Debug> BinarySearchTree<T> {
    /// Contructor creates BinarySearchTree root node
    pub fn new(v: T) -> BinarySearchTree<T> {
        BinarySearchTree {
            val: v,
            left: None,
            right: None
        }
    }
    /// Delegates tree building to `BinarySearchTree::build_recursive()`
    /// This sorts vector input and pass splice to tree builder.
    pub fn from(mut data: Vec<T>) -> BinarySearchTree<T> {
        data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        let n = data.len() as isize;
        let root = BinarySearchTree::build_recursive(&data[0..], 0, n-1);

        match root {
            None => { panic!("Empty node"); },
            Some(r) => { *r }
        }
    }

    /// Recursively builds tree maintaining BST properties.
    /// Uses `O(n)` time.
    pub fn build_recursive(data: &[T], start: isize, end: isize) -> Option<Box<BinarySearchTree<T>>> {

        if start > end {
            return None;
        };

        let mid = (start + end) / 2;

        let root = BinarySearchTree {
            val: data[mid as usize],
            left: BinarySearchTree::build_recursive(&data, start, mid-1),
            right: BinarySearchTree::build_recursive(&data, mid + 1, end)
        };
        Some(Box::new(root))
    }

    /// Inorder traverse tree which yields elements in sorted order.
    /// Uses `O(n)` time.
    pub fn inorder(&self) -> Vec<T> {
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

    /// Traverse tree in preorder.
    /// Uses `O(n)` time.
    pub fn preorder(&self) -> Vec<T> {
        let mut ret: Vec<T> = Vec::new();

        ret.push(self.val);
        match self.left {
            None => {},
            Some(ref node) => {
                let v = node.preorder();
                ret.extend(v);
            }
        }
        match self.right{
            None => {},
            Some(ref node) => {
                let v = node.preorder();
                ret.extend(v);
            }
        }
        ret
    }

    /// Calculates tree maximum height
    /// Worst case O(n)
    pub fn height(&self) -> usize {
        let hl: usize = match self.left {
            None => { 0 },
            Some(ref node) => {
                node.height()
            }
        };

        let hr: usize = match self.right{
            None => { 0 },
            Some(ref node) => {
                node.height()
            }
        };

        max(hl, hr) + 1
    }

    /// Inserts an element in a tree.
    /// Uses `O(n)` time.
    pub fn insert(&mut self, val: T) {
        if self.val > val {
            match self.left {
                None => self.left = Some(Box::new(BinarySearchTree::new(val))),
                Some(ref mut n) => n.insert(val)
            }
        } else {
            match self.right {
                None => self.right = Some(Box::new(BinarySearchTree::new(val))),
                Some(ref mut n) => n.insert(val)
            }
        }
    }


    /// Checks if element exists in a tree.
    /// Uses `O(n)` time.
    pub fn exists(&self, val: T) -> bool {
        match Some(self) {
            None => {},
            Some(ref n) => {
                if n.find(&val).is_some() {
                    return true
                }
                else {
                    return false;
                }
            }
        };

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

    /// Finds element in a tree and returns node
    /// Uses `O(n)`
    pub fn find(&self, value: &T) -> Option<Box<&BinarySearchTree<T>>> {
        if value > &self.val {
            match self.right {
                None => None,
                Some(ref n) => n.find(&value)
            }
        } else if value < &self.val {
            match self.left {
                None => None,
                Some(ref n) => n.find(&value)
            }
        } else {
            Some(Box::from(self))
        }
    }

    /// Removes a node from tree.
    /// Uses `O(n)` time
    pub fn remove(node: &mut Option<Box<BinarySearchTree<T>>>, value: &T) {
        match node {
            None => {},
            Some(ref mut n) => {
                println!("{:?} {:?}", value, &n.val);
                if &n.val < value {
                    BinarySearchTree::remove(&mut n.right, value);
                }
                else if &n.val > value {
                    BinarySearchTree::remove(&mut n.left, value);
                }
                else {
                    match(n.left.as_mut(), n.right.as_mut()) {
                        (None, None) => { swap(&mut None, node) },
                        (Some(_), None) => {
                            let l = n.left.take();
                            swap(&mut n.val, &mut l.unwrap().val);
                            swap(&mut None, &mut n.left);
                        },
                        (None, Some(_)) => {
                            let r = n.right.take();
                            swap(&mut n.val, &mut r.unwrap().val);
                            swap(&mut None, &mut n.right);
                        },
                        (Some(_), Some(_)) => {
                            let mut m = n.right.take().unwrap().find_min();
                            println!("min: {:?}", m);
                            swap(&mut None, &mut n.find(&m));
                            swap(&mut n.val, &mut m);
                        }
                    }
                }
            }
        }
    }
}

/// BinarySearchTreeIterator
pub struct BinarySearchTreeIter<'a, T> {
    nodes: Vec<&'a T>
}

impl<'a, T> BinarySearchTreeIter<'a, T>
    where
        T: PartialOrd + Copy + std::fmt::Debug
{
    /// Construct nodes based on input tree. By default
    /// it uses in-order traversal for iterator.
    fn new(root: &'a BinarySearchTree<T>) -> Self {
        let mut iter = BinarySearchTreeIter {
            nodes: Vec::new()
        };

        iter.inorder(root);

        iter
    }

    /// In-order tree traversal
    fn inorder(&mut self, tree: &'a BinarySearchTree<T>) {
        match tree.right {
            None => {},
            Some(ref node) => {
                self.inorder(node);
            }
        };
        self.nodes.push(&tree.val);
        match tree.left {
            None => {},
            Some(ref node) => {
                self.inorder(node);
            }
        }
    }
}

/// Implement iterator for BinarySearchTreeIter
/// nodes are stored in flat array. It just pop outs node
impl<'a, T> Iterator for BinarySearchTreeIter<'a, T>
    where
        T: PartialOrd + Copy + std::fmt::Debug,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.pop()
    }
}

/// implement consumable IntoIterator for BinarySearchTree
impl<T> IntoIterator for BinarySearchTree<T>
    where
        T: PartialOrd + Copy + std::fmt::Debug,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inorder().into_iter()
    }
}

/// Implement non-consumable IntoIterator for BinarySearchTree
impl<'a, T> IntoIterator for &'a BinarySearchTree<T>
    where
        T: PartialOrd + Copy + std::fmt::Debug{
    type Item = &'a T;
    type IntoIter = BinarySearchTreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        BinarySearchTreeIter::new(self)
    }
}


#[cfg(test)]
mod tests {
    use super::BinarySearchTree;
    #[test]
    fn build() {
        let mut root = BinarySearchTree::from(vec![10, 11, 5, 4, 1, 2, 3, 9 ,8, 7, 6]);
        assert_eq!(root.val, 6);
        root.insert(12);
        assert_eq!(root.exists(12), true);
        assert_eq!(root.exists(13), false);
        assert_eq!(root.exists(1), true);
        assert_eq!(root.find_min(), 1);
        assert_eq!(root.find_max(), 12);

        let sorted: Vec<_> = root.inorder();
        assert_eq!(sorted, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);

        let preorder: Vec<_> = root.preorder();
        assert_eq!(preorder, vec![6, 3, 1, 2, 4, 5, 9, 7, 8, 10, 11, 12]);
    }
    #[test]
    fn build_from_node() {
        let mut root = BinarySearchTree::new(5);
        root.insert(4);
        root.insert(6);
        root.insert(3);
        root.insert(2);
        root.insert(8);
        root.insert(8);

        assert_eq!(root.find_max(), 8);
        assert_eq!(root.find_min(), 2);
    }
    #[test]
    fn even() {
        let root = BinarySearchTree::from(vec![3,4,2,1]);
        assert_eq!(root.val, 2);
    }
    #[test]
    fn float() {
        let mut root = BinarySearchTree::from(vec![1.1, 1.0, 1.5, 1.9, 1.7]);
        assert_eq!(root.val, 1.5);
        root.insert(1.8);
        assert_eq!(root.exists(1.8), true);
        assert_eq!(root.find_max(), 1.9);
    }
    #[test]
    fn iterator_consumable() {
        let root = BinarySearchTree::from(vec![1,2,3]);
        let mut i = 1;

        for v in root {
            assert_eq!(v, i);
            i = i + 1;
        }
        // root is now consumed and cannot be used here
    }
    #[test]
    fn iterator_non_consumable() {
        let root = BinarySearchTree::from(vec![1,2,3]);
        let mut i = 1;
        for v in &root {
            assert_eq!(*v, i);
            i = i + 1;
        };

        assert_eq!(root.find_max(), 3);
        assert_eq!(root.height(), 2);
    }
    #[test]
    fn height() {
        let root = BinarySearchTree::from(vec![1]);
        assert_eq!(root.height(), 1);

        let root2 = BinarySearchTree::from(vec![11,20,29,32,41,65,50,91,72,99]);
        assert_eq!(root2.height(), 4)
    }
    #[test]
    fn remove() {
        let mut root = Some(Box::new(BinarySearchTree::from(vec![1,2,3,4,5,6,7,8,9])));
        assert_eq!(root.as_ref().unwrap().val, 5);
        BinarySearchTree::remove(&mut root, &5);
        //BinarySearchTree::remove(&mut root, &10);
        //BinarySearchTree::remove(&mut root, &4);
        assert_eq!(root.unwrap().inorder(), vec![1,2,3,4,6,7,8,9]);
    }
}
