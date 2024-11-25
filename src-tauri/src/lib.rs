use tauri_plugin_updater::UpdaterExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = update(handle).await {
                    println!("Failed to update: {e}");
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        // Perform download and installation with progress tracking
        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                     if let Some(content_length) = content_length {
                        let percentage = (downloaded as f64 / content_length as f64) * 100.0;
                        println!("downloaded {} bytes from {} bytes ({:.2}%)", downloaded, content_length, percentage);
                    } else {
                        println!("downloaded {} bytes (unknown total size)", downloaded);
                    }
                },
                || {
                    println!("download finished");
                },
            )
            .await?;

        println!("update installed");
        app.restart();
    }
    Ok(())
}
