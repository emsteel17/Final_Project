use crate::read_file;
use crate::degree;
use std::collections::VecDeque;

pub struct Graph {
    n: usize,
    edges: Vec<Vec<usize>>,
}
fn reverse_edges(list:&Vec<(usize, usize)>) -> Vec<(usize,usize)> {
    let mut new_list = vec![];
    for (u,v) in list {
        new_list.push((*v,*u));
    }
    new_list
}

impl Graph {
    fn add_directed_edges(&mut self, edges:&Vec<(usize, usize)>) {
        for (u,v) in edges {
            self.edges[*u].push(*v);
        }
    }
    fn sort_graph_lists(&mut self) {
        for l in self.edges.iter_mut() {
l.sort();
}
    }
    fn create_directed(n:usize,edges:&Vec<(usize,usize)>)-> Graph {
        let mut g = Graph{n,edges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
    fn create_undirected(n:usize,edges:&Vec<(usize,usize)>)-> Graph {
        let mut g = Self::create_directed(n,edges);
        g.add_directed_edges(&reverse_edges(edges));
        g.sort_graph_lists();
        g                                        
    }

}

pub fn closeness1(start: usize, graph: &Graph) -> f64{
    let mut distance: Vec<Option<u32>> = vec![None;graph.n];
    let mut total : Vec<u32> = Vec::new();
    distance[start] = Some(0); 
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front() { 
        for u in graph.edges[v].iter() {
            if let None = distance[*u] { 
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
            }
        }
    }
    for v in 0..graph.n {
        total.push(distance[v].unwrap());
    }
    let mut average : f64 = 0.0;
    let mut count : f64 = -1.0;
    for i in 0..total.len(){
        average = average + (total[i] as f64);
        count = count + 1.0;
    }
    
    return average / count
}

pub fn distances() {
    let mut e : Vec<(usize, usize)> = Vec::new();

    let edges: Vec<(u32, u32)> = read_file:: file();
    let mut closeness : Vec <f64> = Vec::new();
    let mut trial: f64 = 0.0;
    let mut min: f64 = 10000000.0;
    let mut node: usize = 0;
    for i in 0..edges.len(){
        e.push((edges[i].0 as usize, edges[i].1 as usize));
    }
    let nodes = degree::nodes().0;
    let n : usize = nodes.len();
    let graph = Graph::create_undirected(n, &e);
    
    for i in 0..graph.n{
       
        closeness.push(closeness1(i, &graph));
    }
    for i in 0..closeness.len(){
        trial = closeness[i];
       if trial < min{
           min = trial;
            node = i;
        }
    }
    
  println!("The node with the best closeness centrality is: {}, and its closeness centrality is: {}", node, min);
   
}
