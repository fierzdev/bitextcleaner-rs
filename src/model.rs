use pyo3::prelude::*;

#[derive(PartialEq, Debug, Clone)]
#[pyclass]
pub struct BiText {
    #[pyo3(get, set)]
    pub text: String,
    #[pyo3(get, set)]
    pub language: Option<String>,
    #[pyo3(get, set)]
    pub translation: Option<String>,
    #[pyo3(get, set)]
    pub translation_language: Option<String>,
}

impl BiText {
    pub fn new(
        text: String,
        language: Option<String>,
        translation: Option<String>,
        translation_language: Option<String>,
    ) -> Self {
        BiText {
            text,
            language,
            translation,
            translation_language,
        }
    }
}

#[pymethods]
impl BiText {
    #[new]
    pub fn py_new(
        text: String,
        language: Option<String>,
        translation: Option<String>,
        translation_language: Option<String>,
    ) -> Self {
        BiText {
            text,
            language,
            translation,
            translation_language,
        }
    }
}
