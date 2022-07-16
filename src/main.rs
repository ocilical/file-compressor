use clap::{Parser, clap_derive::ArgEnum};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Decompress file
    #[clap(short, long, action)]
    decompress: bool,

    /// Output filepath
    #[clap(short, long, value_parser)]
    output: Option<String>,

    /// Compression algorithm to use
    #[clap(short, long, value_parser, default_value = "huffman")]
    algorithm: Algorithm,

    /// Input filepath
    #[clap(value_parser)]
    input: String,
}

#[derive(Debug, Clone, ArgEnum)]
enum Algorithm {
    Huffman,
    LZW,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
