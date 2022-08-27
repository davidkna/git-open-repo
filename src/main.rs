fn main() {
    for arg in std::env::args_os().skip(1) {
        println!("Repo: {:?}", arg);
        println!("{:#?}", git_repository::open(arg));
    }
}
