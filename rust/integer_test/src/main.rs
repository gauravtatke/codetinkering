fn main(){
    let x:u32 = 43;
    let y = incr(x, 5);
    println!("x = {}, y = {}", x, y);
    let mut num:i32 = 34;
    println!("num before = {}", num);
    incref(&mut num, 3);
    println!("num = {}", num);
    num = num + 54;
    println!("num = {}", num);
}

fn incr(number:u32, increment:u32)->u32{
    return number + increment;
}

fn incref(numref:&mut i32, inc:i32){
    *numref = *numref + inc;
}
