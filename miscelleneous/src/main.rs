fn main() {
    let s = String::from("hello");
    take_ownership(&s);
    println!("{:?}",s);
    let x = 8;
    let x = take_copy(x);
    println!("Y = {},{}", x, x );
}



fn take_ownership( u: &str ) -> usize {
    let y = u.len();
    y
}

fn take_copy(mut x: u8) -> u8 {
    let y = 5;
    x = y + x;
    x
}

