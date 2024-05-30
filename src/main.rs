use std::fs::{self};
use std::path::Path;
use std::io;

fn main() -> io::Result<()> {
    let path = ".";
    visit_dirs(Path::new(path))?;
    println!("Script is done.");
    Ok(())
}

fn visit_dirs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(&path)?;
            }
            
            if let Some(file_name) = path.file_name() {
                if file_name.to_string_lossy().ends_with(":Zone.Identifier") {
                    println!("Deleting file: {:?}", path);
                    fs::remove_file(&path)?;
                }
            }
        }
    }
    Ok(())
}
