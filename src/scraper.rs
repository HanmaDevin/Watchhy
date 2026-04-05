use reqwest::{
    blocking::Client,
    header::{REFERER, USER_AGENT},
};
use serde_json::json;

const AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0";

const ALLANIME_REF: &str = "https://allmanga.to";
const ALLANIME_BASE: &str = "allanime.day";
const ALLANIME_API: &str = "https://api.allanime.day";
const MODE: &str = "sub";

pub fn get_links() {
    let client = Client::new();
    let url = format!("https://");
    // let response_result = client.get(url)
}

fn get_provider_id(provider_name: &str) -> &str {
    todo!()
}

pub fn search_anime(anime_name: &str) -> Vec<(String, String, u64)> {
    let search_gql = r#"query( $search: SearchInput $limit: Int $page: Int $translationType: VaildTranslationTypeEnumType $countryOrigin: VaildCountryOriginEnumType ) { shows( search: $search limit: $limit page: $page translationType: $translationType countryOrigin: $countryOrigin ) { edges { _id name availableEpisodes __typename } }}"#;

    let variables = json!({
      "search": {
        "allowAdult": false,
        "allowUnknown": false,
        "query": anime_name
      },
      "limit": 40,
      "page": 1,
      "translationType": MODE,
      "countryOrigin": "ALL"
    });

    let client = Client::new();
    let response_result = client
        .get(format!("{ALLANIME_API}/api"))
        .header(REFERER, ALLANIME_REF)
        .header(USER_AGENT, AGENT)
        .query(&[
            ("variables", variables.to_string()),
            ("query", search_gql.to_string()),
        ])
        .send();

    let response = response_result.expect("Error in GET Request");
    let body = response.text().expect("Error getting response body");

    let v: serde_json::Value =
        serde_json::from_str(&body).expect("Error serializing response body");

    let mut animes: Vec<(String, String, u64)> = Vec::new();

    if let Some(edges) = v["data"]["shows"]["edges"].as_array() {
        for show in edges {
            let id = show["_id"].as_str().unwrap_or("");
            let name = show["name"].as_str().unwrap_or("");

            // Replicating the sed logic for specific episode counts based on mode
            let episodes = &show["availableEpisodes"][MODE];

            if let Some(count) = episodes.as_u64() {
                if count > 0 {
                    animes.push((id.to_string(), name.to_string(), count));
                    println!("{}\t{} ({} episodes)", id, name, count);
                }
            }
        }
    }
    animes
}

pub fn episode_list(show_id: &str) {
    let episodes_list_gql =
        r#"query ($showId: String!) { show( _id: $showId ) { _id availableEpisodesDetail }}"#;

    let variables = json!({
      "showId": show_id
    });

    let client = Client::new();
    let response_result = client
        .get(format!("{ALLANIME_API}/api"))
        .header(REFERER, ALLANIME_REF)
        .header(USER_AGENT, AGENT)
        .query(&[
            ("variables", variables.to_string()),
            ("query", episodes_list_gql.to_string()),
        ])
        .send();

    let response = response_result.expect("Error in GET Request");
    let body = response.text().expect("Error getting response body");

    let v: serde_json::Value =
        serde_json::from_str(&body).expect("Error serializing response body");

    // 1. Dig into availableEpisodesDetail
    // 2. Access the mode (sub/dub)
    // 3. Extract the array and sort it
    if let Some(ep_list) = v["data"]["show"]["availableEpisodesDetail"][MODE].as_array() {
        let mut episodes: Vec<String> = ep_list
            .iter()
            .filter_map(|json| json.as_str().map(|s| s.to_string()))
            .collect();

        // Sort naturally (equivalent to sort -n)
        episodes.sort_by(|a, b| {
            let a_num: f64 = a.parse().unwrap_or(0.0);
            let b_num: f64 = b.parse().unwrap_or(0.0);
            a_num.partial_cmp(&b_num).unwrap()
        });

        for item in episodes {
            println!("Episode: {item}")
        }
    }
}

pub fn get_episode_url() {
  
}