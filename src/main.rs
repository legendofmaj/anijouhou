use serde_json::json;
use reqwest::{Client, Error};
use text_io::read;
use chrono::Local;

fn main()
{
  // get home path
  let user_data_folder = std::env::var("HOME").unwrap() + "/.config/anijouhou/";
  let user_data_path = user_data_folder.clone() + "config.conf";
  let cache_file: String = std::env::var("HOME").unwrap() + "/.config/anijouhou/" + "cache.conf"; //ToDo: find a better way to store this.

  #[derive(Eq, PartialEq)]
  enum Verbosity 
  {
    All,
    Hours,
    Episodes,
    Minutes,
  }

  let mut verbosity: Verbosity = Verbosity::All;

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
      verbosity = Verbosity::Hours
    }
    else if args[i] == "--episodes" || args[i] == "-e"
    {
      verbosity = Verbosity::Episodes;
    }
    else if args[i] == "--minutes" || args[i] == "-m"
    {
      verbosity = Verbosity::Minutes;
    }
  }

  let mut api_response: serde_json::Value = json!({"error": "0"});

  if std::path::Path::new(&cache_file).exists() == true
  {
    if read_cache(cache_file.clone()) == "none".to_string()
    {
      // go on like before
      // ask user for api_key
      let data: Vec<String> = get_api_key(user_data_folder, user_data_path);
      // send request and save result
      api_response = request(data[0].clone(), data[1].clone());
      // save result locally (and only ask again the next day)
      cache_result(serde_json::to_string_pretty(&api_response).unwrap(), cache_file);
    }
    else 
    {
      api_response = serde_json::from_str(&*read_cache(cache_file.clone())).expect("Couldn't read api response from cache.");
    }
  }

  // parse json
  let minutes = api_response["data"]["User"]["statistics"]["anime"]["minutesWatched"].as_i64().unwrap(); //or as_f64 if I wanted a float.
  let episodes = api_response["data"]["User"]["statistics"]["anime"]["episodesWatched"].as_i64().unwrap(); //or as_f64 if I wanted a float.
  let username = api_response["data"]["User"]["name"].to_string();
  let username = username.replace('"', ""); //remove " from string

  // perform calculation
  let hours = minutes / 60;

  // print to screen
  if verbosity == Verbosity::All
  {
    println!("{} watched {} episodes making for a total of {} hours ({} minutes).", username, episodes, hours, minutes);
  }
  else if verbosity == Verbosity::Hours
  {
    println!("{} hours", hours);
  }
  else if verbosity == Verbosity::Episodes
  {
    println!("{} episodes", episodes);
  }
  else if verbosity == Verbosity::Minutes
  {
    println!("{} minutes", minutes);
  }
  
}

fn get_api_key(user_data_folder: String, user_data_path: String) -> Vec<String>
{
  // create folder if it doesn't exists
  if std::path::Path::new(&user_data_folder).exists() == false
  {
    std::fs::create_dir(&user_data_folder).expect("Folder should be created");
  }

  // declare variables
  let username: String;
  let access_token: String;

  // Check for exisiting user-data
  if std::path::Path::new(&user_data_path).exists() == true 
  {
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
    if answer == 'y' || answer == 'Y'
    {
      // If they do open a browser window with the login url
      open::that("https://anilist.co/api/v2/oauth/authorize?client_id=30455&response_type=token").expect("Should open Browser Window.");
      // Let them enter the data
      println!("Please enter your access token");
      access_token = read!();
    }
    else 
    {
      access_token = "none".to_string();
    }

    println!("Please enter your username.");
    username = read!();

    let final_output: String = username.clone() + "\n" + &access_token;
    std::fs::write(&user_data_path, final_output).expect("Should write to config file.");
  }
  let data = vec![username, access_token]; //ToDo: This "clone" can probably be removed.
  return data;
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

fn read_cache(cache_file: String) -> String
{
  let mut cache_content: String = Default::default();
  // save data in variable
  let cache = read_lines(&cache_file);
  // check the first line
  if cache[0] == Local::now().date_naive().to_string() 
  {
    //println!("File has been created today.");
    for i in 1..cache.len()
    {
      cache_content += &cache[i];
    }
  }
  else 
  {
    cache_content = "none".to_string(); //ToDo: Find a more efficient way of doing this.
  }
  return cache_content;
}


fn cache_result(result: String, cache_file: String)
{
  // check cache
  
  // --write cache--
  // write current data to cache
  let today = Local::now().date_naive().to_string();
  // write result to file
  std::fs::write(cache_file, today + "\n" + &result).expect("Could not write api response to cache.")
}


// helper functions
fn read_lines(filename: &str) -> Vec<String> {
  //Taken from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
  use std::fs::read_to_string;
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}