/*
* This program is made by Manfred
* 2021/01/12
* (c)2021 Manfred MIT
*
*
*/
use std::io::{stdin,stdout,Write};
use std::io::{prelude::*,BufReader};
use std::fs::File;
use std::path::Path;

//These function are functions that load files 
pub fn rfileOpen(names: String) -> String {
    let s = "".to_string();
    let fnm = s+&names;
    let contents = match std::fs::read_to_string(fnm){
        Err(why) => panic!("couldn't read file: {}", why),
        Ok(contents) => contents,
    };
    
    return contents;
}
// gives me a Vector with lines (this bit is not made by Manfred)
pub fn lines_from_file(fnm:String) -> Vec<String> {
    let x: &str = &fnm;
    let mut errcode = "Error".to_owned();
    errcode.push_str(x);
    let file = File::open(x).expect(&errcode);
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
pub fn writeFile(names: String, cont:&[u8]) -> i8 {
    let s = "".to_string();
    let fnm = s+&names;
    /*let mut file = std::fs::File::create(fnm+".txt"){
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    file.write_all(cont).expect({return 3;});
    return 1;*/
    let p: &str = &fnm;
    let path = Path::new(p);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(cont) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
    return 1;
 

}