use chrono::Local;

// either reads api response from cache or runs `get_api_response` to demand it from the AniList server.
pub fn read_api_response(cache_path: String, user_data_folder: String, config_path: String, username: String, api_key: String) -> serde_json::Value {
  let api_response: serde_json::Value;
  
  if std::path::Path::new(&cache_path).exists()
  {
    let file_size = std::fs::metadata(cache_path.clone()).unwrap().len();
    if file_size == 0 // check if file is empty
    {
      api_response = get_api_response(user_data_folder, config_path, cache_path, username, api_key);
    }
    else if crate::cache::read_cache(cache_path.clone()) == "outdated" // clear cache if it was not created today
    {
      api_response = get_api_response(user_data_folder, config_path, cache_path, username, api_key);
    }
    else 
    {
      api_response = serde_json::from_str(&*crate::cache::read_cache(cache_path.clone())).expect("Couldn't read api response from cache.");
    }
  }
  else
  {
    api_response = get_api_response(user_data_folder, config_path, cache_path, username, api_key);
  }
  return api_response;
}

// demands api response from AniList server
fn get_api_response(user_data_folder: String, config_path: String, cache_path: String, username: String, api_key: String) -> serde_json::Value
{
  // ask user for api_key
  let data: Vec<String> = crate::get_user_information(user_data_folder.clone(), config_path, username, api_key);
  // send request and save result
  let api_response = crate::api::request(data[0].clone(), data[1].clone());
  // check if the api response contains errors.
  if api_response["data"]["User"].to_string() == "null"
  {
    println!("The data for this user is not available publicly. Have your set your anilist account to private?");
    println!("Is {} the correct spelling of your username?", data[0]);
    print!("User data will not be saved.");
    // User data should not be saved.
    std::fs::remove_dir_all(user_data_folder).expect("Anijouhou config directory can not be deleted.");
    std::process::exit(404);
  }
  // save result locally
  crate::cache::write_cache(serde_json::to_string_pretty(&api_response).unwrap(), cache_path);
  return api_response;
}

// cache profile picture
pub fn cache_profile_picture (profile_picture_path: String, avatar_url: String) {
  if !std::path::Path::new(&profile_picture_path).exists() 
  {
    crate::api::cache_profile_picture(avatar_url, profile_picture_path).expect("Could not cache profile picture.");
  }
}


// -- helper functions --

/// # Description
/// `write_cache` writes the current date, as well as a specified string to a specified location.
/// # Parameters
/// `result` is the output you want to write to cache. (e.g. a json file). <br>
/// `cache_path` is the path you want to write your cache to.
pub fn write_cache(result: String, cache_path: String)
{
  // write current data to cache
  let today = Local::now().date_naive().to_string();
  // write result to file
  std::fs::write(cache_path, today + "\n" + &result).expect("Could not write api response to cache.");
}


/// # Description
/// `read_cache` checks if a file cached with `write_cache` was written today. <br>
/// If it was it returns the previously saved string, (e.g. a json file)
/// if it wasn't it returns the string "outdated".
/// # Parameters
/// `cache_path` is the path you want to write your cache to.
pub fn read_cache(cache_path: String) -> String
{
  let mut cache_content: String = Default::default();
  // save data in variable
  let cache = read_lines(&cache_path);
  // check the first line
  if cache[0] == Local::now().date_naive().to_string() 
  {
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

/// A function that reads individual lines. <br>
/// Taken from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html. <br>
/// Note that this function is not performance efficient.
pub fn read_lines(filename: &str) -> Vec<String> {
  use std::fs::read_to_string;
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

pub struct AniListApiResponse {
  pub minutes: i64,
  pub hours: i64,
  pub episodes: i64,
  pub username: String,
  pub avatar_url: String,
  pub genre_1: String,
  pub genre_1_hours: i64,
  pub genre_2: String,
  pub genre_2_hours: i64,
  pub genre_3: String,
  pub genre_3_hours: i64
}