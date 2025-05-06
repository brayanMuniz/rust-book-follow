use trpl::{Either, Html};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_1 = &args[1];
        let title_2 = &args[2];
        let (url, maybe_title) = match trpl::race(page_title(title_1), page_title(title_2)).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Title is {title}"),
            None => println!("There was no title"),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());

    (url, title)
}
