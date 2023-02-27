use std::{
    env,
    fmt::{Debug, Formatter, Result},
    fs::{read, remove_file},
    ops::Add,
};
use std::cmp::Ordering;
use std::fs::{create_dir, remove_dir_all};
use std::io::Write;
use std::path::Path;

use dialoguer::Confirm;
use dialoguer::console::TermFamily::File;
use walkdir::WalkDir;

#[derive(Eq, PartialEq)]
struct FileInfo {
    name: String,
    path: String,
    size: u64,
}

impl PartialOrd for FileInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FileInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        let name_cmp = self.name.cmp(&other.name);
        if name_cmp == Ordering::Equal {
            self.size.cmp(&other.size)
        } else {
            name_cmp
        }
    }
}

impl Debug for FileInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("FileInfo")
            .field("name", &self.name)
            .field("path", &self.path)
            .field("size", &self.size.to_string().add(" kb"))
            .finish()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    if Confirm::new()
        .with_prompt("Do you want to continue?")
        .interact()
        .unwrap()
    {
        println!("Searching duplicate files..");
        if clear_folder_duplicates(path) {
            println!("Removed duplicate items.");
        } else {
            println!("No duplicate files found.");
        }
    } else {
        println!("Nevermind then.");
    }
}

fn clear_folder_duplicates(path: &String) -> bool {
    let mut dir_files: Vec<FileInfo> = WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| {
            let file_name = entry.file_name().to_str().unwrap_or_default().to_string();
            let path = entry.path().to_str().unwrap_or_default().to_string();
            let file_size = entry.metadata().unwrap().len();
            FileInfo {
                name: file_name,
                path: path,
                size: file_size,
            }
        })
        .collect();

    dir_files.sort();

    let max_index = dir_files.len() - 1;
    let mut cleared_items = 0;
    for (index, file) in dir_files.iter().enumerate() {
        if index < max_index {
            let next_file = dir_files.get(index + 1).unwrap();
            if file.name == next_file.name && file.size == next_file.size {
                let file_content = read(&file.path);
                let next_file_content = read(&next_file.path);
                let files_read = file_content.is_ok() && next_file_content.is_ok();

                if files_read && file_content.unwrap() == next_file_content.unwrap() {
                    println!("{:?}\n", &next_file);
                    cleared_items += 1;
                    remove_file(&next_file.path).unwrap();
                }
            }
        }
    }
    cleared_items != 0
}

#[cfg(test)]
mod test;
