use utils::{Vec2, Graph};
use entity::EntityMap;
use world::Dungeon;
use player::Player;
use log::Log;

pub enum WorldNode {
    DungeonNode(Dungeon)
}

pub struct World {
    active_node : usize,
    world_map : Graph<WorldNode>
}

impl World {
    pub fn new() -> World {
        let mut world_map = Graph::new();
        let d = Dungeon::new(1);
        let active_node = world_map.new_node(WorldNode::DungeonNode(d));
        World {
            active_node,
            world_map
        }
    }

    // Decide how you want to handle invalid active floor
    // panic for now but maybe just set to some normal value?
    fn get_mut_node(&mut self, node : usize) -> &mut WorldNode{
        let result = self.world_map.get_mut(node);
        if let Some(node) = result {
            return &mut node.data;
        }
        else {
            panic!("Invalid floor and room");
        }
    }

    fn get_node(&self, node : usize) -> &WorldNode {
        let result = self.world_map.get(node);
        if let Some(node) = result {
            return &node.data;
        }
        else {
            panic!("Invalid floor and room");
        }
    }

    pub fn active_node(&self) -> &WorldNode {
        let result = self.get_node(self.active_node);
        return &result;
    }

    pub fn valid_position(&self, pos : Vec2<usize> ) -> bool {
        let result = self.get_node(self.active_node);
        match result {
            WorldNode::DungeonNode(ref dungeon) => dungeon.valid_position(pos),
            _ => true,
        }
    }

    pub fn starting_position(&self) -> Vec2<usize> {
        let result = self.get_node(self.active_node);
        match result {
            WorldNode::DungeonNode(ref dungeon) => dungeon.starting_position(),
            _ => Vec2::new(0,0),
        }

    }

    pub fn get_entities(&self) -> Option<&EntityMap> {
        let result = self.get_node(self.active_node);
        match result {
            WorldNode::DungeonNode(ref dungeon) => return Some(dungeon.get_entities()),
        }
    }

    pub fn get_mut_entities(&mut self) -> Option<&mut EntityMap> {
        let node_id = self.active_node;
        let result = self.get_mut_node(node_id);
        match result {
            WorldNode::DungeonNode(ref mut dungeon) => 
                                return Some(dungeon.get_mut_entities()),
        }
    }

    pub fn step(&mut self, player : &mut Player)  {
        let node_id = self.active_node;
        let result = self.get_mut_node(node_id);
        match result {
            WorldNode::DungeonNode(ref mut dungeon) => dungeon.step(player)
        }
    }

    pub fn handle_player_input( &mut self, 
                                player : &mut Player,
                                new_pos : Vec2<usize>,
                                log : &mut Log)   
    {
        let node_id = self.active_node;
        let result = self.get_mut_node(node_id);
        match result {
            WorldNode::DungeonNode(ref mut dungeon) => 
                                dungeon.handle_player_input(player, new_pos, log)
        }
    }

}