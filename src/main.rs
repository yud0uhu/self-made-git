use std::io;
use std::env;
use std::path::PathBuf;
mod init;

fn main(){
    
    let args: Vec<String> = env::args().collect();

    // unwrap()で中身の値を取り出す
    let path = env::current_dir().unwrap();

    // init
    if args[1] == "init" {
        match init::create_init_file()  {
            Ok(_) => println!("Initialized empty Git repository in {} /.chobit", path.display()),
            Err(e) => println!("{}", e),
        }
    }

    // cat-file
    // if args[1] == "cat-file" {
    //     if args.len() != 3 {
    //         println!("{:?}", args);
    //         return;
    //     }
    // }
    // return;
}