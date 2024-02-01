use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use crate::structure::*;

pub fn add_to_file(action: String, task: String)
{
    let filename: &str = "task.json";

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
                    println!("{}",data);
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
                break;
                
            }
            
        }


        
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
    fs::write(filename, data);


    


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
                            println!("{}",data);
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

    for entry in todo_all
    {
        println!("{}",entry);
    }
    
            
}