use clap::App;
use clap::Arg;
use km_site_crawler::retrieve_urls;
use km_site_crawler::compose_absolute_urls;
use km_site_crawler::download;
use km_site_crawler::extract_extension;

#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

#[tokio::main]
async fn main() {
    let app_m = App::new("KM site scrawler")
        .version(crate_version!())
        .author("brian")
        .about("Site Scrawler")
        .arg(
            Arg::new("explore")
                .short('e')
                .long("explore")
                .takes_value(false)
                .help("View the urls and show the file extensions list.") 
        )
        .arg(
            Arg::new("depth")
                .short('p')
                .long("depth")
                .default_value("1")
                .help("Scrawler depth.")
        )
        .arg(
            Arg::new("ext")
            .short('x')
            .long("ext")
            .default_value("pdf,zip")
            .help("Target file name extension.") 
        )
        .arg(
            Arg::new("dir")
            .short('d')
            .long("dir")
            .default_value(&shellexpand::tilde("~/Downloads").into_owned())
            .help("Target dir.") 
        )
        .arg(
            Arg::new("home_url")
               .required(true)
               .index(1)
               .help("Home Url of the site.")
        )
        .get_matches();

        //TODO: how to make the target_dir available without unwrap and if let statment.
        let target_dir = app_m.value_of("dir").unwrap();
        //The shellexpand has already done by shell command.
        //let target_dir = shellexpand::tilde(cfg_dir).into_owned();
        std::fs::create_dir_all(&target_dir).unwrap();
    
        if let Some(depth) = app_m.value_of("depth") {
            println!("Scrawler depth: {depth}");

            if let Some(home_url) = app_m.value_of("home_url"){
                println!("Home Url: {home_url}");
                match retrieve_urls(home_url).await{
                    Err(why) => println!("Failed to retrieve urls: {why}"),
                    Ok(urls) => { 
                        if app_m.is_present("explore") {
                            let mut exts = std::collections::HashSet::new();
                            for url in &urls {
                                println!("{url}");
                                if let Some(ext) = extract_extension(url) {
                                    exts.insert(ext);
                                }
                            }
                            let v = Vec::from_iter(exts);
                            println!("File Extensions: [{}]", v.join(", "))
                        } else {
                            if let Some(exts) = app_m.value_of("ext") {
                                let exts: Vec<&str> = exts.split_terminator(",").collect();
                                let urls = urls.into_iter().filter(|x|{ 
                                    exts.iter().any(|e| x.ends_with(e))
                                 }).collect::<Vec<String>>();

                                let urls = compose_absolute_urls(home_url, urls);
                                for url in &urls {
                                    if let Err(why) = download(url, target_dir).await {
                                            println!("Download error: {why}");
                                    }
                                } 
                            }
                        }
                    }
                }

            }
        }
}

