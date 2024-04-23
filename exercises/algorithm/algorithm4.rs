// // /*
// // 	binary_search tree
// // 	This problem requires you to implement a basic interface for a binary tree
// // */

// // //I AM NOT DONE
// // use std::cmp::Ordering;
// // use std::fmt::Debug;


// // #[derive(Debug)]
// // struct TreeNode<T>
// // where
// //     T: Ord,
// // {
// //     value: T,
// //     left: Option<Box<TreeNode<T>>>,
// //     right: Option<Box<TreeNode<T>>>,
// // }

// // #[derive(Debug)]
// // struct BinarySearchTree<T>
// // where
// //     T: Ord,
// // {
// //     root: Option<Box<TreeNode<T>>>,
// // }

// // impl<T> TreeNode<T>
// // where
// //     T: Ord,
// // {
// //     fn new(value: T) -> Self {
// //         TreeNode {
// //             value,
// //             left: None,
// //             right: None,
// //         }
// //     }
// // }

// // impl<T> BinarySearchTree<T>
// // where
// //     T: Ord,
// // {

// //     fn new() -> Self {
// //         BinarySearchTree { root: None }
// //     }

// //     // Insert a value into the BST
// //     fn insert(&mut self, value: T) {
// //         //TODO
// //     }

// //     // Search for a value in the BST
// //     fn search(&self, value: T) -> bool {
// //         //TODO
// //         true
// //     }
// // }

// // impl<T> TreeNode<T>
// // where
// //     T: Ord,
// // {
// //     // Insert a node into the tree
// //     fn insert(&mut self, value: T) {
// //         //TODO
// //     }
// // }


// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn test_insert_and_search() {
// //         let mut bst = BinarySearchTree::new();

        
// //         assert_eq!(bst.search(1), false);

        
// //         bst.insert(5);
// //         bst.insert(3);
// //         bst.insert(7);
// //         bst.insert(2);
// //         bst.insert(4);

        
// //         assert_eq!(bst.search(5), true);
// //         assert_eq!(bst.search(3), true);
// //         assert_eq!(bst.search(7), true);
// //         assert_eq!(bst.search(2), true);
// //         assert_eq!(bst.search(4), true);

        
// //         assert_eq!(bst.search(1), false);
// //         assert_eq!(bst.search(6), false);
// //     }

// //     #[test]
// //     fn test_insert_duplicate() {
// //         let mut bst = BinarySearchTree::new();

        
// //         bst.insert(1);
// //         bst.insert(1);

        
// //         assert_eq!(bst.search(1), true);

        
// //         match bst.root {
// //             Some(ref node) => {
// //                 assert!(node.left.is_none());
// //                 assert!(node.right.is_none());
// //             },
// //             None => panic!("Root should not be None after insertion"),
// //         }
// //     }
// // }    



// /*
// 	binary_search tree
// 	This problem requires you to implement a basic interface for a binary tree
// */

// use std::cmp::Ordering;
// use std::fmt::Debug;


// #[derive(Debug)]
// struct TreeNode<T>
// where
//     T: Ord,
// {
//     value: T,
//     left: Option<Box<TreeNode<T>>>,
//     right: Option<Box<TreeNode<T>>>,
// }

// #[derive(Debug)]
// struct BinarySearchTree<T>
// where
//     T: Ord,
// {
//     root: Option<Box<TreeNode<T>>>,
// }

// impl<T> TreeNode<T>
// where
//     T: Ord,
// {
//     fn new(value: T) -> Self {
//         TreeNode {
//             value,
//             left: None,
//             right: None,
//         }
//     }
// }

// impl<T> BinarySearchTree<T>
// where
//     T: Ord + Copy,
// {

//     fn new() -> Self {
//         BinarySearchTree { root: None }
//     }

//     // Insert a value into the BST
//     fn insert(&mut self, value: T) {
//         match self.root {
//             None => self.root = Some(Box::new(TreeNode { value, left: None, right: None })),
//             Some(ref mut n) => n.insert(value)
//         }
//     }

//     // Search for a value in the BST
//     fn search(&self, value: T) -> bool {
//         //TODO
//         let mut v = &self.root;
//         while let Some(n) = v {
//             if n.value == value {
//                 return true;
//             } else if value > n.value {
//                 v = &n.right;
//             } else {
//                 v = &n.left;
//             }
//         }
//         return false;
//     }
// }

// impl<T> TreeNode<T>
// where
//     T: Ord,
// {
//     // Insert a node into the tree
//     fn insert(&mut self, value: T) {
//         if value > self.value {
//             match self.right {
//                 None => self.right = Some(Box::new(TreeNode { value, left: None, right: None })),
//                 Some(ref mut n) => n.insert(value),
//             };
//         } else if value < self.value {
//             match self.left {
//                 None => self.left = Some(Box::new(TreeNode { value, left: None, right: None })),
//                 Some(ref mut n) => n.insert(value),
//             };
//         }
//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_insert_and_search() {
//         let mut bst = BinarySearchTree::new();

        
//         assert_eq!(bst.search(1), false);

        
//         bst.insert(5);
//         bst.insert(3);
//         bst.insert(7);
//         bst.insert(2);
//         bst.insert(4);

        
//         assert_eq!(bst.search(5), true);
//         assert_eq!(bst.search(3), true);
//         assert_eq!(bst.search(7), true);
//         assert_eq!(bst.search(2), true);
//         assert_eq!(bst.search(4), true);

        
//         assert_eq!(bst.search(1), false);
//         assert_eq!(bst.search(6), false);
//     }

//     #[test]
//     fn test_insert_duplicate() {
//         let mut bst = BinarySearchTree::new();

        
//         bst.insert(1);
//         bst.insert(1);

        
//         assert_eq!(bst.search(1), true);

        
//         match bst.root {
//             Some(ref node) => {
//                 assert!(node.left.is_none());
//                 assert!(node.right.is_none());
//             },
//             None => panic!("Root should not be None after insertion"),
//         }
//     }
// }    


/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        let mut visit_order = vec![];

        let mut visited = vec![];
        for i in 0..self.adj.len() {
            visited.push(false)
        }
        let mut stack = vec![start];
        while let Some(n) = stack.pop() {
            if !visited[n] {
                visited[n] = true;
                visit_order.push(n);
                for ne in &self.adj[n] {
                    stack.insert(0, ne.clone());
                }
            }
        }

        visit_order
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}
