use crate::IMG_PATH;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use itertools::Itertools;
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
	LANG.set(json["metadata"]["language_info"]["name"].as_str().unwrap().into())
		.unwrap();

	for cell in json["cells"].as_array().unwrap() {
		// attachments
		let mut attachments: HashMap<String, String> = HashMap::new();
		if let Some(item) = cell["attachments"].as_object() {
			for (name, value) in item {
				let extension = name.split('.').next_back().unwrap();
				let content = value[format!("image/{extension}")].as_str().unwrap();
				let file_path = format!("{}/{}.{}", IMG_PATH.get().unwrap(), md::sha1(content), extension);
				let mut file = File::create(&file_path).unwrap();
				file.write_all(&STANDARD.decode(content).unwrap()).unwrap();
				attachments.insert(name.clone(), file_path);
			}
		};

		// source and output
		output += &format!(
			"#block[\n{}]\n",
			match cell["cell_type"].as_str().unwrap() {
				"markdown" => md::md_to_typst(
					&cell["source"]
						.as_array()
						.unwrap()
						.iter()
						.map(|v| v.as_str().unwrap())
						.join(""),
					attachments,
				),
				"code" => format!(
					"{}]\n#block[\n{}",
					code_parse(
						&cell["source"]
							.as_array()
							.unwrap()
							.iter()
							.map(|v| v.as_str().unwrap())
							.join(""),
						cell["execution_count"].as_i64().unwrap_or_default(),
					),
					code_output_parse(&cell["outputs"]),
				),
				_ => "".into(),
			},
		);
	}

	output
}

fn code_parse(code: &str, count: i64) -> String {
	format!(
		"#code-block(\"{}\"\n, lang: \"{}\", count: {})\n",
		typ::escape_string(code),
		LANG.get().unwrap(),
		count
	)
}

fn code_output_parse(outputs: &Value) -> String {
	let img_path = IMG_PATH.get().unwrap();

	outputs
		.as_array()
		.unwrap()
		.iter()
		.map(|output| {
			match output["output_type"].as_str().unwrap() {
				"stream" => format!(
					"#result-block(\"{}\")\n",
					output["text"]
						.as_array()
						.unwrap()
						.iter()
						.map(|v| v.as_str().unwrap())
						.join("")
				),
				"display_data" | "execute_result" => {
					let data = &output["data"];
					if let Some(img) = data["image/svg+xml"].as_array() {
						fs::create_dir_all(img_path).unwrap();
						let content = img.iter().map(|v| v.as_str().unwrap()).join("");
						let file_path = format!("{}/{}.svg", img_path, md::sha1(&content));
						let mut file = File::create(&file_path).unwrap();
						file.write_all(content.as_bytes()).unwrap();
						format!("#image(\"./{file_path}\")\n")
					} else if let Some(img) = data["image/png"].as_str() {
						// base 64 image data
						fs::create_dir_all(img_path).unwrap();
						let file_path = format!("{}/{}.png", img_path, md::sha1(img));
						let mut file = File::create(&file_path).unwrap();
						file.write_all(&STANDARD.decode(img).unwrap()).unwrap();
						format!("#image(\"./{file_path}\")\n")
					} else if let Some(text) = data["text/plain"].as_array() {
						format!(
							"#result-block(\"{}\")\n",
							typ::escape_string(&text.iter().map(|v| v.as_str().unwrap()).join("")),
						)
					} else if let Some(text) = data["text/latex"].as_array() {
						katex::text_to_typst(&text.iter().map(|v| v.as_str().unwrap()).join("").replace("$$", "$"))
							.unwrap()
					} else if let Some(text) = data["text/html"].as_array() {
						md::html_to_typst(text.iter().map(|v| v.as_str().unwrap()).join("").as_str())
					} else {
						"".into()
					}
				}
				"error" => format!(
					"#result-block(\"{}\")\n",
					output["traceback"]
						.as_array()
						.unwrap()
						.iter()
						.map(|v| v.as_str().unwrap())
						.join("\n"),
				),
				other => unreachable!("unhandled output type: {other}\n"),
			}
		})
		.join("")
}
