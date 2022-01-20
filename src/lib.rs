use std::collections::HashSet;
use select::document::Document;
use select::predicate::Name;

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub async fn retrieve_urls(home_url: &str) -> Result<HashSet<String>> {
    let res = reqwest::get(home_url).await?;
    println!("Status for {}: {}", home_url, res.status());

    let body  = res.text().await?;
    let mut set = HashSet::new();
       
    Document::from(body.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| { set.insert(String::from(x)); } );

    Ok(set)
}
