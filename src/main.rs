
use std::cmp::Ordering;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

use docopt::{self, Docopt};
use lazy_static::lazy_static;
use lscolors::{LsColors, Style};
use nu_ansi_term::AnsiGenericString;
use serde_derive::{Deserialize};


lazy_static!{
    static ref LSCOLORS: LsColors = {
        LsColors::from_env().unwrap_or_default()
    };
}


//sorts PathBufs lexicgraphically by filename
fn pathbuf_sort_fn(a: &PathBuf, b: &PathBuf) -> Ordering {
    let a_file = a.as_path().file_name().unwrap().to_str().unwrap();
    let b_file = b.as_path().file_name().unwrap().to_str().unwrap();
    a_file.cmp(&b_file)
}

// builds prefix of spaces and continuing vertical lines
fn build_prefix(verts: &Vec<bool>) -> String {
    let mut result: String = String::new();
    for entry in verts {
        if *entry == true {
            result.push_str("\u{2502}   "); // vert space space space
        } else {
            result.push_str("    "); // space space space space
        }
    }
    result
}

fn colorize(path: &PathBuf) -> AnsiGenericString<str> {
    let file_name = path.as_path().file_name().unwrap_or(path.as_os_str()).to_str().unwrap(); //PathBuf -> Path -> OsStr -> &str
    let style = LSCOLORS.style_for_path(path);
    let ansi_style = style.map(Style::to_nu_ansi_term_style).unwrap_or_default();
    ansi_style.paint(file_name)
}

fn get_paths(dir: &Path) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = read_dir(dir)
        .unwrap()
        .map(|res| res.unwrap().path())
        .collect();
    paths.sort_by(|a, b| pathbuf_sort_fn(&a, &b));
    paths
}

//recursive tree print
fn tree_dir(dir: &Path, dist: usize, verts: &mut Vec<bool>, depth_limit: usize) -> (i32, i32) {
    if dist >= depth_limit {
        return (0, 0);
    }

    let mut file_count = 0;
    let mut dir_count = 0;

    let paths: Vec<PathBuf> = get_paths(dir);
    let size = paths.len();

    let prefix = build_prefix(&verts);

    for (i, path) in paths.iter().enumerate() {
        let file_name = path.as_path().file_name().unwrap().to_str().unwrap(); //PathBuf -> Path -> OsStr -> &str

        //ignore hidden files
        if file_name.starts_with(".") {
            continue;
        }

        //determine vertical line before filename => vert
        let mut vert = "\u{2514}"; // right angle up/right
        if size > 0 && i != size - 1 {
            // not the last in the folder
            vert = "\u{251c}"; // vertical T right
        }

        if path.is_dir() {
            dir_count += 1;

            println!(
                "{}{}\u{2500}\u{2500} {}",
                prefix,
                vert,
                colorize(path)
            );

            //setup next continuation lines
            if size > 0 && i != size - 1 {
                verts.push(true);
            } else {
                verts.push(false);
            }

            //recurse and sum counts
            let (rec_dir, rec_file) = tree_dir(path.as_path(), dist + 1, verts, depth_limit);
            verts.pop();
            dir_count += rec_dir;
            file_count += rec_file;
        } else {
            // path.is_file() == true
            println!("{}{}\u{2500}\u{2500} {}", prefix, vert, colorize(path)); //dash dash
            file_count += 1;
        }
    }
    (dir_count, file_count)
}

// recursive driver
fn print_tree(dir: &Path, depth_limit: usize) -> (i32, i32) {
    let mut verts: Vec<bool> = Vec::new();
    tree_dir(dir, 0, &mut verts, depth_limit)
}

const USAGE: &'static str = "
rusty_tree - an subset of the traditional tree file lister

Usage:
  rusty_tree [options] [<dir>...]
  rusty_tree (--help | --version)

Options:
  -h --help                 Show this screen.
  --version                 Show version.
  -d, --depth <depth>       Depth limit of directories shown [default: 64].
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_dir: Option<Vec<PathBuf>>,
    flag_depth: usize,
}

fn version() -> String {
    let (maj, min, pat) = (
        option_env!("CARGO_PKG_VERSION_MAJOR"),
        option_env!("CARGO_PKG_VERSION_MINOR"),
        option_env!("CARGO_PKG_VERSION_PATCH"),
    );
    match (maj, min, pat) {
        (Some(maj), Some(min), Some(pat)) => format!("rusty_tree v{}.{}.{}", maj, min, pat),
        _ => "rusty_tree vUnknown".to_owned(),
    }
}

// main executable
fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some(version())).deserialize())
        .unwrap_or_else(|e| e.exit());

    let dir_default = PathBuf::from(".");
    let dirs = args.arg_dir.unwrap_or(vec![dir_default]);
    let depth = args.flag_depth;

    let mut total_dir_count = 0;
    let mut total_file_count = 0;

    for dir in dirs.iter() {
        println!("{}", colorize(dir));
        let (dir_count, file_count) = print_tree(dir, depth);
        total_dir_count += dir_count;
        total_file_count += file_count;
    }
    println!(
        "\n{} directories, {} files",
        total_dir_count, total_file_count
    );
}
