use crate::model::BiText;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use regex::Regex;

trait Cleaner {
    fn clean(text: BiText) -> Option<BiText>;
}

pub fn whitespace_cleaner(bitext: Vec<BiText>) -> Vec<BiText> {
    let regex = Regex::new(r" +").unwrap();
    regex_cleaner(regex, bitext)
}

pub fn regex_cleaner(regex: Regex, bitext: Vec<BiText>) -> Vec<BiText>{
        let filtered = bitext
        .into_par_iter()
        .map(|mut text| {
            text.text = regex.replace_all(&*text.text, " ").parse().unwrap();
            match text.translation {
                Some(translation) => {
                    let translation: String =
                        regex.replace_all(&*translation, " ").parse().unwrap();
                    text.translation = Option::from(translation)
                }
                _ => {}
            }
            text
        })
        .collect();
    filtered
}

pub fn diacritics_cleaner(bitext: Vec<BiText>) -> Vec<BiText>{
    bitext.into_par_iter().map(
        |mut x| {
            x.text = diacritics::remove_diacritics(&x.text);
            match x.translation {
                Some(translation) => x.translation = Some(diacritics::remove_diacritics(translation.as_str())),
                None => {}
            };
            x
        }
    ).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_whitespace_cleaner() {
        let expected: Vec<BiText> = vec!["123 123 123", "123 123"]
            .into_iter()
            .map(|x| BiText::new(String::from(x), None, None, None))
            .collect();
        let bitexts: Vec<BiText> = vec!["123  123     123", "123 123"]
            .into_iter()
            .map(|x| BiText::new(String::from(x), None, None, None))
            .collect();
        assert_eq!(expected, whitespace_cleaner(bitexts))
    }

    #[test]
    pub fn test_diacrititcs_cleaner(){
        let bitexts: Vec<BiText> = vec!["abcde", "äöüéàõ"]
            .into_iter()
            .map(|x| BiText::new(String::from(x), None, None, None))
            .collect();
        let expected: Vec<BiText> = vec!["abcde", "aoueao"]
            .into_iter()
            .map(|x| BiText::new(String::from(x), None, None, None))
            .collect();
        assert_eq!(diacritics_cleaner(bitexts), expected);
    }
}
