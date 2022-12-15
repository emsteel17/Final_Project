use crate::degree;
use std::collections::VecDeque;

fn bfs(start: usize, end:usize, n:usize) -> Vec<usize>{
    let mut visited_nodes = vec![false; n];
    let mut previous_nodes = vec![100000;n];
    let mut queue = VecDeque::new();

    visited_nodes[start] = true;
    queue.push_back(start);

    while queue.is_empty() == false{
        let current = queue.pop_front().unwrap();
        if current == end{
            break;
        }

        for i in degree::neighbor(current as u32){
            if visited_nodes[i as usize] == false{
                visited_nodes[i as usize] = true;
                previous_nodes[i as usize] = current;
                queue.push_back(i as usize);
            }
        }
    }
    let mut shortest_path = Vec::new();
    let mut node = end;
    while node != start{
        shortest_path.push(node);
        node = previous_nodes[node];
    }
    shortest_path.push(start);

    return shortest_path
}


pub fn betweenness(){
    let mut edges: Vec<(u32, u32)> = degree::nodes().1;
    let mut edge: Vec<(usize,usize)> = Vec::new();
    let mut all_paths : Vec<Vec<usize>> = Vec::new();
    for i in 0..edges.len(){
        edge.push((edges[i].0 as usize, edges[i].1 as usize));
    }
    edges.sort();
    let n : usize = degree::nodes().0.len();
    let mut paths2 : Vec<Vec<usize>> = Vec::new();
    for i in 0..n{
        for j in 0..n{
            let paths : Vec<usize> = bfs(i,j,n);
            if paths.len() > 2{
                paths2.push(paths);
            }
        }
    }
    for i in 0..paths2.len(){
        paths2[i].remove(0);
        let mut length = paths2[i].len();
        paths2[i].remove(length-1);
        
    }
    let mut count : f32 = 0.0;
    let divide : f32 = (n as f32 -1.0) * (n as f32-2.0);
    let mut between : Vec<(usize, f32)> = Vec::new();
    for i in 0..n{
        count = 0.0;
        for j in 0..paths2.len(){
            if paths2[j].contains(&i) == true{
                count = count + 1.0;
            }
        }
        between.push((i, count / divide));
    }
    let mut max : f32 = 0.0;
    let mut max_node : usize = 0;
    for i in 0..between.len(){
        if between[i].1 > max{
            max = between[i].1;
            max_node = i;
        }
    }
   println!("The node with the highest betweenness is : {} and it is {}", max_node, max);
   assert_eq!(bfs(0, 25, n),[25, 0]);
   assert_eq!(bfs(7,18,n), [18, 4, 7] );

}

