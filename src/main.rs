use std::{env, fs, path::Path};

fn rename_file(source_path_str: &str) -> Result<(), String> {
    let source_path = Path::new(source_path_str);
    if !source_path.exists() {
        return Err(format!("file {} does not exist", source_path_str));
    }

    let mut editor = match rustyline::DefaultEditor::new() {
        Ok(editor) => editor,
        Err(err) => {
            return Err(format!("initializing default editor: {}", err));
        }
    };

    let destination_path_str = editor
        .readline_with_initial("New name: ", (source_path_str, ""))
        .unwrap();

    let destination_path = Path::new(&destination_path_str);
    if destination_path.exists() {
        return Err(format!("file '{}' already exists", destination_path_str));
    }

    let Ok(_) = fs::rename(source_path, destination_path) else {
        return Err(format!(
            "unable to rename file to '{}'",
            destination_path_str
        ));
    };

    Ok(())
}

fn main() -> Result<(), i32> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: rname <file>");
        return Err(1);
    }

    match rename_file(&args[1]) {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("rname: {}", err);
            Err(1)
        }
    }
}
