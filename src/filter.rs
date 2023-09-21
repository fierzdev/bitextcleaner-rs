use crate::model::BiText;
use levenshtein;
use rayon::prelude::*;
use regex;
use regex::Regex;
use std::cmp::{max, min};
use std::str::FromStr;
use unicode_segmentation::UnicodeSegmentation;

use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};

pub trait Filter {
    fn filter_text(self, texts: Vec<BiText>) -> Vec<BiText>;
}

// LengthFilter
#[derive(PartialEq)]
pub enum LengthFilterUnit {
    Char,
    Word,
}

pub struct LengthFilter {
    min_length: i16,
    max_length: i16,
    unit: LengthFilterUnit,
}

impl LengthFilter {
    pub(crate) fn new(min_length: i16, max_length: i16, unit: LengthFilterUnit) -> LengthFilter {
        LengthFilter {
            min_length,
            max_length,
            unit,
        }
    }
}

impl Filter for LengthFilter {
    fn filter_text(self, texts: Vec<BiText>) -> Vec<BiText> {
        let mut filtered;
        match self.unit {
            LengthFilterUnit::Char => {
                filtered = texts
                    .into_par_iter()
                    .filter(|x| {
                        let length = x.text.graphemes(true).count();
                         length >= self.min_length as usize
                            && length <= self.max_length as usize
                    })
                    .collect::<Vec<BiText>>();
            }
            LengthFilterUnit::Word => {
                filtered = texts
                    .into_par_iter()
                    .filter(|x| {
                        let length = x.text.split(" ").count();
                        length >= self.min_length as usize
                            && length <= self.max_length as usize
                    })
                    .collect::<Vec<BiText>>();
            }
        }

        filtered
    }
}

pub struct LengthRatioFilter {
    threshold: f32,
    unit: LengthFilterUnit,
}

impl LengthRatioFilter {
    pub fn new(threshold: f32, unit: LengthFilterUnit) -> Self {
        Self { threshold, unit }
    }

    fn get_length(&mut self, segment: &String) -> usize {
        if self.unit == LengthFilterUnit::Word {
            return segment.split(" ").count();
        }
        segment.len()
    }

    fn accept(&mut self, bitext: &BiText) -> bool {
        let src_len = self.get_length(&bitext.text);
        let trg_len = match &bitext.translation {
            Some(x) => self.get_length(x),
            None => 0usize,
        };
        return if src_len == 0 && trg_len == 0 {
            false
        } else if src_len == 0 || trg_len == 0 {
            false
        } else {
            min(src_len, trg_len) as f32 / max(src_len, trg_len) as f32 >= self.threshold
        };
    }
}

impl Filter for LengthRatioFilter {
    fn filter_text(mut self, texts: Vec<BiText>) -> Vec<BiText> {
        texts
            .into_iter()
            .filter(|x| self.accept(x))
            .collect::<Vec<BiText>>()
    }
}

// LongWord Filter
pub struct LongWordFilter {
    threshold: i16,
}

impl LongWordFilter {
    pub fn new(threshold: i16) -> Self {
        Self { threshold }
    }
}

impl Filter for LongWordFilter {
    fn filter_text(self, texts: Vec<BiText>) -> Vec<BiText> {
        texts
            .into_par_iter()
            .filter(|x| {
                let mut keep = true;
                for word in x.text.split(" ") {
                    if word.graphemes(true).count() > self.threshold as usize {
                        keep = false;
                        break;
                    }
                }
                keep
            })
            .collect()
    }
}

pub struct RegExpFilter {
    regexp: Regex,
    accept: bool,
}

impl RegExpFilter {
    pub fn new(regexp: &str, accept: bool) -> RegExpFilter {
        RegExpFilter {
            regexp: Regex::new(regexp).unwrap(),
            accept,
        }
    }
}

impl Filter for RegExpFilter {
    fn filter_text(self, texts: Vec<BiText>) -> Vec<BiText> {
        texts
            .into_par_iter()
            .filter(|x| {
                let is_match = self.regexp.is_match(x.text.as_str());
                is_match && self.accept || !is_match && !self.accept
            })
            .collect()
    }
}

pub struct LangIdFilter {
    lang: Language,
    model: LanguageDetector,
}

impl LangIdFilter {
    pub fn new(lang: String) -> Self {
        let mut model = LanguageDetectorBuilder::from_all_languages();
        let model = model.build();

        LangIdFilter {
            lang: Language::from_str(&lang).expect("Invalid Language"),
            model,
        }
    }
}

impl Filter for LangIdFilter {
    fn filter_text(self, texts: Vec<BiText>) -> Vec<BiText> {
        texts
            .into_par_iter()
            .filter(|x| match self.model.detect_language_of(x.text.as_str()) {
                Some(val) => {
                    val.eq(&self.lang)
                }
                None => false,
            })
            .collect()
    }
}

pub struct SimilarityFilter {
    threshold: i16,
    lowercase: bool,
}

