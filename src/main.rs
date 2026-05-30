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

const FILE_EXTENSIONS_STYLES: [(&str, &str); 75] = [
    (".mp3", "🎵"),
    (".mp4", "🎬"),
    (".mov", "🎬"),
    (".gif", "🎬"),
    (".tar.gz", "📦"),
    (".zip", "📦"),
    (".7z", "📦"),
    (".doc", "📘"),
    (".docx", "📘"),
    (".xls", "📗"),
    (".xlsx", "📗"),
    (".ppt", "📙"),
    (".pptx", "📙"),
    (".pdf", "📕"),
    (".jar", "☕"),
    (".java", "☕"),
    (".kt", "🌴"),
    (".kts", "🌴"),
    (".rs", "🦀"),
    (".bat", "💲"),
    (".sh", "💲"),
    (".vbs", "💲"),
    (".zshrc", "💲"),
    (".zig", "⚡"),
    (".conf", "🔧"),
    (".config", "🔧"),
    (".json", "🔧"),
    (".js", "🟨"),
    (".jsx", "🟨"),
    (".ts", "🟦"),
    (".tsx", "🟦"),
    (".py", "🐍"),
    (".gitignore", "🔸"),
    (".bf", "🧠"),
    (".md", "📑"),
    (".dockerignore", "🐳"),
    (".lock", "🔒"),
    (".db", "💾"),
    (".sql", "💾"),
    (".rb", "💎"),
    (".go", "🐹"),
    (".cs", "🎸"),
    (".swift", "🍎"),
    (".html", "🌐"),
    (".css", "🎨"),
    (".php", "🐘"),
    (".h", "📝"),
    (".c", "📝"),
    (".cpp", "📝"),
    (".pl", "🐪"),
    (".pm", "🐪"),
    (".ml", "🐫"),
    (".asm", "⚙️"),
    (".yml", "🧶"),
    (".yaml", "🧶"),
    (".nix", "❄️"),
    (".lua", "🌙"),
    (".hs", "📐"),
    (".ex", "💧"),
    (".exs", "💧"),
    (".erl", "💧"),
    (".wasm", "🕸️"),
    (".dll", "📚"),
    (".log", "📜"),
    (".vim", "🟩"),
    (".nim", "👑"),
    (".cr", "🔮"),
    (".gleam", "⭐"),
    (".sb3", "🐱"),
    (".sb2", "🐱"),
    (".pub", "🔓"),
    (".key", "🔑"),
    (".pem", "🔑"),
    (".ipynb", "🪐"),
    (".dart", "🎯")
];

