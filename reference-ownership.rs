fn main(){
    
    let s = get_string();

    let p:i32 = put_get_string(3);

    // let z:&str = "yashwant";
    let z:&str = "yashwant";
    let j =  put_reference(&z);
    println!("s:{}, p:{} j:{}",s,p,j);

}

fn put_reference(name:&str)->usize{
    return name.len()
}

fn put_get_string(num:i32)->i32{
    return num+2
}


fn get_string()-> i32{
    return 2;
}