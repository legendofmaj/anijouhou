// this is the same thing as `using namespace` in C++
use serde_json::json;
use reqwest::Client;

fn main()
{
  // send request and 
  let result = request();

  // parse json
  let minutes = result["data"]["User"]["statistics"]["anime"]["minutesWatched"].as_i64().unwrap(); //or as_f64 if I wanted a float.
  let episodes = result["data"]["User"]["statistics"]["anime"]["episodesWatched"].as_i64().unwrap(); //or as_f64 if I wanted a float.
  let username = result["data"]["User"]["name"].to_string();
  let username = username.replace('"', ""); //remove " from string

  // perform calculation
  let hours = minutes / 60;

  // print to screen
  println!("{} watched {} episodes making for a total  of {} hours ({} minutes).", username, episodes, hours, minutes);
}

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
async fn request() -> serde_json::Value {
    let client = Client::new();
    // Define query and variables
    let json = json!({"query": QUERY, "variables": {"name": "legendofmajjp"}});
    // Make HTTP post request
    let resp = client.post("https://graphql.anilist.co/")
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .body(json.to_string())
                .send()
                .await
                .unwrap()
                .text()
                .await;
    // Get json
    let result: serde_json::Value = serde_json::from_str(&resp.unwrap()).unwrap();
    
    return result;
}