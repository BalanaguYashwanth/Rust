
#[derive(Debug)]
enum Coin{
    Russian,
    Italian,
    Indian(State),
}
#[derive(Debug)]
enum State{
    Andhra,
    Tamil,
    Karnataka,
}


fn get(coin:Coin)-> u8{
    match coin{
        Coin::Russian => 2,
        Coin::Italian => 3,
        Coin::Indian(State) => {
            println!("hello {:?}, ", State);
            return 4
        }
    }
}


fn main(){
    let output = get(Coin::Indian(State::Andhra));
    println!("{}",output);
}