pub fn (){
    match File::open(path){
        Ok(mut file) => {
            let = file.read_to_end(&mut buffer).unwrap();
            let inner = common::zlib::zlib_deconder(&buffer);
            
        }
    }
}