fn file_name_to_styled(file_name: String) -> String {
    if file_name.to_lowercase().ends_with(".exe") || file_name.to_lowercase().ends_with(".msi") || file_name.to_lowercase().ends_with(".bin") || file_name.to_lowercase().ends_with(".deb") || file_name.to_lowercase().ends_with(".out") || file_name.to_lowercase().ends_with(".iso") {
        return format!("💿 \x1b[91m{}\x1b[0m", file_name);
    } else if file_name.to_lowercase().ends_with(".png") || file_name.to_lowercase().ends_with(".jpg") || file_name.to_lowercase().ends_with(".jpeg") || file_name.to_lowercase().ends_with(".webp") || file_name.to_lowercase().ends_with(".ico") {
        return format!("📷 \x1b[96m{}\x1b[0m", file_name);
    }

    for item in FILE_EXTENSIONS_STYLES {
        if file_name.to_lowercase().ends_with(item.0) {
            return format!("{} {}", item.1, file_name)
        }
    }

    format!("📄 {}", file_name)
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
                                                    final_string += &format!("{} ", file_name_to_styled(file_name));

                                                    total_files += 1;
                                                } else {
                                                    final_string += &format!("📁 \x1b[94m{}\x1b[0m ", file_name);

                                                    total_folders += 1;
                                                }
                                            }
                                        },
                                        None => {
                                            let file_name = remove_first_n_chars(dir.path().display().to_string(), path.len() as i32);

                                            if metadata.is_file() {
                                                final_string += &format!("{} ", file_name_to_styled(file_name));

                                                total_files += 1;
                                            } else {
                                                final_string += &format!("📁 \x1b[94m{}\x1b[0m ", file_name);

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

fn scan_for_folder(path: &Path) -> Option<String> {
    let path_res: Result<fs::ReadDir, std::io::Error> = fs::read_dir(path);

    match path_res {
        Ok(paths) => {
            let collected: Vec<Result<DirEntry, std::io::Error>> = paths.collect();

            if collected.len() > 1 || collected.len() == 0 {
                return None
            }

            let dir = match &collected[0] {
                Ok(v) => v,
                Err(_e) => {
                    return None
                }
            };

            let metadata = match dir.metadata() {
                Ok(v) => v,
                Err(_e) => {
                    return None
                }
            };

            if metadata.is_dir() {
                return Some(format!("{}", dir.file_name().display().to_string()))   
            }
        },
        Err(_e) => {
            // println!("{}", e);
        }
    }

    None
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
                                                    final_string += &format!("{:<10} {}\n", format_size(metadata.len() as f32), file_name_to_styled(file_name));

                                                    total_files += 1;
                                                } else {
                                                    if recursive_folder_scan {
                                                        final_string += &format!("{:<10} 📁 \x1b[94m{}\x1b[0m\n", format_size(scan_dir(dir.path().as_path())), file_name);
                                                    } else {
                                                        // final_string += &format!("{:<10} 📁 \x1b[94m{}\x1b[0m\n", format_size(metadata.len() as f32), file_name);
                                                        match scan_for_folder(&dir.path()) {
                                                            Some(v) => {
                                                                final_string += &format!("{:<10} 📁 \x1b[94m{}\x1b[0m 👉 \x1b[94m{}\x1b[0m\n", format_size(metadata.len() as f32), file_name, v);
                                                            },
                                                            None => {
                                                                final_string += &format!("{:<10} 📁 \x1b[94m{}\x1b[0m\n", format_size(metadata.len() as f32), file_name);
                                                            }
                                                        }
                                                    }

                                                    total_folders += 1;
                                                }
                                            }
                                        },
                                        None => {
                                            let file_name = remove_first_n_chars(dir.path().display().to_string(), path.len() as i32);

                                            if metadata.is_file() {
                                                final_string += &format!("{:<10} {}\n", format_size(metadata.len() as f32), file_name_to_styled(file_name));

                                                total_files += 1;
                                            } else {
                                                if recursive_folder_scan {
                                                    final_string += &format!("{:<10} 📁 \x1b[94m{}\x1b[0m\n", format_size(scan_dir(dir.path().as_path())), file_name);
                                                } else {
                                                    match scan_for_folder(&dir.path()) {
                                                        Some(v) => {
                                                            final_string += &format!("{:<10} 📁 \x1b[94m{}\x1b[0m 👉 \x1b[94m{}\x1b[0m\n", format_size(metadata.len() as f32), file_name, v);
                                                        },
                                                        None => {
                                                            final_string += &format!("{:<10} 📁 \x1b[94m{}\x1b[0m\n", format_size(metadata.len() as f32), file_name);
                                                        }
                                                    }
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

fn scan_for_folders_tree(path: &Path) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    let path_res: Result<fs::ReadDir, std::io::Error> = fs::read_dir(path);

    match path_res {
        Ok(paths) => {
            for res_path in paths {
                match res_path {
                    Ok(dir) => {
                        let file_metadata = dir.metadata();

                        match file_metadata {
                            Ok(metadata) => {
                                let file_name = dir.file_name().display().to_string();

                                if metadata.is_file() {
                                    vec.push(format!("{}", file_name_to_styled(file_name.clone())));
                                } else {
                                    vec.push(format!("📁 \x1b[94m{}\x1b[0m", file_name));
                                }
                            },
                            Err(_e) => {}
                        }
                    },
                    Err(_e) => {}
                }
            }
        },
        Err(_e) => {}
    }

    vec
}

fn display_formatted_tree(path: &str, recursive_folder_scan: bool, grep_search_phrase: Option<String>) {
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
                                                    final_string += &format!("{:<10} {}\n", format_size(metadata.len() as f32), file_name_to_styled(file_name));

                                                    total_files += 1;
                                                } else {
                                                    if recursive_folder_scan {
                                                        let data_for_path = scan_for_folders_tree(&dir.path());

                                                        final_string += &format!("{:<10} 📁 \x1b[94m{}\x1b[0m\n", format_size(scan_dir(dir.path().as_path())), file_name);

                                                        for i in 0..data_for_path.len() {
                                                            if i == data_for_path.len() - 1 {
                                                                final_string += &format!("{:<10} └─ {}\n", "", data_for_path[i]);
                                                            } else {
                                                                final_string += &format!("{:<10} ├─ {}\n", "", data_for_path[i]);
                                                            }
                                                        }
                                                    } else {
                                                        // final_string += &format!("{:<10} 📁 \x1b[94m{}\x1b[0m\n", format_size(metadata.len() as f32), file_name);
                                                        let data_for_path = scan_for_folders_tree(&dir.path());

                                                        final_string += &format!("{:<10} 📁 \x1b[94m{}\x1b[0m\n", format_size(metadata.len() as f32), file_name);

                                                        for i in 0..data_for_path.len() {
                                                            if i == data_for_path.len() - 1 {
                                                                final_string += &format!("{:<10} └─ {}\n", "", data_for_path[i]);
                                                            } else {
                                                                final_string += &format!("{:<10} ├─ {}\n", "", data_for_path[i]);
                                                            }
                                                        }
                                                    }

                                                    total_folders += 1;
                                                }
                                            }
                                        },
                                        None => {
                                            let file_name = remove_first_n_chars(dir.path().display().to_string(), path.len() as i32);

                                            if metadata.is_file() {
                                                final_string += &format!("{:<10} {}\n", format_size(metadata.len() as f32), file_name_to_styled(file_name));

                                                total_files += 1;
                                            } else {
                                                if recursive_folder_scan {
                                                    let data_for_path = scan_for_folders_tree(&dir.path());

                                                    final_string += &format!("{:<10} 📁 \x1b[94m{}\x1b[0m\n", format_size(scan_dir(dir.path().as_path())), file_name);

                                                    for i in 0..data_for_path.len() {
                                                        if i == data_for_path.len() - 1 {
                                                            final_string += &format!("{:<10} └─ {}\n", "", data_for_path[i]);
                                                        } else {
                                                            final_string += &format!("{:<10} ├─ {}\n", "", data_for_path[i]);
                                                        }
                                                    }
                                                } else {
                                                    let data_for_path = scan_for_folders_tree(&dir.path());

                                                    final_string += &format!("{:<10} 📁 \x1b[94m{}\x1b[0m\n", format_size(metadata.len() as f32), file_name);

                                                    for i in 0..data_for_path.len() {
                                                        if i == data_for_path.len() - 1 {
                                                            final_string += &format!("{:<10} └─ {}\n", "", data_for_path[i]);
                                                        } else {
                                                            final_string += &format!("{:<10} ├─ {}\n", "", data_for_path[i]);
                                                        }
                                                    }

                                                    // ├  └  ─  │
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
    let mut tree_display = false;

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

        if item.to_lowercase() == "-t" {
            tree_display = true;
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
    } else if formatted_display && tree_display {
        display_formatted_tree(ls_dir, recursive_folder_scan, grep_search_phrase);
    } else if formatted_display {
        display_formatted(ls_dir, recursive_folder_scan, grep_search_phrase);
    }
}
