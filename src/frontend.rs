use image::DynamicImage;
use terminal_size::{Width, Height, terminal_size};
use viuer::{print, Config};
use colored::{ColoredString, Colorize};
use image::ImageReader;

fn configuration(column_size: u16, username: String, watchtime_hours: i64 , watchtime_minutes: i64, episodes: i64) {
    // print text on the right
    print_in_second_column(column_size, username.clone().bold());
    print_in_second_column(column_size, "--------------".to_string().normal());

    print_in_second_column_themed(column_size, "", watchtime_hours.to_string(), "hours");
    print_in_second_column_themed(column_size, "󰟴", watchtime_minutes.to_string(), "minutes");
    print_in_second_column_themed(column_size, "󰆙", episodes.to_string(), "episodes");

    print_in_second_column_themed(column_size, "Debug: ", "This module has not been completed yet".to_string(), "");
    print_in_second_column_themed(column_size, "Debug: ", "This module has not been completed yet".to_string(), "");
    print_in_second_column_themed(column_size, "Debug: ", "This module has not been completed yet".to_string(), "");
}

pub fn main(profile_picture_path: String, username: String, watchtime_hours: i64 , watchtime_minutes: i64, episodes: i64, scaling_factor: f64) -> Result<(), Box<dyn std::error::Error>> {
  let profile_picture = ImageReader::open(profile_picture_path)?.decode()?;

  let size: Option<(Width, Height)> = terminal_size();
    
  if let Some((Width(w), Height(_h))) = size {
    // divided by 4 and divided by 4.5 are just values that I think look good
      let column_size = (w/4) as f64 * scaling_factor;
      let image_size = ((w*2)/9) as f64 * scaling_factor;

      // clear the terminal and position cursor at 1,1
      print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
      
      configuration(column_size as u16, username, watchtime_hours, watchtime_minutes, episodes);
      
      // print image on the left
      print_image(image_size as u16, profile_picture);
  }
  Ok(())
}

// helper functions
fn print_image(image_size: u16, image: DynamicImage) {
    let conf = Config {
        // set start coordinates
        x: 0,
        y: 0,
        
        // set dimensions
        width: Some(image_size.into()),
        height: Some((image_size/2).into()),
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