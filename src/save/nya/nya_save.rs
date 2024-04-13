use std::{fs, path::PathBuf, usize};

use serde::{Deserialize, Serialize};

use crate::save::common::{save_slot::SaveSlot, user_data_10::ProfileSummary};

#[derive(Clone, Serialize, Deserialize)]
pub struct NyaSave {
    pub name: String,
    pub items: NyaItems,
    pub stats: NyaStats,
    pub ver: Option<u32>, // This gets added for validation
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NyaStats {
    rl: u16,
    arc: u16,
    dex: u16,
    fth: u16,
    int: u16,
    mnd: u16,
    str: u16,
    vig: u16,
    vit: u16,
}

impl NyaSave {
    pub fn get_profile_summary(&self) -> ProfileSummary {
        let mut profile_summary = ProfileSummary::default();

        profile_summary
    }

    pub fn as_save_slot(&self) -> SaveSlot {
        // let mut br = BinaryReader::from_u8(&vagabond());
        // br.set_endian(binary_reader::Endian::Little);
        // let save_slot = SaveSlot::read(&mut br);

        // save_slot.unwrap()
        SaveSlot::default()
    }
}

impl Default for NyaStats {
    fn default() -> Self {
        Self {
            rl: 0,
            arc: 0,
            dex: 0,
            fth: 0,
            int: 0,
            mnd: 0,
            str: 0,
            vig: 0,
            vit: 0,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NyaItems {
    pub crystal_tears: [Option<String>; 2],
    pub ammo: NyaAmmo,
    pub tools: NyaTools,
    pub flasks: NyaFlasks,
}

impl Default for NyaItems {
    fn default() -> Self {
        Self {
            crystal_tears: Default::default(),
            ammo: Default::default(),
            tools: Default::default(),
            flasks: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NyaFlasks {
    pub level: u16,
    pub total: u16,
    pub crimson: u16,
    pub cerulean: u16,
}

impl Default for NyaFlasks {
    fn default() -> Self {
        Self {
            level: Default::default(),
            total: Default::default(),
            crimson: Default::default(),
            cerulean: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NyaAmmo {
    pub slots: Vec<String>,
}

impl Default for NyaAmmo {
    fn default() -> Self {
        Self {
            slots: Default::default(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct NyaTools {
    pub slots: Vec<String>,
    pub sorting: NyaToolsSorting,
}

impl Default for NyaTools {
    fn default() -> Self {
        Self {
            slots: Default::default(),
            sorting: NyaToolsSorting::default(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct NyaToolsSorting {
    pub method: String,
    pub direction: String,
}

impl Default for NyaToolsSorting {
    fn default() -> Self {
        Self {
            method: String::default(),
            direction: String::default(),
        }
    }
}

impl NyaSave {
    pub fn from_path(path: &PathBuf) -> Result<Self, std::io::Error> {
        let data = fs::read_to_string(path).expect("Should have some data");
        let nya_save: NyaSave =
            serde_json::from_str(&data).expect("Should have been able to deserialize");

        Ok(nya_save)
    }
}

impl Default for NyaSave {
    fn default() -> Self {
        Self {
            name: String::default(),
            items: NyaItems::default(),
            stats: NyaStats::default(),
            ver: None,
        }
    }
}
