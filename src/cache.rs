use chrono::Local;

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
  std::fs::write(cache_path, today + "\n" + &result).expect("Could not write api response to cache.")
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