use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
/// Attempts to represent the gender identity of a user
pub struct Gender {
    /// A list of all genders the user has specified. An empty vector *is* a valid value, and could
    /// represent either an user who does not experience gender at all, or a user who preferred not
    /// to answer, or both
    genders: Vec<Gender>,
    /// Indicates whether the user is transgender, if they chose to specify. `Some(true)` means
    /// yes transgender, `Some(false)` means not transgender, and `None` could either mean "not
    /// applicable" or "prefer not to answer" or both
    is_trans: Option<bool>,
}

enum GenderOption {
    Female,
    Male,
    NonBinary,
    Queer,
    Agender,
    Other,
    None,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Serialize)]
pub enum Nationality {
    #[serde(rename = "AU")]
    Australian,
    #[serde(rename = "BR")]
    Brazilian,
    #[serde(rename = "CA")]
    Canadian,
    #[serde(rename = "CH")]
    Swiss,
    #[serde(rename = "DE")]
    German,
    #[serde(rename = "DK")]
    Danish,
    #[serde(rename = "ES")]
    Spanish,
    #[serde(rename = "FI")]
    Finnish,
    #[serde(rename = "FR")]
    French,
    #[serde(rename = "GB")]
    British,
    #[serde(rename = "IE")]
    Irish,
    #[serde(rename = "IN")]
    Indian,
    #[serde(rename = "IR")]
    Iranian,
    #[serde(rename = "MX")]
    Mexican,
    #[serde(rename = "NL")]
    Dutch,
    #[serde(rename = "NO")]
    Norwegian,
    #[serde(rename = "NZ")]
    NewZealander,
    #[serde(rename = "RS")]
    Serbian,
    #[serde(rename = "TR")]
    Turkish,
    #[serde(rename = "UA")]
    Ukrainian,
    #[serde(rename = "US")]
    American,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Name {
    pub title: String,
    pub first: String,
    pub last: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Street {
    pub number: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Coordinates {
    pub latitude: String,
    pub longitude: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Timezone {
    pub offset: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Location {
    pub street: Street,
    pub city: String,
    pub state: String,
    pub country: String,
    #[serde(deserialize_with = "deserialize_as_string")]
    pub postcode: String,
    pub coordinates: Coordinates,
    pub timezone: Timezone,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Login {
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RandomDate {
    pub date: DateTime<Utc>,
    pub age: i32,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Identity {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Picture {
    pub large: String,
    pub medium: String,
    pub thumbnail: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RandomUser {
    pub gender: Gender,
    pub name: Name,
    pub location: Location,
    pub email: String,
    pub login: Login,
    #[serde(rename = "dob")]
    pub birthday: RandomDate,
    pub registered: RandomDate,
    pub phone: String,
    pub cell: String,
    pub id: Identity,
    pub picture: Picture,
    #[serde(rename = "nat")]
    pub nationality: Nationality,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RandomUserInfo {
    pub seed: String,
    pub results: i32,
    pub page: i32,
    pub version: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RandomUserResult {
    pub results: Vec<RandomUser>,
    pub info: RandomUserInfo,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum RandomUserResponse {
    #[serde(rename = "error")]
    Error(String),
    #[serde(untagged)]
    Result(RandomUserResult),
}

fn deserialize_as_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let json: serde_json::value::Value = serde_json::value::Value::deserialize(deserializer)?;
    match json {
        serde_json::Value::String(s) => Ok(s),
        _ => Ok(json.to_string()),
    }
}
