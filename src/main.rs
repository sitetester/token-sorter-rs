mod token_sorter;

use crate::token_sorter::TokenSorter;
use clap::{arg, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./data/in")]
    in_path: String,

    #[arg(short, long, default_value = "./data/out")]
    out_path: String,

    #[arg(short, long, default_value = "name")]
    sort: String,
}

const SORT_FIELD_NAME: &str = "name";
const SORT_FIELD_ADDRESS: &str = "address";

fn main() {
    let args = Args::parse();
    let field = &args.sort;
    if !TokenSorter::is_valid_sort_field(field) {
        panic!("Only `name` or `address` could be used for sorting");
    }

    let mut ts = TokenSorter {
        in_path: args.in_path,
        out_path: args.out_path,
        field: args.sort,
    };
    match ts.sort() {
        Ok(()) => {}
        Err(e) => println!("{}", e),
    }
}
