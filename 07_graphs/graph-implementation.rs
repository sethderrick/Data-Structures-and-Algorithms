/*
  A simple graph implementation in Rust needs to account for Rust's ownership 
  and type system. 
  
  This Rust implementation uses HashMap for adjacency lists and encapsulates 
  the graph functionalities within a struct and its methods.

  Type System: Rust requires explicit types, so we use HashMap<String, Vec<String>> 
  for the adjacency list.

  Error Handling: In Rust, you have to handle possible errors explicitly. 
  For instance, unwrap() is used here for simplicity, assuming that nodes exist 
  when edges are added. In production, you should handle these cases more 
  gracefully.

  Ownership and Borrowing: Methods that modify the state of the struct take 
  &mut self, while those that don't modify the state take &self.
  
  Formatting: Implementing the Display trait allows us to customize the output 
  format of the struct instance, similar to the __str__ method in Python.
*/

use std::collections::HashMap;
use std::fmt;

struct Graph {
    number_of_nodes: usize,
    adjacent_list: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            number_of_nodes: 0,
            adjacent_list: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, node: &str) {
        self.adjacent_list.insert(node.to_string(), Vec::new());
        self.number_of_nodes += 1;
    }

    fn add_edge(&mut self, node1: &str, node2: &str) {
        self.adjacent_list.get_mut(node1).unwrap().push(node2.to_string());
        self.adjacent_list.get_mut(node2).unwrap().push(node1.to_string());
    }

    fn show_connections(&self) {
        for (vertex, neighbors) in &self.adjacent_list {
            println!("{} --> {}", vertex, neighbors.join(" "));
        }
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.adjacent_list)
    }
}

fn main() {
    let mut my_graph = Graph::new();
    my_graph.add_vertex("0");
    my_graph.add_vertex("1");
    my_graph.add_vertex("2");
    my_graph.add_vertex("3");
    my_graph.add_vertex("4");
    my_graph.add_vertex("5");
    my_graph.add_vertex("6");
    my_graph.add_edge("3", "1");
    my_graph.add_edge("3", "4");
    my_graph.add_edge("4", "2");
    my_graph.add_edge("4", "5");
    my_graph.add_edge("1", "2");
    my_graph.add_edge("1", "0");
    my_graph.add_edge("0", "2");
    my_graph.add_edge("6", "5");

    println!("{}", my_graph);
    my_graph.show_connections();
}
