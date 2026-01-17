use image::DynamicImage;
use terminal_size::{Width, Height, terminal_size};
use viuer::{print, Config};
use colored::{ColoredString, Colorize};

fn configuration(column_size: u16, username: String, watchtime_hours: i64 , watchtime_minutes: i64, episodes: i64) {
      // print text on the right
      print_in_second_column(column_size, username.clone().bold());
      print_in_second_column(column_size, " ".to_string().normal());
      print_in_second_column_themed(column_size, "", watchtime_hours.to_string(), "hours");
      print_in_second_column_themed(column_size, "󰟴", watchtime_minutes.to_string(), "minutes");
      print_in_second_column_themed(column_size, "󰆙", episodes.to_string(), "episodes");
}

pub fn main(avatar_url: String, username: String, watchtime_hours: i64 , watchtime_minutes: i64, episodes: i64) -> Result<(), Box<dyn std::error::Error>> {  
  // get image from url
  // thanks to https://www.reddit.com/r/rust/comments/g2zeps/how_do_i_get_an_image_from_a_url/
  let img_bytes = reqwest::blocking::get(avatar_url)?
      .bytes()?;
    
  let image = image::load_from_memory(&img_bytes)?;

  let size: Option<(Width, Height)> = terminal_size();
    
  if let Some((Width(w), Height(_h))) = size {
      // get basic values
      let column_size = (w*2)/7;
      let image_size = w/4;

      // clear the terminal and position cursor at 1,1
      print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
      
      configuration(column_size, username, watchtime_hours, watchtime_minutes, episodes);
      
      // print image on the left
      print_image(image_size, image);
      // print user in beneath the image (in the middle)
      //let username_len = convert_usize(username.len());
      //let start_username_at = (image_size - username_len) / 2;
      //for _i in 0..(start_username_at) {print!(" ");}
      //println!("{}", username.bold());
  }
  Ok(())
}

// helper functions
fn print_image(image_size: u16, image: DynamicImage) {

    let conf = Config {
        // Set dimensions.
        width: Some(image_size.into()),
        //height: Some(25),
        ..Default::default()
    };

    print(&image, &conf).expect("Image printing failed.");
}

fn print_in_second_column(column_size: u16, input: ColoredString) {
    for _i in 0..column_size {print!(" ");}
    println!("{}", input);
}

fn print_in_second_column_themed(column_size: u16, icon: &str, value: String, unit: &str){
      for _i in 0..column_size {print!(" ");}
      println!("{} {} {}", icon.blue(), value, unit);  
}