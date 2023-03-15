use std::{fs::{self, File}, io::{self, Write, Read}, path::{Path, PathBuf}, ops::Deref};

use ansi_term::{Color};
use json::{object, JsonValue};

fn config_file_exists(path: String) -> bool{
	return Path::new(&(path + "\\configs.json")).exists()
}

fn create_config_file(path: String) -> JsonValue{
	let json_template = object! {
		current_config: ""
	};
	if !config_file_exists(path.clone()){
		let mut file = File::create(path + "\\configs.json").unwrap();
		file.write_all(json_template.to_string().as_bytes()).unwrap();
	}
	json_template
}

fn get_current_config(path: &str) -> String{
	let mut config_file = File::open(&(path.to_string() + "\\configs.json")).unwrap();
	let mut buffer = String::new();
	config_file.read_to_string(&mut buffer).unwrap();
	json::parse(&buffer).unwrap()["current_config"].clone().to_string()
}

fn get_all_configs(path: &str) -> Vec<String>{
	fs::read_dir(path).unwrap()
		.filter(|file| file.as_ref().unwrap().path().is_dir())
		.map(|file| file.unwrap().path().to_str().unwrap().to_string())
		.collect()
}

fn get_all_mod_files_in_dir(path: &str) -> Vec<String>{
	fs::read_dir(path).unwrap()
		.filter(|file|	file.as_ref().unwrap().path().is_file() &&
													file.as_ref().unwrap().path().to_str().unwrap().ends_with(".jar"))
		.map(|file| file.unwrap().path().to_str().unwrap().to_string())
		.collect()
}

fn get_name_of_dir_or_file(path: &str) -> String{
	return Path::new(path).file_name().unwrap().to_str().unwrap().to_string();
}

fn swap_configs(current_config: &mut String, path: &str, next_config: &str) {
	let current_mods = get_all_mod_files_in_dir(path);
	
	if current_config != ""{
		for file in current_mods {
			let temp_path = Path::new(&file);
			let temp_name = temp_path.file_name().unwrap();
			let mut destination = PathBuf::from(path);
			destination.push(current_config.clone());
			destination.push(temp_name);

			fs::rename(temp_path.to_str().unwrap(), destination.to_str().unwrap()).unwrap();
		}
	}
	if current_config == next_config{
		current_config.clear();
		return;
	}
	let mut mod_folder = PathBuf::from(path);
	mod_folder.push(next_config);
	let next_config_mods = get_all_mod_files_in_dir(mod_folder.to_str().unwrap());

	
	for file in next_config_mods{
		let temp_path = Path::new(&file);
		let temp_name = temp_path.file_name().unwrap();
		let mut destination = PathBuf::from(path);
		destination.push(temp_name);

		fs::rename(temp_path.to_str().unwrap(), destination.to_str().unwrap()).unwrap();
	}
	current_config.clear();
	current_config.push_str(next_config);
}

fn main() {
	output_vt100::init();
	let mods_path = include_str!("../mods_path.txt");

	let mut json_template = create_config_file(mods_path.to_string());

	let configurations: Vec<String> = get_all_configs(mods_path);

	let mut current_config = get_current_config(mods_path);

	println!("Current configuration: {}\n", if current_config != "" {get_name_of_dir_or_file(&current_config)} else {"none".to_string()});

	println!("Configurations:");
	let mut config_counter = 1;
	for i in configurations.clone(){
		println!("\t{} {}", Color::RGB(125, 125, 125).paint(format!("({config_counter})")), Color::Cyan.underline().paint(get_name_of_dir_or_file(&i)));
		config_counter += 1;
	}
	println!("");

	loop{
		let mut input: String = String::new();
		input = get_input(&mut input);
		let command = input.split(' ').collect::<Vec<&str>>()[0];

		if command == "exit"{
			break;
		}

		if command == "swap"{
			let number = input.split(' ').collect::<Vec<&str>>();
			if number.len() == 2 {
				let first_arg = number[1].parse::<i32>();
				if first_arg.is_err() {
					println!("{}", Color::Red.underline().paint("First argument: This is not a number"));
				}
				else{
					let temp = first_arg.unwrap();
					swap_configs(&mut current_config, mods_path, &configurations[(temp-1) as usize]);
					json_template["current_config"] = JsonValue::String(current_config.clone());
					println!("Current configuration: {}", if current_config != "" {get_name_of_dir_or_file(&current_config.clone())} else {"none".to_string()});
				}
			}
		}
	}
	fs::write(mods_path.to_string() + "\\configs.json", json_template.to_string()).unwrap();
}



fn get_input(buffer: &mut String) -> String {
	io::stdin().read_line(buffer).unwrap();
	buffer.trim().to_string()
}