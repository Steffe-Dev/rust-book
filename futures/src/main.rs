use futures::page_title_with_url;
use trpl::Either;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    trpl::run(async {
        let title_fut_1 = page_title_with_url(&args[1]);
        let title_fut_2 = page_title_with_url(&args[2]);

        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("The title was {title}"),
            None => println!("It had no title"),
        }
    })
}
