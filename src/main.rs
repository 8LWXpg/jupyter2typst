use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "A simple tool to convert a Jupyter Notebook to Typst format")]
struct Args {
    /// the input file
    #[argh(option)]
    input: String,

    /// the output file name
    #[argh(option)]
    output: String,
}

fn main() {
    let args: Args = argh::from_env();
}
