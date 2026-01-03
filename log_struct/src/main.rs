use std::sync::Mutex;
use std::thread;
use std::sync::Arc;

pub struct LogStruct {
    buffer: Mutex<Vec<String>>,
}

impl LogStruct {

    fn new() -> Self  {
        LogStruct{
            buffer : Mutex::new(Vec::new())
        }
    }
    fn log(&self, msr: &str){
        let mut ptr = self.buffer.lock().unwrap();
        let log = ptr.push(msr.to_string());
        log
    }
}

fn main() {
    let logger = Arc::new(LogStruct::new());
    
    let mut handles = vec![];

    for i in 0..5 {
        let logger_clone = Arc::clone(&logger);
        handles.push(thread::spawn(move ||{
            logger_clone.log(&format!("message recievd by handle {}", i))
        }))
            }
    for handle in handles {
        handle.join().unwrap();
    }

    let finall = logger.buffer.lock().unwrap();

    println!("total logs {}", finall.len());

    for i in finall.iter(){
        println!("{}", i);
    }

}
