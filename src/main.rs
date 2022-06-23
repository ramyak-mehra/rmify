use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::{env, fs, io, path::Path};
fn main() {
    let mut args = env::args();
    let _ = args.next();

    let (sender, rcv) = mpsc::channel();

    match args.next() {
        Some(dir) => {
            let path = Path::new(&dir);
            let _ = check_rust_dir(path, sender);
        }
        None => {
            let path = Path::new("../");
            let _ = check_rust_dir(path, sender);
        }
    };

    let handle = thread::spawn(move || {
        for r in rcv.iter() {
            let result = delete_target_dir(r);
            if result.is_err() {
                println!("error deleting {:?}", result.err());
            }
        }
    });
    handle.join().unwrap();
}

fn delete_target_dir(path: PathBuf) -> Result<(), io::Error> {
    let target_dir = path.join("target");

    if target_dir.exists() {
        println!(
            "Do you want to delete the target folder in {}? [y/n]",
            path.display()
        );
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer)?;
        let input = buffer.trim();
        if input == "y" || input == "Y" {
            println!("Deleting {}", target_dir.display());
            let attr = fs::metadata(&target_dir)?;
            println!("dir size :{}" , attr.len());
            fs::remove_dir_all(&target_dir)?;
        }
    }
    Ok(())
}

fn check_rust_dir(dir_path: &Path, sender: mpsc::Sender<PathBuf>) -> Result<(), io::Error> {
    let dir_contents = fs::read_dir(dir_path)?;
    let mut dirs = Vec::new();
    let mut found = false;
    for entris in dir_contents {
        let entry = entris?;
        if entry.file_name() == "Cargo.toml" {
            found = true;
            break;
        }
        if entry.path().is_dir() {
            dirs.push(entry.path());
        }
    }
    if found {
        println!("Found Cargo.toml in {}", dir_path.display());
        dirs.clear();
        let _ = sender.send(dir_path.to_path_buf());
    }
    for dir in dirs {
        check_rust_dir(&dir, sender.clone())?;
    }
    Ok(())
}
