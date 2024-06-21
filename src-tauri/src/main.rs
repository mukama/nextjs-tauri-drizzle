// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use tauri_plugin_sql::Migration;
use tauri_plugin_sql::MigrationKind;

fn read_file_to_string<P: AsRef<Path>>(file_path: P) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_migration_filename(file_name: &str) -> Option<(i64, String)> {
    // Example file name format: 0000_description_text.sql
    let parts: Vec<&str> = file_name.split('_').collect();
    if parts.len() >= 2 {
        if let Some(version) = parts[0].parse::<i64>().ok() {
            let description = parts[1..].join("_").replace(".sql", "");
            return Some((version, description));
        }
    }
    None
}

fn generate_migrations(folder_path: &str) -> Result<Vec<Migration>, io::Error> {
    let mut migrations: Vec<Migration> = Vec::new();

    // Read directory entries and collect file paths
    for entry in std::fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a file and ends with ".sql"
        if path.is_file() && path.extension().unwrap_or_default() == "sql" {
            if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                if let Some((version, description)) = parse_migration_filename(file_name) {
                    let sql_content = read_file_to_string(&path)?;

                    // Create Migration struct and add to migrations vector
                    let migration = Migration {
                        version,
                        description: Box::leak(description.into_boxed_str()),
                        sql: Box::leak(sql_content.into_boxed_str()),
                        kind: MigrationKind::Up, // Set the kind appropriately
                    };
                    migrations.push(migration);
                }
            }
        }
    }

    Ok(migrations)
}


fn main() {
    let migration_folder = "migrations";
    let _migrations = match generate_migrations(migration_folder) {
        Ok(migrations) => migrations,
        Err(e) => {
            eprintln!("Error generating migrations: {}", e);
            return; // Exit early if there's an error
        },
    };

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:test.db", _migrations)
                .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
