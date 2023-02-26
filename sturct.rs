struct User{
    name:String,
    room:i32,
    present:bool,
    health:String
}

fn main(){
    let mut yash = User{
        name:String::from("yash"),
        room:32,
        present:true,
        health:String::from("good")
    };

    yash.name=String::from("bash");
    println!("{}",yash.name);

    let goodName = getNewStructure();
    println!("goodName{}",goodName.name);

    let newUser = User{
        present:true,
        ..goodName
    };
    println!("newUser {} name {}", newUser.present,newUser.name);
}

fn getNewStructure()->User{
    return User{
        name:String::from("gash"),
        room:90,
        present:false,
        health:String::from("avaerage")
    }
    
}