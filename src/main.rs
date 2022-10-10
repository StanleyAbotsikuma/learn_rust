fn main() {
    let mut testString = String::new();
    testString = "stanley".to_string();
println!("the returning value is {}",test(testString));
let x = 5;
let y = 5003321;
println!("{} + {} is {}",x , y ,add(x,y));
}

fn test(params:String )-> String
{
return params;
}
fn add( x:u32, y:u32)->u32
{
    return x + y ;
}