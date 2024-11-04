pub use crate::proverb::Proverb;
use anyhow::{Context, Result};

pub fn proverb_search(word: &str) -> Result<Vec<Proverb>> {
    let body =
        reqwest::blocking::get(format!("https://sozluk.gov.tr/atasozu?ara={}", word))?.text()?;
    let v: serde_json::Value = serde_json::from_str(&body)?;
    let proverbs = v.as_array().with_context(|| "Can not retrieve proverbs")?;

    let pv = proverbs.iter().map(|v| parse_proverb(v)).collect();

    Ok(pv)
}

pub fn parse_proverb(v: &serde_json::Value) -> Proverb {
    Proverb {
        id: v
            .get("soz_id")
            .map(|result| result.as_str())
            .map_or(0, |result| result.unwrap().parse().unwrap()),
        proverb: v
            .get("sozum")
            .map_or(String::new(), |result| result.to_string()),
        meaning: v
            .get("anlami")
            .map_or(String::new(), |result| result.to_string()),
        proverb_type: v
            .get("turu2")
            .map_or(String::new(), |result| result.to_string()),
    }
}
