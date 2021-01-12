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
mod filehandler;




fn getin(quest: String) -> String{
    let mut s=String::new();
    print!("{}",quest);
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    return s;

}

fn join(fnn:String, f1:Vec<String>, f2:Vec<String>,diff:Vec<i64>){
    // this is the join command that joins the 2 vectors together into the new file
    let mut f1: Vec<String> = f1;
    let mut f2: Vec<String> = f2;
    let ogf2s = f2.len();
    //print!("gogf2s{}",ogf2s);
    let mut new: Vec<String> = vec!("".to_string());
    //makes sure they are the correct lenght 
    for x in 1..f2.len(){
        if f1.len() >= f2.len(){
            f2.push("\n".to_string());
        }else{
            break;
        }
    }
    //makes sure they are the correct lenght 
    for x in 1..f2.len(){
        if f2.len() >= f1.len() {
            f1.push("\n".to_string());
        }else{
            break;
        }
    }
    //makes sure they are the correct lenght 

    for x in 1..f2.len(){
        if f2.len() >= new.len() {
            new.push("".to_string());
        }else{
            break;
        }
    }
    // string that later contains the newly compiled file
    let mut newfile:String = "".to_string();
    
    /*for x in 1..diff.len(){
        let xf: &str = &f2[diff[x as usize] as usize];
        let xe = xf.to_owned()+"\n";

        f1[diff[x as usize] as usize] = xe.to_string();

    }*/
    // compiles together the vectors 
    for x in 1..f2.len(){
        if f1[x] == f2[x]{
            
            let xxx1 = &f2[x];
            let xs = xxx1.to_string()+"\n";
            new[x].push_str(&xs);
            
        }else{
            
            let xxx1 = &f2[x];
            let xs = xxx1.to_string()+"\n";
            new[x].push_str(&xs);
        }
    }
    // after this the need to be "post compiled" to remove unnessary lines created by the combiner
    let mut xc: i64 = 0;
    for x in new{
        if xc <= ogf2s as i64 -1 {
            let xe = &x;
            newfile.push_str(&xe);

        }else{
            //if x == "\n".to_string(){
                //print!("xc ogf2s {}  {}",xc,ogf2s);

            /*}else{
                let xe = x;
                newfile.push_str(&xe)
            }*/
        }
        
        xc +=1;
    }
    
    /*for x in f1{
        print!("x{}\n",x);
        newfile.push_str(&x);
    }*/
    // writes to the file
    filehandler::writeFile(fnn, newfile.as_bytes());


}
fn checkdif(f1:Vec<String>, f2:Vec<String>) -> Vec<i64>{
    // declaring 2 vectors that can be used for edeting the files or just using them
    let mut f1: Vec<String> = f1;
    let mut f2: Vec<String> = f2;

    //these 2 for functions makes sure that both files are equally big as to not get any out of vector errors
    for x in 1..f2.len(){
        if f1.len() >= f2.len(){
            f2.push("\n".to_string());
        }else{
            break;
        }
    }
    
    for x in 1..f2.len(){
        if f2.len() >= f1.len() {
            f1.push("\n".to_string());
        }else{
            break;
        }
    }
   // creates a vector that tells what rows have been changed 
    let mut lines: Vec<i64> = vec!(0);
    for x in 1..f2.len(){
        if f1[x] == f2[x]{

        }else{
            print!("Line :{} is diffrent\n",x);
            lines.push(x as i64);
        }
    }
    // tells the end user how many rows have been changed 
    if lines.len() -1  == 0{
        print!("No Differences found\n");
    }else{
        if lines.len() -1 == 1{
            print!("{} Difference found\n",lines.len());

        }else{
        print!("{} Differences found\n",lines.len());
            
        }
        

    }
    return lines;


}

fn main() {
    /*
    
    this is the main function
    
    
    */
    // asks for the first file then the second file
    let file1 = getin("File 1: ".to_string());
    let file2 = getin("File 2: ".to_string());
   
    // these are the vectors that contain the files after they are handeld 
    let mut fx: Vec<String> = vec!("".to_string());
    let mut fx2: Vec<String> = vec!("".to_string());
    
    //and the files to the fx and fx2 vectors
    fx = filehandler::lines_from_file(file1.to_string());
    fx2 = filehandler::lines_from_file(file2.to_string());
    // this is so that the fx and fx2 varaibles can be used elsewere
    let xx3 = &fx;
    let xx4 = &fx2;
    let ll = checkdif(xx3.to_owned(), xx4.to_owned());
    // asks the question if the user still wants to mash the 2 files 
    let yesno = getin("do you still want to mash? (y/n)".to_string());
    
    // gets the message then calls the function to run it
    if yesno == "y".to_string(){
        let xx1 = &fx;
        let xx2 = &fx2;
        
        join(file1.to_string(),xx1.to_owned(), xx2.to_owned(), ll);


    }
    
    
}



