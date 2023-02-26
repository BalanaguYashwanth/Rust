#[derive(Debug)]
enum Resource{
    Ip1(String),
    Ip2(String)
}

struct All{
    kind:Resource,
    status:bool,
}
//  impl Message{
//     fn get_Message(){
//         println!("ash")
//     }
//  }
#[derive(Debug)]
enum AllResources{
    Data(String,i32),
    Play(String),
    Status(i32),
}

fn main(){
    let majourResources = AllResources::Data(String::from("yash"),2);
    let human  = Resource::Ip1(String::from("gello"));
    println!("{:?}",majourResources);
}