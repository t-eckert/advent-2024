use bytes::Bytes;
use reqwest::blocking::Client;

pub fn pull(session: &str, day: u8) -> Result<Bytes, anyhow::Error> {
    let url = format!("https://adventofcode.com/2024/day/{}/input", day);

    let client = Client::new();

    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session))
        .send()?;

    if response.status().is_success() {
        Ok(response.bytes()?)
    } else {
        Err(anyhow::anyhow!("Failed to pull puzzle data"))
    }
}
