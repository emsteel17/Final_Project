use crate::read_file;


pub fn nodes() -> (Vec<u32>, Vec<(u32,u32)>){
    let edges_1 = read_file:: file();
    let mut missing_neighbors: Vec<u32> = Vec::new();
    let mut nodes: Vec<u32> = Vec::new(); 
    for i in 0..edges_1.len(){
        if nodes.contains(&(i as u32)) == false{
            nodes.push(edges_1[i].0); 
        }
        }
        for i in 0..edges_1.len(){
            if missing_neighbors.contains(&(i as u32)) == false{
                missing_neighbors.push(edges_1[i].1); 
            }
        }
        missing_neighbors.dedup();
        for i in 0..missing_neighbors.len(){
            if nodes.contains(&(missing_neighbors[i])) == false{
                nodes.push(missing_neighbors[i]);
            }
        }
    nodes.dedup();
    nodes.sort();
    return (nodes,edges_1)
}

pub fn neighbor(node: u32) -> Vec<u32>{
    let mut neighbors: Vec<u32> = Vec::new();
    let edges: Vec<(u32, u32)> = nodes().1;
    for j in 0..edges.len(){
        if node == edges[j].0{
            neighbors.push(edges[j].1);
         }
        if node == edges[j].1{
            neighbors.push(edges[j].0);
        }       
     }
   return neighbors
}


pub fn degree_of_nodes(){
    let edges: Vec<u32> = nodes().0;
    let mut degrees : Vec<(u32, usize)> = Vec::new();
    let mut highest : usize = 0;
    let mut highest_node : u32 = 0;
    for i in 0..edges.len(){
        let node = edges[i];
        let num_neighbors : usize = neighbor(node).len();
        degrees.push((node,num_neighbors));
        
    }

    for i in 0..degrees.len(){
        if degrees[i].1 > highest{
            highest = degrees[i].1;
            highest_node = degrees[i].0;
        }
    }
    println!("The node with the highest degree is node: {}, and its degree is: {}",highest_node, highest);

    assert_eq!(neighbor(6), [4, 5, 0, 2, 19, 18, 7]);
}

