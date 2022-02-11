// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use std::{collections::BTreeSet, path::PathBuf};

use crate::{hero::InventoryItem, Result};

use anyhow::bail;
use uuid::Uuid;

#[derive(
    Debug,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[serde(rename_all = "lowercase")]
pub enum StorableInventoryItem {
    Boot,
    Glove,
    Clamp,
}

impl std::convert::Into<InventoryItem> for StorableInventoryItem {
    fn into(self) -> InventoryItem {
        match self {
            StorableInventoryItem::Boot => InventoryItem::Boot,
            StorableInventoryItem::Glove => InventoryItem::Glove,
            StorableInventoryItem::Clamp => InventoryItem::Clamp,
        }
    }
}

impl std::convert::TryFrom<InventoryItem> for StorableInventoryItem {
    type Error = ();

    fn try_from(
        item: InventoryItem,
    ) -> Result<StorableInventoryItem, Self::Error> {
        match item {
            InventoryItem::Boot => Ok(StorableInventoryItem::Boot),
            InventoryItem::Glove => Ok(StorableInventoryItem::Glove),
            InventoryItem::Clamp => Ok(StorableInventoryItem::Clamp),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveGame {
    pub score: u64,
    pub game_id: Uuid,
    pub firepower: u8,
    pub health: u8,
    pub inventory: BTreeSet<StorableInventoryItem>,
    pub finished_level: usize,
}

impl SaveGame {
    pub fn savepath(episode_name: &str, slot: usize) -> PathBuf {
        crate::data_dir()
            .join("savegames")
            .join(episode_name)
            .join(format!("{}.toml", slot))
    }

    pub fn exists(episode_name: &str, slot: usize) -> bool {
        dbg!(Self::savepath(episode_name, slot)).exists()
    }

    pub fn load(episode_name: &str, slot: usize) -> Result<SaveGame> {
        let path = Self::savepath(episode_name, slot);
        if path.exists() {
            let s = std::fs::read_to_string(&path)?;
            Ok(toml::de::from_str(&s)?)
        } else {
            bail!(
                "Savegame slot {} for episode {:?} doesn't exist",
                episode_name,
                slot
            )
        }
    }

    pub fn save(&self, episode_name: &str, slot: usize) -> Result<()> {
        let path = Self::savepath(episode_name, slot);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(
            &path,
            toml::ser::to_string(self).unwrap().as_bytes(),
        )?;
        Ok(())
    }
}
