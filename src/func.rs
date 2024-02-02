use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use crate::structure::*;
use chrono::Local;
use chrono::DateTime;


pub fn add_to_file(action: String, task: String)
{
    let filename: &str = "task.json";
    let local: DateTime<Local> = Local::now();

    let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename);
    let mut todo_all: Vec<todo> = vec![];
    let mut data: String = String::from("");
  

    match file
                {
                Ok(mut f) => 
                {
                    f.read_to_string(&mut data).expect("Unable to read file");
        
                },
        
                Err(e) =>
                {

                    println!("{:?}",e);
                }
        
                };
    
    if fs::metadata(filename).unwrap().len() != 0 
    {
        todo_all = serde_json::from_str(&data).unwrap();
    }

    if action == "done"
    {
        for element in &mut todo_all
        {
            if element.get_index() == task.trim().parse::<i32>().unwrap()
            {
                element.set_action(String::from("done"));
                element.set_finish_date(local.format("%a %d %b %Y %T").to_string());
                break;
                
            }
            
        }


        
    }

    else if action == "del"
    {
        let in_ = todo_all.iter_mut().position(|x| x.get_index() == task.trim().parse::<i32>().unwrap()); 
        todo_all.remove(in_.unwrap());
      
    }

    else if action == "add"
    {
        todo_all.push(todo::new(action,task));

        let mut index=1;
        for element in &mut todo_all
        {
            element.set_index(index);
            index+=1;
        }


    }


    let data = serde_json::to_string(&todo_all).unwrap(); 
    let _ = fs::write(filename, data);



}

pub fn fetch_entries()
{
    let filename: &str = "task.json";

    let file = OpenOptions::new()
            .read(true)
            .open(filename);

            let mut todo_all: Vec<todo> = vec![];
            let mut data: String = String::from("");
          
        
            match file
                        {
                        Ok(mut f) => 
                        {
                            f.read_to_string(&mut data).expect("Unable to read file");
                      
                        },
                
                        Err(e) =>
                        {
        
                            println!("{:?}",e);
                        }
                
                        };
    if fs::metadata(filename).unwrap().len() != 0 
    {
        todo_all = serde_json::from_str(&data).unwrap();
    }

    println!("    {:^30}{:^30}{:^30}","Task".to_string(), "Creation".to_string(), "Finished".to_string());
    println!("{:-^90}","".to_string());
    for entry in todo_all
    {
        println!("{}",entry);
    }
    println!();
    println!();
    
            
}