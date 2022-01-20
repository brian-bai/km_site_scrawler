use clap::App;
use clap::Arg;
use km_site_crawler::retrieve_urls;
//use clap::crate_version;
#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app_m = App::new("KM site scrawler")
        .version(crate_version!())
        .author("brian")
        .about("Site Scrawler")
        .arg(
            Arg::new("depth")
                .short('d')
                .long("depth")
                .default_value("1")
                .help("Scrawler depth.")
        )
        .arg(
            Arg::new("home_url")
               .required(true)
               .index(1)
               .help("Home Url of the site.")
        )
        .get_matches();
    
        if let Some(depth) = app_m.value_of("depth") {
            println!("Scrawler depth: {depth}");

            if let Some(home_url) = app_m.value_of("home_url"){
                println!("Home Url: {home_url}");

                match retrieve_urls(home_url).await{
                    Err(why) => println!("Failed to retrieve urls: {why}"),
                    Ok(urls) => {
                            for url in &urls {
                                println!("{url}");
                            }     
                       }
                }

            }
        }
}

