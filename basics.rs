fn main(){
    //shading
   let mut name =  "yash";
    name = "bash";
    println!("{}",name);

    const SUBS:i32 = 1000_000;
    println!("{}",SUBS);

    let value:i32 = hello(2);
    println!("{}",value);

    let all = (3,3,"sjhbs");
    println!("{:?}",all);

   let (channel,sub,main) = all;
   println!("main {}",main);


    let nums = [2,5];
    println!("{:?}",nums);

    let all_nums = [22,34,32];
    for i in all_nums.iter() {
        println!("{}",i);
    }

    for i in 4..8 {
        println!("{}",i);
    }

    let mut s:String = String::from("hello");
    s=String::from("man");
    println!("{}",s);

    let s1 = String::from("part");
    let s2 = s1.clone();
    println!("{}",s1);

    let mut m1 = 5;
    let _m2 = m1;
    m1=4;
    println!("{}",_m2)

}

fn hello(tool:i32)-> i32{
    println!("{}",tool);
    return tool+2;
}

