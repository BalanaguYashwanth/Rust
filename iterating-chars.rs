fn main(){
    let mut name:&str = "yash wanth";
    name.push_str("hello"); 
    let name_bytes  = name.as_bytes();
    for i in name.chars() {
 //    for i in name.graphemes(true){
     println!("{}",i);
    }
 }