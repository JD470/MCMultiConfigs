use std::{fs::{self, File}, io::{self, Read}, path::{Path, PathBuf}, num::ParseIntError, process::exit};

use ansi_term::Color;

fn config_file_exists(path: String) -> bool{
	return Path::new(&(path + "\\configs.json")).exists()
}

fn create_config_file(path: String){
	if !config_file_exists(path.clone()){
		File::create(path + "\\configs.json").unwrap();
	}
}

fn get_current_config(path: &str) -> String{
	let mut config_file = File::open(path.to_string() + "\\configs.json").unwrap();
	let mut buffer = String::new();
	config_file.read_to_string(&mut buffer).unwrap();
	buffer
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
	
	if !current_config.is_empty(){ // Pushes all the mods in the used config into the config's folder
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
		update_config_file(path, &current_config);
		return;
	}

	let mut mod_folder = PathBuf::from(path);
	mod_folder.push(next_config);
	let next_config_mods = get_all_mod_files_in_dir(mod_folder.to_str().unwrap());

	for file in next_config_mods{ // Pushes all the mods of the desired config into the mod folder
		let temp_path = Path::new(&file);
		let temp_name = temp_path.file_name().unwrap();
		let mut destination = PathBuf::from(path);
		destination.push(temp_name);

		fs::rename(temp_path.to_str().unwrap(), destination.to_str().unwrap()).unwrap();
	}

	current_config.clear();
	current_config.push_str(next_config);
	update_config_file(path, &current_config);
}

fn update_config_file(path: &str, current_config: &String){
	fs::write(path.to_string() + "\\configs.json", current_config).unwrap();
	if current_config.is_empty(){
		let file = File::create(path.to_string() + "\\configs.json").unwrap();
		file.set_len(0).unwrap();
	}
}

#[cfg(target_os = "windows")]
fn initialize_vt100(){
	output_vt100::init();
}

const IN_PARENTHESIS: Color = Color::RGB(125, 125, 125);
const NORMAL_TEXT: Color = Color::RGB(165, 165, 165);
const CONFIGURATION_COLOR: Color = Color::Cyan;
const ERROR: Color = Color::Red;

fn main() {
	cfg_if::cfg_if! {
		if #[cfg(target_os = "windows")] {
			initialize_vt100();
		}
	}

	ctrlc::set_handler(||{
		exit(0);
	}).unwrap();
	
	let mods_path = include_str!("../mods_path.txt");
	
	create_config_file(mods_path.to_string());
	
	let configurations: Vec<String> = get_all_configs(mods_path);
	
	let mut current_config = get_current_config(mods_path);

	println!("Current configuration: {}", if !current_config.is_empty() {get_name_of_dir_or_file(&current_config)} else {"none".to_string()});

	println!();

	println!("Commands:\n\t{}\n\t{}\n\t{}",
				NORMAL_TEXT.paint("exit Exits the program."), 
				IN_PARENTHESIS.paint("(Can also Ctrl+C or end the terminal without messing things up if you already swapped the config)"),
				NORMAL_TEXT.paint("swap <number> Swaps current config with the config corresponding to the config."));

	println!();

	println!("Configurations:");
	
	configurations.clone().iter().enumerate().map(|(index, value)| {
		println!("\t{} {}", IN_PARENTHESIS.paint(format!("({})", index+1)), CONFIGURATION_COLOR.underline().paint(get_name_of_dir_or_file(&value)));
	}).for_each(drop);

	println!();

	loop{
		let mut input: String = String::new();
		input = get_input(&mut input);
		let command = input.split(' ').collect::<Vec<&str>>()[0];

		if command == "exit"{
			break;
		}

		if command == "swap"{
			let number = input.split(' ').collect::<Vec<&str>>();
			let first_arg: Result<i16, ParseIntError>;
			
			if number.len() == 2 {
				first_arg = number[1].parse();
			}
			else{
				println!("{}", ERROR.underline().paint("There needs to be two arguments!"));
				continue;
			}

			if first_arg.is_ok() {
				let first_arg_ok = first_arg.unwrap();
				swap_configs(&mut current_config, mods_path, &configurations[(first_arg_ok-1) as usize]);
				println!("Current configuration: {}", if !current_config.is_empty() {get_name_of_dir_or_file(&current_config.clone())} else {"none".to_string()});
			}
			else{
				println!("{}", ERROR.underline().paint("First argument: This is not a number"));
			}
		}
	}
	update_config_file(mods_path, &current_config);
}

fn get_input(buffer: &mut String) -> String {
	io::stdin().read_line(buffer).unwrap();
	buffer.trim().to_string()
}