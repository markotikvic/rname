use rustyline::DefaultEditor;
use std::{env, fs, path::Path, process};

fn rename_file(source_path_str: &str) -> bool {
    let source_path = Path::new(source_path_str);
    if !source_path.exists() {
        eprintln!("source file '{}' does not exist", source_path_str);
        return false;
    }

    let mut editor = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(err) => {
            eprintln!("initializing default editor: {}", err);
            return false;
        }
    };

    let destination_path_str = editor
        .readline_with_initial("New name: ", (source_path_str, ""))
        .unwrap();

    let destination_path = Path::new(&destination_path_str);
    if destination_path.exists() {
        eprintln!(
            "can't overrwrite destination: file '{}' already exists",
            destination_path_str
        );
        return false;
    }

    let Ok(_) = fs::rename(source_path, destination_path) else {
        eprintln!("unable to rename file to '{}'", destination_path_str);
        return false;
    };

    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: rname <file>");
        process::exit(1);
    }

    if !rename_file(&args[1]) {
        process::exit(1);
    }
}
