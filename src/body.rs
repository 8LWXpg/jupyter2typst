use crate::IMG_PATH;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use once_cell::sync::OnceCell;
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

#[path = "md.rs"]
mod md;
#[path = "typ.rs"]
mod typ;

#[path = "katex.rs"]
mod katex;

static LANG: OnceCell<String> = OnceCell::new();
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
                let extension = name.split(".").last().unwrap();
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
        output.push_str("#block[\n");
        match cell["cell_type"].as_str().unwrap() {
            "markdown" => {
                output.push_str(&md::md_to_typst(
                    cell["source"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| v.as_str().unwrap())
                        .collect::<Vec<&str>>(),
                    attachments,
                ));
            }
            "code" => {
                output.push_str(&code_parse(
                    cell["source"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| v.as_str().unwrap())
                        .collect::<Vec<&str>>(),
                    cell["execution_count"].as_i64().unwrap(),
                ));
                output.push_str("\n]\n#block[\n");
                output.push_str(&code_output_parse(
                    cell["outputs"].clone(),
                    IMG_PATH.get().unwrap(),
                ));
            }
            _ => {}
        };
        output.push_str("]\n");
    }

    output
}

fn code_parse(code: Vec<&str>, count: i64) -> String {
    let mut context = String::new();

    context.push_str("#code-block(\"");
    context.push_str(&typ::escape_string(code.join("")));
    context.push_str(
        format!(
            "\"\n, lang: \"{}\", count: {})\n",
            LANG.get().unwrap(),
            count
        )
        .as_str(),
    );

    context
}

fn code_output_parse(outputs: Value, img_path: &str) -> String {
    let mut context = String::new();

    for output in outputs.as_array().unwrap() {
        match output["output_type"].as_str().unwrap() {
            "stream" => context.push_str(&format!(
                "#result-block(\"{}\")\n",
                output["text"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap())
                    .collect::<Vec<&str>>()
                    .join("")
            )),
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
                    let mut file = File::create(file_path.clone()).unwrap();
                    file.write_all(content.as_bytes()).unwrap();
                    context.push_str(format!("#image(\"./{}\")", file_path).as_str())
                } else if let Some(img) = data["image/png"].as_str() {
                    fs::create_dir_all(img_path).unwrap();
                    let content = img;
                    let file_path = format!("{}/{}.png", img_path, md::sha1(content));
                    let mut file = File::create(file_path.clone()).unwrap();
                    file.write_all(&STANDARD.decode(img).unwrap()).unwrap();
                    context.push_str(format!("#image(\"./{}\")", file_path).as_str())
                } else if let Some(text) = data["text/plain"].as_array() {
                    context.push_str(&format!(
                        "#result-block(\"{}\")\n",
                        typ::escape_string(
                            text.iter()
                                .map(|v| v.as_str().unwrap())
                                .collect::<Vec<&str>>()
                                .join("")
                        ),
                    ))
                } else if let Some(text) = data["text/latex"].as_array() {
                    // TODO only support KaTeX currently
                    context.push_str(&katex::latex_text_to_typst(
                        text.iter()
                            .map(|v| v.as_str().unwrap())
                            .collect::<Vec<&str>>()
                            .join("")
                            .replace("$$", "$"),
                    ))
                } else if let Some(text) = data["text/html"].as_array() {
                    // TODO test html
                    context.push_str(&md::html_to_typst(
                        text.iter()
                            .map(|v| v.as_str().unwrap())
                            .collect::<Vec<&str>>()
                            .join("")
                            .as_str(),
                    ))
                }
            }
            "error" => context.push_str(&format!(
                "#result-block(\"{}\")\n",
                output["traceback"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap())
                    .collect::<Vec<&str>>()
                    .join(""),
            )),
            other => {
                println!("unhandled output type: {other}\n");
            }
        }
    }

    context
}
