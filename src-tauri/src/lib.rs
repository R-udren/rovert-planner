use std::process::Command;

#[tauri::command]
fn is_ollama_installed() -> Result<bool, String> {
    let output = Command::new("ollama").arg("--version").output();

    match output {
        Ok(_) => Ok(true),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                Ok(false)
            } else {
                Err(format!("Error checking Ollama installation: {}", e))
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![is_ollama_installed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
