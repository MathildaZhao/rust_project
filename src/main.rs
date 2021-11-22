use std::borrow::BorrowMut;

mod AVLTree;

fn main() {
    let mut TestNode = AVLTree::AVLTree::new();
    for i in vec![3,5,6,8,2] {
        TestNode.insert(i);
    }
    println!("{:#?}",TestNode);
    println!("Is the searched value exist? The answer is {:?}.",TestNode.search(5));
    println!("Is the searched value exist? The answer is {:?}.",TestNode.search(11));
    println!("Height is: {}", TestNode.height());
    println!("If the tree is empty? The answer is {}.", TestNode.is_empty());
    println!("Here is the in-order traversal of the tree: {:?}",TestNode.in_order_traversal());
    println!("Here is the pre-order traversal of the tree: {:?}",TestNode.pre_order_traversal());
    println!("Here is the post-order traversal of the tree: {:?}",TestNode.post_order_traversal());

}
