// #[derive(Debug)]
// pub struct BSTNode {
//     value: i32,
//     left: Option<Box<BSTNode>>,
//     right: Option<Box<BSTNode>>,
// }
//
// impl BSTNode {
//     pub fn new(&self)->Self{
//
//     }
//     pub fn insert(&mut self, val: i32) {
//         if val > self.value {
//             match self.left {
//                 None => self.left = Some(Box::new(BSTNode { value: val, left: None, right: None })),
//                 Some(ref mut node) => node.insert(val)
//             }
//         } else {
//             match self.right {
//                 None => self.right = Some(Box::new(BSTNode { value: val, left: None, right: None })),
//                 Some(ref mut node) => node.insert(val)
//             }
//         }
//     }
//
//     pub fn find(&mut self, val: i32) -> bool {
//         if val == self.value {
//             return true;
//         }
//         if val > self.value {
//             match self.left {
//                 None => false,
//                 Some(ref mut node) => node.find(val)
//             }
//         } else {
//             match self.right {
//                 None => false,
//                 Some(ref mut node) => node.find(val)
//             }
//         }
//     }
// }

use std::cmp::Ord;
use std::cmp::Ordering;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum BST<T: Ord> {
    Leaf {
        value: T,
        left: Box<BST<T>>,
        right: Box<BST<T>>,
    },
    Empty,
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST::Empty
    }

    pub fn create(value: T) -> Self {
        BST::Leaf {
            value,
            left: Box::new(BST::Empty),
            right: Box::new(BST::Empty),
        }
    }

    pub fn insert(&mut self, new_value: T) {
        match self {
            BST::Leaf {
                ref value,
                ref mut left,
                ref mut right,
            } => match new_value.cmp(value) {
                Ordering::Less => left.insert(new_value),
                Ordering::Greater => right.insert(new_value),
                _ => {}
            },
            BST::Empty => {
                *self = BST::create(new_value);
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            BST::Empty => true,
            BST::Leaf { .. } => false,
        }
    }

    pub fn find(&self, find_value: T) -> bool {
        match self {
            BST::Leaf {
                ref value,
                ref left,
                ref right,
            } => match find_value.cmp(value) {
                Ordering::Less => left.find(find_value),
                Ordering::Greater => right.find(find_value),
                Ordering::Equal => true,
            },
            BST::Empty => false,
        }
    }
}

impl<T: Ord> Default for BST<T> {
    fn default() -> Self {
        Self::new()
    }
}