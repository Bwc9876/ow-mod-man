use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

use anyhow::anyhow;
use anyhow::Result;
use owmods_core::{
    alerts::{get_warnings, save_warning_shown},
    config::Config,
    db::LocalDatabase,
    socket::SocketMessage,
};
use tauri::{api::dialog, AppHandle, Window, WindowBuilder};
use tempdir::TempDir;
use tokio::fs::remove_file;

pub async fn make_log_window(handle: &AppHandle, port: u16) -> Result<Window> {
    let label = format!("game-{port}");
    let log_window = WindowBuilder::new(
        handle,
        &label,
        tauri::WindowUrl::App("/logs/index.html".parse()?),
    );
    let window = log_window
        .center()
        .title(format!("Game Logs (Port {port})"))
        .min_inner_size(450.0, 450.0)
        .enable_clipboard_access()
        .build()?;
    Ok(window)
}

pub fn show_warnings(window: &Window, local_db: &LocalDatabase, config: &Config) -> Result<Config> {
    let warnings = get_warnings(local_db, config)?;
    let mut config = config.clone();
    for (unique_name, warning) in warnings {
        dialog::blocking::message(Some(window), &warning.title, &warning.body);
        config = save_warning_shown(unique_name, &config)?;
    }
    Ok(config)
}

pub fn write_log(log_dir: &TempDir, msg: SocketMessage) -> Result<()> {
    let log_file = File::options()
        .create(true)
        .append(true)
        .open(log_dir.path().join("log.txt"))?;
    let mut buffer = BufWriter::new(log_file);
    writeln!(buffer, "{}", serde_json::to_string(&msg)?)?;
    Ok(())
}

pub fn get_log_from_line(log_dir: &TempDir, line: usize) -> Result<SocketMessage> {
    let log_file = File::open(log_dir.path().join("log.txt"))?;
    let buffer = BufReader::new(log_file);
    let line = buffer
        .lines()
        .nth(line)
        .ok_or_else(|| anyhow!("Line Not In File"))??;
    let msg = serde_json::from_str::<SocketMessage>(&line)?;
    Ok(msg)
}

pub async fn clear_game_logs(log_dir: &TempDir) -> Result<()> {
    remove_file(log_dir.path().join("log.txt")).await?;
    Ok(())
}
