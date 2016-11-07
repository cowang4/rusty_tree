
use std::env;
use std::fs;
use std::path::Path;

fn print_dir(dir: &Path, dist: usize) -> () {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.as_path().file_name().unwrap();
        let gap = std::iter::repeat(" ").take(dist*3).collect::<String>();
        println!("{}|---{}", gap, file_name.to_str().unwrap());
        if path.is_dir() {
            print_dir(&path, dist+1);
        }
    }
}

fn main() {
    let pwd = env::current_dir().unwrap();
    println!("."); //print pwd as .
    print_dir(&pwd.as_path(), 0);
}
