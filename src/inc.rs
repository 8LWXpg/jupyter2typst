use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde_json::Value;
use sha1::{Digest, Sha1};
use std::{
    fs::{self, File},
    io::Write,
};

pub fn ipynb_parse(json: Value, img_path: &str) -> String {
    let mut output = String::new();

    for cell in json["cells"].as_array().unwrap() {
        output.push_str("#block[\n");
        match cell["cell_type"].as_str().unwrap() {
            "markdown" => {
                output.push_str(&md_to_typst(
                    cell["source"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| v.as_str().unwrap())
                        .collect::<Vec<&str>>(),
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
                output.push_str(&code_output_parse(cell["outputs"].clone(), img_path));
            }
            _ => {}
        };
        output.push_str("\n]\n");
    }

    output
}

fn md_to_typst(md: Vec<&str>) -> String {
    // TODO convert basic markdown to typst in test4
    // TODO download url image
    // TODO make blockquote function in typst
    // TODO escape latex by `$$`
    // TODO escape html by `<` and `>`

    md.join("")
}

fn code_parse(code: Vec<&str>, count: i64) -> String {
    // TODO https://nbformat.readthedocs.io/en/latest/format_description.html#code-cells

    code.join("")
}

fn code_output_parse(outputs: Value, img_path: &str) -> String {
    let mut ret = String::new();

    fn sha1(s: &str) -> String {
        let mut sha1 = Sha1::new();
        sha1.update(s);
        sha1.finalize()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }

    for output in outputs.as_array().unwrap() {
        match output["output_type"].as_str().unwrap() {
            "stream" => ret.push_str(
                output["text"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap())
                    .collect::<Vec<&str>>()
                    .join("")
                    .as_str(),
            ),
            "display_data" | "execute_result" => {
                let data = output["data"].clone();
                if let Some(img) = data["image/svg+xml"].as_array() {
                    fs::create_dir_all(img_path).unwrap();
                    let content = img
                        .iter()
                        .map(|v| v.as_str().unwrap())
                        .collect::<Vec<&str>>()
                        .join("");
                    let file_path = format!("{}/{}.svg", img_path, sha1(&content));
                    let mut file = File::create(file_path.clone()).unwrap();
                    let _ = file.write_all(content.as_bytes());
                    ret.push_str(format!("#image(\"./{}\")", file_path).as_str())
                } else if let Some(img) = data["image/png"].as_str() {
                    fs::create_dir_all(img_path).unwrap();
                    let content = img;
                    let file_path = format!("{}/{}.png", img_path, sha1(content));
                    let mut file = File::create(file_path.clone()).unwrap();
                    let _ = file.write_all(&STANDARD.decode(img).unwrap());
                    ret.push_str(format!("#image(\"./{}\")", file_path).as_str())
                } else if let Some(text) = data["text/plain"].as_array() {
                    ret.push_str(
                        text.iter()
                            .map(|v| v.as_str().unwrap())
                            .collect::<Vec<&str>>()
                            .join("")
                            .as_str(),
                    )
                } else if let Some(text) = data["text/html"].as_array() {
                    ret.push_str(
                        text.iter()
                            .map(|v| v.as_str().unwrap())
                            .collect::<Vec<&str>>()
                            .join("")
                            .as_str(),
                    )
                } else if let Some(text) = data["text/latex"].as_array() {
                    ret.push_str(
                        text.iter()
                            .map(|v| v.as_str().unwrap())
                            .collect::<Vec<&str>>()
                            .join("")
                            .as_str(),
                    )
                }
            }
            "error" => ret.push_str(
                output["traceback"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap())
                    .collect::<Vec<&str>>()
                    .join("")
                    .as_str(),
            ),
            _ => {
                panic!("Unknown output type")
            }
        }
    }

    ret
}
