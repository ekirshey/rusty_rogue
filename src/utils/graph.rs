type NodeId = usize;


pub struct Node<T> {
    neighbors : Vec<NodeId>,

    pub data: T
}

pub struct Graph<T> {
    nodes : Vec<Node<T>>
}

impl<T> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph {
            nodes : Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn new_node(&mut self, data: T) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(
            Node {
                neighbors : Vec::new(),
                data
            }
        );
        next_index
    }

    pub fn add_neighbor(&mut self, node : NodeId, neighbor : NodeId) -> Result<(), &str> {
        if node != neighbor && 
           node < self.nodes.len() && 
           neighbor < self.nodes.len() 
        {
            self.nodes[node].neighbors.push(neighbor);
            return Ok(());
        }

        Err("Invalid NodeId")
    }

    pub fn add_new_neighbor(&mut self, node : NodeId, data : T) -> Result<NodeId, &str> {
        if node < self.nodes.len() {
            let neighbor = self.new_node(data);
            self.nodes[node].neighbors.push(neighbor);
            return Ok(neighbor);
        }

        Err("Invalid NodeId")
    }

    pub fn get_neighbors(&self, node : NodeId) -> Option<&Vec<NodeId>> {
        if node < self.nodes.len() {
            return Some(&self.nodes[node].neighbors);
        }

        None
    }

    pub fn get(&self, node : NodeId) -> Option<&Node<T>> {
        self.nodes.get(node)
    }

    pub fn get_mut(&mut self, node : NodeId) -> Option<&mut Node<T>> {
        self.nodes.get_mut(node)
    }
}