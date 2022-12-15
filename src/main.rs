use final_project::degree;
use final_project::closeness;
use final_project::page_rank;
use final_project::betweeness;


fn main(){   
   degree::degree_of_nodes();
   closeness::distances();
   page_rank::walks();
   betweeness::betweenness();
}

