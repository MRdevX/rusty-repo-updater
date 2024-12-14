use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::path::Path;

fn main() {
    // Ask for the directory path
    print!("Enter the directory path: ");
    io::stdout().flush().unwrap();

    let mut dir_path = String::new();
    io::stdin().read_line(&mut dir_path).unwrap();
    let dir_path = dir_path.trim();

    // Check if the provided path is valid
    let path = Path::new(dir_path);
    if !path.is_dir() {
        eprintln!("The provided path is not a valid directory.");
        return;
    }

    println!("Starting to iterate through subdirectories in {:?}", path);

    // Iterate through each subdirectory
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            println!("Checking directory: {:?}", path);
            let git_dir = path.join(".git");
            if git_dir.exists() {
                println!("Found git repository in {:?}", path);
                let branches = ["develop", "main", "master"];
                for branch in &branches {
                    println!("Fetching branch '{}' in {:?}", branch, path);
                    let output = Command::new("git")
                        .arg("fetch")
                        .arg("-p")
                        .arg("origin")
                        .arg(branch)
                        .current_dir(&path)
                        .output()
                        .expect("Failed to execute git fetch -p");
                    println!("Fetch output for branch '{}':\n{}", branch, String::from_utf8_lossy(&output.stdout));
                    println!("Fetching errors for branch '{}':\n{}", branch, String::from_utf8_lossy(&output.stderr));

                    println!("Pulling branch '{}' in {:?}", branch, path);
                    let output = Command::new("git")
                        .arg("pull")
                        .arg("origin")
                        .arg(branch)
                        .current_dir(&path)
                        .output()
                        .expect("Failed to execute git pull");
                    println!("Pull output for branch '{}':\n{}", branch, String::from_utf8_lossy(&output.stdout));
                    println!("Pull errors for branch '{}':\n{}", branch, String::from_utf8_lossy(&output.stderr));
                }
                println!("Finished updating {:?}", path);
            } else {
                println!("{:?} is not a git repository", path);
            }
        } else {
            println!("{:?} is not a directory", path);
        }
    }

    println!("Finished iterating through subdirectories.");
}