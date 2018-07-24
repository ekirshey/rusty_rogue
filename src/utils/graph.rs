type NodeId = usize;

pub struct Node<T> {
    neighbors : Vec<NodeId>,

    pub data: T
}

pub struct Graph<T> {
    nodes : Vec<Node<T>>
}

impl<T> Graph<T> {
    fn new() -> Graph<T> {
        Graph {
            nodes : Vec::new()
        }
    }

    fn new_node(&mut self, data: T) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(
            Node {
                neighbors : Vec::new(),
                data
            }
        );
        next_index
    }

    fn add_neighbor(&mut self, node : NodeId, neighbor : NodeId) -> Result<(), &str> {
        if node != neighbor && 
           node < self.nodes.len() && 
           neighbor < self.nodes.len() 
        {
            self.nodes[node].neighbors.push(neighbor);
            return Ok(());
        }

        Err("Invalid NodeId")
    }

    fn add_new_neighbor(&mut self, node : NodeId, data : T) -> Result<NodeId, &str> {
        if node < self.nodes.len() {
            let neighbor = self.new_node(data);
            self.nodes[node].neighbors.push(neighbor);
            return Ok(neighbor);
        }

        Err("Invalid NodeId")
    }

    fn get_neighbors(&self, node : NodeId) -> Result<&Vec<NodeId>, &str> {
        if node < self.nodes.len() {
            &self.nodes[node].neighbors;
        }

        Err("Invalid NodeId")
    }
}