
extern crate ansi_term;

use std::env::current_dir;
use std::fs::read_dir;
use std::path::Path;
use ansi_term::Colour::*;

fn print_dir(dir: &Path, dist: usize) -> (i32, i32) {
    if dist >= 3 { return (0,0); }
    let mut file_count = 0;
    let mut dir_count = 0;
    let dir_result = read_dir(dir).unwrap();
    let size = read_dir(dir).unwrap().count();
    for (i, entry) in dir_result.enumerate() {
        let path = entry.unwrap().path(); //DirEntry -> PathBuf
        let file_name = path.as_path().file_name().unwrap().to_str().unwrap(); //PathBuf -> Path -> OsStr -> &str
        if file_name.starts_with(".") { continue; } //ignore hidden files
        let gap = std::iter::repeat(" ").take(dist*3).collect::<String>();
        let mut vert;
        if size > 0 && i != size-1 { vert = "|"; }
        else { vert = "`"; }
        if path.is_dir() {
            println!("{}{}---{}", gap, vert, Blue.bold().paint(file_name));
            dir_count+=1;
            let (rec_dir, rec_file) = print_dir(&path, dist+1);
            dir_count += rec_dir;
            file_count += rec_file;
        }
        else {
            println!("{}{}---{}", gap, vert, file_name);
            file_count+=1;
        }
    }
    (dir_count, file_count)
}

fn main() {
    let pwd = current_dir().unwrap();
    println!("."); //print pwd as .
    let (dir_count, file_count) = print_dir(&pwd.as_path(), 0);
    println!("\n{} directories, {} files", dir_count, file_count);
}
