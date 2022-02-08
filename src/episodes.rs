// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use super::data::original_data_dir;
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Episodes {
    current: usize,
    count: usize,
}

impl Episodes {
    pub fn find_installed() -> Self {
        let count = Self::count_installed();
        Self {
            current: 0usize,
            count,
        }
    }

    fn count_installed() -> usize {
        let mut count = 0;
        for i in 0..9 {
            if Self::is_installed(i) {
                count = i;
            }
        }
        count
    }

    fn is_installed(number: usize) -> bool {
        let data_path = original_data_dir();
        for f in super::data::required_file_names() {
            let f = format!("{}.dn{}", f, number);
            let file_path = data_path.join(f);
            match std::fs::File::open(file_path) {
                Ok(_) => {}
                Err(_) => {
                    return false;
                }
            }
        }
        true
    }

    pub fn switch_next(&mut self) -> usize {
        self.current += 1;
        self.current %= self.count;
        self.current
    }

    pub fn switch_to(&mut self, episode: usize) -> Result<()> {
        if self.count > episode {
            self.current = episode;
            Ok(())
        } else {
            Err(anyhow!(
                "Episode with number {} not installed",
                episode + 1
            ))
        }
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn file_extension(&self) -> String {
        format!("dn{}", self.current + 1)
    }
}
