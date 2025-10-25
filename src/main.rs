use text_io::read;
use chrono::Local;
use std::thread;

pub mod api;

fn main()
{
  // get home path
  let user_data_folder = std::env::var("HOME").unwrap() + "/.config/anijouhou/";
  let user_data_path = user_data_folder.clone() + "config.conf";
  let cache_file: String = std::env::var("HOME").unwrap() + "/.config/anijouhou/" + "cache.conf";

  #[derive(Eq, PartialEq)]
  enum Verbosity 
  {
    All,
    Hours,
    Episodes,
    Minutes,
  }

  let mut verbosity: Verbosity = Verbosity::All;
  let mut username: String  = "none".to_string();
  let mut api_key: String = "none".to_string();

  // Check for command line arguments
  let args: Vec<String> = std::env::args().collect();
  for i in 0..args.len()
  {
    if args[i] == "--delete" || args[i] == "-d"
    {
      println!("Deleting user data");
      std::fs::remove_dir_all(&user_data_folder).expect("anijouhou config directory cannot be deleted.");
      std::process::exit(0);
    }
    else if args[i] == "--clear-cache" || args[i] == "-c"
    {
      println!("Clearing cache");
      std::fs::remove_file(cache_file.clone()).expect("Cache directory cannot be deleted.");
      std::process::exit(0);
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
    else if args[i] == "-u" || args[i] == "--username"
    {
      username = args[i+1].clone();
    }
    else if args[i] == "-k" || args[i] == "--api-key"
    {
      api_key = args[i+1].clone();
    }
  }

  // get api response
  let api_response: serde_json::Value;
  
  if std::path::Path::new(&cache_file).exists() == true
  {
    let file_size = std::fs::metadata(cache_file.clone()).unwrap().len();
    if file_size == 0 // check if file is empty
    {
      api_response = save_user_information(user_data_folder, user_data_path, cache_file, username, api_key);
    }
    else if read_cache(cache_file.clone()) == "outdated" // clear cache if it was not created today
    {
      api_response = save_user_information(user_data_folder, user_data_path, cache_file, username, api_key);
    }
    else 
    {
      api_response = serde_json::from_str(&*read_cache(cache_file.clone())).expect("Couldn't read api response from cache.");
    }
  }
  else 
  {
    api_response = save_user_information(user_data_folder, user_data_path, cache_file, username, api_key);
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

fn save_user_information(user_data_folder: String, user_data_path: String, cache_file: String, username: String, api_key: String) -> serde_json::Value
{
  // ask user for api_key
  let data: Vec<String> = get_api_key(user_data_folder.clone(), user_data_path, username, api_key);
  // send request and save result
  let api_response = api::request(data[0].clone(), data[1].clone());
  // check if the api response contains errors.
  if api_response["data"]["User"].to_string() == "null"
  {
    println!("The data for this user is not available publicly. Have your set your anilist account to private?");
    println!("Is {} the correct spelling of your username?", data[0]);
    print!("User data will not be saved.");
    // User data should not be saved.
    std::fs::remove_dir_all(user_data_folder).expect("anijouhou config directory cannot be deleted.");
    std::process::exit(404);
  }
  // save result locally
  write_cache(serde_json::to_string_pretty(&api_response).unwrap(), cache_file);
  return api_response;
}

fn get_api_key(user_data_folder: String, user_data_path: String, mut username: String, mut api_key: String) -> Vec<String>
{
  // create folder if it doesn't exists
  if std::path::Path::new(&user_data_folder).exists() == false
  {
    std::fs::create_dir(&user_data_folder).expect("Folder should be created");
  }

  // Check for exisiting user-data
  if std::path::Path::new(&user_data_path).exists() == true 
  {
    // Read user data
    let user_data = read_lines(&user_data_path);
    username = user_data[0].clone();
    api_key = user_data[1].clone();
  }
  else 
  {
    // Ask the user for their username
    if username == "none"
    {
      println!("Please enter your username.");
      username = read!();
    }
    // Ask the user if they want to log in
    if api_key == "none"
    {
      println!("Do you want to log in?[y|n]");
      println!("If your account is set to private this is required.");
      let answer: char = read!();
      if answer == 'y' || answer == 'Y'
      {
        // If they do open a browser window with the login url
        thread::spawn(|| {
          open::that("https://anilist.co/api/v2/oauth/authorize?client_id=30455&response_type=token").expect("Should open Browser Window.");
        });
        
        // Let them enter their data
        println!("Please enter your access token");
        api_key = read!();
      }
      else if answer == 'n' || answer == 'N'
      {
        api_key = "skip".to_string();
      }
      else
      {
        println!("Please answer only with either 'y' or 'n'.");
      }
    }

    let final_output: String = username.clone() + "\n" + &api_key;
    std::fs::write(&user_data_path, final_output).expect("Should write to config file.");
  }
  let data = vec![username, api_key];
  return data;
}


fn write_cache(result: String, cache_file: String)
{
  // write current data to cache
  let today = Local::now().date_naive().to_string();
  // write result to file
  std::fs::write(cache_file, today + "\n" + &result).expect("Could not write api response to cache.")
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
    return "outdated".to_string();
  }
  return cache_content;
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