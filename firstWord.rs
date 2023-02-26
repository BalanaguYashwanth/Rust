fn main(){
    let name:&str = "balanagu yashwanth";
    let output = sending(name);   
    println!("output {}",output);
}

fn sending(name:&str)->&str{
    let name_to_bytes = name.as_bytes();
    for (i,&n) in name_to_bytes.iter().enumerate() {
        if n == b' ' {
            return &name[..i];
        }       
    }
    &name[..]
}