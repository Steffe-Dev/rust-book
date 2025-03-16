use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title_el| title_el.inner_html())
}

pub async fn page_title_with_url(url: &str) -> (&str, Option<String>) {
    let page_title = page_title(url).await;
    (url, page_title)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_trpl() {
        let result = trpl::run(async {
            let url = &"http://www.rust-lang.org";
            match page_title(url).await {
                Some(title) => title,
                None => String::new(),
            }
        });
        assert_eq!(result.trim(), "Rust Programming Language");
    }

    #[test]
    fn it_works_for_google() {
        let result = trpl::run(async {
            let url = &"https://www.google.com";
            match page_title(url).await {
                Some(title) => title,
                None => String::new(),
            }
        });
        assert_eq!(result.trim(), "Google");
    }
}
