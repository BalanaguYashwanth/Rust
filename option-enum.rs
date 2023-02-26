fn main(){
    let name = Some(2);
    let output = put_value(name);
    println!("output {:#?}", output);
}

fn put_value(name:Option<i32>)->Option<i32>{
    match name{
        Some(num)=>{
            return Some(num+2)
        },
        _=>None,
    }
}