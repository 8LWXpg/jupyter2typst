use crate::IMG_PATH;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde_json::Value;
use std::sync::OnceLock;
use std::{
	collections::HashMap,
	fs::{self, File},
	io::Write,
};

use crate::{katex, md, typ};

static LANG: OnceLock<String> = OnceLock::new();
static TEMPLATE: &str = "#import \"template.typ\": *\n#show: template\n\n";

pub fn ipynb_parse(json: Value) -> String {
	// https://nbformat.readthedocs.io/en/latest/format_description.html
	let mut output = String::from(TEMPLATE);
	LANG.set(
		json["metadata"]["language_info"]["name"]
			.as_str()
			.unwrap()
			.to_string(),
	)
	.unwrap();

	for cell in json["cells"].as_array().unwrap() {
		// attachments
		let mut attachments: HashMap<String, String> = HashMap::new();
		if let Some(item) = cell["attachments"].as_object() {
			for (name, value) in item {
				let extension = name.split('.').last().unwrap();
				let content = value[format!("image/{}", extension)].as_str().unwrap();
				let file_path = format!(
					"{}/{}.{}",
					IMG_PATH.get().unwrap(),
					md::sha1(content),
					extension
				);
				let mut file = File::create(file_path.clone()).unwrap();
				file.write_all(&STANDARD.decode(content).unwrap()).unwrap();
				attachments.insert(name.clone(), file_path);
			}
		};

		// source and output
		output += "#block[\n";
		match cell["cell_type"].as_str().unwrap() {
			"markdown" => {
				output += &md::md_to_typst(
					cell["source"]
						.as_array()
						.unwrap()
						.iter()
						.map(|v| v.as_str().unwrap())
						.collect::<Vec<&str>>(),
					attachments,
				);
			}
			"code" => {
				output += &code_parse(
					cell["source"]
						.as_array()
						.unwrap()
						.iter()
						.map(|v| v.as_str().unwrap())
						.collect::<Vec<&str>>(),
					cell["execution_count"].as_i64().unwrap_or_default(),
				);
				output += "]\n#block[\n";
				output += &code_output_parse(cell["outputs"].clone(), IMG_PATH.get().unwrap());
			}
			_ => {}
		};
		output += "]\n";
	}

	output
}

fn code_parse(code: Vec<&str>, count: i64) -> String {
	let mut context = String::new();

	context += "#code-block(\"";
	context += &typ::escape_string(code.join(""));
	context += format!(
		"\"\n, lang: \"{}\", count: {})\n",
		LANG.get().unwrap(),
		count
	)
	.as_str();

	context
}

fn code_output_parse(outputs: Value, img_path: &str) -> String {
	let mut context = String::new();

	for output in outputs.as_array().unwrap() {
		match output["output_type"].as_str().unwrap() {
			"stream" => {
				context += &format!(
					"#result-block(\"{}\")\n",
					output["text"]
						.as_array()
						.unwrap()
						.iter()
						.map(|v| v.as_str().unwrap())
						.collect::<Vec<&str>>()
						.join("")
				);
			}
			"display_data" | "execute_result" => {
				let data = output["data"].clone();
				if let Some(img) = data["image/svg+xml"].as_array() {
					fs::create_dir_all(img_path).unwrap();
					let content = img
						.iter()
						.map(|v| v.as_str().unwrap())
						.collect::<Vec<&str>>()
						.join("");
					let file_path = format!("{}/{}.svg", img_path, md::sha1(&content));
					let mut file = File::create(&file_path).unwrap();
					file.write_all(content.as_bytes()).unwrap();
					context += format!("#image(\"./{}\")\n", file_path).as_str();
				} else if let Some(img) = data["image/png"].as_str() {
					// base 64 image data
					fs::create_dir_all(img_path).unwrap();
					let file_path = format!("{}/{}.png", img_path, md::sha1(img));
					let mut file = File::create(&file_path).unwrap();
					file.write_all(&STANDARD.decode(img).unwrap()).unwrap();
					context += format!("#image(\"./{}\")\n", file_path).as_str();
				} else if let Some(text) = data["text/plain"].as_array() {
					context += &format!(
						"#result-block(\"{}\")\n",
						typ::escape_string(
							text.iter()
								.map(|v| v.as_str().unwrap())
								.collect::<Vec<&str>>()
								.join("")
						),
					);
				} else if let Some(text) = data["text/latex"].as_array() {
					context += &katex::text_to_typst(
						&text
							.iter()
							.map(|v| v.as_str().unwrap())
							.collect::<Vec<&str>>()
							.join("")
							.replace("$$", "$"),
					)
					.unwrap();
				} else if let Some(text) = data["text/html"].as_array() {
					context += &md::html_to_typst(
						text.iter()
							.map(|v| v.as_str().unwrap())
							.collect::<Vec<&str>>()
							.join("")
							.as_str(),
					);
				}
			}
			"error" => {
				context += &format!(
					"#result-block(\"{}\")\n",
					output["traceback"]
						.as_array()
						.unwrap()
						.iter()
						.map(|v| v.as_str().unwrap())
						.collect::<Vec<&str>>()
						.join("\n"),
				);
			}
			other => {
				println!("unhandled output type: {other}\n");
				unreachable!();
			}
		}
	}

	context
}
