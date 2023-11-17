use std::collections::{BTreeMap, HashMap};
use std::fs;
use serde_yaml;
use crate::cleaner::*;
use crate::model::BiText;
use phf::phf_map;
use crate::filter::LengthFilterUnit;

static COUNTRIES: phf::Map<&'static str, fn (Vec<BiText>)->Vec<BiText>> = phf_map! {
    "whitespace_cleaner" => crate::cleaner::whitespace_cleaner,
    "diacritics_cleaner" => crate::cleaner::diacritics_cleaner,
    // "length_filter" => crate::filter::LengthRatioFilter { threshold: 1.2, unit: LengthFilterUnit::Char }:filter_text,
};

pub(crate) fn parse_config(config: &str) -> BTreeMap<String,String>{
    let config = fs::read_to_string(config).expect("Config not found");
    let config: BTreeMap<String, String> = serde_yaml::from_str(&config).expect("Parsing failed");
    config.keys().into_iter().for_each(|x| println!("{}", x));
    config
}