use crate::core::config::AppSettings;
use anyhow::Result;
use directories::ProjectDirs;
use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::{debug, info};

pub struct StorageService {
    data_dir: PathBuf,
}

impl StorageService {
    pub fn new() -> Result<Self> {
        let data_dir = ProjectDirs::from("app", "solian", "Solian")
            .map(|dirs| dirs.data_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));

        fs::create_dir_all(&data_dir)?;
        info!("Storage initialized at: {:?}", data_dir);

        Ok(Self { data_dir })
    }

    pub fn get_settings_path(&self) -> PathBuf {
        self.data_dir.join("settings.json")
    }

    pub fn save_settings(&self, settings: &AppSettings) -> Result<()> {
        let path = self.get_settings_path();
        let json = serde_json::to_string_pretty(settings)?;
        fs::write(&path, json)?;
        debug!("Settings saved to {:?}", path);
        Ok(())
    }

    pub fn load_settings(&self) -> Result<AppSettings> {
        let path = self.get_settings_path();
        if !path.exists() {
            return Ok(AppSettings::default());
        }
        let json = fs::read_to_string(&path)?;
        let settings = serde_json::from_str(&json)?;
        debug!("Settings loaded from {:?}", path);
        Ok(settings)
    }

    pub fn save<T: Serialize>(&self, key: &str, value: &T) -> Result<()> {
        let path = self.data_dir.join(format!("{}.json", key));
        let json = serde_json::to_string_pretty(value)?;
        fs::write(&path, json)?;
        debug!("Data saved to {:?}", path);
        Ok(())
    }

    pub fn load<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        let path = self.data_dir.join(format!("{}.json", key));
        if !path.exists() {
            return Ok(None);
        }
        let json = fs::read_to_string(&path)?;
        let value = serde_json::from_str(&json)?;
        debug!("Data loaded from {:?}", path);
        Ok(Some(value))
    }

    pub fn delete(&self, key: &str) -> Result<()> {
        let path = self.data_dir.join(format!("{}.json", key));
        if path.exists() {
            fs::remove_file(&path)?;
            debug!("Data deleted from {:?}", path);
        }
        Ok(())
    }
}

impl Default for StorageService {
    fn default() -> Self {
        Self::new().expect("Failed to create storage service")
    }
}
