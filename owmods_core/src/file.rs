use std::{
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

pub fn deserialize_from_json<T>(file_path: &Path) -> Result<T>
where
    for<'a> T: Deserialize<'a>,
{
    let file = File::open(file_path)?;
    let buffer = BufReader::new(file);
    let result = serde_json::from_reader(buffer)?;
    Ok(result)
}

pub fn serialize_to_json<T>(obj: &T, out_path: &Path, create_parents: bool) -> Result<()>
where
    T: Serialize,
{
    if create_parents {
        if let Some(parent_path) = out_path.parent() {
            create_dir_all(parent_path)?;
        }
    }
    let file = File::create(out_path)?;
    let buffer = BufWriter::new(file);
    serde_json::to_writer_pretty(buffer, obj)?;
    Ok(())
}

pub fn get_app_path() -> Result<PathBuf> {
    let app_data_path = ProjectDirs::from("com", "ow-mods", "ow-mod-man");
    match app_data_path {
        Some(app_data_path) => Ok(app_data_path.data_dir().to_path_buf()),
        None => Err(anyhow!("Can't find user's app data dir")),
    }
}

pub fn fix_json(path: &Path) -> Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    // BOMs are really really annoying
    buffer = fix_bom(&mut buffer);

    let mut file = File::create(path)?;
    write!(file, "{}", buffer)?;

    Ok(())
}

pub fn create_all_parents(file_path: &Path) -> Result<()> {
    if let Some(parent_path) = file_path.parent() {
        create_dir_all(parent_path)?;
    }
    Ok(())
}

pub fn fix_bom(str: &mut String) -> String {
    str.strip_prefix('\u{FEFF}').unwrap_or(str).to_string()
}
