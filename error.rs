//errors
use std::fs::File;
fn main(){
    // let output =  File::open("hellos.txt");
    // let output = match output {
    //     Ok(x)=>x,
    //     Err(error)=> panic!("error {:?}",error)
    // };

    let output  = File::open("jilio.txt").expect("hey opeople file is error"); //good error handling
    let output1  = File::open("jilio.txt")?;
    Ok(());
    // println!("output-- {?}",output1);
}