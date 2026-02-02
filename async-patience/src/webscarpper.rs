use std::future::Future;
use trpl::{Either, Html};

fn main() {
    let args : Vec<String> = std::env::args().collect();
    trpl::run(async{
        let title_fut1 = page_title(&args[1]);
        let title_fut2 = page_title(&args[2]);
        
        let (url, maybe_title)=
            match trpl::race(title_fut1, title_fut2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };
        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("The title of the {url} is {title}"),
            None => println!("Te {url} as no title")
        }
    })
}


fn page_title(url : &str) -> impl Future<Output = (&str, Option<String>)> {
    async move {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text).select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}}
