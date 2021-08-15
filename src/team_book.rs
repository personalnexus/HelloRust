use std::collections::HashMap;
use std::i32;
use std::time;
use std::time::SystemTime;

#[derive(Clone)]
enum Presence {
    Unknown,
    OutOfOffice(i32), // exit code
    Working(String), // last message
    OnBreak,
}

#[derive(Clone)]
pub struct Member {
    name: String,
    presence: Presence,
    last_activity: time::SystemTime
}


impl Member {
    fn new() -> Self {
        Self { 
            name: String::new(), 
            presence: Presence::Unknown, 
            last_activity: SystemTime::UNIX_EPOCH
        }
    }

    fn get_status(&self) -> String {
        let presence = match &self.presence  {
            Presence::Unknown => "doing who knows what".to_string(),
            Presence::OutOfOffice(exit_code) => format!("gone ('{}')", exit_code),
            Presence::Working(last_message) => format!("working {})", if last_message.len() > 0 { format!("('{}')", last_message) } else { "in silence".to_string() }),
            Presence::OnBreak => "on break".to_string()
        };
        format!("{} is currently {}. Last activity: {:#?}.", self.name, presence, self.last_activity)
    }

    fn update(&mut self, message: &str) {
        self.presence = self.parse_presence(message);
        self.last_activity = time::SystemTime::now();
    }

    fn parse_presence(&self, message: &str) -> Presence {
        match message.to_lowercase().trim()
        {
            "begin" => Presence::Working("just came in".to_string()),
            "break" => Presence::OnBreak,
            "continue" => Presence::Working("Back from break. What did I miss?".to_string()),
            "exit" => Presence::OutOfOffice(0),
            x => self.parse_extended_presence(x)
        }
    }

    fn parse_extended_presence(&self, message: &str) -> Presence {
        
        if message.starts_with("exit(") && message.ends_with(")") {
            let exit_code = &message[5..message.len()-1];
            Presence::OutOfOffice(exit_code.parse::<i32>().unwrap())
        } else {
            match self.presence {
                Presence::OutOfOffice(_) => Presence::Working(message.to_string()),
                Presence::Working(_) => Presence::Working(message.to_string()),
                _ => Presence::Unknown
            }
        }
    }
}

pub struct Members {
    members_by_name: HashMap<String, Member>
}

impl Members {
    
    pub fn new() -> Self { 
        Self { 
            members_by_name: HashMap::new()
        } 
    }

    pub fn update(&mut self, name: &str, message: &str) {
        let member = self.members_by_name.entry(String::from(name)).or_insert_with(Member::new);
        member.update(message);
    }

    pub fn get_member_status(&self, name: &str) -> String {
        self.members_by_name.get(name).unwrap().get_status()
    }
}

