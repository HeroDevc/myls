use std::{env, fs::{self, DirEntry}, path::{Path, PathBuf}};

fn get_file_count(path: &str) -> i32 {
    let is_valid_path = PathBuf::from(path);

    if is_valid_path.as_os_str().to_str().is_some() && is_valid_path.exists() {
        let count = fs::read_dir(path).expect("Expected ReadDir")
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.metadata()
                .map(|meta| meta.is_file())
                .unwrap_or(false)
        })
        .count();

        return count as i32
    }

    0
}

fn get_folder_count(path: &str) -> i32 {
    let is_valid_path = PathBuf::from(path);

    if is_valid_path.as_os_str().to_str().is_some() && is_valid_path.exists() {
        let count = fs::read_dir(path).expect("Expected ReadDir")
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.metadata()
                .map(|meta| meta.is_dir())
                .unwrap_or(false)
        })
        .count();

        return count as i32
    }

    0
}

fn remove_first_n_chars(mut text: String, mut n: i32) -> String {
    while n > 0 {
        text.remove(0);

        n -= 1;
    }

    if text.starts_with("\\") || text.starts_with("/") {
        text.remove(0);
    }

    text
}

fn display_default(path: &str) {
    let is_valid_path = PathBuf::from(path);

    if is_valid_path.as_os_str().to_str().is_some() && is_valid_path.exists() {
        let path_res: Result<fs::ReadDir, std::io::Error> = fs::read_dir(path);

        let mut final_string = String::new();

        match path_res {
            Ok(paths) => {
                for res_path in paths {
                    match res_path {
                        Ok(dir) => {
                            let file_metadata = dir.metadata();

                            match file_metadata {
                                Ok(metadata) => {
                                    if metadata.is_file() {
                                        final_string += &format!("{} ", remove_first_n_chars(dir.path().display().to_string(), path.len() as i32));
                                    } else {
                                        final_string += &format!("/{} ", remove_first_n_chars(dir.path().display().to_string(), path.len() as i32));
                                    }
                                },
                                Err(_e) => {
                                    // println!("{}", e);
                                }
                            }
                        },
                        Err(_e) => {
                            // println!("{}", e);
                        }
                    }
                }
            },
            Err(_e) => {
                // println!("{}", e);
            }
        }

        println!("{}", final_string);
    }
}

fn format_size(bytes: f32) -> String {
    // checking if the value is lower than 1KB
    if (bytes / 1024 as f32) < 1.0 {
        return format!("{}B", bytes).to_string()
        // checking if the value is lower than 1MB
    } else if ((bytes / 1024 as f32) / 1024 as f32) < 1.0 {
        return format!("{:.2}KB", bytes / 1024 as f32)
        // checking if the value is lower than 1GB
    } else if (((bytes / 1024 as f32) / 1024 as f32) / 1024 as f32) < 1.0 {
        return format!("{:.2}MB", bytes / 1024 as f32 / 1024 as f32)
        // checking if the value is lower than 1TB
    } else if ((((bytes / 1024 as f32) / 1024 as f32) / 1024 as f32) / 1024 as f32) < 1.0 {
        return format!("{:.2}GB", bytes / 1024 as f32 / 1024 as f32 / 1024 as f32)
        // checking if the value is lower than 1PB
    } else if (((((bytes / 1024 as f32) / 1024 as f32) / 1024 as f32) / 1024 as f32) / 1024 as f32) < 1.0 {
        return format!("{:.2}TB", bytes / 1024 as f32 / 1024 as f32 / 1024 as f32 / 1024 as f32)
    }

    format!("{}B", bytes).to_string()
}

fn scan_dir(path: &Path) -> f32 {
    // println!("path to read: {}", path);

    let mut bytes: u64 = 0;

    let path_res: Result<fs::ReadDir, std::io::Error> = fs::read_dir(path);

    let mut unchecked_folders: Vec<DirEntry> = Vec::new();

    match path_res {
        Ok(paths) => {
            for res_path in paths {
                match res_path {
                    Ok(dir) => {
                        let file_metadata = dir.metadata();

                        match file_metadata {
                            Ok(metadata) => {
                                if metadata.is_file() {
                                    bytes += metadata.len();
                                } else {
                                    unchecked_folders.push(dir);
                                }
                            },
                            Err(_e) => {
                                // println!("{}", e);
                            }
                        }
                    },
                    Err(_e) => {
                        // println!("{}", e);
                    }
                }
            }
        },
        Err(_e) => {
            // println!("{}", e);
        }
    }

    while unchecked_folders.len() > 0 {
        let file = unchecked_folders.remove(0);

        let path_data = fs::read_dir(file.path());

        match path_data {
            Ok(readdir) => {
                for item in readdir {
                    match item {
                        Ok(entry) => {
                            let file_metadata = entry.metadata();

                            match file_metadata {
                                Ok(metadata) => {
                                    if metadata.is_file() {
                                        bytes += metadata.len();
                                    } else {
                                        unchecked_folders.push(entry);
                                    }
                                },
                                Err(_e) => {
                                    // println!("{}", e);
                                }
                            }
                        },
                        Err(_e) => {
                            // println!("{}", e);
                        }
                    }
                }
            },
            Err(_e) => {
                // println!("{}", e);
            }
        }
    }

    bytes as f32
}

fn display_formatted(path: &str, recursive_folder_scan: bool) {
    let is_valid_path = PathBuf::from(path);

    if is_valid_path.as_os_str().to_str().is_some() && is_valid_path.exists() {
        let path_res: Result<fs::ReadDir, std::io::Error> = fs::read_dir(path);

        match path_res {
            Ok(paths) => {
                for res_path in paths {
                    match res_path {
                        Ok(dir) => {
                            let file_metadata = dir.metadata();

                            match file_metadata {
                                Ok(metadata) => {
                                    if metadata.is_file() {
                                        println!("{:<10} {}", format_size(metadata.len() as f32), remove_first_n_chars(dir.path().display().to_string(), path.len() as i32));
                                    } else {
                                        // println!("{:<10} /{}", format_size(metadata.len() as f32), remove_first_n_chars(dir.path().display().to_string(), path.len() as i32));
                                        if recursive_folder_scan {
                                            println!("{:<10} /{}", format_size(scan_dir(dir.path().as_path())), remove_first_n_chars(dir.path().display().to_string(), path.len() as i32));
                                        } else {
                                            println!("{:<10} /{}", format_size(metadata.len() as f32), remove_first_n_chars(dir.path().display().to_string(), path.len() as i32));
                                        }
                                    }
                                },
                                Err(_e) => {
                                    // println!("{}", e);
                                }
                            }
                        },
                        Err(_e) => {
                            // println!("{}", e);
                        }
                    }
                }
            },
            Err(_e) => {
                // println!("{}", e);
            }
        }

    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut ls_dir = "./";

    let mut formatted_display = false;
    let mut recursive_folder_scan = false;

    let mut i = 0;
    for item in args.iter() {
        if !item.starts_with("-") && i == 1 {
            ls_dir = &args[i];
        }

        if item.to_lowercase() == "-l" {
            formatted_display = true;
        }

        if item.to_lowercase() == "-r" {
            recursive_folder_scan = true;
        }

        i += 1;
    }

    let file_count = get_file_count(ls_dir);
    let folder_count = get_folder_count(ls_dir);

    println!("Total {}: {} files, {} folders", file_count + folder_count, file_count, folder_count);

    if !formatted_display {
        display_default(ls_dir);
    } else if formatted_display {
        println!("{:<10} {}", "Size", "Name");

        display_formatted(ls_dir, recursive_folder_scan);
    }
}