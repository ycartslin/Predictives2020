use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Filing {
    pub text: String,
    pub outcome: String,
}

pub fn extract_features(filings: Vec<Filing>) -> Vec<(usize, String)> {
    filings.iter().map(|f| (f.text.len(), f.outcome.clone())).collect()
}
