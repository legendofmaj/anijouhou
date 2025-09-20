// this is the same thing as `using namespace` in C++
use serde_json::json;
use reqwest::{Client, Error};
use text_io::read;


fn main()
{
  // get home path
  let user_data_folder = std::env::var("HOME").unwrap() + "/.config/anijouhou/";
  let user_data_path = user_data_folder.clone() + "config.conf";

  // "global" variable
  let mut verbosity: String = "all".to_string();

  // Check for command line arguments
  let args: Vec<String> = std::env::args().collect();
  for i in 0..args.len()
  {
    if args[i] == "--delete" || args[i] == "-d"
    {
      println!("Deleting user data");
      std::fs::remove_dir_all(&user_data_folder).expect("Directory should be deleted.");
    }
    else if args[i] == "--hours" || args[i] == "-h"
    {
      verbosity = "hours".to_string();
    }
    else if args[i] == "--episodes" || args[i] == "-e"
    {
      verbosity = "episodes".to_string();
    }
    else if args[i] == "--minutes" || args[i] == "-m"
    {
      verbosity = "minutes".to_string();
    }
  }

  // create folder if it doesn't exists
  if std::path::Path::new(&user_data_folder).exists() == false
  {
    std::fs::create_dir(&user_data_folder).expect("Folder should be created");
  }

  // define variables
  let username: String;
  let access_token: String;

  // Check for exisiting user-data
  if std::path::Path::new(&user_data_path).exists() == true 
  {
    //println!("User data exists.");
    // Read user data
    let user_data = read_lines(&user_data_path);
    username = user_data[0].clone();
    access_token = user_data[1].clone();
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
    std::fs::write(&user_data_path, final_output).expect("Should write to config file.");
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
  if verbosity == "all"
  {
    println!("{} watched {} episodes making for a total of {} hours ({} minutes).", username, episodes, hours, minutes);
  }
  else if verbosity == "hours"
  {
    println!("{} hours", hours);
  }
  else if verbosity == "episodes"
  {
    println!("{} episodes", episodes);
  }
  else if verbosity == "minutes"
  {
    println!("{} minutes", minutes);
  }
  
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
    let resp: Result<String, Error>;
    if access_token == "none" 
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

fn read_lines(filename: &str) -> Vec<String> {
  //Taken from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
  use std::fs::read_to_string;
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}