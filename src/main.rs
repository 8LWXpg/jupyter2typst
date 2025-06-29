mod body;
mod katex;
mod md;
mod typ;

use serde_json::Value;
use std::{
	env,
	fs::{self, File},
	io::{Read, Write},
	path::Path,
	sync::OnceLock,
};

use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "A simple tool to convert a Jupyter Notebook to Typst format")]
struct Args {
	/// the input file
	#[argh(positional)]
	input: String,

	/// the output file path (without extension)
	#[argh(option, short = 'o')]
	output: Option<String>,

	/// the output image path, defaults to "img"
	#[argh(option, short = 'i', default = "String::from(\"img\")")]
	img_path: String,
}

static IMG_PATH: OnceLock<String> = OnceLock::new();

fn main() {
	let args: Args = argh::from_env();
	IMG_PATH.set(args.img_path).unwrap();

	let extension = args.input.split('.').next_back().expect("Failed to get file extension");
	let json: Value = match extension {
		"ipynb" => {
			let mut file = File::open(&args.input).expect("Failed to open file");
			let mut contents = String::new();
			file.read_to_string(&mut contents).expect("Failed to read file");
			serde_json::from_str(&contents).expect("Failed to parse JSON")
		}
		_ => panic!("Invalid file extension"),
	};

	let abs_path = env::current_dir()
		.expect("Failed to get current directory")
		.join(&args.input);
	env::set_current_dir(abs_path.parent().unwrap()).expect("Failed to set current directory");
	fs::create_dir_all(IMG_PATH.get().unwrap()).expect("Failed to create image directory");
	let output = body::ipynb_parse(json);

	let out_file = args.output.unwrap_or_else(|| {
		Path::new(&args.input)
			.file_stem()
			.unwrap()
			.to_str()
			.unwrap()
			.to_string()
	});
	let mut file = File::create(format!("{out_file}.typ")).expect("Failed to create/open file");
	file.write_all(output.as_bytes()).expect("Failed to write file");
}
