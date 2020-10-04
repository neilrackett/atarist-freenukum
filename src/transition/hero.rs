use crate::FN_EVENT_HEROSCORED;

pub struct Score {
    count: u64,
}

impl Score {
    pub fn add(&mut self, amount: u64) {
        self.count = self.count.saturating_add(amount);
        self.emit_score_update();
    }

    pub fn reset(&mut self) {
        self.count = 0;
        self.emit_score_update();
    }

    fn emit_score_update(&self) {
        transdl::event::push_user_event(FN_EVENT_HEROSCORED);
    }
}

pub mod ffi {
    pub type FnHeroScore = super::Score;

    #[no_mangle]
    pub extern "C" fn fn_hero_score_create() -> *mut FnHeroScore {
        Box::into_raw(Box::new(FnHeroScore { count: 0 }))
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_score_free(ptr: *mut FnHeroScore) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_score_add(
        score: *mut FnHeroScore,
        amount: u64,
    ) {
        assert!(!score.is_null());
        let score: &mut FnHeroScore = unsafe { &mut (*score) };
        score.add(amount);
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_score_reset(score: *mut FnHeroScore) {
        assert!(!score.is_null());
        let score: &mut FnHeroScore = unsafe { &mut (*score) };
        score.reset();
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_score_get(score: *const FnHeroScore) -> u64 {
        assert!(!score.is_null());
        let score: &FnHeroScore = unsafe { &(*score) };
        score.count
    }
}
