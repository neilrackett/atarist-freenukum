use super::data::original_data_dir;

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
        return true;
    }

    pub fn switch(&mut self) -> usize {
        self.current += 1;
        self.current %= self.count;
        self.current
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

pub mod ffi {
    pub type FnEpisodes = super::Episodes;

    #[no_mangle]
    pub extern "C" fn fn_episodes_find_installed() -> *mut FnEpisodes {
        Box::into_raw(Box::new(super::Episodes::find_installed()))
    }

    #[no_mangle]
    pub extern "C" fn fn_episodes_switch(
        episodes: *mut FnEpisodes,
    ) -> usize {
        assert!(!episodes.is_null());
        let episodes = unsafe { &mut (*episodes) };
        episodes.switch()
    }

    #[no_mangle]
    pub extern "C" fn fn_episodes_count(
        episodes: *const FnEpisodes,
    ) -> usize {
        assert!(!episodes.is_null());
        let episodes = unsafe { &(*episodes) };
        episodes.count()
    }

    #[no_mangle]
    pub extern "C" fn fn_episodes_current(
        episodes: *const FnEpisodes,
    ) -> usize {
        assert!(!episodes.is_null());
        let episodes = unsafe { &(*episodes) };
        episodes.current()
    }

    #[no_mangle]
    pub extern "C" fn fn_episodes_free(ptr: *mut FnEpisodes) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }
}
