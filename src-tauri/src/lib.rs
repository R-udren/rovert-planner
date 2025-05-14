use std::process::Command;

#[tauri::command]
fn is_ollama_installed() -> Result<bool, String> {
    println!("Checking if Ollama is installed...");
    let output = Command::new("ollama").arg("--version").output();
    println!("Ollama command output: {:?}", output);

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

#[tauri::command]
fn start_ollama_server() -> Result<(), String> {
    println!("Starting Ollama server...");
    let output = Command::new("ollama").arg("serve").output();

    match output {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error starting Ollama server: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            is_ollama_installed,
            start_ollama_server
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
