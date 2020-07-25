pub fn run() -> Vec<Vec<i32>> {
    let input = vec![
        vec![1,2],
        vec![3],
        vec![3],
        Vec::new()
    ];

    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut output = Vec::new();
        fn recurse(
            node: i32,
            mut path: Vec<i32>,
            graph: &Vec<Vec<i32>>, 
            output: &mut Vec<Vec<i32>>
            ) {
            path.push(node);
            if node as usize == graph.len() - 1 {
                output.push(path);
            }
            else {
                for edge in &graph[node as usize] {
                    recurse(*edge, path.to_vec(), graph, output);
                }
            }
        }

        recurse(0, Vec::new(), &graph, &mut output);
        output
    }

    all_paths_source_target(input)
}