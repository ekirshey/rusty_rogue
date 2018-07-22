use super::chrono::prelude::*;

use super::Player;
use super::attack::CombatResult;

pub struct Log {
    contents : Vec<String>,
    max_size : usize,
    front : usize,
}

impl Log {
    pub fn new(max_size : usize) -> Log {
        Log {
            contents : Vec::new(),
            max_size : max_size,
            front : 0
        }
    }

    pub fn log_combat(&mut self, player : &Player, combat_results : &CombatResult) {
        let mut log_msg = String::new();
        let local : DateTime<Local> = Local::now();

        if combat_results.target_alive {
            log_msg = format!("{}:{}:{}: {} attacked {}", 
                                    local.hour(),
                                    local.minute(),
                                    local.second(),
                                    player.name(), 
                                    combat_results.target_name);
        }
        else {
            log_msg = format!("{}:{}:{}: {} killed {}", 
                                    local.hour(),
                                    local.minute(),
                                    local.second(),
                                    player.name(), 
                                    combat_results.target_name);
        }
        self.add_message(&log_msg);
    }

    pub fn add_message(&mut self, msg : &str) {
        if self.contents.len() < self.max_size {
            self.contents.push(String::from(msg));
        }
        else {
            self.contents[self.front] = String::from(msg);
            self.front += 1;
            if self.front >= self.max_size {
                self.front = 0;
            }
        }
    }

    pub fn last_n_messages(&self, n : usize) -> &[String] {
        if self.contents.len() == 0 {
            return &[]
        }

        let mut msg_count = n;
        if n >= self.contents.len() {
            msg_count = self.contents.len();
        }

        let start = self.contents.len() - msg_count;
        let end = self.contents.len();
        return &self.contents[start..end];
    }
}