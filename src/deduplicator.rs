use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Rem;
use std::sync::RwLock;

use crate::model::BiText;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

pub trait Deduplicator{
    fn deduplicate(bitext: Vec<BiText>, lowercase: bool) ->Vec<BiText>;
}

pub struct TargetDeduplicator{
    cache: dyn Iterator<Item=BiText>
}

pub struct TargetDeduplicatorParallel{
    cache: dyn Iterator<Item=BiText>
}


pub struct RemoveDuplicates;

impl Deduplicator for RemoveDuplicates {
    fn deduplicate(bitext: Vec<BiText>, lowercase: bool) -> Vec<BiText> {
        let mut hashset: HashSet<String> = HashSet::new();
        let bitext = bitext.into_iter().filter(
            |x| {
                let val = x.translation.as_deref().unwrap_or_default();
                let mut string: String = String::from(val) + &*x.text;
                if lowercase{
                    string = string.to_lowercase(); 
                }
                match hashset.contains(&*string) {
                    true => { false }
                    false => {
                        hashset.insert(string);
                        true
                    }
                }
            }
        ).collect();
        bitext
    }
}

impl Deduplicator for TargetDeduplicator {
    fn deduplicate(bitext: Vec<BiText>, lowercase:bool) -> Vec<BiText> {
        let mut hashset: HashSet<String> = HashSet::new();
        let bitext = bitext.
            into_iter().
            filter(|x| {
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
    fn deduplicate(bitext: Vec<BiText>, lowercase: bool) -> Vec<BiText> {
        let hashset: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
        println!("{}", "here");

        let bitext = bitext.
            into_par_iter().
            filter(|x| {
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
        let deduplicated = TargetDeduplicator::deduplicate(bitexts, false);
        assert_eq!(deduplicated.len(), 4);
    }

    #[test]
    pub fn test_deduplicator_parallel(){
        let bitexts = vec!["unique", "non-unique", "non-unique", "1", "2","non-unique"];
        let bitexts = bitexts.into_iter().map(
            |x| BiText::new(String::from(x), None, Some(String::from(x)), None)
        ).collect();
        let deduplicated = TargetDeduplicatorParallel::deduplicate(bitexts, false);
        assert_eq!(deduplicated.len(), 4);
    }
}



