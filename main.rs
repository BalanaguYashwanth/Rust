fn main(){
    let yash:&str = "bashw";
    let bash:&str = "yash";
    let  output = lengthOfString(yash,bash);
    println!("output--{}",output);
}

fn lengthOfString<'a>(x:&'a str,y:&'a str)-> &'a str{
    if x.len() < y.len() {
        x
    }else{
        y
    }
}