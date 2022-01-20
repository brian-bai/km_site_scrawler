use std::collections::HashSet;
use select::document::Document;
use select::predicate::Name;
use url::{Url, ParseError};

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub async fn retrieve_urls(home_url: &str) -> Result<HashSet<String>> {

//     let res = reqwest::get(home_url)
//     .await?
//     .text()
//     .await?;

//   Document::from(res.as_str())
//     .find(Name("a"))
//     .filter_map(|n| n.attr("href"))
//     .for_each(|x| println!("{}", x));

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

pub fn compose_absolute_urls(home_url: &str, urls: HashSet<String>) -> HashSet<String> {

    let base_url = match Url::parse(home_url) {
        Ok(url) => url,
        Err(why) => panic!("Cannot pare base url: {why}")
    };
    println!("Base Url: {base_url}");
    let mut set = HashSet::new();
    for url in &urls {
        match Url::parse(url) {
            Ok(aurl) => println!("{aurl}"),
            Err(why) => {
                
                match why {
                    ParseError::RelativeUrlWithoutBase => {
                        match base_url.join(url) {
                            Ok(new_url) => {set.insert(new_url.to_string());},
                            Err(e) => println!("Error {e}"),
                        }
                        
                    }
                    _ => println!("Other error"),
                }
            }
        }
    }    
    set 
}

