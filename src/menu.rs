use std::fmt::Display;

use chrono::{Datelike, NaiveDate, Weekday};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Meal {
    value: String,
}

impl Display for Meal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Deserialize)]
pub struct MenuDay {
    pub date: NaiveDate,
    pub meals: Vec<Meal>,
}

impl MenuDay {
    pub fn to_field(self) -> (String, String, bool) {
        (
            date_se(&self.date),
            self.meals
                .into_iter()
                .map(|m| m.to_string())
                .collect::<Vec<_>>()
                .join("; "),
            false,
        )
    }
}

fn date_se(date: &NaiveDate) -> String {
    let weekday = match date.weekday() {
        Weekday::Mon => "Måndag",
        Weekday::Tue => "Tisdag",
        Weekday::Wed => "Onsdag",
        Weekday::Thu => "Torsdag",
        Weekday::Fri => "Fredag",
        Weekday::Sat => "Lördag",
        Weekday::Sun => "Söndag",
    };

    let month = match date.month() {
        1 => "januari",
        2 => "februari",
        3 => "mars",
        4 => "april",
        5 => "maj",
        6 => "juni",
        7 => "juli",
        8 => "augusti",
        9 => "september",
        10 => "oktober",
        11 => "november",
        12 => "december",
        _ => unreachable!(),
    };

    format!("{} {} {}", weekday, date.day(), month)
}

pub async fn week_menu(menu_id: &str) -> Result<Vec<MenuDay>, reqwest::Error> {
    let client = Client::new();
    let url = format!(
        "https://api.skolorna.com/v1/mp/menus/{}/days",
        urlencoding::encode(menu_id)
    );

    let data: Vec<MenuDay> = client.get(url).send().await?.json().await?;

    Ok(data)
}
