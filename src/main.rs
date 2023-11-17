extern crate clap;

use std::time::Instant;
use clap::{Parser, Subcommand};

mod cleaner;
mod filter;
mod model;
mod moses;
mod configparser;
mod pipelines;
mod deduplicator;

fn main() {
    let app = CliArgs::parse();
    println!("{}", app.src_file);
    let now = Instant::now();
    let mut bitext = moses::align_moses(&*app.src_file, &*app.trg_file, Some(app.src_lang), Some(app.trg_lang));
    println!("{}", &bitext.len());
    bitext = pipelines::default_pipeline(bitext);
    println!("{}", &bitext.len());
    println!("{}", now.elapsed().as_secs());
}

#[derive(Parser, Default, Debug)]
struct CliArgs {
    src_file: String,
    trg_file: String,
    src_lang: String,
    trg_lang: String,
}

