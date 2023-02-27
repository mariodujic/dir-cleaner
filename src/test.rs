use std::fs::{create_dir, File, remove_dir_all};
use std::path::Path;

use crate::clear_folder_duplicates;

#[test]
fn should_travers_dir_and_remove_duplicate_file() {
    create_dir("test-dir").unwrap();
    File::create("test-dir/file.txt").unwrap();
    create_dir("test-dir/nested-dir").unwrap();
    File::create("test-dir/nested-dir/file.txt").unwrap();

    let root_file_exists = Path::new("test-dir/file.txt").exists();
    let nested_file_exists = Path::new("test-dir/nested-dir/file.txt").exists();
    assert!(root_file_exists == true && nested_file_exists == true);

    clear_folder_duplicates(&String::from("test-dir"));

    let root_file_exists = Path::new("test-dir/file.txt").exists();
    let nested_file_exists = Path::new("test-dir/nested-dir/file.txt").exists();
    assert_ne!(root_file_exists, nested_file_exists);

    remove_dir_all("test-dir").unwrap()
}

#[test]
fn should_travers_dir_and_not_remove_files_with_same_name_and_different_content() {
    use std::fs::File;
    use std::io::prelude::*;

    create_dir("test-dir-sec").unwrap();
    File::create("test-dir-sec/file.txt").unwrap();
    create_dir("test-dir-sec/nested-dir").unwrap();
    let mut nested_file = File::create("test-dir-sec/nested-dir/file.txt").unwrap();
    nested_file.write_all("Hello world".as_bytes()).unwrap();

    clear_folder_duplicates(&String::from("test-dir-sec"));

    let root_file_exists = Path::new("test-dir-sec/file.txt").exists();
    let nested_file_exists = Path::new("test-dir-sec/nested-dir/file.txt").exists();
    assert!(root_file_exists == true && nested_file_exists == true);

    remove_dir_all("test-dir-sec").unwrap()
}