use regex::Regex;

extern crate clap;
use clap::{App, Arg};
use reqwest::blocking;
use unicode_segmentation::{UnicodeSegmentation, UnicodeSentences};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use structopt::{StructOpt};


 fn main() -> Result<(),Box<dyn std::error::Error>> {
    let app = App::new("rr")
        .author("hapash. <boymintyfresh@gmail.com>")
        .bin_name("my_horrible_rust_program")
        .arg(
            Arg::with_name("get")
                .short("g")
                .value_name("urls")
                .required(true)
                .takes_value(true)
                .multiple(true),
        );
    let matches = app.get_matches();
    let regex_of_url = Regex::new("^(http://)|^(https://)")?;
    let urls = matches.values_of("get").unwrap();
    let mut vals: Vec<String> = urls.map(move | str| str.to_string()).collect();
    for vale in & mut vals{
        if regex_of_url.is_match(vale.as_str())==true{
            println!("{}",vale);
            continue;
        }
        else {
            vale.insert_str(0, "http://");
            println!("{}",vale);
        }
    }
    let client = reqwest::blocking::Client::new();

    // let get = client.get("http://gnu.org").send().await?;
    // let data = &get.text().await?;
    // println!("{}", data);
    let data = handle_urls(&client, &vals)?;
    println!("{}",data[0]);

    Ok(())
}
fn handle_urls(
    client: &reqwest::blocking::Client,
    urls: &[String],
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut textcontent = Vec::<String>::new();
    for url in urls {
        let result = client.get(url).send();
        let respone = match result {
            Ok(res) => {
                // println!("cool");
                Ok(res)
            }
            Err(e) => {
                eprintln!("the url is:{}\n{}", url, &e);
                continue;
                Err(e)
            }
        };
        let text = respone?.text()?;
        textcontent.push(text);
        std::hint::spin_loop();
    }
    Ok(textcontent)
}

