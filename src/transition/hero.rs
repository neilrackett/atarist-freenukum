use super::UserEvent;

pub struct Score {
    count: u64,
}

impl Score {
    pub fn add(&mut self, amount: u64) {
        self.count = self.count.saturating_add(amount);
        self.emit_update();
    }

    pub fn reset(&mut self) {
        self.count = 0;
        self.emit_update();
    }

    fn emit_update(&self) {
        transdl::event::push_user_event(UserEvent::HeroScored as i32);
    }
}

pub struct Health {
    life: u8,
}

impl Default for Health {
    fn default() -> Self {
        Health { life: 8u8 }
    }
}

impl Health {
    pub const MAX: u8 = 8;

    pub fn increase(&mut self, count: u8) {
        self.life = std::cmp::min(Self::MAX, self.life + count);
        self.emit_update();
    }

    pub fn decrease(&mut self, count: u8) {
        if self.life > count {
            self.life -= count;
        } else {
            self.life = 0;
        }
        self.emit_update();
    }

    pub fn fill_max(&mut self) {
        self.life = Self::MAX;
        self.emit_update();
    }

    pub fn kill(&mut self) {
        self.life = 0;
        self.emit_update();
    }

    fn emit_update(&self) {
        transdl::event::push_user_event(
            UserEvent::HeroHealthChanged as i32,
        );
    }
}

pub struct Firepower {
    shots: u8,
}

impl Default for Firepower {
    fn default() -> Self {
        Firepower { shots: 1u8 }
    }
}

impl Firepower {
    pub const MAX: u8 = 4;

    pub fn increase(&mut self, count: u8) {
        self.shots = std::cmp::min(Self::MAX, self.shots + count);
        self.emit_update();
    }

    pub fn reset(&mut self) {
        self.shots = 1;
        self.emit_update();
    }

    pub fn num_shots(&self) -> u8 {
        self.shots
    }

    fn emit_update(&self) {
        transdl::event::push_user_event(
            UserEvent::HeroFirepowerChanged as i32,
        );
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum InventoryItem {
    KeyRed,
    KeyGreen,
    KeyBlue,
    KeyPink,
    Boot,
    Glove,
    Clamp,
    AccessCard,
}

#[derive(Default)]
pub struct Inventory {
    items: std::collections::BTreeSet<InventoryItem>,
}

impl Inventory {
    pub fn clear(&mut self) {
        self.items.clear();
        self.emit_update();
    }

    pub fn set(&mut self, item: InventoryItem) {
        self.items.insert(item);
        self.emit_update();
    }

    pub fn unset(&mut self, item: InventoryItem) {
        self.items.remove(&item);
        self.emit_update();
    }

    pub fn is_set(&self, item: InventoryItem) -> bool {
        self.items.contains(&item)
    }

    fn emit_update(&self) {
        transdl::event::push_user_event(
            UserEvent::HeroInventoryChanged as i32,
        );
    }
}

pub mod ffi {
    pub type FnHeroScore = super::Score;
    pub type FnHeroHealth = super::Health;
    pub type FnHeroFirepower = super::Firepower;
    pub type FnHeroInventoryItem = super::InventoryItem;
    pub type FnHeroInventory = super::Inventory;

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

    #[no_mangle]
    pub extern "C" fn fn_hero_health_create() -> *mut FnHeroHealth {
        Box::into_raw(Box::new(FnHeroHealth::default()))
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_health_free(ptr: *mut FnHeroHealth) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_health_increase(
        health: *mut FnHeroHealth,
        amount: u8,
    ) {
        assert!(!health.is_null());
        let health: &mut FnHeroHealth = unsafe { &mut (*health) };
        health.increase(amount);
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_health_decrease(
        health: *mut FnHeroHealth,
        amount: u8,
    ) {
        assert!(!health.is_null());
        let health: &mut FnHeroHealth = unsafe { &mut (*health) };
        health.decrease(amount);
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_health_fill_max(health: *mut FnHeroHealth) {
        assert!(!health.is_null());
        let health: &mut FnHeroHealth = unsafe { &mut (*health) };
        health.fill_max();
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_health_kill(health: *mut FnHeroHealth) {
        assert!(!health.is_null());
        let health: &mut FnHeroHealth = unsafe { &mut (*health) };
        health.kill();
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_health_get(
        health: *const FnHeroHealth,
    ) -> u8 {
        assert!(!health.is_null());
        let health: &FnHeroHealth = unsafe { &(*health) };
        health.life
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_firepower_create() -> *mut FnHeroFirepower {
        Box::into_raw(Box::new(FnHeroFirepower::default()))
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_firepower_free(ptr: *mut FnHeroFirepower) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_firepower_increase(
        firepower: *mut FnHeroFirepower,
        amount: u8,
    ) {
        assert!(!firepower.is_null());
        let firepower: &mut FnHeroFirepower = unsafe { &mut (*firepower) };
        firepower.increase(amount);
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_firepower_reset(
        firepower: *mut FnHeroFirepower,
    ) {
        assert!(!firepower.is_null());
        let firepower: &mut FnHeroFirepower = unsafe { &mut (*firepower) };
        firepower.reset();
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_firepower_num_shots(
        firepower: *const FnHeroFirepower,
    ) -> u8 {
        assert!(!firepower.is_null());
        let firepower: &FnHeroFirepower = unsafe { &(*firepower) };
        firepower.num_shots()
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_inventory_create() -> *mut FnHeroInventory {
        Box::into_raw(Box::new(FnHeroInventory::default()))
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_inventory_free(ptr: *mut FnHeroInventory) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_inventory_clear(
        inventory: *mut FnHeroInventory,
    ) {
        assert!(!inventory.is_null());
        let inventory: &mut FnHeroInventory = unsafe { &mut (*inventory) };
        inventory.clear();
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_inventory_set(
        inventory: *mut FnHeroInventory,
        item: FnHeroInventoryItem,
    ) {
        assert!(!inventory.is_null());
        let inventory: &mut FnHeroInventory = unsafe { &mut (*inventory) };
        inventory.set(item);
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_inventory_unset(
        inventory: *mut FnHeroInventory,
        item: FnHeroInventoryItem,
    ) {
        assert!(!inventory.is_null());
        let inventory: &mut FnHeroInventory = unsafe { &mut (*inventory) };
        inventory.unset(item);
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_inventory_is_set(
        inventory: *const FnHeroInventory,
        item: FnHeroInventoryItem,
    ) -> bool {
        assert!(!inventory.is_null());
        let inventory: &FnHeroInventory = unsafe { &(*inventory) };
        inventory.is_set(item)
    }
}
