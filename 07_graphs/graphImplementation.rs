fn main() {
    // Edge List
    // Each tuple represents an edge between two nodes.
    let edge_list = vec![(0, 2), (2, 3), (2, 1), (1, 3)];

    // Adjacency List
    // Each sub-vector represents a list of nodes adjacent to the node at that index.
    let adjacency_list = vec![vec![2], vec![2, 3], vec![0, 1, 3], vec![1, 2]];

    // Adjacency Matrix
    // A two-dimensional vector where `1` indicates the presence of an edge between nodes, and `0` indicates no edge.
    let adjacency_matrix = vec![
        vec![0, 0, 1, 0],
        vec![0, 0, 1, 1],
        vec![1, 1, 0, 1],
        vec![0, 1, 1, 0],
    ];

    // Example of printing or using the structures
    println!("Edge List: {:?}", edge_list);
    println!("Adjacency List: {:?}", adjacency_list);
    println!("Adjacency Matrix:");
    for row in adjacency_matrix.iter() {
        println!("{:?}", row);
    }
}
