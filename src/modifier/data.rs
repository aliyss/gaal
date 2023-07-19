pub fn init(dir: String, make_dir: fn(String) -> bool) {
    make_dir(dir);
}
