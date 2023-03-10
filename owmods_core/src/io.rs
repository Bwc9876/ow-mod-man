use crate::{download::install_mods_parallel, file::deserialize_from_json};
use anyhow::Result;
use std::path::{Path, PathBuf};

use super::{
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    toggle::{get_mod_enabled, toggle_mod},
};

pub fn export_mods(db: &LocalDatabase) -> Result<String> {
    let enabled_mods: Vec<&String> = db
        .active()
        .iter()
        .filter_map(|m| {
            if m.enabled {
                Some(&m.manifest.unique_name)
            } else {
                None
            }
        })
        .collect();
    let result = serde_json::to_string_pretty(&enabled_mods)?;
    Ok(result)
}

pub async fn import_mods(
    config: &Config,
    local_db: &LocalDatabase,
    remote_db: &RemoteDatabase,
    file_path: &Path,
    disable_missing: bool,
) -> Result<()> {
    let unique_names: Vec<String> = deserialize_from_json(file_path)?;
    let mut needed_install: Vec<String> = vec![];

    if disable_missing {
        for local_mod in local_db.mods.values() {
            let mod_path = &PathBuf::from(&local_mod.mod_path);
            if get_mod_enabled(&PathBuf::from(&mod_path))? {
                toggle_mod(mod_path, local_db, false, false)?;
            }
        }
    }
    for name in unique_names.iter() {
        let local_mod = local_db.get_mod(name);
        if let Some(local_mod) = local_mod {
            let mod_path = &PathBuf::from(&local_mod.mod_path);
            if !get_mod_enabled(&PathBuf::from(&mod_path))? {
                toggle_mod(mod_path, local_db, true, false)?;
            }
        } else {
            needed_install.push(name.to_string());
        }
    }

    install_mods_parallel(needed_install, config, remote_db, local_db).await?;

    Ok(())
}
