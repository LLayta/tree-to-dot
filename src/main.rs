#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

// probably stupid, should associate importing with related things but whatever
use std::{
    fs,
    collections::VecDeque,
};

use petgraph::{
    Graph,
    dot::{ Dot, Config }
};

#[derive(Debug, Clone)]
struct Node<T>
where T: Ord + ToString {
    value: T,
    left:  Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> 
where T: Ord + ToString {
    fn new(value: T) -> Self {
        Self {
            value,
            left:  None,
            right: None,
        }
    }
}

#[derive(Debug)]
struct Tree<T> 
where T: Ord + ToString {
    root: Option<Box<Node<T>>>
}

impl<T> Tree<T> 
where T: Ord + ToString {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(mut self, value: T) -> Self {
        match self.root {
            None => {
                self.root = Some(Box::new(Node::new(value)));
            }

            Some(ref mut node) => {
                Self::insert_helper(node, value);
            }
        }

        self
    }

    fn insert_helper(node: &mut Node<T>, value: T) {
        if node.value > value {
            match node.left {
                Some(ref mut left) => {
                    Self::insert_helper(left, value);
                }

                None => {
                    node.left = Some(Box::new(Node::new(value)));
                }
            }
        }

        else {
            match node.right {
                Some(ref mut right) => {
                    Self::insert_helper(right, value);
                }

                None => {
                    node.right = Some(Box::new(Node::new(value)));
                }
            }
        }
    }

    fn generate_dot_graph(
        tree_graph: &mut Graph<String, String>,
        root: &Node<T>
    ) {
        let mut nodes = VecDeque::new();
        let root_idx = tree_graph.add_node(root.value.to_string());

        nodes.push_back((root, root_idx));

        while let Some((curr_node, curr_idx)) = nodes.pop_front() {
            if let Some(ref left) = &curr_node.left {
                let left_idx = tree_graph.add_node(left.value.to_string());

                tree_graph.extend_with_edges([
                    (curr_idx, left_idx) 
                ]);

                nodes.push_back((left, left_idx));
            }

            if let Some(ref right) = &curr_node.right {
                let right_idx = tree_graph.add_node(right.value.to_string());

                tree_graph.extend_with_edges([
                    (curr_idx, right_idx)
                ]);

                nodes.push_back((right, right_idx))
            }
        }
    }

    fn write_dot_file(&self) {
        let mut tree_graph = Graph::<String, String>::new();
        
        if let Some(ref root) = &self.root {
            Self::generate_dot_graph(&mut tree_graph, root);
        }

        fs::write("result.dot", format!(
            "{}",
            Dot::with_config(&tree_graph, &[Config::EdgeNoLabel])
        ))
        .expect("Failure to write .dot file to 'result.dot'");
    }
}

fn main() {
    let tree = Tree::new()
        .insert(5)
        .insert(1)
        .insert(10)
        .insert(3);
    
    tree.write_dot_file();
    println!("{:#?}", tree);
}

