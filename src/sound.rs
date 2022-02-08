// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::Result;
use anyhow::bail;
use lazy_static::lazy_static;
use sdl2::{
    audio::{AudioQueue, AudioSpecDesired},
    AudioSubsystem,
};
use std::{
    collections::BTreeMap, fs::File, io::Read, path::Path, str::FromStr,
};

pub const SAMPLE_RATE: i32 = 48000;
const AUDIO_CHANNELS: usize = 5;
const BASE_FREQUENCY: f64 = 1193180.0;
const TONE_VOLUME: i16 = 3000;
const TONE_DURATION_MS: usize = 11;
const TONE_DURATION_SAMPLES: usize =
    SAMPLE_RATE as usize / 1000 * TONE_DURATION_MS;

lazy_static! {
    pub static ref AUDIO_SPEC: AudioSpecDesired = AudioSpecDesired {
        freq: Some(SAMPLE_RATE),
        channels: Some(1),
        samples: None,
    };
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SoundLevel {
    Low,
    High,
}

impl SoundLevel {
    fn change(&mut self) {
        *self = match *self {
            SoundLevel::High => SoundLevel::Low,
            SoundLevel::Low => SoundLevel::High,
        }
    }
}

struct SquareFrequencyIterator {
    countdown: usize,
    level: SoundLevel,
    frequency_divider: u16,
}

impl SquareFrequencyIterator {
    fn set_frequency_divider(&mut self, frequency_divider: u16) {
        self.frequency_divider = frequency_divider;
        let maxcount = equal_frames_for_divider(frequency_divider);
        self.countdown = self.countdown.min(maxcount);
    }
}

impl Iterator for SquareFrequencyIterator {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.countdown == 0 {
            self.countdown =
                equal_frames_for_divider(self.frequency_divider);
            self.level.change();
        }
        if self.countdown == 0 {
            None
        } else {
            self.countdown -= 1;
            Some(match self.level {
                SoundLevel::High => TONE_VOLUME,
                SoundLevel::Low => -TONE_VOLUME,
            })
        }
    }
}

fn equal_frames_for_divider(divider: u16) -> usize {
    let frequency = BASE_FREQUENCY / divider as f64;
    (SAMPLE_RATE as f64 / frequency) as usize / 2
}

fn build_sound_from_raw_data(data: &[u16]) -> Vec<i16> {
    let mut frequency_iterator = SquareFrequencyIterator {
        countdown: 0,
        level: SoundLevel::Low,
        frequency_divider: 0,
    };

    let mut frames = Vec::new();
    for d in data {
        frequency_iterator.set_frequency_divider(*d);
        for _ in 0..TONE_DURATION_SAMPLES {
            if let Some(s) = frequency_iterator.next() {
                frames.push(s);
            }
        }
    }

    frames
}

fn read_raw_data(raw: &[u8]) -> Result<BTreeMap<String, Vec<i16>>> {
    if raw[..4]
        != "SND"
            .as_bytes()
            .into_iter()
            .copied()
            .chain(std::iter::once(0u8))
            .collect::<Vec<u8>>()
    {
        bail!("Invalid sound data");
    }

    let mut entries = BTreeMap::new();
    let mut previous_info = None;

    for i in 1..=24 {
        let offset: usize = 16 * i;
        let address =
            u16::from_le_bytes([raw[offset], raw[offset + 1]]) as usize;
        let name_bytes: Vec<u8> = raw[offset + 4..offset + 16]
            .iter()
            .cloned()
            .filter(|c| *c != 0u8)
            .collect();
        let name = std::str::from_utf8(&name_bytes)?.to_string();
        if let Some((previous_address, previous_name)) =
            previous_info.clone()
        {
            let raw_u16 = raw[previous_address..address]
                .chunks(2)
                .filter_map(|c| {
                    if c.len() == 2 {
                        Some(u16::from_le_bytes([c[0], c[1]]))
                    } else {
                        None
                    }
                })
                .collect::<Vec<u16>>();
            let sound_data = build_sound_from_raw_data(&raw_u16);
            entries.insert(previous_name, sound_data);
        }
        previous_info = Some((address as usize, name));
    }

    Ok(entries)
}

pub struct SoundCache {
    sounds: BTreeMap<SoundIndex, Vec<i16>>,
}

impl SoundCache {
    pub fn load_from_path(path: &Path) -> Result<Self> {
        let mut sounds = BTreeMap::new();

        for filename in &["duke1.dn1", "duke1-b.dn1"] {
            let path = path.join(filename);
            let mut raw = Vec::new();
            let mut file = File::open(path)?;
            file.read_to_end(&mut raw)?;
            for (name, sound) in read_raw_data(&raw)? {
                if let Ok(index) = SoundIndex::from_str(&name) {
                    sounds.insert(index, sound);
                }
            }
        }

        Ok(SoundCache { sounds })
    }

