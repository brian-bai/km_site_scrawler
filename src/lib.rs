use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use url::{ParseError, Url};
//use tempfile::Builder;

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

    let body = res.text().await?;
    let mut set = HashSet::new();

    Document::from(body.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| {
            set.insert(String::from(x));
        });

    Ok(set)
}

pub fn compose_absolute_urls(home_url: &str, urls: Vec<String>) -> HashSet<String> {
    let base_url = match Url::parse(home_url) {
        Ok(url) => url,
        Err(why) => panic!("Cannot pare base url: {why}"),
    };
    println!("Base Url: {base_url}");
    let mut set = HashSet::new();
    for url in &urls {
        match Url::parse(url) {
            Ok(aurl) => println!("{aurl}"),
            Err(why) => match why {
                ParseError::RelativeUrlWithoutBase => match base_url.join(url) {
                    Ok(new_url) => {
                        set.insert(new_url.to_string());
                    }
                    Err(e) => println!("Error {e}"),
                },
                _ => println!("Other error"),
            },
        }
    }
    set
}

pub async fn download(absolute_url: &str, target_dir: &str) -> Result<()> {
    let response = reqwest::get(absolute_url).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);

        let path = std::path::Path::new(target_dir);
        let fname = path.join(fname);
        println!("will be located under: '{:?}'", fname);
        std::fs::File::create(fname)?
    };

    let mut content = std::io::Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut dest)?;
    Ok(())
}

pub fn extract_extension(url: &str) -> Option<String> {
    let re = regex::Regex::new(r"(?x)\w+\.(?P<ext>\w+)$").unwrap();
    re.captures(url)
        .and_then(|cap| cap.name("ext").map(|ext| String::from(ext.as_str())))
}
