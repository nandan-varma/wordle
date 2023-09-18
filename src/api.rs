use chrono::{Datelike, Utc};
use rand::{thread_rng, Rng};
use reqwest::Client;

pub async fn fetch_word_from_api() -> Result<String, reqwest::Error> {
    let date = generate_random_date_in_2022_to_present();
    let api_url = format!("https://www.nytimes.com/svc/wordle/v2/{}.json", date);
    let response = Client::new()
        .get(&api_url)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    let solution = response["solution"].as_str().unwrap_or_default().to_owned();
    Ok(solution)
}

fn generate_random_date_in_2022_to_present() -> String {
    let mut rng = thread_rng();
    let current_date = Utc::now();
    let random_year = rng.gen_range(2022..=current_date.year());
    let random_month = rng.gen_range(1..=12);
    let random_day = rng.gen_range(1..=31); // We assume maximum days in a month is 31 for simplicity
    format!("{:04}-{:02}-{:02}", random_year, random_month, random_day)
}
