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
    avatar {
      large
    }
    updatedAt
  }
}
";

/// # Description
/// Function that makes the api request to anilist.
/// # Parameters
/// `username` is the anilist-username of the account you want to access data from. <br>
/// `access_token` is the anilist access token. This is only needed for private accounts. <br>
/// for non-private accounts pass "skip" as the access-token and the function will make a
/// request without attempting to attach an access-token.
#[tokio::main]
pub async fn request(username: String, access_token: String) -> serde_json::Value {
    let client = Client::new();
    // define query and variables
    let json = json!({"query": QUERY, "variables": {"name": username}});
    // make HTTP post request
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

    // get json
    let result: serde_json::Value = serde_json::from_str(&resp.unwrap()).unwrap();
    return result;
}