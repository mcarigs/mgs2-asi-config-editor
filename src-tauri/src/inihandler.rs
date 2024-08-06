use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

// Ensure that all necessary INI files exist in the executable directory
#[tauri::command]
pub fn ensure_ini_files_exist(app: tauri::AppHandle) -> Result<(), String> {
    // Resolve the path to the .editordata resource directory
    let resource_dir = app
        .path_resolver()
        .resolve_resource(".editordata")
        .ok_or_else(|| "Failed to resolve resource directory".to_string())?;

    // Get the current executable directory path
    let exe_dir = env::current_dir().map_err(|e| e.to_string())?;

    // Iterate through all .ini files in the resource directory (.editordata)
    for entry in fs::read_dir(resource_dir).map_err(|e| e.to_string())? {
        // Get instance of an entry inside of the resource directory on the filesystem
        let entry = entry.map_err(|e| e.to_string())?;
        // Parse .ini filename for current entry
        let file_name = entry.file_name();
        // Append filename w/ extension to the executable directory path
        let exe_file_path = exe_dir.join(&file_name);

        // If the file doesn't already exist in the executable directory, create it
        if !exe_file_path.exists() {
            fs::File::create(&exe_file_path).map_err(|e| {
                format!(
                    "Failed to create file {}: {}",
                    file_name.to_string_lossy(),
                    e
                )
            })?;

            // Copy content from resource file to new file
            let resource_content = fs::read_to_string(entry.path()).map_err(|e| {
                format!(
                    "Failed to read resource file {}: {}",
                    file_name.to_string_lossy(),
                    e
                )
            })?;
            fs::write(&exe_file_path, resource_content).map_err(|e| {
                format!(
                    "Failed to write to file {}: {}",
                    file_name.to_string_lossy(),
                    e
                )
            })?;
        }
    }
    Ok(())
}

// Read an INI file and return its contents as a nested HashMap
#[tauri::command]
pub fn read_ini_file(
    handle: tauri::AppHandle,
    path: &str,
) -> Result<HashMap<String, HashMap<String, String>>, String> {
    // Resolve the resource path - .editordata/${path}
    let resource_path = handle
        .path_resolver()
        .resolve_resource(path)
        .expect("Failed to resolve resource");

    // Read the entire file content
    let mut file = std::fs::File::open(resource_path).map_err(|e| e.to_string())?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| e.to_string())?;

    // Initialize some variables before parsing the file content
    let mut result = HashMap::new();
    let mut current_section = String::from("");
    let mut multiline_key = String::new();
    let mut multiline_content = Vec::new();
    let mut in_multiline = false;

    // Parse the INI file content
    for line in content.lines() {
        let trimmed_line = line.trim();

        // Skip empty lines and comments
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        // Handle section headers
        if trimmed_line.starts_with('[') && trimmed_line.ends_with(']') {
            current_section = trimmed_line[1..trimmed_line.len() - 1].to_string();
            result
                .entry(current_section.clone())
                .or_insert_with(HashMap::new);
        } else if in_multiline {
            // Handle multiline values
            if trimmed_line == "EOS" {
                let multiline_value = multiline_content.join("\n");
                result
                    .entry(current_section.clone())
                    .or_insert_with(HashMap::new)
                    .insert(multiline_key.clone(), multiline_value);
                in_multiline = false;
                multiline_content.clear();
            } else {
                multiline_content.push(line.to_string());
            }
        } else if trimmed_line.contains('=') {
            // Handle key-value pairs
            let parts: Vec<&str> = trimmed_line.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim();
                if value.starts_with("<<<EOS") {
                    // Start of multiline value
                    in_multiline = true;
                    multiline_key = key.to_string();
                } else {
                    result
                        .entry(current_section.clone())
                        .or_insert_with(HashMap::new)
                        .insert(key.to_string(), value.to_string());
                }
            }
        }
    }
    Ok(result)
}

