use serde::Serialize;
use serde_json::json;
use std::fs;

#[derive(Serialize)]
struct AlgorithmInfo {
    algorithm: String,
    model: Vec<String>,
}

#[tauri::command]
pub fn get_algorithms() -> String {
    let path = "../data/";
    let mut folders: Vec<AlgorithmInfo> = vec![];
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(dir_entry) => {
                        if dir_entry.path().is_dir() {
                            if let Ok(folder) = dir_entry.file_name().into_string() {
                                let mut model: Vec<String> = vec![];
                                let model_path = format!("{}{}/weights/", path, folder);
                                if let Ok(model_entries) = fs::read_dir(model_path) {
                                    for model_entry in model_entries {
                                        match model_entry {
                                            Ok(model_dir_entry) => {
                                                if model_dir_entry.path().is_file() {
                                                    if let Some(extension) =
                                                        model_dir_entry.path().extension()
                                                    {
                                                        if extension == "onnx" {
                                                            if let Ok(model_name) = model_dir_entry
                                                                .file_name()
                                                                .into_string()
                                                            {
                                                                model.push(model_name);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                println!("{:?}", e);
                                            }
                                        }
                                    }
                                }
                                folders.push(AlgorithmInfo {
                                    algorithm: folder,
                                    model,
                                });
                            }
                        }
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("{}", std::env::current_dir().unwrap().as_path().display());
            println!("{:?}", e)
        }
    }
    json!(folders).to_string()
}
