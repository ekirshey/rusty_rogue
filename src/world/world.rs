use utils::Graph;
use world::Dungeon;

pub enum WorldNode {
    DungeonNode(Dungeon)
}

pub struct World {
    width : usize,
    height : usize,
    active_node : usize,
    world_map : Graph<WorldNode>
}

impl World {
    pub fn new(width : usize, height : usize) -> World {
        let mut world_map = Graph::new();
        let d = Dungeon::new(1);
        let active_node = world_map.new_node(WorldNode::DungeonNode(d));
        World {
            width,
            height,
            active_node,
            world_map
        }
    }

    pub fn active_node(&self) -> &WorldNode {
        let result = self.world_map.get(self.active_node);
        if let Some(node) = result {
            return &node.data;
        }
        else {
            panic!("World active node is invalid!");
        }
    }
}