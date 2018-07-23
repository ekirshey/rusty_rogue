use super::chrono::prelude::*;

use super::Player;
use super::attack::Attackable;
use super::attack::CombatResult;

// Change it to hold a struct type so I can style the messages.
// Like color the "killed" message red
pub struct Log {
    contents : Vec<String>,
    max_size : usize,
    front : usize,
}

impl Log {
    pub fn new(max_size : usize) -> Log {
        Log {
            contents : Vec::new(),
            max_size,
            front : 0
        }
    }

    pub fn log_combat(&mut self, player : &Player, combat_results : &CombatResult) {
        let log_msg : String;
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
        if self.contents.is_empty() {
            return &[]
        }

        let msg_count = if n >= self.contents.len() {self.contents.len()} else {n};

        let start = self.contents.len() - msg_count;
        let end = self.contents.len();
        &self.contents[start..end]
    }
}