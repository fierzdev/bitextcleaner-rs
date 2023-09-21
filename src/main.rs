use std::time::Instant;


mod cleaner;
mod filter;
mod model;
mod moses;
mod configparser;
mod pipelines;
mod deduplicator;

fn main() {
    //let mut bitext = moses::align_moses("resources/moses.de", "resources/moses.en",Some(String::from("de")), Some(String::from("en")));
    let now = Instant::now();
    let mut bitext = moses::align_moses("/data/de-en.txt/CCAligned.de-en.de", "/data/de-en.txt/CCAligned.de-en.en", Some(String::from("de")), Some(String::from("en")));
    println!("{}", &bitext.len());
    bitext = pipelines::default_pipeline(bitext);
    println!("{}", &bitext.len());
    println!("{}", now.elapsed().as_secs());
}
