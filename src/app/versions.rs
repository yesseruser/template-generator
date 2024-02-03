// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub const LOOM_VERSION: &'static str = "1.5-SNAPSHOT";
pub const PLUGIN_VERSION: &'static str = "3.4-SNAPSHOT";

#[derive(Clone, Copy, Serialize, Deserialize, EnumIter)]
pub enum MinecraftVersion {
    #[serde(rename = "1.16.5")]
    Minecraft1_16_5,
    #[serde(rename = "1.17.1")]
    Minecraft1_17_1,
    #[serde(rename = "1.18.1")]
    Minecraft1_18_1,
    #[serde(rename = "1.18.2")]
    Minecraft1_18_2,
    #[serde(rename = "1.19")]
    Minecraft1_19,
    #[serde(rename = "1.19.1")]
    Minecraft1_19_1,
    #[serde(rename = "1.19.2")]
    Minecraft1_19_2,
    #[serde(rename = "1.19.3")]
    Minecraft1_19_3,
    #[serde(rename = "1.19.4")]
    Minecraft1_19_4,
    #[serde(rename = "1.20.1")]
    Minecraft1_20_1,
    #[serde(rename = "1.20.2")]
    Minecraft1_20_2,
    #[serde(rename = "1.20.4")]
    Minecraft1_20_4,
}

impl MinecraftVersion {
    pub fn latest() -> Self {
        use strum::IntoEnumIterator;
        Self::iter().last().unwrap()
    }

    pub fn version(&self) -> &'static str {
        match self {
            MinecraftVersion::Minecraft1_16_5 => "1.16.5",
            MinecraftVersion::Minecraft1_17_1 => "1.17.1",
            MinecraftVersion::Minecraft1_18_1 => "1.18.1",
            MinecraftVersion::Minecraft1_18_2 => "1.18.2",
            MinecraftVersion::Minecraft1_19 => "1.19",
            MinecraftVersion::Minecraft1_19_1 => "1.19.1",
            MinecraftVersion::Minecraft1_19_2 => "1.19.2",
            MinecraftVersion::Minecraft1_19_3 => "1.19.3",
            MinecraftVersion::Minecraft1_19_4 => "1.19.4",
            MinecraftVersion::Minecraft1_20_1 => "1.20.1",
            MinecraftVersion::Minecraft1_20_2 => "1.20.2",
            MinecraftVersion::Minecraft1_20_4 => "1.20.4",
        }
    }

    pub fn java_version(&self) -> JavaVersion {
        match self {
            MinecraftVersion::Minecraft1_16_5 => JavaVersion::Java8,
            MinecraftVersion::Minecraft1_17_1 => JavaVersion::Java9OrNewer(16),
            _ => JavaVersion::Java9OrNewer(17),
        }
    }

    pub fn architectury_package(&self) -> &'static str {
        match self {
            MinecraftVersion::Minecraft1_16_5 => "me.shedaniel.architectury",
            _ => "dev.architectury",
        }
    }

    pub fn fabric_api_branch(&self) -> &'static str {
        match self {
            MinecraftVersion::Minecraft1_16_5 => "1.16",
            MinecraftVersion::Minecraft1_17_1 => "1.17",
            MinecraftVersion::Minecraft1_18_1 => "1.18",
            _ => self.version(),
        }
    }

    pub fn forge_major_version(&self) -> &'static str {
        match self {
            MinecraftVersion::Minecraft1_16_5 => "36",
            MinecraftVersion::Minecraft1_17_1 => "37",
            MinecraftVersion::Minecraft1_18_1 => "39",
            MinecraftVersion::Minecraft1_18_2 => "40",
            MinecraftVersion::Minecraft1_19 => "41",
            MinecraftVersion::Minecraft1_19_1 => "42",
            MinecraftVersion::Minecraft1_19_2 => "43",
            MinecraftVersion::Minecraft1_19_3 => "44",
            MinecraftVersion::Minecraft1_19_4 => "45",
            MinecraftVersion::Minecraft1_20_1 => "47",
            MinecraftVersion::Minecraft1_20_2 => "48",
            MinecraftVersion::Minecraft1_20_4 => "49",
        }
    }

    pub fn architectury_api_version(self) -> &'static str {
        match self {
            MinecraftVersion::Minecraft1_16_5 => "1",
            MinecraftVersion::Minecraft1_17_1 => "2",
            MinecraftVersion::Minecraft1_18_1 => "3",
            MinecraftVersion::Minecraft1_18_2 => "4",
            MinecraftVersion::Minecraft1_19 => "5",
            MinecraftVersion::Minecraft1_19_1 => "6.3",
            MinecraftVersion::Minecraft1_19_2 => "6",
            MinecraftVersion::Minecraft1_19_3 => "7",
            MinecraftVersion::Minecraft1_19_4 => "8",
            MinecraftVersion::Minecraft1_20_1 => "9",
            MinecraftVersion::Minecraft1_20_2 => "10",
            MinecraftVersion::Minecraft1_20_4 => "11",
        }
    }

    pub fn neoforge_loader_major(self) -> Option<&'static str> {
        match self {
            MinecraftVersion::Minecraft1_20_4 => Some("2"),
            _ => None,
        }
    }

    pub fn neoforge_major(self) -> Option<&'static str> {
        match self {
            MinecraftVersion::Minecraft1_20_4 => Some("20.4"),
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum JavaVersion {
    Java8,
    Java9OrNewer(u32),
}

impl JavaVersion {
    pub fn gradle_java_version(&self) -> String {
        match self {
            Self::Java8 => "1_8".to_owned(),
            Self::Java9OrNewer(version) => version.to_string(),
        }
    }

    pub fn java_major_version(&self) -> u32 {
        match self {
            Self::Java8 => 8,
            Self::Java9OrNewer(version) => *version,
        }
    }
}
