extern crate id_tree;

use id_tree::InsertBehavior::*;

use id_tree::Node;
use id_tree::NodeIdError::*;
use id_tree::RemoveBehavior::*;

use id_tree::Tree;
use id_tree::TreeBuilder;

#[test]
fn test_old_node_id() {
    let mut tree: Tree<i32> = TreeBuilder::new().build();

    let root_node = Node::new(1);

    let root_id = tree.insert(root_node, AsRoot).ok().unwrap();
    let root_id_copy = root_id.clone(); // this is essential to getting the Result::Err()

    let root_node = tree.remove_node(root_id, OrphanChildren);
    assert!(root_node.is_ok());

    let root_node_again = tree.remove_node(root_id_copy, OrphanChildren);
    assert!(root_node_again.is_err());

    let error = root_node_again.err().unwrap();
    assert_eq!(error, NodeIdNoLongerValid);
}

#[test]
fn test_ancestors_old_id() {
    let mut a = Tree::new();

    let root_id = a.insert(Node::new(1), AsRoot).unwrap();
    // `.clone()` required to get this error
    let root_id_clone = root_id.clone();
    let _ = a.remove_node(root_id, DropChildren).unwrap();

    // note usage of cloned `NodeId`
    let ancestors = a.ancestors(&root_id_clone);

    assert!(ancestors.is_err());
    let error = ancestors.err().unwrap();
    assert_eq!(error, NodeIdNoLongerValid);
}

#[test]
fn test_ancestor_ids_old_id() {
    let mut a = Tree::new();

    let root_id = a.insert(Node::new(1), AsRoot).unwrap();
    // `.clone()` required to get this error
    let root_id_clone = root_id.clone();
    let _ = a.remove_node(root_id, DropChildren).unwrap();

    // note usage of cloned `NodeId`
    let ancestors = a.ancestor_ids(&root_id_clone);

    assert!(ancestors.is_err());
    let error = ancestors.err().unwrap();
    assert_eq!(error, NodeIdNoLongerValid);
}

#[test]
fn test_children_old_id() {
    let mut a = Tree::new();

    let root_id = a.insert(Node::new(1), AsRoot).unwrap();
    // `.clone()` required to get this error
    let root_id_clone = root_id.clone();
    let _ = a.remove_node(root_id, DropChildren).unwrap();

    // note usage of cloned `NodeId`
    let ancestors = a.children(&root_id_clone);

    assert!(ancestors.is_err());
    let error = ancestors.err().unwrap();
    assert_eq!(error, NodeIdNoLongerValid);
}

#[test]
fn test_children_ids_old_id() {
    let mut a = Tree::new();

    let root_id = a.insert(Node::new(1), AsRoot).unwrap();
    // `.clone()` required to get this error
    let root_id_clone = root_id.clone();
    let _ = a.remove_node(root_id, DropChildren).unwrap();

    // note usage of cloned `NodeId`
    let ancestors = a.children_ids(&root_id_clone);

    assert!(ancestors.is_err());
    let error = ancestors.err().unwrap();
    assert_eq!(error, NodeIdNoLongerValid);
}