impl SimilarityFilter {
    pub fn new(threshold: i16, lowercase: bool) -> SimilarityFilter {
        SimilarityFilter {
            threshold,
            lowercase,
        }
    }
}
impl Filter for SimilarityFilter {
    fn filter_text(self, texts: Vec<BiText>) -> Vec<BiText> {
        texts
            .into_par_iter()
            .filter(|x| match &x.translation {
                Some(translation) => {
                    if !self.lowercase {
                        ((x.text.len() as i16-translation.len() as i16).abs() as i16<=self.threshold)&&(levenshtein::levenshtein(&*x.text, &*translation) > self.threshold as usize)
                    } else {
                        levenshtein::levenshtein(
                            &*x.text.to_lowercase(),
                            &*translation.to_lowercase(),
                        ) > self.threshold as usize
                    }
                }
                None => true,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_filter_short() {
        let test_vectors = vec!["", "abc", "DDD", ""]
            .into_iter()
            .map(|x| BiText::new(String::from(x), None, None, None))
            .collect();
        let cleaner = LengthFilter::new(4, 10, LengthFilterUnit::Char);
        let cleaned = cleaner.filter_text(test_vectors);
        assert_eq!(cleaned.len(), 0);
    }

    #[test]
    fn test_length_filter_long() {
        let test_vectors = vec!["aaaaaaaaaaaaaaaaaaaaa", "abcd", "DDDy", "コココ", "🇸🇹🇸🇹🇸🇹a"]
            .into_iter()
            .map(|x| BiText::new(String::from(x), None, None, None))
            .collect();
        let cleaner = LengthFilter::new(3, 3, LengthFilterUnit::Char);
        let cleaned = cleaner.filter_text(test_vectors);
        assert_eq!(cleaned.len(), 1);
    }

    #[test]
    fn test_length_filter_passing_tests() {
        let test_vectors = vec![
            "ABcDE",
            "abc",
            "DDD",
            "コココ",
            "🇸🇹🇸🇹🇸🇹🇸🇹",
            "as",
            "🇸🇹🇸🇹",
            "🇸🇹2🇸🇹3🇸🇹",
        ]
        .into_iter()
        .map(|x| BiText::new(String::from(x), None, None, None))
        .collect();
        let cleaner = LengthFilter::new(2, 5, LengthFilterUnit::Char);
        let cleaned = cleaner.filter_text(test_vectors);
        assert_eq!(cleaned.len(), 8);
    }

    #[test]
    fn test_length_ratio_filter_short() {
        let test_vectors = vec!["", "abcd", "DDD", "abcde"]
            .into_iter()
            .map(|x| BiText::new(String::from(x), None, Some(String::from("abc")), None))
            .collect();
        let cleaner = LengthRatioFilter::new(0.75, LengthFilterUnit::Char);
        let cleaned = cleaner.filter_text(test_vectors);
        assert_eq!(cleaned.len(), 2);
    }

    #[test]
    fn test_long_word_filter_passing() {
        let test_vectors = vec!["All of this is okay."]
            .into_iter()
            .map(|x| BiText::new(String::from(x), None, None, None))
            .collect();
        let cleaner = LongWordFilter::new(10);
        let cleaned = cleaner.filter_text(test_vectors);
        assert_eq!(cleaned.len(), 1)
    }

    #[test]
    fn test_long_word_filter_negative() {
        let test_vectors = vec![
            "Some of this is baaaaaaaaad.",
            "This is a simplesimple\n test",
        ]
        .into_iter()
        .map(|x| BiText::new(String::from(x), None, None, None))
        .collect();
        let cleaner = LongWordFilter { threshold: 10 };
        let cleaned = cleaner.filter_text(test_vectors);
        assert_eq!(cleaned.len(), 0)
    }

    #[test]
    fn test_regexp_filter_accept() {
        let test_vectors = vec![
            "Some of this is baaaaaaaaad.",
            "This is a simplesimple\n test",
            "Testing",
        ]
        .into_iter()
        .map(|x| BiText::new(String::from(x), None, None, None))
        .collect();
        let cleaner = RegExpFilter::new("a+", true);
        let cleaned = cleaner.filter_text(test_vectors);
        assert_eq!(cleaned.len(), 2)
    }

    #[test]
    fn test_regexp_filter_reject() {
        let test_vectors = vec![
            "Some of this is baaaaaaaaad.",
            "This is a simplesimple\n test",
            "Testing",
        ]
        .into_iter()
        .map(|x| BiText::new(String::from(x), None, None, None))
        .collect();
        let cleaner = RegExpFilter::new("a+", false);
        let cleaned = cleaner.filter_text(test_vectors);
        assert_eq!(cleaned.len(), 1)
    }

    #[test]
    fn test_langid_filter_reject() {
        let test_vectors = vec![
            "Some of this is bad.",
            "This is a simple test",
            "C'est une test",
            "Das ist ein test",
        ]
        .into_iter()
        .map(|x| BiText::new(String::from(x), None, None, None))
        .collect();
        let cleaner = LangIdFilter::new(String::from("English"));
        let cleaned = cleaner.filter_text(test_vectors);
        assert_eq!(cleaned.len(), 2)
    }

    #[test]
    fn levenshtein() {
        let test_vectors: Vec<BiText> = vec![
            "This is close ",
            "This is close 234",
            "This is close 123",
            "Das ist ein test",
            "THIS IS CLOSE 123",
            "This is close コココ",
        ]
        .into_iter()
        .map(|x| {
            BiText::new(
                String::from(x),
                None,
                Some(String::from("This is close 123")),
                None,
            )
        })
        .collect();
        let cleaner = SimilarityFilter::new(2, false);
        let cleaned = cleaner.filter_text(test_vectors.clone());
        assert_eq!(cleaned.len(), 2);
        let cleaner = SimilarityFilter::new(2, false);
        let cleaned = cleaner.filter_text(test_vectors);
        assert_eq!(cleaned.len(), 1);
    }
}
