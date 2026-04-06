use regex::Regex;
use reqwest::{
    blocking::Client,
    header::{CONTENT_TYPE, REFERER, USER_AGENT},
};
use serde_json::json;

const AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0";

const ALLANIME_REF: &str = "https://allmanga.to";
const ALLANIME_API: &str = "https://api.allanime.day";
const ALLANIME_BASE: &str = "allanime.day";
const MODE: &str = "sub";

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
        .post(format!("{ALLANIME_API}/api"))
        .header(REFERER, ALLANIME_REF)
        .header(USER_AGENT, AGENT)
        .header(CONTENT_TYPE, "application/json")
        .body(
            json!({
              "variables": variables,
              "query": search_gql
            })
            .to_string(),
        )
        .send();

    let response = response_result.expect("Error in POST Request");
    let body = response.text().expect("Error getting response body");

    let v: serde_json::Value =
        serde_json::from_str(&body).expect("Error serializing response body");

    let mut animes: Vec<(String, String, u64)> = Vec::new();

    if let Some(edges) = v["data"]["shows"]["edges"].as_array() {
        for show in edges {
            let id = show["_id"].as_str().unwrap_or("");
            let name = show["name"].as_str().unwrap_or("");

            let episodes = &show["availableEpisodes"][MODE];

            if let Some(count) = episodes.as_u64() {
                if count > 0 {
                    animes.push((id.to_string(), name.to_string(), count));
                }
            }
        }
    }
    animes
}

pub fn episode_list(show_id: &str) -> Vec<String> {
    let episodes_list_gql =
        r#"query ($showId: String!) { show( _id: $showId ) { _id availableEpisodesDetail }}"#;

    let variables = json!({
      "showId": show_id
    });

    let client = Client::new();
    let response_result = client
        .post(format!("{ALLANIME_API}/api"))
        .header(REFERER, ALLANIME_REF)
        .header(CONTENT_TYPE, "application/json")
        .header(USER_AGENT, AGENT)
        .body(
            json!({
              "variables": variables,
              "query": episodes_list_gql
            })
            .to_string(),
        )
        .send();

    let response = response_result.expect("Error in POST Request");
    let body = response.text().expect("Error getting response body");

    let v: serde_json::Value =
        serde_json::from_str(&body).expect("Error serializing response body");

    if let Some(ep_list) = v["data"]["show"]["availableEpisodesDetail"][MODE].as_array() {
        let mut episodes: Vec<String> = ep_list
            .iter()
            .filter_map(|json| json.as_str().map(|s| s.to_string()))
            .collect();

        episodes.sort_by(|a, b| {
            let a_num: f64 = a.parse().unwrap_or(0.0);
            let b_num: f64 = b.parse().unwrap_or(0.0);
            a_num.partial_cmp(&b_num).unwrap()
        });

        return episodes;
    }
    Vec::new()
}

pub fn get_episode_urls(show_id: &str, ep_no: &str) -> Vec<String> {
    let episode_embed_gql = r#"query ($showId: String!, $translationType: VaildTranslationTypeEnumType!, $episodeString: String!) { episode( showId: $showId translationType: $translationType episodeString: $episodeString ) { episodeString sourceUrls }}"#;

    let variables = json!({
      "showId": show_id,
      "translationType": MODE,
      "episodeString": ep_no
    });

    let client = Client::new();
    let response = client
        .post(format!("{ALLANIME_API}/api"))
        .header(REFERER, ALLANIME_REF)
        .header(USER_AGENT, AGENT)
        .header(CONTENT_TYPE, "application/json")
        .body(
            json!({
              "variables": variables,
              "query": episode_embed_gql
            })
            .to_string(),
        )
        .send()
        .expect("Error in POST Request");

    let body = response.text().expect("Error getting response body");

    let v: serde_json::Value =
        serde_json::from_str(&body).expect("Error serializing response body");

    let mut links: Vec<String> = Vec::new();
    let sources = v["data"]["episode"]["sourceUrls"].as_array().unwrap();
    for source in sources {
        match source["sourceName"].as_str().unwrap() {
            "Yt-mp4" => links.push(source["sourceUrl"].to_string()),
            "S-mp4" => links.push(source["sourceUrl"].to_string()),
            "Luf-Mp4" => links.push(source["sourceUrl"].to_string()),
            "Default" => links.push(source["sourceUrl"].to_string()),
            _ => links.push(String::new()),
        }
    }
    let mut decoded = links
        .iter()
        .map(|f| decode_provider_id(f.as_str()))
        .collect::<Vec<String>>();
    decoded.retain(|f| !f.is_empty());
    decoded
}

