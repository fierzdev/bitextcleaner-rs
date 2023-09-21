use std::fs;
use std::iter::zip;
use crate::model::BiText;

pub fn align_moses(file_src: &str, file_trg: &str, src_lang: Option<String>, trg_lang: Option<String>) -> Vec<BiText>{
    zip(

        fs::read_to_string(file_src).expect("Src file invalid").split("\n"),
        fs::read_to_string(file_trg).expect("Trg file invalid").split("\n")
    ).into_iter().map(|(x, y)| BiText::new(x.parse().unwrap(), src_lang.clone(), Some(y.parse().unwrap()), trg_lang.clone())).collect()
}