    pub fn get_sound(&self, index: SoundIndex) -> Option<&Vec<i16>> {
        self.sounds.get(&index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SoundIndex {
    PLAYERDEATH,
    GETFOODITEM,
    COKECANHIT,
    GETBALLON,
    GETPOWERUP,
    BRIDGEXTEND,
    PLAYERGUN,
    TELEPORT,
    PLAYERQUIT,
    GETKEY,
    BOXEXPLODE,
    ENEMYSHOT,
    SPECIALITEM,
    PLAYERJUMP,
    PLAYERLAND,
    PLAYERHIT,
    GETBONUSOBJ,
    HITREACTOR,
    SMALLDEATH,
    LEVELDONE,
    ELEVATOR,
    FORCEFIELD,
    WALKING,
    HIGHSCORE,
    CHEATMODE,
    STARTGAME,
    CLINGHOOKS,
    READNOTE,
    MONITOR,
    ROCKET,
    OPENKEYDOOR,
    DANDERSIGN,
    BOMBEXPLODE,
    MINEBOUNCE,
    RABBITGONE,
    REACTORSND,
    GETDUKESND,
    HITHEAD,
    DOORSND,
    BADGUYGOUP,
    BADGUYISDED,
    HITABREAKER,
    TORCHON,
    THEND,
}

impl FromStr for SoundIndex {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "PLAYERDEATH" => Ok(SoundIndex::PLAYERDEATH),
            "GETFOODITEM" => Ok(SoundIndex::GETFOODITEM),
            "COKECANHIT" => Ok(SoundIndex::COKECANHIT),
            "GETBALLON" => Ok(SoundIndex::GETBALLON),
            "GETPOWERUP" => Ok(SoundIndex::GETPOWERUP),
            "BRIDGEXTEND" => Ok(SoundIndex::BRIDGEXTEND),
            "PLAYERGUN" => Ok(SoundIndex::PLAYERGUN),
            "TELEPORT" => Ok(SoundIndex::TELEPORT),
            "PLAYERQUIT" => Ok(SoundIndex::PLAYERQUIT),
            "GETKEY" => Ok(SoundIndex::GETKEY),
            "BOXEXPLODE" => Ok(SoundIndex::BOXEXPLODE),
            "ENEMYSHOT" => Ok(SoundIndex::ENEMYSHOT),
            "SPECIALITEM" => Ok(SoundIndex::SPECIALITEM),
            "PLAYERJUMP" => Ok(SoundIndex::PLAYERJUMP),
            "PLAYERLAND" => Ok(SoundIndex::PLAYERLAND),
            "PLAYERHIT" => Ok(SoundIndex::PLAYERHIT),
            "GETBONUSOBJ" => Ok(SoundIndex::GETBONUSOBJ),
            "HITREACTOR" => Ok(SoundIndex::HITREACTOR),
            "SMALLDEATH" => Ok(SoundIndex::SMALLDEATH),
            "LEVELDONE" => Ok(SoundIndex::LEVELDONE),
            "ELEVATOR" => Ok(SoundIndex::ELEVATOR),
            "FORCEFIELD" => Ok(SoundIndex::FORCEFIELD),
            "WALKING" => Ok(SoundIndex::WALKING),
            "HIGHSCORE" => Ok(SoundIndex::HIGHSCORE),
            "CHEATMODE" => Ok(SoundIndex::CHEATMODE),
            "STARTGAME" => Ok(SoundIndex::STARTGAME),
            "CLINGHOOKS" => Ok(SoundIndex::CLINGHOOKS),
            "READNOTE" => Ok(SoundIndex::READNOTE),
            "MONITOR" => Ok(SoundIndex::MONITOR),
            "ROCKET" => Ok(SoundIndex::ROCKET),
            "OPENKEYDOOR" => Ok(SoundIndex::OPENKEYDOOR),
            "DANDERSIGN" => Ok(SoundIndex::DANDERSIGN),
            "BOMBEXPLODE" => Ok(SoundIndex::BOMBEXPLODE),
            "MINEBOUNCE" => Ok(SoundIndex::MINEBOUNCE),
            "RABBITGONE" => Ok(SoundIndex::RABBITGONE),
            "REACTORSND" => Ok(SoundIndex::REACTORSND),
            "GETDUKESND" => Ok(SoundIndex::GETDUKESND),
            "HITHEAD" => Ok(SoundIndex::HITHEAD),
            "DOORSND" => Ok(SoundIndex::DOORSND),
            "BADGUYGOUP" => Ok(SoundIndex::BADGUYGOUP),
            "BADGUYISDED" => Ok(SoundIndex::BADGUYISDED),
            "HITABREAKER" => Ok(SoundIndex::HITABREAKER),
            "TORCHON" => Ok(SoundIndex::TORCHON),
            "THEND" => Ok(SoundIndex::THEND),
            other => bail!("Unknown sound index {:?}", other),
        }
    }
}

pub struct SoundPlayer<'a> {
    devices: Vec<(AudioQueue<i16>, Option<SoundIndex>)>,
    soundcache: &'a SoundCache,
}

impl<'a> SoundPlayer<'a> {
    pub fn create(
        audio_subsystem: &AudioSubsystem,
        soundcache: &'a SoundCache,
    ) -> Self {
        let devices: Vec<_> = (0..AUDIO_CHANNELS)
            .into_iter()
            .filter_map(|_| {
                audio_subsystem
                    .open_queue::<i16, _>(None, &AUDIO_SPEC)
                    .ok()
            })
            .map(|d| (d, None))
            .collect();

        SoundPlayer {
            devices,
            soundcache,
        }
    }

    pub fn play_sound(&mut self, index: SoundIndex) {
        if let Some(sound) = self.soundcache.get_sound(index) {
            if let Some(i) = self.reserve_device_for_sound(index) {
                self.devices[i].0.clear();
                self.devices[i].0.queue_audio(sound).ok();
                self.devices[i].0.resume();
            }
        }
    }

    fn reserve_device_for_sound(
        &mut self,
        index: SoundIndex,
    ) -> Option<usize> {
        for i in 0..self.devices.len() {
            if self.devices[i].0.size() == 0 {
                self.devices[i].1 = None;
            }
        }

        for i in 0..self.devices.len() {
            if self.devices[i].1 == Some(index) {
                return Some(i);
            }
        }

        for i in 0..self.devices.len() {
            if self.devices[i].1 == None {
                self.devices[i].1 = Some(index);
                return Some(i);
            }
        }

        None
    }
}
