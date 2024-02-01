use std::env;
use serde::{Serialize, Deserialize};
use std::fmt;


#[derive(Serialize, Deserialize, Debug)]
pub struct todo
{
    action: String,
    task: String,
    task_no: i32,
    
}

impl todo
{
    pub fn new(action: String, task: String) -> todo
    {
        todo
        {
            action,
            task,
            task_no: 0i32
        }
    }

    pub fn set_index(self: &mut Self,index: i32)
    {
        self.task_no = index;
    }

    pub fn get_index(self: &mut Self) -> i32
    {
        self.task_no
    }

    pub fn set_action(self: &mut Self,action: String)
    {
        self.action = action;
    }
}

impl fmt::Display for todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.action == "done"
        {
            write!(f, "[✔] {}", self.task)
        }

        else 
        {
            write!(f, "[ ] {}", self.task)
        }
        

        
    }
}




