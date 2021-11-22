pub mod AVLTree;


// fn r_get_num_leaves(node: &Rc<RefCell<AVLNode<u32>>>, mut current_count: u32) -> u32 {
//     let node = node.as_ref().borrow_mut();
//     if let Some(left_node) = &node.left {
//         current_count = r_get_num_leaves(left_node, current_count);
//     }
//     if let Some(right_node) = &node.right {
//         current_count = r_get_num_leaves(right_node, current_count);
//     }
//     if node.left.is_none() && node.right.is_none() {
//         current_count = current_count + 1;
//     }
//     current_count
// }