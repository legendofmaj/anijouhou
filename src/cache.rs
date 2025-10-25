use chrono::Local;

pub fn write_cache(result: String, cache_path: String)
{
  // write current data to cache
  let today = Local::now().date_naive().to_string();
  // write result to file
  std::fs::write(cache_path, today + "\n" + &result).expect("Could not write api response to cache.")
}

pub fn read_cache(cache_path: String) -> String
{
  let mut cache_content: String = Default::default();
  // save data in variable
  let cache = read_lines(&cache_path);
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
pub fn read_lines(filename: &str) -> Vec<String> {
  //Taken from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
  use std::fs::read_to_string;
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}