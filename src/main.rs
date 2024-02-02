mod structure;
mod func;

use crate::func::{add_to_file, fetch_entries};
use std::env;


fn main() {
    
    let argv: Vec<String> = env::args().collect();
    let mut task: String = String::from("");


    let action = argv.iter().nth(1).expect("Invalid Action!");
    println!(r"


   __            __              ___ 
   / /_____  ____/ /___     _____/ (_)
  / __/ __ \/ __  / __ \   / ___/ / / 
 / /_/ /_/ / /_/ / /_/ /  / /__/ / /  
 \__/\____/\__,_/\____/   \___/_/_/   
                                      
 
 
");

    if action == "show"
    {
        
        fetch_entries();
        return;
    }

   


    for index in 2..=argv.len()
    {
        let val = argv.get(index);


        match val
        {
            Some(ref i) => 
            {
                task.push_str(i);
                task.push_str(" ");
            }

            __ =>
            {
                break;
            }
        }
        
        
    }
    
    
    

    add_to_file(action.clone(),task.clone());
    fetch_entries();

   
}
