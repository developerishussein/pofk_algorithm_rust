//! Examples for all tree_algorithms
use pofk_algorithms::tree_algorithms::binary_tree_traversal::{TreeNode, inorder_traversal, preorder_traversal, postorder_traversal};
use pofk_algorithms::tree_algorithms::diameter_of_tree::diameter_of_tree;
use pofk_algorithms::tree_algorithms::balanced_tree::is_balanced;
use pofk_algorithms::tree_algorithms::level_order_traversal::level_order_traversal;
use pofk_algorithms::tree_algorithms::lowest_common_ancestor::lowest_common_ancestor;
use pofk_algorithms::tree_algorithms::invert_tree::invert_tree;
use pofk_algorithms::tree_algorithms::zigzag_traversal::zigzag_traversal;
use pofk_algorithms::tree_algorithms::validate_bst::validate_bst;
use pofk_algorithms::tree_algorithms::tree_depth::tree_depth;
use pofk_algorithms::tree_algorithms::serialize_deserialize::{serialize, deserialize};

fn main() {
    // Build a simple tree:   1
    //                      / \
    //                     2   3
    let mut root = Some(Box::new(TreeNode::new(1)));
    root.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
    root.as_mut().unwrap().right = Some(Box::new(TreeNode::new(3)));

    println!("Inorder: {:?}", inorder_traversal(&root));
    println!("Preorder: {:?}", preorder_traversal(&root));
    println!("Postorder: {:?}", postorder_traversal(&root));
    println!("Diameter: {}", diameter_of_tree(&root));
    println!("Is balanced: {}", is_balanced(&root));
    println!("Level order: {:?}", level_order_traversal(&root));
    println!("Zigzag order: {:?}", zigzag_traversal(&root));
    println!("Is BST: {}", validate_bst(&root));
    println!("Depth: {}", tree_depth(&root));
    let ser = serialize(&root);
    println!("Serialized: {:?}", ser);
    let deser = deserialize(&ser);
    println!("Deserialized inorder: {:?}", inorder_traversal(&deser));
    // LCA (for values 2 and 3)
    println!("LCA of 2 and 3: {:?}", lowest_common_ancestor(&root, &2, &3));
    // Invert tree
    let mut root2 = root.clone();
    invert_tree(&mut root2);
    println!("Inverted inorder: {:?}", inorder_traversal(&root2));
}
