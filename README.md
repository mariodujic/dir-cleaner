# Info

Removes duplicate files across all sub-directories of a given folder.

# How it works

Scans a target directory and its subdirectories to identify and remove duplicate files. It compares files based on their
name and size, and if two files with identical names and sizes are found, it checks their contents and removes the
duplicate file if they match.

# Usage

```
git clone https://github.com/mariodujic/dir-cleaner
cd dir-cleaner
cargo run <target_directory_path>
```
