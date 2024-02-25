use rustyline::DefaultEditor;
use std::{env, fs, path::Path, process};

fn rename_file(target: &str) -> bool {
    let source_path = Path::new(target);
    if !source_path.exists() {
        eprintln!("source file '{}' does not exist", target);
        return false;
    }

    let mut editor = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(err) => {
            eprintln!("initializing default editor: {}", err);
            return false;
        }
    };

    let dest = editor
        .readline_with_initial("New name: ", (target, ""))
        .unwrap();

    let dest_path = Path::new(&dest);
    if dest_path.exists() {
        eprintln!(
            "can't overrwrite destination: file '{}' already exists",
            dest
        );
        return false;
    }

    if let Err(err) = fs::rename(source_path, dest_path) {
        eprintln!("unable to rename file: {}", err);
        return false;
    }

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
