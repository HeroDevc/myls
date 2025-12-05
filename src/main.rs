use std::{env, fs::{self, DirEntry}, path::{Path, PathBuf}, process::exit};

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

fn display_default(path: &str, grep_search_phrase: Option<String>) {
    let is_valid_path = PathBuf::from(path);

    let mut total_files = 0;
    let mut total_folders = 0;

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
                                    match &grep_search_phrase {
                                        Some(phrase) => {
                                            let file_name = remove_first_n_chars(dir.path().display().to_string(), path.len() as i32);

                                            if file_name.to_lowercase().contains(&phrase.to_lowercase()) {
                                                if metadata.is_file() {
                                                    final_string += &format!("{} ", file_name);

                                                    total_files += 1;
                                                } else {
                                                    final_string += &format!("/{} ", file_name);

                                                    total_folders += 1;
                                                }
                                            }
                                        },
                                        None => {
                                            let file_name = remove_first_n_chars(dir.path().display().to_string(), path.len() as i32);

                                            if metadata.is_file() {
                                                final_string += &format!("{} ", file_name);

                                                total_files += 1;
                                            } else {
                                                final_string += &format!("/{} ", file_name);

                                                total_folders += 1;
                                            }
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

        println!("Total {}: {} files, {} folders", total_files + total_folders, total_files, total_folders);
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

fn display_formatted(path: &str, recursive_folder_scan: bool, grep_search_phrase: Option<String>) {
    let is_valid_path = PathBuf::from(path);

    let mut total_files = 0;
    let mut total_folders = 0;

    let mut final_string = String::new();

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
                                    match &grep_search_phrase {
                                        Some(phrase) => {
                                            let file_name = remove_first_n_chars(dir.path().display().to_string(), path.len() as i32);

                                            if file_name.to_lowercase().contains(&phrase.to_lowercase()) {
                                                if metadata.is_file() {
                                                    final_string += &format!("{:<10} {}\n", format_size(metadata.len() as f32), file_name);

                                                    total_files += 1;
                                                } else {
                                                    if recursive_folder_scan {
                                                        final_string += &format!("{:<10} /{}\n", format_size(scan_dir(dir.path().as_path())), file_name);
                                                    } else {
                                                        final_string += &format!("{:<10} /{}\n", format_size(metadata.len() as f32), file_name);
                                                    }

                                                    total_folders += 1;
                                                }
                                            }
                                        },
                                        None => {
                                            let file_name = remove_first_n_chars(dir.path().display().to_string(), path.len() as i32);

                                            if metadata.is_file() {
                                                final_string += &format!("{:<10} {}\n", format_size(metadata.len() as f32), file_name);

                                                total_files += 1;
                                            } else {
                                                if recursive_folder_scan {
                                                    final_string += &format!("{:<10} /{}\n", format_size(scan_dir(dir.path().as_path())), file_name);
                                                } else {
                                                    final_string += &format!("{:<10} /{}\n", format_size(metadata.len() as f32), file_name);
                                                }

                                                total_folders += 1;
                                            }
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

    final_string.truncate(final_string.len() - 1);

    println!("Total {}: {} files, {} folders", total_files + total_folders, total_files, total_folders);
    println!("{:<10} {}", "Size", "Name");
    println!("{}", final_string);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut ls_dir = "./";

    let mut formatted_display = false;
    let mut recursive_folder_scan = false;
    let mut grep_search_phrase: Option<String> = None;

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

        if item.to_lowercase() == "-grep" {
            if args.len() - 1 >= i + 1 {
                let word = &args[i + 1];

                grep_search_phrase = Some(word.to_string());
            } else {
                println!("Missing -grep arguments!");
                
                exit(0);
            }
        }

        i += 1;
    }

    if !formatted_display {
        display_default(ls_dir, grep_search_phrase);
    } else if formatted_display {
        display_formatted(ls_dir, recursive_folder_scan, grep_search_phrase);
    }
}