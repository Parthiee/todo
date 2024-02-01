mod structure;
mod func;

use crate::func::{add_to_file, fetch_entries};
use std::env;

fn main() {
    
    let argv: Vec<String> = env::args().collect();
    let mut task: String = String::from("");
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
    
    let action = argv.iter().nth(1).expect("Invalid Action!");


    add_to_file(action.clone(),task.clone());
    fetch_entries();
}
