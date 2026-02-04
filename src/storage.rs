use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::Mood;

#[derive(Debug, Serialize, Deserialize)]
pub struct PersistedPet {
    pub mood: Mood,
    pub energy: i32,
    pub xp: u32,
    pub level: u32,
}

pub fn storage_dir() -> PathBuf {
    let mut dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    dir.push(".terminal-pet");
    dir
}

pub fn default_path() -> PathBuf {
    let mut p = storage_dir();
    p.push("pet.json");
    p
}

pub fn load(path: Option<&PathBuf>) -> Option<PersistedPet> {
    let p = path.cloned().unwrap_or_else(default_path);
    if !p.exists() {
        return None;
    }
    let mut f = File::open(&p).ok()?;
    let mut s = String::new();
    f.read_to_string(&mut s).ok()?;
    serde_json::from_str(&s).ok()
}

pub fn save(path: Option<&PathBuf>, pet: &PersistedPet) -> std::io::Result<()> {
    let p = path.cloned().unwrap_or_else(default_path);
    if let Some(parent) = p.parent() {
        create_dir_all(parent)?;
    }
    let mut f = File::create(&p)?;
    let s = serde_json::to_string_pretty(pet)?;
    f.write_all(s.as_bytes())?;
    Ok(())
}
