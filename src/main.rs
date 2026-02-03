use std::thread;

pub mod api;
pub mod cache;
pub mod frontend;

fn main()
{
  let user_data_folder: String;

  if cfg!(target_os = "windows")
  {
    user_data_folder = std::env::var("APPDATA").expect("No APP_DATA directory present.") + r"\anijouhou\";
  }
  else
  {
    user_data_folder = std::env::var("HOME").expect("No $HOME directory present.") + "/.config/anijouhou/";
  }
  
  let config_path = user_data_folder.clone() + "credentials.conf";
  let cache_path: String = user_data_folder.clone() + "cache.conf";
  let profile_picture_path: String = user_data_folder.clone() + "profile_picture.png";
  let frontend_config_path: String = user_data_folder.clone() + "config.toml";

  #[derive(Eq, PartialEq)]
  enum Verbosity 
  {
    All,
    Hours,
    Episodes,
    Minutes,
    Text,
  }

  let mut verbosity: Verbosity = Verbosity::All;
  let mut username: String  = "none".to_string();
  let mut api_key: String = "none".to_string();
  let mut close_automatically: bool = true;
  if cfg!(target_os = "windows"){close_automatically = false;}

  // check for command line arguments
  let args: Vec<String> = std::env::args().collect();
  for i in 0..args.len()
  {
    if args[i] == "-d" || args[i] == "--delete"
    {
      println!("Deleting user data");
      std::fs::remove_dir_all(&user_data_folder).expect("anijouhou config directory can not be deleted.");
      std::process::exit(0);
    }
    else if args[i] == "-c" || args[i] == "--clear-cache"
    {
      println!("Clearing cache");
      std::fs::remove_file(cache_path.clone()).expect("Cache directory can not be deleted.");
      std::fs::remove_file(profile_picture_path.clone()).expect("Cached profile picture could not be deleted");
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
    else if args[i] == "-t" || args[i] == "--text"
    {
      verbosity = Verbosity::Text;
    }
    else if args[i] == "-u" || args[i] == "--username" 
    {
      // clear config directory
      if std::path::Path::new(&user_data_folder).exists() 
      {
        std::fs::remove_dir_all(&user_data_folder).expect("Anijouhou config directory can not be deleted.");
      }
      username = args[i+1].clone();
    }
    else if args[i] == "-k" || args[i] == "--api-key"
    {
      if i >= 2 
      {
        api_key = args[i+1].clone();
      }
      else 
      {
        println!("Please always enter a username AND an api key.");
        std::process::exit(1);
      }
    }
    else if args[i] == "-a" || args[i] == "--close-automatically"
    {
      close_automatically = !close_automatically;
    }

    // assume that an argument without a flag is the username of a user with a public profile.
    else if !args[i].to_string().contains("anijouhou") && i <= 2
    {
      // clear config directory
      if std::path::Path::new(&user_data_folder).exists() 
      {
        std::fs::remove_dir_all(&user_data_folder).expect("Anijouhou config directory can not be deleted.");
      }

      if !args[i].to_string().is_empty()
      {
        username = args[i].clone();
        api_key = "skip".to_string();
      }
    }
  }


  // get api response either from cache or AniList server
  let api_response = cache::read_api_response(cache_path, user_data_folder, config_path, username, api_key);
  // parse json
  let parsed_data = cache::AniListApiResponse {
    minutes: api_response["data"]["User"]["statistics"]["anime"]["minutesWatched"].as_i64().unwrap(), //or as_f64 if I wanted a float.
    hours: api_response["data"]["User"]["statistics"]["anime"]["minutesWatched"].as_i64().unwrap() / 60,
    episodes: api_response["data"]["User"]["statistics"]["anime"]["episodesWatched"].as_i64().unwrap(), //or as_f64 if I wanted a float.
    username: api_response["data"]["User"]["name"].to_string().replace('"', ""),
    avatar_url: api_response["data"]["User"]["avatar"]["large"].to_string().replace('"', ""),
    genre_1: api_response["data"]["User"]["statistics"]["anime"]["genres"][0]["genre"].to_string().replace('"', ""),
    genre_1_hours: api_response["data"]["User"]["statistics"]["anime"]["genres"][0]["minutesWatched"].as_i64().unwrap() / 60,
    genre_2: api_response["data"]["User"]["statistics"]["anime"]["genres"][1]["genre"].to_string().replace('"', ""),
    genre_2_hours: api_response["data"]["User"]["statistics"]["anime"]["genres"][1]["minutesWatched"].as_i64().unwrap() / 60,
    genre_3: api_response["data"]["User"]["statistics"]["anime"]["genres"][2]["genre"].to_string().replace('"', ""),
    genre_3_hours: api_response["data"]["User"]["statistics"]["anime"]["genres"][2]["minutesWatched"].as_i64().unwrap() / 60,
  };

  cache::cache_profile_picture(profile_picture_path.clone(), parsed_data.avatar_url.clone());

  // print to screen
  if verbosity == Verbosity::All
  {
    frontend::main(profile_picture_path, frontend_config_path, parsed_data).expect("Could not run frontend.");
    if !close_automatically
    {
      std::io::stdin().read_line(&mut String::new()).unwrap();
    }
  }
  else if verbosity == Verbosity::Text
  {
    println!("{} watched {} episodes making for a total of {} hours ({} minutes).", parsed_data.username, parsed_data.episodes, parsed_data.hours, parsed_data.minutes);
    if !close_automatically
    {
      std::io::stdin().read_line(&mut String::new()).unwrap();
    }
  }
  else if verbosity == Verbosity::Hours
  {
    print!("{}", parsed_data.hours);
  }
  else if verbosity == Verbosity::Episodes
  {
    print!("{}", parsed_data.episodes);
  }
  else if verbosity == Verbosity::Minutes
  {
    print!("{}", parsed_data.minutes);
  }
  
}


fn get_user_information(user_data_folder: String, config_path: String, mut username: String, mut api_key: String) -> Vec<String>
{
  // create folder if it doesn't exists
  if !std::path::Path::new(&user_data_folder).exists()
  {
    std::fs::create_dir(&user_data_folder).expect("Config directory could not be created.");
  }

  // check for exisiting user-data
  if std::path::Path::new(&config_path).exists()
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
          open::that("https://anilist.co/api/v2/oauth/authorize?client_id=30455&response_type=token").expect("Could not open browser window.");
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
    std::fs::write(&config_path, final_output).expect("Could not write to configuration file.");
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
  // remove CRLF newlines
  if input.ends_with('\r')
  {
    input = input.replace("\r", "");
  }
  return input;
}