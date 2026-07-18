use trpl::{Either, Html};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (url1, url2) = if args.len() < 3 {
        println!("Usage: {} <url1> <url2>", args[0]);
        println!("No URLs provided. Using default URLs...\n");
        ("https://www.rust-lang.org", "https://www.wikipedia.org")
    } else {
        (args[1].as_str(), args[2].as_str())
    };

    trpl::block_on(async {
        println!("Racing two URLs concurrently:");
        println!("URL 1: {}", url1);
        println!("URL 2: {}\n", url2);

        let title_fut_1 = page_title(url1);
        let title_fut_2 = page_title(url2);

        let (url, maybe_title) = match trpl::select(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("\n{} returned first!", url);
        match maybe_title {
            Some(title) => println!("Its page title was: '{}'", title),
            None => println!("It had no title."),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}
