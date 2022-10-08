fn main() {
    println!("Hello, world!");
    let mut x = 4;
    // type will be implicit :type will be determine by compiler
    // the immutable
    x = x +5;

    // you can create or override variable
    let a =5;
    println!("x is : {}",a);
    let a = 45;
    println!("x is : {}",a);
    println!("x is : {}",x);

    // you can scope or shadow

    {
        x = 6;
        println!("x is : {}",x);
    }


    const THE_VALUE : u32 = 60;
    println!("x is : {}",THE_VALUE);
}
