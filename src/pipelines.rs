use crate::{filter, cleaner, deduplicator};
use crate::deduplicator::{Deduplicator, TargetDeduplicator};
use crate::filter::{Filter, LengthFilterUnit};
use crate::model::BiText;


pub(crate) fn default_pipeline(bitext: Vec<BiText>) -> Vec<BiText> {
    let mut bitext = cleaner::whitespace_cleaner(bitext);
    bitext = TargetDeduplicator::deduplicate(bitext, false);
    bitext = filter::LengthFilter::new(5,40, LengthFilterUnit::Word).filter_text(bitext);
    println!("lengthfilter: {}", bitext.len());
    // bitext = filter::LangIdFilter::new(String::from("German")).filter_text(bitext);
    //bitext = filter::LangIdFilter::new(String::from("English")).filter_text(bitext);
    println!("langid {}", bitext.len());
    bitext = filter::LengthRatioFilter::new(0.8, LengthFilterUnit::Word).filter_text(bitext);
    println!("lengthratiofilter {}", bitext.len());
    bitext = filter::LongWordFilter::new(30).filter_text(bitext);
    println!("longword {}", bitext.len());
    //bitext = filter::SimilarityFilter::new(2, true).filter_text(bitext);
    println!("similarity {}", bitext.len());
    bitext = cleaner::diacritics_cleaner(bitext);
    return bitext
}