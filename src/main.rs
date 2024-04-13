mod path;

use path::Path;

fn main() {
    println!("{}", Path::pwd(None));
    println!("{}", Path::pwd(Some(String::from("src/models"))));

    println!("{}", Path::bin(None));
    println!("{}", Path::bin(Some(String::from("main.rs"))));

    println!("{}", Path::src(None));
    println!("{}", Path::src(Some(String::from("models"))));
}
