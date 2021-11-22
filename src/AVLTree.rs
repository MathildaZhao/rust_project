use std::rc::Rc;
use std::{
    cell::RefCell,
    cmp::{max, Ordering},
    mem::{replace, swap},
};

// use rand::seq::SliceRandom;
// use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq, Clone)]
struct AVLNode<T: Ord + Copy> {
    pub key: T,
    height: i32,
    left: Option<Rc<RefCell<AVLNode<T>>>>,
    right: Option<Rc<RefCell<AVLNode<T>>>>,
}

impl<T: Ord + Copy> AVLNode<T> {
    fn new(val: T) -> Self {
        AVLNode {
            key: val,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn left_height(&self) -> i32 {
        // get the height of the left
        self.left
            .as_ref()
            .map_or(0, |left| left.as_ref().borrow().height())
    }

    fn right_height(&self) -> i32 {
        // get the height of the right
        self.right
            .as_ref()
            .map_or(0, |right| right.as_ref().borrow().height())
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn update_height(&mut self) {
        //height = 1 + maximum (left or right) height
        self.height = 1 + max(self.left_height(), self.right_height());
    }

    fn get_balance(&self) -> i32 {
        //balance is the subtraction between the two height.
        self.left_height() - self.right_height()
    }

    pub fn rotate_right(&mut self) -> bool {
        if self.left.is_none() {
            // check whether it has left child
            return false;
        }
        // if it has, get the nodes.
        // An example
        //         D
        //        / \
        //       B   E
        //      / \
        //     A   C
        let left_node = self.left.as_mut().unwrap();
        // In the example, LN = B (with A&C)
        let left_left_node = left_node.as_ref().borrow_mut().left.take();
        // In the example, LLN = A
        let left_right_node = left_node.as_ref().borrow_mut().right.take();
        // In the example, LRN = C
        let new_right_node = replace(&mut self.left, left_left_node);
        // A is the new LN
        swap(&mut self.key, &mut new_right_node.as_ref().unwrap().as_ref().borrow_mut().key);
        // Swap B and D.
        let right_node = self.right.take();
        // E is the RN
        //         B
        //        / \
        //       A   E
        let new_right_tree = new_right_node.as_ref().unwrap().clone();
        // clone a NRD let it be the NRT
        new_right_tree.as_ref().borrow_mut().left = left_right_node;
        // C is the left one
        //         D
        //        /
        //       C
        new_right_tree.as_ref().borrow_mut().right = right_node;
        // E is the right one
        //         D
        //        / \
        //       C   E
        self.right = new_right_node;
        // NRN is
        //         D
        //        / \
        //       C   E
        // so after rotation, the
        //       B
        //      / \
        //     A   D
        //        / \
        //       C   E
        if let Some(node) = self.right.as_mut() {
            node.as_ref().borrow_mut().update_height();
        }
        self.update_height();
        // update the height
        true
    }

    pub fn rotate_left(&mut self) -> bool {
        if self.right.is_none() {
            //check whether it has right child
            return false;
        }
        // if it has, get the nodes.
        // An example
        //       B
        //      / \
        //     A   D
        //        / \
        //       C   E
        let right_node = self.right.as_ref().unwrap().clone();
        // RN is D (with C&E )
        let right_right_node = right_node.as_ref().borrow_mut().right.take();
        // RRN is E
        let right_left_node = right_node.as_ref().borrow_mut().left.take();
        // RLN is C
        let new_left_node = replace(&mut self.right, right_right_node);
        // E is the new RN
        swap(&mut self.key, &mut new_left_node.as_ref().unwrap().as_ref().borrow_mut().key);
        // Swap B and D
        let left_node = self.left.take();
        //A is the LN
        //       D
        //      / \
        //     A   E
        let new_left_tree = new_left_node.as_ref().unwrap().clone();
        // clone a NLN and name it as NLT
        new_left_tree.as_ref().borrow_mut().right = right_left_node;
        // C is the right one
        //         B
        //          \
        //           C
        new_left_tree.as_ref().borrow_mut().left = left_node;
        // A is the left one
        //         B
        //        / \
        //       A   C
        self.left = new_left_node;
        // After rotation, it is:
        //         D
        //        / \
        //       B   E
        //      / \
        //     A   C
        if let Some(node) = self.left.as_mut() {
            node.as_ref().borrow_mut().update_height();
        }
        self.update_height();
        // update the height
        true
    }

    fn rebalance(&mut self) -> bool {
        let balance = self.get_balance();
        //get the balance
        if balance > 1 {
            // balance greater than one means the left node has mor child than the right.
            let left_node = self.left.clone();
            if let Some(node) = &left_node {
                if node.as_ref().borrow_mut().get_balance() == 1 {
                    // if the node in the left node is right heavy
                    node.as_ref().borrow_mut().rotate_left();
                    // do a left rotation inner the left node first
                }
            }
            self.rotate_right();
            // do a right rotation to rebalance it.

            return true;
        } else if balance < -1 {
            // balance smaller than one means the right node has mor child than the right.
            let right_node = self.right.clone();
            if let Some(node) = &right_node {
                if node.as_ref().borrow_mut().get_balance() == 1 {
                    // if the node in the right node is left heavy
                    node.as_ref().borrow_mut().rotate_right();
                    // do a right rotation inner the right node first
                }
            }
            self.rotate_left();
            // do a right rotation to rebalance it.

            return true;
        }
        false
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AVLTree {
    root: Option<Rc<RefCell<AVLNode<u32>>>>,
}

impl AVLTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn search(&self, val: u32) -> bool {
        if self.root.is_none() {
            return false;
        } else {
            let mut current_tree = self.root.clone();
            while let Some(cur_node) = current_tree {
                if cur_node.as_ref().borrow_mut().key.lt(&val) {
                    // for example, if val is 7
                    //       5
                    //     / \
                    //    2   7
                    //   / \
                    //  1   3
                    if let Some(node) = &cur_node.as_ref().borrow_mut().right {
                        //next node is 7
                        current_tree = Some(node.clone());
                    } else {
                        return false;
                    }
                } else if cur_node.as_ref().borrow_mut().key.gt(&val) {
                    // for example, if val is 1
                    //      5
                    //     / \
                    //    2   7
                    //   / \
                    //  1   3
                    if let Some(node) = &cur_node.as_ref().borrow_mut().left {
                        //    2
                        //   / \
                        //  1   3
                        current_tree = Some(node.clone());
                    } else {
                        return false;
                    }
                } else {
                    return true;
                }
            }
        }
        false
    }

    pub fn insert(&mut self, val: u32) -> Result<(), String> {
        if self.root.is_none() {
            self.root = Some(Rc::new(RefCell::new(AVLNode::new(val))));
            Ok(())
        } else {
            let mut path: Vec<Rc<RefCell<AVLNode<u32>>>> = Vec::new();
            let mut current_tree = self.root.clone();
            while let Some(node) = current_tree {
                path.push(node.clone());
                let mut current_node = node.as_ref().borrow_mut();
                if current_node.key < val {
                    if let Some(node) = &current_node.right {
                        current_tree = Some(node.clone());
                    } else {
                        current_node.right =
                            Some(Rc::new(RefCell::new(AVLNode::new(val))));
                        break;
                    }
                } else if current_node.key > val {
                    if let Some(node) = &current_node.left {
                        current_tree = Some(node.clone());
                    } else {
                        current_node.left =
                            Some(Rc::new(RefCell::new(AVLNode::new(val))));

                        break;
                    }
                } else {
                    return Err(format!("Node already exists").to_string());
                }
            }

            for node in path.into_iter().rev() {
                node.as_ref().borrow_mut().update_height();
                node.as_ref().borrow_mut().rebalance();
            }
            Ok(())
        }
    }

    pub fn height(&self) -> i32 {
        match &self.root {
            Some(node) => node.as_ref().borrow_mut().height(),
            None => 0i32,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn in_order_get(&self, node: &Option<Rc<RefCell<AVLNode<u32>>>>, vec: &mut Vec<u32>) {
        if let Some(current_node) = node {
            self.in_order_get(&current_node.borrow_mut().left, vec);
            vec.push(current_node.borrow().key);
            self.in_order_get(&current_node.borrow_mut().right, vec);
        }
    }

    pub fn in_order_traversal(&self) -> Vec<u32> {
        // in order traversal example
        //        A
        //       / \
        //      B   C
        //    /    / \
        //   D    E   F
        //  / \    \
        // G   H    I
        // the root is in the middle, from left to right
        // the left child is always in front of the root
        // the root is always in front of the right child
        //So in this example, after in order traversal, the result is G-D-H-B-A-E-I-C-F
        let mut vec = Vec::new();
        self.in_order_get(&self.root, &mut vec);
        vec
    }

    fn pre_order_get(&self, node: &Option<Rc<RefCell<AVLNode<u32>>>>, vec: &mut Vec<u32>) {
        if let Some(current_node) = node {
            vec.push(current_node.borrow().key);
            self.pre_order_get(&current_node.borrow_mut().left, vec);
            self.pre_order_get(&current_node.borrow_mut().right, vec);
        }
    }

    pub fn pre_order_traversal(&self) -> Vec<u32> {
        // pre order traversal example
        //        A
        //       / \
        //      B   C
        //    /    / \
        //   D    E   F
        //  / \    \
        // G   H    I
        // the roots are in front, from left to right,
        // the roots of a tree are always in front of the left child
        // the left child always in front of the right child
        // So in this example, after pre order traversal, the result is A-B-D-G-H-C-E-I-F
        let mut vec = Vec::new();
        self.pre_order_get(&self.root, &mut vec);
        vec
    }

    fn post_order_get(&self, node: &Option<Rc<RefCell<AVLNode<u32>>>>, vec: &mut Vec<u32>) {
        if let Some(current_node) = node {
            self.post_order_get(&current_node.borrow_mut().left, vec);
            self.post_order_get(&current_node.borrow_mut().right, vec);
            vec.push(current_node.borrow().key);
        }
    }

    pub fn post_order_traversal(&self) -> Vec<u32> {
        // post order traversal example
        //        A
        //       / \
        //      B   C
        //    /    / \
        //   D    E   F
        //  / \    \
        // G   H    I
        // the root is the last, from left to right,
        // the left child is always in front of the right child
        // the right child is always in front of the root
        // So in this example, after pre order traversal, the result is G-H-D-B-I-E-F-C-A
        let mut vec = Vec::new();
        self.post_order_get(&self.root, &mut vec);
        vec
    }
}