pub fn get_episode_streams(show_id: &str, ep_no: &str) -> Vec<(String, String)> {
    let decoded_urls = get_episode_urls(show_id, ep_no);
    let client = Client::new();
    let mut all_streams: Vec<(String, String)> = Vec::new();

    for url in decoded_urls {
        let api_url = if url.starts_with("http") {
            continue;
        } else {
            format!("https://{ALLANIME_BASE}{url}")
        };

        let response = client
            .get(&api_url)
            .header(USER_AGENT, AGENT)
            .header(REFERER, ALLANIME_REF)
            .send();

        if let Ok(res) = response {
            let body = res.text().unwrap_or_default();
            let json: serde_json::Value = serde_json::from_str(&body).unwrap_or(json!({}));

            if let Some(links) = json["links"].as_array() {
                for link in links {
                    let src = link["link"].as_str().unwrap_or("");
                    let res_str = link["resolutionStr"].as_str().unwrap_or("Unknown");

                    // Handle Wixmp (repackager) logic
                    if src.contains("repackager.wixmp.com") {
                        let base_link = src
                            .replace("repackager.wixmp.com/", "")
                            .split(".urlset")
                            .next()
                            .unwrap_or("")
                            .to_string();
                        // Simplified: capturing the quality from the resolutionStr
                        all_streams.push((res_str.to_string(), base_link));
                    } else {
                        all_streams.push((res_str.to_string(), src.to_string()));
                    }
                }
            }

            // Handle HLS/m3u8 case from the response
            if let Some(hls_url) = json["hls"]["url"].as_str() {
                all_streams.push(("HLS".to_string(), hls_url.to_string()));
            }
        }
    }
    all_streams
}

pub fn video_url_with_quality(quality: &str, stream: &str) -> String {
    let regex =
        Regex::new(r"(?mu)(?<base>https://[\w\.]+/\w+/\w+/)[,\w]+(?<extension>.*)").unwrap();
    let substitution = format!("${{base}}{quality}$extension");

    regex
        .replace_all(stream, substitution.as_str())
        .into_owned()
}

fn decode_provider_id(input: &str) -> String {
    let cleaned = input.replace("--", "").replace('"', "");

    let translated: String = cleaned
        .as_bytes()
        .chunks(2)
        .map(|chunk| {
            let segment = std::str::from_utf8(chunk).unwrap_or("");
            match segment.trim() {
                "79" => "A",
                "7a" => "B",
                "7b" => "C",
                "7c" => "D",
                "7d" => "E",
                "7e" => "F",
                "7f" => "G",
                "70" => "H",
                "71" => "I",
                "72" => "J",
                "73" => "K",
                "74" => "L",
                "75" => "M",
                "76" => "N",
                "77" => "O",
                "68" => "P",
                "69" => "Q",
                "6a" => "R",
                "6b" => "S",
                "6c" => "T",
                "6d" => "U",
                "6e" => "V",
                "6f" => "W",
                "60" => "X",
                "61" => "Y",
                "62" => "Z",
                "59" => "a",
                "5a" => "b",
                "5b" => "c",
                "5c" => "d",
                "5d" => "e",
                "5e" => "f",
                "5f" => "g",
                "50" => "h",
                "51" => "i",
                "52" => "j",
                "53" => "k",
                "54" => "l",
                "55" => "m",
                "56" => "n",
                "57" => "o",
                "48" => "p",
                "49" => "q",
                "4a" => "r",
                "4b" => "s",
                "4c" => "t",
                "4d" => "u",
                "4e" => "v",
                "4f" => "w",
                "40" => "x",
                "41" => "y",
                "42" => "z",
                "08" => "0",
                "09" => "1",
                "0a" => "2",
                "0b" => "3",
                "0c" => "4",
                "0d" => "5",
                "0e" => "6",
                "0f" => "7",
                "00" => "8",
                "01" => "9",
                "15" => "-",
                "16" => ".",
                "67" => "_",
                "46" => "~",
                "02" => ":",
                "17" => "/",
                "07" => "?",
                "1b" => "#",
                "63" => "[",
                "65" => "]",
                "78" => "@",
                "19" => "!",
                "1c" => "$",
                "1e" => "&",
                "10" => "(",
                "11" => ")",
                "12" => "*",
                "13" => "+",
                "14" => ",",
                "03" => ";",
                "05" => "=",
                "1d" => "%",
                _ => segment, // Keep as is if no match
            }
        })
        .collect();

    // 3. The final sed: s/\/clock/\/clock\.json/
    translated.replace("/clock", "/clock.json")
}
