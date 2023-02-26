pub trait Summary{
    fn summarize(&self) -> String;

   //  fn summarizePaper(&self) -> String{
   //     String::from("go there and readmore...")
   // }
}

struct Newspaper {
   name:String,
   description:String,
}

struct Article{
   title:String,
   label:String,
}

impl Summary for Newspaper{
   fn summarize(&self)->String{
       format!("here it goes {}",&self.name)
   }
}

fn main(){
   let newspaper = Newspaper{
       name:String::from("yash"),
       description: String::from("some context here..."),
   };
   println!("something here,..--{} ",newspaper.summarize());
}
