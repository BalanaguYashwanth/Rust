#[derive(Debug)]
enum Option<T>{
    Some(T),
    None,
}

fn main(){
    let name:Option<&str>= Some("name");
    println!("name{:#?}",name);
}

