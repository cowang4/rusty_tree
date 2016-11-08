
extern crate ansi_term;

use std::env::current_dir;
use std::fs::read_dir;
use std::path:: { Path, PathBuf };
use ansi_term::Colour::*;
use std::cmp::Ordering;


//sorts PathBufs lexicgraphically by filename
fn sort_fn(a: &PathBuf, b: &PathBuf) -> Ordering {
    let a_file = a.as_path().file_name().unwrap().to_str().unwrap();
    let b_file = b.as_path().file_name().unwrap().to_str().unwrap();
    a_file.cmp(&b_file)
}

// builds prefix of spaces and continuing vertical lines
fn build_prefix(verts: &Vec<bool>) -> String {
    let mut result: String = String::new();
    for entry in verts {
        if *entry == true { result.push_str("\u{2502}  "); } // vert space space
        else { result.push_str("    "); } // space space space
    }
    result
}

//recursive tree print
fn tree_dir(dir: &Path, dist: usize, verts: &Vec<bool>, depth_limit: usize) -> (i32, i32) {
    if dist >= depth_limit { return (0,0); }
    let mut file_count = 0;
    let mut dir_count = 0;
    let mut paths: Vec<PathBuf> = read_dir(dir).unwrap().map(|res| res.unwrap().path()).collect();
    paths.sort_by(|a,b| sort_fn(&a,&b));
    let size = paths.len();
    for (i, path) in paths.iter().enumerate() {
        let file_name = path.as_path().file_name().unwrap().to_str().unwrap(); //PathBuf -> Path -> OsStr -> &str
        if file_name.starts_with(".") { continue; } //ignore hidden files
        let prefix = build_prefix(verts);
        //determine vertical line before filename
        let mut vert = "\u{2514}"; // right angle up/right
        if size > 0 && i != size-1 { vert = "\u{251c}"; } // vertical tee right
        if path.is_dir() {
            //determine whether or not to make a children branch
            let mut child_chr = "\u{2500}"; // horizontal
            if dist != depth_limit-1 && read_dir(&path).unwrap().nth(0).is_some() { child_chr = "\u{252c}" } // horiz tee down
            //print
            println!("{}{}\u{2500}{} {}", prefix, vert, child_chr, Blue.bold().paint(file_name));
            dir_count+=1;
            //setup next continuation lines
            let mut new_verts = verts.clone();
            if size > 0 && i != size -1 { new_verts.push(true); }
            else { new_verts.push(false); }
            //recurse and sum counts
            let (rec_dir, rec_file) = tree_dir(path.as_path(), dist+1, &new_verts, depth_limit);
            dir_count += rec_dir;
            file_count += rec_file;
        }
        else { // path.is_file()
            println!("{}{}\u{2500}\u{2500} {}", prefix, vert, file_name);
            file_count+=1;
        }
    }
    (dir_count, file_count)
}

//handler for overloaded fn
fn print_tree(dir: &Path, depth_limit: usize) -> (i32, i32) {
    let verts: Vec<bool> = Vec::new();
    tree_dir(dir, 0, &verts, depth_limit)
}

// main executable
fn main() {
    println!("{}", Blue.bold().paint(".")); //print pwd as .
    let pwd = current_dir().unwrap(); // get pwd
    let (dir_count, file_count) = print_tree(&pwd.as_path(), usize::max_value());
    println!("\n{} directories, {} files", dir_count, file_count); // print counts
}
