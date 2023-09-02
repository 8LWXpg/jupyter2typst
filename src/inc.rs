use serde_json::Value;

pub fn ipynb_parse(json: Value) -> String {
    let mut output = String::new();

    for cell in json["cells"].as_array().unwrap() {
        let cell_type = cell["cell_type"].as_str().unwrap();
        let source = cell["source"].as_str().unwrap();
        match cell_type {
            "markdown" => {
                output.push_str(&md_to_typst(source));
            }
            "code" => {
                output.push_str(&code_parse(source));
                let outputs = cell["outputs"].as_array().unwrap();
                output.push_str(&code_output_parse(outputs));
            }
            _ => panic!("Invalid cell type"),
        };
    }

    output
}

fn md_to_typst(md: &str) -> String {
    let mut output = String::new();

    // TODO

    output
}

fn code_parse(code: &str) -> String {
    let mut output = String::new();

    // TODO https://nbformat.readthedocs.io/en/latest/format_description.html#code-cells

    output
}

fn code_output_parse(code: &Vec<Value>) -> String {
    let mut output = String::new();

    // TODO https://nbformat.readthedocs.io/en/latest/format_description.html#code-cell-outputs

    output
}
