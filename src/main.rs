// this is the same thing as `using namespace` in C++
use serde_json::json;
use reqwest::Client;
// use text_io::read;

// fn print() 
// {
//   println!("Type something...");
//   let word: String = read!();
//   println!("You typed: {}", word);
// }

fn main()
{
  // send request and 
  let result = request();

  // parse json
  let minutes = result["data"]["User"]["statistics"]["anime"]["minutesWatched"].as_i64().unwrap(); //or as_f64 if I wanted a float.
  let username = result["data"]["User"]["name"].to_string();
  let username = username.replace('"', "");

  // perform calculations
  let hours = minutes / 60;

  // print to screen
  println!("{} watched {} hours of TV ({} minutes)", username, hours, minutes);
}

const QUERY: &str = "
query ($name: String) { # Define which variables will be used in the query (id)
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

    // parse json
    //println!("{}", result);
    //println!("Minutes watched: {}", result["data"]["User"]["statistics"]["anime"]["minutesWatched"]);
    //let minutes = result["data"]["User"]["statistics"]["anime"]["minutesWatched"].clone();
   // println!("{}", minutes);
    

    //println!("{:#}", result);
    //println!("{}", minutes);
}