// Write data to an INI file
#[tauri::command]
pub fn write_ini_file(
    app: tauri::AppHandle,
    path: String,
    data: HashMap<String, HashMap<String, String>>,
) -> Result<(), String> {
    println!("write_ini_file called with path: {}", path);

    // Write to resource directory - .editordata
    let resource_path = app
        .path_resolver()
        .resolve_resource(format!(".editordata/{}", path.clone()))
        .ok_or_else(|| "Failed to resolve resource directory".to_string())?;
    write_to_file(&resource_path, &data)?;

    // Get the current executable directory path
    let exe_dir = env::current_dir().map_err(|e| e.to_string())?;
    // Append filename w/ extension to the executable directory path
    let exe_file_path = exe_dir.join(PathBuf::from(path).file_name().unwrap());

    // Create the file in the executable path if it doesn't exist
    if !exe_file_path.exists() {
        fs::File::create(&exe_file_path).map_err(|e| format!("Failed to create file: {}", e))?;
    }
    // Write modified file content to the file in the executable directory
    write_to_file(&exe_file_path, &data)?;

    println!("Files written successfully");
    Ok(())
}

// Helper function to write INI data to a file
fn write_to_file(
    file_path: &PathBuf,
    data: &HashMap<String, HashMap<String, String>>,
) -> Result<(), String> {
    // Read the existing file
    let existing_file = File::open(file_path).map_err(|e| e.to_string())?;

    // Create a buffer to read the file contents
    let reader = BufReader::new(existing_file);

    // Read all lines in the file
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;

    // Create a temporary file for writing
    let temp_path = file_path.with_extension("tmp");
    let mut temp_file = File::create(&temp_path).map_err(|e| e.to_string())?;

    // Initialize some variables before processing the each line in the file
    let mut current_section = String::new();
    let mut in_section = false;
    let mut in_multiline = false;
    let mut i = 0;

    // Write the updated content to the temporary file
    while i < lines.len() {
        let line: &String = &lines[i];
        let trimmed = line.trim();

        // Section header
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_section = true;
            current_section = trimmed[1..trimmed.len() - 1].to_string();
            writeln!(temp_file, "{}", line).map_err(|e| e.to_string())?;
        } else if trimmed.starts_with('#') && in_section {
            // Preserve comments outside of section grouping
            writeln!(temp_file, "{}", line.replace('#', "")).map_err(|e| e.to_string())?;
        } else if trimmed.starts_with('#') {
            // Preserve comments outside of section grouping
            writeln!(temp_file, "{}", line).map_err(|e| e.to_string())?;
        } else if !in_multiline && trimmed.contains('=') {
            // Key-value pair
            let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                if let Some(section) = data.get(&current_section) {
                    if let Some(new_value) = section.get(key) {
                        if new_value.contains('\n') {
                            // Handle multiline values
                            writeln!(temp_file, "{}=<<<EOS", key).map_err(|e| e.to_string())?;
                            writeln!(temp_file, "{}", new_value).map_err(|e| e.to_string())?;
                            writeln!(temp_file, "EOS").map_err(|e| e.to_string())?;
                            in_multiline = true;
                        } else {
                            writeln!(temp_file, "{}={}", key, new_value)
                                .map_err(|e| e.to_string())?;
                        }
                    } else {
                        writeln!(temp_file, "{}", line).map_err(|e| e.to_string())?;
                    }
                } else {
                    writeln!(temp_file, "{}", line).map_err(|e| e.to_string())?;
                }
            }
        } else if in_multiline {
            if trimmed == "EOS" {
                in_multiline = false;
            }
            // Skip multiline content as it's already been written
        } else {
            // Preserve other lines (empty lines, etc.)
            writeln!(temp_file, "{}", line).map_err(|e| e.to_string())?;
            in_section = false;
        }
        i += 1;
    }

    // Rename temporary file to original file
    std::fs::rename(temp_path, file_path).map_err(|e| e.to_string())?;
    Ok(())
}
