use image::DynamicImage;
use terminal_size::{Width, Height, terminal_size};
use viuer::{print, Config};
use colored::Colorize;
use image::ImageReader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FrontendConfig {
  scaling_factor: f64,
  modules: Vec<Modules>,
}

#[derive(Debug, Deserialize)]
struct Modules {
  icon: Option<String>,
  icon_color: Option<String>,
  icon_bold: Option<bool>,
  value: Option<String>,
  value_color: Option<String>,
  value_bold: Option<bool>,
  unit: Option<String>,
  unit_color: Option<String>,
  unit_bold: Option<bool>
}

pub fn main(profile_picture_path: String, frontend_config_path: String, parsed_data: crate::cache::AniListApiResponse) -> Result<(), Box<dyn std::error::Error>> {
  create_default_config(frontend_config_path.clone());

  // read the config file
  let frontend_config: String  = std::fs::read_to_string(frontend_config_path.clone())?;

  let frontend_config_struct: FrontendConfig = toml::from_str(&frontend_config).expect("Could not read frontend config.");

  let profile_picture = ImageReader::open(profile_picture_path)?.decode()?;

  let size: Option<(Width, Height)> = terminal_size();
    
  if let Some((Width(w), Height(_h))) = size {
    // divided by 4 and divided by 4.5 are just values that I think look good
    let column_size = (w/4) as f64 * frontend_config_struct.scaling_factor;
    let image_size = ((w*2)/9) as f64 * frontend_config_struct.scaling_factor;
    // clear the terminal and position cursor at 1,1
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    
    print_modules(column_size as u16, frontend_config_struct, parsed_data);
    
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

fn print_modules(column_size: u16, frontend_config_struct: FrontendConfig, parsed_data: crate::cache::AniListApiResponse) {
  // variable to check if image spacing should be printed
  let mut on_same_line = false;

  for i in frontend_config_struct.modules {
    // -- handling for empty options --
    let icon = match i.icon {
      // a value is present
      Some(x) => x,
      // print a newline
      None => "".to_string()
    };
    // check for colors options
    let icon_color = match i.icon_color {
      Some(x) => x,
      None => "".to_string()
    };
    let icon_bold = match i.icon_bold {
      Some(x) => x,
      None => false
    };
    let mut value = match i.value {
      Some(x) => x,
      None => "".to_string()
    };
    let value_color = match i.value_color {
      Some(x) => x,
      None => "".to_string()
    };
    let value_bold = match i.value_bold {
      Some(x) => x,
      None => false
    };
    let unit = match i.unit {
      Some(x) => x,
      None => "".to_string()
    };
    let unit_color = match i.unit_color {
      Some(x) => x,
      None => "".to_string()
    };
    let unit_bold = match i.unit_bold {
      Some(x) => x,
      None => false
    };

    // -- value parsing --
    if value == "username".to_string() {
      value = parsed_data.username.clone();
    }
    else if value == "watchtime_hours" {
      value = parsed_data.hours.to_string();
    }
    else if value == "watchtime_minutes" {
      value = parsed_data.minutes.to_string();
    }
    else if value == "episodes" {
      value = parsed_data.episodes.to_string();
    }
    else if value == "genre_1" {
      value = parsed_data.genre_1.clone();
    }
    else if value == "genre_1_hours" {
      value = parsed_data.genre_1_hours.clone().to_string();
    }
    else if value == "genre_2" {
      value = parsed_data.genre_2.clone();
    }
    else if value == "genre_2_hours" {
      value = parsed_data.genre_2_hours.clone().to_string();
    }
    else if value == "genre_3" {
      value = parsed_data.genre_3.clone();
    }
    else if value == "genre_3_hours" {
      value = parsed_data.genre_3_hours.clone().to_string();
    }
    else if value != "" {
      value = "Unknown option".to_string();
    }

    // -- printing --
    // print image spacing
    if !on_same_line {
      for _i in 0..column_size {print!(" ");}
    }
    // check if the bold or normal version should be printed
    if icon_bold {
      print!("{}", icon.color(icon_color).bold());
    }
    else {
      print!("{}", icon.color(icon_color));
    }
    if value_bold {
      print!("{}",value.color(value_color).bold());
    }
    else {
      print!("{}",value.color(value_color));
    }
    // option to not print a new line
    if unit.contains("!NoNewLine") {
      on_same_line = true;
      // don't print the !NoNewLine as part of the unit
      if unit_bold {
        print!("{}", unit.replace("!NoNewLine", "").color(unit_color.clone()).bold());
      }
      else {
        print!("{}", unit.replace("!NoNewLine", "").color(unit_color.clone()));
      }
    }
    else {
      on_same_line = false;
      if unit_bold {
        println!("{}",unit.color(unit_color).bold());
      }
      else {
        println!("{}",unit.color(unit_color));
      }
    }
  }
}

fn create_default_config(frontend_config_path: String) {
  let toml_str = r##"# Changes the size of the profile picture and spacing between profile picture and text.
scaling_factor = 0.8
# You can add as many modules as you like, however they will be cut off once they take up more space than your profile picture.
[[modules]]
# Valid values are: `username`, `watchtime_hours`, `watchtime_minutes` and `episodes`.
# Everything else will be regarded as a normal string.
value = "username"
value_bold = true
[[modules]]
icon = "--------------"
icon_bold = false
[[modules]]
# `value` and `unit` also have `value.bold` / `unit_color` options.
icon = "󰦖 "
icon_color = "#1e66f5"
icon_bold = false
value = "watchtime_hours"
unit = " hours"
[[modules]]
icon = " "
icon_color = "#1e66f5"
icon_bold = false
value = "watchtime_minutes"
unit = " minutes"
[[modules]]
icon = "󰻏 "
icon_color = "#1e66f5"
icon_bold = false
value = "episodes"
unit = " episodes"
[[modules]]
icon = " "
icon_color = "#1e66f5"
icon_bold = false
value = "genre_1"
unit = "!NoNewLine"
[[modules]]
icon = " ("
icon_bold = false
value = "genre_1_hours"
unit = " hours)"
[[modules]]
icon = "󰲣 "
icon_color = "#1e66f5"
icon_bold = false
value = "genre_2"
unit = "!NoNewLine"
[[modules]]
icon = " ("
icon_bold = false
value = "genre_2_hours"
unit = " hours)"
[[modules]]
icon = "󰲥 "
icon_color = "#1e66f5"
icon_bold = false
value = "genre_3"
unit = "!NoNewLine"
[[modules]]
icon = " ("
icon_bold = false
value = "genre_3_hours"
unit = " hours)""##;
  
  // create default config if none is present
  if std::path::Path::new(&frontend_config_path).exists() == false {
    std::fs::write(frontend_config_path, &toml_str).expect("Could not save default frontend configuration.");
  }
}