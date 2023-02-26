fn main(){
    let rect = (2,10);
    let mut result = getRectangleShape(rect);
    println!("result {}",result);
 }
 
 fn getRectangleShape(dimension:(u32,u32))->u32{    
     return dimension.0 * dimension.1;
 }