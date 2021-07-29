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
    fn get_status(&self) -> String {
        let presence = match &self.presence
        {
            Presence::Unknown => "doing who knows what".to_string(),
            Presence::OutOfOffice(exit_code) => format!("gone ('{}')", exit_code),
            Presence::Working(last_message) => format!("working {})", if last_message.len() > 0 { format!("('{}')", last_message) } else { "in silence".to_string() }),
            Presence::OnBreak => "on break".to_string()
        };
        format!("{} is currently {}. Last activity: {:#?}.", self.name, presence, self.last_activity)
    }

    fn update(&self, name: &str, message: &str) -> Member {
        Member { 
            name: name.to_string(),
            presence: self.parse_presence(message), 
            last_activity: time::SystemTime::now()
        }
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

static EMPTY_MEMBER: Member = Member { name: String::new(), presence: Presence::Unknown, last_activity: SystemTime::UNIX_EPOCH };

impl Members {
    
    pub fn new(members_by_name: Option<HashMap<String, Member>> ) -> Self 
    { 
        Self { 
            members_by_name: members_by_name.unwrap_or_else(HashMap::new) 
        } 
    }

    pub fn update(&self, name: &str, message: &str) -> Members
    {
        let old_member = self.members_by_name.get(name).unwrap_or(&EMPTY_MEMBER);

        let new_member = old_member.update(name, message);
        
        let mut new_members_by_name = self.members_by_name.clone();
        new_members_by_name.insert(name.to_string(), new_member);
        Members::new(Some(new_members_by_name))
    }

    pub fn get_member_status(&self, name: &str) -> String {
        self.members_by_name.get(name).unwrap().get_status()
    }
}

