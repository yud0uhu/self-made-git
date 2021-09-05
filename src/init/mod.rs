use std::fs;
use std::fs::File;

pub fn create_init_file() -> Result<(), String> {
    match fs::create_dir("./.chobit"){
        Ok(_)=> {}
        Err(_) => {
            return Err("failed to create file".to_string());
        }
    }

    match File::create("./.chobit/HEAD") {
        Ok(_)=> {}
        Err(_) => {
            return Err("failed to create file".to_string());
        }
    }

    match fs::create_dir("./.chobit/branches") {
        Ok(_) => {}
        Err(_) => {
            return Err("failed to create dir".to_string());
        }
    }

    match File::create("./.chobit/config") {
        Ok(_)=> {}
        Err(_) => {
            return Err("failed to create file".to_string());
        }
    }

    match File::create("./.chobit/description") {
        Ok(_)=> {}
        Err(_) => {
            return Err("failed to create file".to_string());
        }
    }

    match fs::create_dir("./.chobit/hooks") {
        Ok(_) => {}
        Err(_) => {
            return Err("failed to create dir".to_string());
        }
    } 

    match fs::create_dir("./.chobit/info") {
        Ok(_) => {}
        Err(_) => {
            return Err("failed to create dir".to_string());
        }
    }

    match fs::create_dir("./.chobit/objects") {
        Ok(_) => {}
        Err(_) => {
            return Err("failed to create dir".to_string());
        }
    }
    
    match fs::create_dir("./.chobit/refs") {
        Ok(_) => {}
        Err(_) => {
            return Err("failed to create dir".to_string());
        }
    }    

   return Ok(());
}