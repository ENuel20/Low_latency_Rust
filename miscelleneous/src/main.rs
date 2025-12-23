use std::fmt::Display;

trait Processor {
    fn process<T:Display>(&self, value: T) -> String;
}

struct MyProcessor;

impl Processor for MyProcessor{
    fn process<T:Display>(&self, value:T) -> String {
        format!("{}",value)
    }
}

fn get_processor<P>(processor: P) -> impl for<'a> Fn(&str) -> String
where
    P : Processor
{
    move |value| processor.process(value) 
}


fn main() {
    let processor = MyProcessor;
    let process_closure = get_processor(processor);

    let item_1 = 10.to_string();
    let item_2 = 20.to_string();
    
    println!("{}",process_closure(&item_1));

    println!("{}",process_closure(&item_2));
}
