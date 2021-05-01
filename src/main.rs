use std::env;
use std::process;
use std::fs;
use std::path::Path;
use clap::{Arg, App, ArgMatches};
use serde_json::{json, Value};

// Concatenating constant strings of any kind is a pain here so some convoluted logic for now :(
fn get_home() -> String {
  return env::var("HOME").unwrap();
}

fn get_dir_path() -> String {
  let mut home = get_home();
  home.push_str("/.clipboard");
  return home;
}

fn get_file_path() -> String {
  let mut file = get_dir_path();
  file.push_str("/map.json");
  return file;
}

fn init() {
  let dir_temp: String = get_dir_path();
  let dir_path: &str = dir_temp.as_str();
  if !Path::new(dir_path).exists() {
    match fs::create_dir(dir_path) {
        Ok(_file) => println!("Created dir {}", dir_path),
        Err(error) => {
            panic!("Problem when creating dir {}. Error {}", dir_path, error);
        }
    }
   }
   let file_temp = get_file_path();
   let file_path = file_temp.as_str();
   if !Path::new(file_path).exists() {
    match fs::File::create(file_path) {
	Ok(_file) => println!("Created file {}", file_path),
	Err(error) => {
	    panic!("Problem when creating file {}. Error {}", file_path, error);
	}
    }
  }
}

fn main() {

    init();
    let args = App::new("Clipboard")
	.version("0.1.0")
	.about("Clipboard for terminal")
	.arg(Arg::with_name("action")
	    .short("a")
	    .long("action")
	    .takes_value(true)
	    .help("Action can be get, put or delete"))
	.arg(Arg::with_name("key")
	    .short("k")
	    .long("key")
	    .takes_value(true)
	    .help("Key to act against"))
	.arg(Arg::with_name("value")
	    .short("v")
	    .long("value")
	    .takes_value(true)
	    .help("Append value to key"))
	.arg(Arg::with_name("index")
	    .short("i")
	    .long("index")
	    .takes_value(true)
	    .help("Get i'th value for a key"))
	.get_matches();

	let action = args.value_of("action").unwrap_or_default();
	let key = args.value_of("key").unwrap_or_default();
        if action.is_empty() || key.is_empty() {
	    println!("Action and key are mandatory fields. Please enter them to peform an action"); 
            process::exit(1);
	}

	match action {
		"get" => handle_get(&args, key),
		"put" => handle_put(&args, key), 
		"delete" => println!("Delete not implemented yet"),
		default => { println!("Invalid action {}. Please enter a valid action", default); process::exit(1); }
	}

}

fn handle_get(args: &ArgMatches, key: &str) {
  let map = read_json_from_file();
  if map[key] == Value::Null {
    println!("No key found");
    return;
  }
  let idx = args.value_of("index").unwrap_or_default();
  let arr = &map[key];
  if idx.is_empty() {
    display(arr.as_array().unwrap(), key, None);
  } else {
    let index: usize = idx.parse().expect("Please pass a valid index");
    display(arr.as_array().unwrap(), key, Some(index)); 
  }
}

fn handle_put(args: &ArgMatches, key: &str) {
  let value = args.value_of("value").unwrap_or_default();
  if value.is_empty() {
    println!("Value cannot be empty");
    return;
  }
  let mut map = read_json_from_file();
  if map[key] == Value::Null {
    let arr = json!([value]);
    map[key] = arr; 
  } else {
    let arr = map[key].as_array_mut().unwrap();
    if !arr.iter().any(|i| i==value) {
      arr.push(value.into());
    }
  }
  write_to_json(map);
}

fn display(arr: &Vec<Value>, key: &str, index: Option<usize>) {
  match index {
    None => {
      println!("All values for key {} are:", key);
      for val in arr.iter() {
        println!("{}", val.as_str().unwrap());
      }      
    },
    Some(index) => {
      if index > arr.len()-1 {
        println!("Index out of bounds for this key");
      } else { 
        println!("Value for key {} at index {} is:.", index, key);
        println!("{}", arr[index].as_str().unwrap());
      }
    }
  }
}

fn write_to_json(map: Value) {
  match fs::write(get_file_path(), map.to_string()) {
    Ok(_file) => println!("Updated!"),
    Err(err) => {
      panic!("Problem when updating value {}", err);
    }
  }
}

fn read_json_from_file() -> Value {
  let mut contents = fs::read_to_string(get_file_path())
        .expect("Something went wrong reading the file");
  if contents.is_empty() {
    contents = String::from("{}"); 
  }
  let json: Value = serde_json::from_str(contents.as_str()).unwrap();
  return json; 
}
