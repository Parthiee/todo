use serde::{Serialize, Deserialize};
use std::fmt;
use chrono::prelude::*;
use chrono::Local;
use colored::Colorize;

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub struct todo
{
    action: String,
    task: String,
    task_no: i32,
    date_created: String,
    date_completed: String
    
}


impl todo
{
    pub fn new(action: String, task: String) -> todo
    {
        let local: DateTime<Local> = Local::now();
        
        todo
        {
            action,
            task,
            task_no: 0i32,
            date_created: local.format("%a %d %b %Y %T").to_string(),
            date_completed: " ".to_string()
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

    pub fn set_finish_date(self: &mut Self, date:  String)
    {
        self.date_completed = date;
    }
}

impl fmt::Display for todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.action == "done"
        {

            write!(f, "[✔] {:30} {:30} {:30}", self.task.truecolor(189,224,254).strikethrough(), self.date_created.truecolor(189,224,254), self.date_completed.truecolor(189,224,254))
        }

        else 
        {
            write!(f, "[ ] {:30} {:30} {:30}", self.task.truecolor(255,255,138), self.date_created.truecolor(255,255,138), self.date_completed.truecolor(255,255,138))
        }
        

        
    }
}




