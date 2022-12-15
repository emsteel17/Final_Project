use crate:: degree;
use rand::Rng;



pub fn steps() -> u32{
    let mut _edges = degree::nodes().1;
    let  total_nodes = degree::nodes().0;
    let  nodes_with_neighbors: Vec<u32> = Vec::new();
    let mut count: u32 = 0;
    let mut final_node:u32 = 0;
    let mut start = rand::thread_rng().gen_range(0..total_nodes.len()) as u32;
    for _i in 0..100{
        if nodes_with_neighbors.contains(&start) == false{
            start = rand::thread_rng().gen_range(0..total_nodes.len()) as u32;
            count = count + 1;
        }
        else{
            let chance = rand::thread_rng().gen_range(1..11) as u32;
            if chance <= 9{
                let random_neighbor = rand::thread_rng().gen_range(0..degree::neighbor(start).len());
                let neighbors =degree::neighbor(start);
                start = neighbors[random_neighbor];
                count = count +1;
            }
            else{
                start = rand::thread_rng().gen_range(0..total_nodes.len()-1) as u32;
                let random_neighbor = rand::thread_rng().gen_range(0..degree::neighbor(start).len());
                let neighbors = degree::neighbor(start);
                start = neighbors[random_neighbor];
                count = count +1;
            }
        }
        if count == 100{
            final_node = start;
        }    
    }
    return final_node
}
pub fn walks(){
    let mut page_rank: Vec<(u32, u32)> = Vec::new();
    let mut rank: Vec<(u32, u32)> = Vec::new();
    let mut page_rank_final: Vec<u32> = Vec::new();
    let mut outcomes: Vec<u32> = Vec::new();
    let mut count: u32 = 0;
    for _i in 0..100{
         let out: u32 = steps();
         outcomes.push(out);
    }
    for i in 0..degree::nodes().0.len(){
        count = 0;
        for j in 0..outcomes.len(){
            if degree::nodes().0[i] == outcomes[j]{
            count = count + 1;
            } 
        }
        page_rank.push((degree::nodes().0[i] as u32, count));
        page_rank_final.push(count);
    }
    page_rank_final.sort_by(|a, b| b.cmp(a));
    for i in 0..page_rank_final.len(){
        for j in 0..page_rank.len(){
            if page_rank_final[i] == page_rank[j].1{
                rank.push((page_rank[j].0, page_rank_final[i]));
            }
        }
    }
    rank.sort();
    rank.dedup();
    rank.sort_by_key(|x| x.1);
    rank.reverse();
    for i in 0..1{
        println!("The node with the highest page rank: {:?}, the page rank value: {:?}", rank[i].0, rank[i].1 as f64 / 100.0 );
    }
}