// // /*
// // 	bfs
// // 	This problem requires you to implement a basic BFS algorithm
// // */

// // //I AM NOT DONE
// // use std::collections::VecDeque;

// // // Define a graph
// // struct Graph {
// //     adj: Vec<Vec<usize>>, 
// // }

// // impl Graph {
// //     // Create a new graph with n vertices
// //     fn new(n: usize) -> Self {
// //         Graph {
// //             adj: vec![vec![]; n],
// //         }
// //     }

// //     // Add an edge to the graph
// //     fn add_edge(&mut self, src: usize, dest: usize) {
// //         self.adj[src].push(dest); 
// //         self.adj[dest].push(src); 
// //     }

// //     // Perform a breadth-first search on the graph, return the order of visited nodes
// //     fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        
// // 		//TODO

// //         let mut visit_order = vec![];
// //         visit_order
// //     }
// // }


// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn test_bfs_all_nodes_visited() {
// //         let mut graph = Graph::new(5);
// //         graph.add_edge(0, 1);
// //         graph.add_edge(0, 4);
// //         graph.add_edge(1, 2);
// //         graph.add_edge(1, 3);
// //         graph.add_edge(1, 4);
// //         graph.add_edge(2, 3);
// //         graph.add_edge(3, 4);

// //         let visited_order = graph.bfs_with_return(0);
// //         assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
// //     }

// //     #[test]
// //     fn test_bfs_different_start() {
// //         let mut graph = Graph::new(3);
// //         graph.add_edge(0, 1);
// //         graph.add_edge(1, 2);

// //         let visited_order = graph.bfs_with_return(2);
// //         assert_eq!(visited_order, vec![2, 1, 0]);
// //     }

// //     #[test]
// //     fn test_bfs_with_cycle() {
// //         let mut graph = Graph::new(3);
// //         graph.add_edge(0, 1);
// //         graph.add_edge(1, 2);
// //         graph.add_edge(2, 0);

// //         let visited_order = graph.bfs_with_return(0);
// //         assert_eq!(visited_order, vec![0, 1, 2]);
// //     }

// //     #[test]
// //     fn test_bfs_single_node() {
// //         let mut graph = Graph::new(1);

// //         let visited_order = graph.bfs_with_return(0);
// //         assert_eq!(visited_order, vec![0]);
// //     }
// // }


// /*
// 	bfs
// 	This problem requires you to implement a basic BFS algorithm
// */

// use std::collections::VecDeque;

// // Define a graph
// struct Graph {
//     adj: Vec<Vec<usize>>,
// }

// impl Graph {
//     // Create a new graph with n vertices
//     fn new(n: usize) -> Self {
//         Graph {
//             adj: vec![vec![]; n],
//         }
//     }

//     // Add an edge to the graph
//     fn add_edge(&mut self, src: usize, dest: usize) {
//         self.adj[src].push(dest);
//         self.adj[dest].push(src);
//     }

//     // Perform a breadth-first search on the graph, return the order of visited nodes
//     fn bfs_with_return(&self, start: usize) -> Vec<usize> {
//         let mut visit_order = vec![];

//         let mut visited = vec![];
//         for i in 0..self.adj.len() {
//             visited.push(false)
//         }
//         let mut stack = vec![start];
//         while let Some(n) = stack.pop() {
//             if !visited[n] {
//                 visited[n] = true;
//                 visit_order.push(n);
//                 for ne in &self.adj[n] {
//                     stack.insert(0, ne.clone());
//                 }
//             }
//         }

//         visit_order
//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_bfs_all_nodes_visited() {
//         let mut graph = Graph::new(5);
//         graph.add_edge(0, 1);
//         graph.add_edge(0, 4);
//         graph.add_edge(1, 2);
//         graph.add_edge(1, 3);
//         graph.add_edge(1, 4);
//         graph.add_edge(2, 3);
//         graph.add_edge(3, 4);

//         let visited_order = graph.bfs_with_return(0);
//         assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
//     }

//     #[test]
//     fn test_bfs_different_start() {
//         let mut graph = Graph::new(3);
//         graph.add_edge(0, 1);
//         graph.add_edge(1, 2);

//         let visited_order = graph.bfs_with_return(2);
//         assert_eq!(visited_order, vec![2, 1, 0]);
//     }

//     #[test]
//     fn test_bfs_with_cycle() {
//         let mut graph = Graph::new(3);
//         graph.add_edge(0, 1);
//         graph.add_edge(1, 2);
//         graph.add_edge(2, 0);

//         let visited_order = graph.bfs_with_return(0);
//         assert_eq!(visited_order, vec![0, 1, 2]);
//     }

//     #[test]
//     fn test_bfs_single_node() {
//         let mut graph = Graph::new(1);

//         let visited_order = graph.bfs_with_return(0);
//         assert_eq!(visited_order, vec![0]);
//     }
// }

/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/

use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src); 
    }

    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        //TODO
        visit_order.push(v);
        visited.insert(v);
        for ne in &self.adj[v] {
            if visited.contains(ne) {
                continue;
            }
            self.dfs_util(*ne, visited, visit_order);
        }
    }

    // Perform a depth-first search on the graph, return the order of visited nodes
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut visit_order = Vec::new(); 
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]); 
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]); 
    }
}
