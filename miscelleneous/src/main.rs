fn main() {
    let mut data = vec![1,2,3];

    let mut add = move || {
        &data.push(4);
        println!("{:?}",data);

    };

    add();

    println!("{:?}", data);
}


