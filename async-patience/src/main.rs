use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream,Stream, StreamExt};

fn main() {
    trpl::run(async {
        let messages = get_message().timeout(Duration::from_millis(200));
        let intervals = get_interval().map(|count| format!("interval : {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_millis(10));
        let merge = messages.merge(intervals).take(20);
        let mut stream = pin!(merge);

        while let Some(result) = stream.next().await {
            match result{
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("problem: {reason:?}")
            }
        }
    });
}

fn get_message() -> impl Stream<Item=String> {
    let (tx,rx) = trpl::channel();
    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
        let time_to_sleep = if index % 2 ==  0 {100} else{300};
        trpl::sleep(Duration::from_millis(time_to_sleep)).await;
        if let Err(send_error) = tx.send(format!("message : '{message}'")){
            eprintln!("cannot send '{message}': {send_error}");
            break;
        }
      }
    });

    ReceiverStream::new(rx)
}

fn get_interval() -> impl Stream<Item=u64> {
    let (tx, rx) = trpl::channel();
    trpl::spawn_task(async move {
        let mut count = 0;
        loop{
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            if let Err(send_error) = tx.send(count) {
                eprintln!("cannot send interval '{count}' : {send_error}");
                break;
            }
        }
    });
    
    ReceiverStream::new(rx)
}

