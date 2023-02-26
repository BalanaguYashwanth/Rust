//heaps
use std::collections::HashMap;

fn main(){
    let mut rad = HashMap::new();
    // rad.insert(String::from("yash"),20);

    let mainName = String::from("rashing corner corner");

    for i in mainName.chars() {
        rad.insert(i,0);
        //  rad.entry(i).or_insert(0);
       
    }

    println!("{:?}",rad);
}