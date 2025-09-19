// this is the same thing as `using namespace` in C++
use serde_json::json;
use reqwest::Client;
use text_io::read;

const USER_DATA_PATH: &str = ".config/config.conf";

fn main()
{
  // define variables
  let username: String;
  let access_token: String;

  // Check for exisiting user-data
  if std::path::Path::new(USER_DATA_PATH).exists() == true 
  {
    println!("User data exists.");
    // Read user data
    let user_data = read_lines(USER_DATA_PATH);
    username = user_data[0].clone();
    access_token = user_data[1].clone();
    println!("Debug: Username: {}, Access_Token: {}", username, access_token);
  }
  else 
  {
    // Ask the user if they want to log in
    println!("Do you want to log in?[y|n]");
    let answer: char = read!();
    if answer == 'y'
    {
      // If they do open a browser window with the login url
      open::that("https://anilist.co/api/v2/oauth/authorize?client_id=30455&response_type=token").expect("Should open Browser Window.");
      // Let them enter the data
      println!("Please enter your access token");
      access_token = read!();
      //Write the data to a string
      //fs::write(USER_DATA_PATH, &access_token).expect("Should be able to write to `/.config/config.conf`");
    }
    else 
    {
      access_token = "none".to_string();
    }

    println!("Please enter your username.");
    username = read!();

    let final_output: String = username.clone() + "\n" + &access_token;
    // println!("{}", final_output);
    std::fs::write(USER_DATA_PATH, final_output).expect("Should write to config file.");
  }

println!("Data gathered: Username: {}, Access Token: {}", username, access_token);

fn read_lines(filename: &str) -> Vec<String> {
  //Taken from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
  use std::fs::read_to_string;
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

  // send request and 
  let result = request(username, access_token);

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
async fn request(username: String, access_token: String) -> serde_json::Value {
    let client = Client::new();
    // Define query and variables
    let json = json!({"query": QUERY, "variables": {"name": username}});
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