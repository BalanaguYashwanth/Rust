use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){

    loop{
        let mut name = String::new();
        let secret_number = rand::thread_rng().gen_range(1,101);

        println!(" secret number {}",secret_number);
        
        io::stdin().read_line(&mut name).expect("Error");
    
        let name:u32 = match name.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue,
        };
    
        match name.cmp(&secret_number) {
            Ordering::Less=>println!("{}","less"),
            Ordering::Equal=>println!("{}","equal"),
            Ordering::Greater=>{
                println!("{}","greater");
                break;
        },
        }
    }

}

