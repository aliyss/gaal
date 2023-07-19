mod modifier;

use modifier::parser;

fn make_dir(dir: String) -> bool {
    println!("{dir}");
    true
}

fn main() {
    /*
    Using main for tests at the moment.
    */
    parser::init(parser::Config { git_dir: None }, make_dir);
}
