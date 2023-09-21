use std::collections::HashSet;
use std::sync::RwLock;
use bloomfilter::Bloom;
use pyo3::pyclass::boolean_struct::{False, True};
use crate::model::BiText;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator};
use rayon::iter::ParallelIterator;

pub trait Deduplicator{
    fn deduplicate(bitext: Vec<BiText>) ->Vec<BiText>;
}

pub struct TargetDeduplicator{
    cache: dyn Iterator<Item=(BiText)>
}

pub struct TargetDeduplicatorParallel{
    cache: dyn Iterator<Item=(BiText)>
}


impl Deduplicator for TargetDeduplicator {
    fn deduplicate(bitext: Vec<BiText>) -> Vec<BiText> {
        let mut hashset: HashSet<String> = HashSet::new();
        let bitext = bitext.
            into_iter().
            filter(|mut x| {
                match hashset.contains(&*x.text) {
                    true => { false }
                    false => {
                        hashset.insert(String::from(x.text.clone()));
                        true
                    }
                }
            }).collect();
        bitext
    }
}

impl Deduplicator for TargetDeduplicatorParallel {
    // Not working, as we have deadlocks. I know why, but are not sure how to best solve it yet.
    // As this isn't really necessary to have yet, I'll ignore it for now.
    fn deduplicate(bitext: Vec<BiText>) -> Vec<BiText> {
        let hashset: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
        println!("{}", "here");

        let bitext = bitext.
            into_par_iter().
            filter(|mut x| {
                let reader = hashset.read();
                println!("{}", "here");
                match reader.unwrap().contains(&*x.text) {
                    true =>{
                        false
                    }
                    false => {
                        hashset.write().unwrap().insert(x.text.clone());
                        true
                    }
                }
            }).collect();
        bitext
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    pub fn test_deduplicator(){
        let bitexts = vec!["unique", "non-unique", "non-unique", "1", "2","non-unique"];
        let bitexts = bitexts.into_iter().map(
            |x| BiText::new(String::from(x), None, Some(String::from(x)), None)
        ).collect();
        let deduplicated = TargetDeduplicator::deduplicate(bitexts);
        assert_eq!(deduplicated.len(), 4);
    }

    #[test]
    pub fn test_deduplicator_parallel(){
        let bitexts = vec!["unique", "non-unique", "non-unique", "1", "2","non-unique"];
        let bitexts = bitexts.into_iter().map(
            |x| BiText::new(String::from(x), None, Some(String::from(x)), None)
        ).collect();
        let deduplicated = TargetDeduplicatorParallel::deduplicate(bitexts);
        assert_eq!(deduplicated.len(), 4);
    }
}



