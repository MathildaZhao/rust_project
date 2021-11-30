use crate::avl_tree::{Avl, Node};
use std::{io, env, process};

mod avl_tree;

// fn main() {
//     let mut node = avl_tree::Node::generate_new_node(7);
//     for i in vec![6,2,1,8,9,3] {
//         node.insert_node(i)
//     };
//     println!("Leaves: {}", node.count_leaves());
//     println!("Height: {}", node.get_tree_height());
//     println!("Inorder: {:?}", node.get_inorder());
//     println!("Preorder: {:?}", node.get_preorder());
//     println!("Postorder: {:?}", node.get_postorder());
//     println!("Empty? {}", node.is_empty());
//     node.pretty_print();
//
//     for i in vec![2,8,0,12,9,3] {
//         println!("Does {} exist? {}", i, node.is_exist(i));
//     }
//
//     node.delete_node(2);
//     node.delete_node(8);
//     node.delete_node(3);
//     println!("Inorder: {:?}", node.get_inorder());
//     node.pretty_print();
//     println!("Leaves: {}", node.count_leaves());
//     println!("Height: {}", node.get_tree_height());
//     println!("Empty? {}", node.is_empty());
// }
pub fn AVL_choices() {
    println!("---------------------------------------------");
    println!("Please choose the operation you want: (input corresponding number)");
    println!("1. Insert a node to the red-black tree.");
    println!("2. Delete a node from the red-black tree.");
    println!("3. Count the number of leaves in a tree.");
    println!("4. Return the height of a tree.");
    println!("5. Print In-order traversal of the tree.");
    println!("6. Print Pre-order traversal of the tree.");
    println!("7. Print Post-order traversal of the tree.");
    println!("8. Check if the tree is empty.");
    println!("9. Check if an element exists in the tree.");
    println!("10. Pretty print the tree");
    println!("11. Quit.");
    println!("---------------------------------------------")
}

// pub fn get_user_input() -> i32 {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("Wrong input!");
//     match input.trim().parse() {
//         Ok(num) => {
//             num
//         }
//         Err(_) => 0
//         }
//     }

fn main() {
    let args: Vec<String> = env::args().collect();
    let length = args.len();
    if length < 2{
        println!("If you want to use the AVL Tree, please type cargo run AVLTree");
        println!("If you want to use the RB Tree, please type cargo run RBTree");
    } else {
        let key_word = &args[1];
        match key_word.as_str() {
            "RBTree" => {
                todo!()
                //please add your RBTree part
            }
            "AVLTree" => {
                if length == 2 {
                    let mut root: Node<_> = Avl::generate_new_tree();
                    loop {
                        AVL_choices();
                        let mut user_choice = String::new();
                        io::stdin().read_line(&mut user_choice).expect("Failed to read line!");
                        let user_choice : i32= user_choice.trim().parse().expect("Wrong!");

                        match user_choice {
                            1 => {
                                println!("please input the value you want to insert: ");
                                let mut insert_input = String::new();
                                io::stdin().read_line(&mut insert_input).expect("Failed to read line!");
                                let insert_input : i32= insert_input.trim().parse().expect("Wrong!");
                                root.insert_node(insert_input);
                            }
                            2 => {
                                println!("please input the value you want to delete: ");
                                let mut delete_input = String::new();
                                io::stdin().read_line(&mut delete_input).expect("Failed to read line!");
                                let delete_input : i32= delete_input.trim().parse().expect("Wrong!");
                                root.delete_node(delete_input);
                            }
                            3 => {
                                let mut leaves_result = root.count_leaves();
                                println!("Leaves:{:?}", leaves_result);
                            }
                            4 => {
                                let mut height_result = root.get_tree_height();
                                println!("Tree height: {:?}", height_result);
                            }
                            5=>{
                                let mut in_order = root.get_inorder();
                                println!("In-order traversal: {:?}", in_order);
                            }
                            6=>{
                                let mut pre_order = root.get_preorder();
                                println!("Pre-order traversal: {:?}", pre_order);
                            }
                            7=>{
                                let mut post_order = root.get_postorder();
                                println!("Post-order traversal: {:?}", post_order);
                            }
                            8=>{
                                println!("Is the tree empty? {:?}",root.is_empty());
                            }
                            9 =>{
                                println!("please input the value you want to search: ");
                                let mut search_input = String::new();
                                io::stdin().read_line(&mut search_input).expect("Failed to read line!");
                                let search_input : i32= search_input.trim().parse().expect("Wrong!");
                                println!("Is the value exist? {:?}",root.is_exist(search_input));
                            }
                            10=>{
                                root.pretty_print();
                            }
                            11=>{
                                println!("Bye!");
                                break;
                            }
                            _ => {
                                println!("Invalid input!");
                            }
                        }
                    }
                } else {
                    eprintln!("Wrong!");
                    process::exit(1);
                }
            }
            _ => {
                eprintln!("Wrong!");
                process::exit(1);
            }
        }
    }
}