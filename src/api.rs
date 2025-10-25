use serde_json::json;
use reqwest::{Client, Error};

const QUERY: &str = "
query ($name: String) {
  User(name: $name)
  {
    name
    statistics {
      anime {
        episodesWatched
        minutesWatched
      }
    }
  }
}
";

#[tokio::main]
pub async fn request(username: String, access_token: String) -> serde_json::Value {
    let client = Client::new();
    // Define query and variables
    let json = json!({"query": QUERY, "variables": {"name": username}});
    // Make HTTP post request
    let resp: Result<String, Error>;
    if access_token == "skip"
    {
      resp = client.post("https://graphql.anilist.co/")
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .body(json.to_string())
                .send()
                .await
                .unwrap()
                .text()
                .await;
    }
    else 
    {
      resp = client.post("https://graphql.anilist.co/")
                      .header("Authorization", String::from("Bearer ") + &access_token)
                      .header("Content-Type", "application/json")
                      .header("Accept", "application/json")
                      .body(json.to_string())
                      .send()
                      .await
                      .unwrap()
                      .text()
                      .await;
    }

    // Get json
    let result: serde_json::Value = serde_json::from_str(&resp.unwrap()).unwrap();
    return result;
}