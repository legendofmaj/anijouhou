use std::thread;

pub mod api;
pub mod cache;

fn main()
{
  //--os specific file paths--
  let user_data_folder: String;

  if cfg!(target_os = "linux") || cfg!(target_os = "android")
  {
    user_data_folder = std::env::var("HOME").expect("No HOME directory present") + "/.config/anijouhou/";
  }
  else if cfg!(target_os = "windows")
  {
    user_data_folder = std::env::var("APPDATA").expect("No APP_DATA directory present") + r"\anijouhou\";
  }
  else 
  {
    println!("Your operating system is not supported.\n
    Don't worry, if your OS supports a Unix like file structure, simply download the source code and add your OS to the list of supported ones.");
    std::process::exit(1);
  }
  
  let config_path = user_data_folder.clone() + "config.conf";
  let cache_path: String = user_data_folder.clone() + "cache.conf";

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

  // check for command line arguments
  let args: Vec<String> = std::env::args().collect();
  for i in 0..args.len()
  {
    if args[i] == "-d" || args[i] == "--delete"
    {
      println!("Deleting user data");
      std::fs::remove_dir_all(&user_data_folder).expect("anijouhou config directory cannot be deleted.");
      std::process::exit(0);
    }
    else if args[i] == "-c" || args[i] == "--clear-cache"
    {
      println!("Clearing cache");
      std::fs::remove_file(cache_path.clone()).expect("Cache directory cannot be deleted.");
      std::process::exit(0);
    }
    else if args[i] == "-h" || args[i] == "--hours"
    {
      verbosity = Verbosity::Hours
    }
    else if args[i] == "-e" || args[i] == "--episodes"
    {
      verbosity = Verbosity::Episodes;
    }
    else if args[i] == "-m" || args[i] == "--minutes"
    {
      verbosity = Verbosity::Minutes;
    }
    else if args[i] == "-u" || args[i] == "--username" 
    {
      // clear config directory
      if std::path::Path::new(&user_data_folder).exists() 
      {
        std::fs::remove_dir_all(&user_data_folder).expect("anijouhou config directory cannot be deleted.");
      }
      username = args[i+1].clone();
    }
    else if args[i] == "-k" || args[i] == "--api-key"
    {
      if i >= 2 {api_key = args[i+1].clone();}
      else 
      {
        println!("Please always enter a username AND an api key.");
        std::process::exit(1);
      }
    }
    else if args[i].to_string().contains("anijouhou") == false && i <= 2
    {
      println!("{}", args[i]);
      println!("{}", i);
      // clear config directory
      if std::path::Path::new(&user_data_folder).exists() 
      {
        std::fs::remove_dir_all(&user_data_folder).expect("anijouhou config directory cannot be deleted.");
      }

      // assume that an argument without a flag is a public user.
      if args[i].to_string().is_empty() == false 
      {
        username = args[i].clone();
        api_key = "skip".to_string();
      }
    }
  }

  // get api response
  let api_response: serde_json::Value;
  
  if std::path::Path::new(&cache_path).exists() == true
  {
    let file_size = std::fs::metadata(cache_path.clone()).unwrap().len();
    if file_size == 0 // check if file is empty
    {
      api_response = save_user_information(user_data_folder, config_path, cache_path, username, api_key);
    }
    else if cache::read_cache(cache_path.clone()) == "outdated" // clear cache if it was not created today
    {
      api_response = save_user_information(user_data_folder, config_path, cache_path, username, api_key);
    }
    else 
    {
      api_response = serde_json::from_str(&*cache::read_cache(cache_path.clone())).expect("Couldn't read api response from cache.");
    }
  }
  else
  {
    api_response = save_user_information(user_data_folder, config_path, cache_path, username, api_key);
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

fn save_user_information(user_data_folder: String, config_path: String, cache_path: String, username: String, api_key: String) -> serde_json::Value
{
  // ask user for api_key
  let data: Vec<String> = get_api_key(user_data_folder.clone(), config_path, username, api_key);
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
  cache::write_cache(serde_json::to_string_pretty(&api_response).unwrap(), cache_path);
  return api_response;
}

fn get_api_key(user_data_folder: String, config_path: String, mut username: String, mut api_key: String) -> Vec<String>
{
  // create folder if it doesn't exists
  if std::path::Path::new(&user_data_folder).exists() == false
  {
    std::fs::create_dir(&user_data_folder).expect("Folder should be created");
  }

  // check for exisiting user-data
  if std::path::Path::new(&config_path).exists() == true 
  {
    // read user data
    let user_data = cache::read_lines(&config_path);
    username = user_data[0].clone();
    api_key = user_data[1].clone();
  }
  else 
  {
    // ask the user for their username
    if username == "none"
    {
      println!("Please enter your username.");
      username = read();
    }
    // ask the user if they want to log in
    if api_key == "none"
    {
      println!("Do you want to log in?[y|n]");
      println!("If your account is set to private this is required.");
      let answer: String = read();
      if answer == "y" || answer == "Y"
      {
        // if they do open a browser window with the login url
        thread::spawn(|| {
          open::that("https://anilist.co/api/v2/oauth/authorize?client_id=30455&response_type=token").expect("Should open Browser Window.");
        });
        
        // let them enter their data
        println!("Please enter your access token");
        api_key = read();
      }
      else if answer == "n" || answer == "N"
      {
        api_key = "skip".to_string();
      }
      else
      {
        println!("Please answer only with either 'y' or 'n'.");
      }
    }

    let final_output: String = username.clone() + "\n" + &api_key;
    std::fs::write(&config_path, final_output).expect("Should write to config file.");
  }
  let data = vec![username, api_key];
  return data;
}

fn read() -> String
{
  let mut input: String = String::new();
  std::io::stdin()
    .read_line(&mut input)
    .expect("Couldn't read or store user input");
  // clear any unnecessary formatting
  input = input.replace("\n", "");
  return input;
}