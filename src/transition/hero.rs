use super::geometry::Geometry;
use super::level::solids::LevelSolids;
use super::{HorizontalDirection, UserEvent};
use crate::{LEVEL_HEIGHT, LEVEL_WIDTH, TILE_HEIGHT, TILE_WIDTH};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct HeroData {
    pub position: Position,
    pub score: Score,
    pub health: Health,
    pub firepower: Firepower,
    pub inventory: Inventory,
    pub fetched_letter_state: FetchedLetterState,
    pub hidden: bool,
    pub direction: HorizontalDirection,
}

impl HeroData {
    pub fn new() -> Self {
        HeroData {
            position: Position::new(),
            score: Score::new(),
            health: Health::new(),
            firepower: Firepower::new(),
            inventory: Inventory::new(),
            fetched_letter_state: FetchedLetterState::new(),
            hidden: false,
            direction: HorizontalDirection::Right,
        }
    }

    pub fn reset(&mut self) {
        self.position.reset();
        self.score.reset();
        self.health.reset();
        self.firepower.reset();
        self.inventory.reset();
        self.fetched_letter_state.reset();
        self.hidden = false;
        self.direction = HorizontalDirection::Right;
    }

    pub fn reset_for_level(&mut self) {
        self.direction = HorizontalDirection::Right;
    }
}

#[derive(Debug)]
pub struct Position {
    pub geometry: Geometry,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            geometry: Self::default_geometry(),
        }
    }
}

impl Position {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.geometry = Self::default_geometry();
    }

    fn default_geometry() -> Geometry {
        Geometry {
            x: 0,
            y: 0,
            w: TILE_WIDTH as u16,
            h: TILE_HEIGHT as u16 * 2,
        }
    }

    pub fn move_to(&mut self, x: u16, y: u16) {
        self.geometry.x = std::cmp::min(
            x as i16,
            LEVEL_WIDTH as i16 * TILE_WIDTH as i16,
        );
        self.geometry.y = std::cmp::min(
            y as i16,
            LEVEL_HEIGHT as i16 * TILE_HEIGHT as i16,
        );
        self.emit_update()
    }

    pub fn move_x_to(&mut self, x: u16) {
        self.move_to(x, self.geometry.y as u16);
    }

    pub fn move_y_to(&mut self, y: u16) {
        self.move_to(self.geometry.x as u16, y);
    }

    pub fn move_x_by(&mut self, x: i16) {
        self.move_x_to((self.geometry.x + x) as u16);
    }

    pub fn move_y_by(&mut self, y: i16) {
        self.move_y_to((self.geometry.y + y) as u16);
    }

    pub fn push_vertically(
        &mut self,
        solids: &LevelSolids,
        offset: i16,
    ) -> i16 {
        if offset == 0 {
            return 0;
        }
        let mut geometry = self.geometry;
        geometry.y += offset;

        if !solids.collides(geometry) {
            self.move_y_to(geometry.y as u16);
            return offset;
        }

        let offset_absolute = offset.abs();
        let direction = offset / offset_absolute;

        for i in 0..offset_absolute {
            geometry.y -= direction as i16;
            if !solids.collides(geometry) {
                self.move_y_to(geometry.y as u16);
                return i * direction;
            }
        }
        return 0;
    }

    pub fn push_horizontally(
        &mut self,
        solids: &LevelSolids,
        offset: i16,
    ) -> i16 {
        if offset == 0 {
            return 0;
        }
        let mut geometry = self.geometry;
        geometry.x += offset;

        if !solids.collides(geometry) {
            self.move_x_to(geometry.x as u16);
            return offset;
        }

        let offset_absolute = offset.abs();
        let direction = offset / offset_absolute;

        for i in 0..offset_absolute {
            geometry.x -= direction as i16;
            if !solids.collides(geometry) {
                self.move_x_to(geometry.x as u16);
                return i * direction;
            }
        }
        return 0;
    }

    fn emit_update(&self) {
        transdl::event::push_user_event(UserEvent::HeroMoved as i32);
    }
}

#[derive(Debug)]
pub struct Score {
    count: u64,
}

impl Score {
    pub fn new() -> Self {
        Score { count: 0 }
    }

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

#[derive(Debug)]
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

    pub fn new() -> Self {
        Health::default()
    }

    pub fn reset(&mut self) {
        self.life = Self::MAX;
    }

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

#[derive(Debug)]
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

    pub fn new() -> Firepower {
        Firepower::default()
    }

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

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
#[repr(C)]
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

#[derive(Default, Debug)]
pub struct Inventory {
    items: std::collections::BTreeSet<InventoryItem>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            items: Default::default(),
        }
    }

    pub fn reset(&mut self) {
        self.items.clear();
    }

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

#[derive(Debug)]
pub struct FetchedLetterState {
    last_fetched: Option<FetchedLetter>,
}

impl FetchedLetterState {
    pub fn new() -> Self {
        FetchedLetterState { last_fetched: None }
    }

    pub fn picked(&mut self, letter: FetchedLetter) {
        use FetchedLetter as L;
        self.last_fetched = match (self.last_fetched, letter) {
            (_, L::D) => Some(L::D),
            (Some(L::D), L::U) => Some(L::U),
            (Some(L::U), L::K) => Some(L::K),
            (Some(L::K), L::E) => Some(L::E),
            _ => None,
        }
    }

    pub fn succeeded(&self) -> bool {
        self.last_fetched == Some(FetchedLetter::E)
    }

    pub fn reset(&mut self) {
        self.last_fetched = None;
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum FetchedLetter {
    D,
    U,
    K,
    E,
}

impl TryFrom<char> for FetchedLetter {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'D' | 'd' => Ok(FetchedLetter::D),
            'U' | 'u' => Ok(FetchedLetter::U),
            'K' | 'k' => Ok(FetchedLetter::K),
            'E' | 'e' => Ok(FetchedLetter::E),
            c => Err(format!("Unknown hero letter {:?}", c)),
        }
    }
}

pub mod ffi {
    use super::super::geometry::ffi::FnGeometry;
    use super::super::level::solids::ffi::FnLevelSolids;
    use super::super::HorizontalDirection;
    use libc::c_char;

    pub type FnHeroData = super::HeroData;
    pub type FnHeroPosition = super::Position;
    pub type FnHeroScore = super::Score;
    pub type FnHeroHealth = super::Health;
    pub type FnHeroFirepower = super::Firepower;
    pub type FnHeroInventoryItem = super::InventoryItem;
    pub type FnHeroInventory = super::Inventory;
    pub type FnHeroFetchedLetterState = super::FetchedLetterState;

    #[no_mangle]
    pub extern "C" fn fn_hero_data_create() -> *mut FnHeroData {
        Box::into_raw(Box::new(FnHeroData::new()))
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_data_free(ptr: *mut FnHeroData) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_data_get_position(
        ptr: *mut FnHeroData,
    ) -> *mut FnHeroPosition {
        assert!(!ptr.is_null());
        let d: &mut FnHeroData = unsafe { &mut (*ptr) };
        &mut d.position
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_data_set_hidden(
        ptr: *mut FnHeroData,
        hidden: bool,
    ) {
        assert!(!ptr.is_null());
        let d: &mut FnHeroData = unsafe { &mut (*ptr) };
        d.hidden = hidden;
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_data_get_hidden(
        ptr: *const FnHeroData,
    ) -> bool {
        assert!(!ptr.is_null());
        let d: &FnHeroData = unsafe { &(*ptr) };
        d.hidden
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_data_get_score(
        ptr: *mut FnHeroData,
    ) -> *mut FnHeroScore {
        assert!(!ptr.is_null());
        let d: &mut FnHeroData = unsafe { &mut (*ptr) };
        &mut d.score
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_data_get_health(
        ptr: *mut FnHeroData,
    ) -> *mut FnHeroHealth {
        assert!(!ptr.is_null());
        let d: &mut FnHeroData = unsafe { &mut (*ptr) };
        &mut d.health
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_data_get_firepower(
        ptr: *mut FnHeroData,
    ) -> *mut FnHeroFirepower {
        assert!(!ptr.is_null());
        let d: &mut FnHeroData = unsafe { &mut (*ptr) };
        &mut d.firepower
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_data_get_inventory(
        ptr: *mut FnHeroData,
    ) -> *mut FnHeroInventory {
        assert!(!ptr.is_null());
        let d: &mut FnHeroData = unsafe { &mut (*ptr) };
        &mut d.inventory
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_data_get_fetched_letter_state(
        ptr: *mut FnHeroData,
    ) -> *mut FnHeroFetchedLetterState {
        assert!(!ptr.is_null());
        let d: &mut FnHeroData = unsafe { &mut (*ptr) };
        &mut d.fetched_letter_state
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_position_move_to(
        position: *mut FnHeroPosition,
        x: u16,
        y: u16,
    ) {
        assert!(!position.is_null());
        let position: &mut FnHeroPosition = unsafe { &mut (*position) };
        position.move_to(x, y);
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_position_move_x_to(
        position: *mut FnHeroPosition,
        x: u16,
    ) {
        assert!(!position.is_null());
        let position: &mut FnHeroPosition = unsafe { &mut (*position) };
        position.move_x_to(x);
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_position_move_y_to(
        position: *mut FnHeroPosition,
        y: u16,
    ) {
        assert!(!position.is_null());
        let position: &mut FnHeroPosition = unsafe { &mut (*position) };
        position.move_y_to(y);
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_position_push_horizontally(
        position: *mut FnHeroPosition,
        solids: *const FnLevelSolids,
        offset: i16,
    ) -> i16 {
        assert!(!position.is_null());
        let position: &mut FnHeroPosition = unsafe { &mut (*position) };
        let solids: &FnLevelSolids = unsafe { &(*solids) };
        position.push_horizontally(solids, offset)
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_position_push_vertically(
        position: *mut FnHeroPosition,
        solids: *const FnLevelSolids,
        offset: i16,
    ) -> i16 {
        assert!(!position.is_null());
        let position: &mut FnHeroPosition = unsafe { &mut (*position) };
        let solids: &FnLevelSolids = unsafe { &(*solids) };
        position.push_vertically(solids, offset)
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_position_move_x_by(
        position: *mut FnHeroPosition,
        x: i16,
    ) {
        assert!(!position.is_null());
        let position: &mut FnHeroPosition = unsafe { &mut (*position) };
        position.move_x_by(x);
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_position_move_y_by(
        position: *mut FnHeroPosition,
        y: i16,
    ) {
        assert!(!position.is_null());
        let position: &mut FnHeroPosition = unsafe { &mut (*position) };
        position.move_y_by(y);
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_position_get_geometry(
        position: *const FnHeroPosition,
    ) -> FnGeometry {
        assert!(!position.is_null());
        let position: &FnHeroPosition = unsafe { &(*position) };
        position.geometry
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
    pub extern "C" fn fn_hero_data_reset(data: *mut FnHeroData) {
        assert!(!data.is_null());
        let data: &mut FnHeroData = unsafe { &mut (*data) };
        data.reset();
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_data_reset_for_level(data: *mut FnHeroData) {
        assert!(!data.is_null());
        let data: &mut FnHeroData = unsafe { &mut (*data) };
        data.reset_for_level();
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_data_get_direction(
        data: *const FnHeroData,
    ) -> HorizontalDirection {
        assert!(!data.is_null());
        let data: &FnHeroData = unsafe { &(*data) };
        data.direction
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_data_set_direction(
        data: *mut FnHeroData,
        direction: HorizontalDirection,
    ) {
        assert!(!data.is_null());
        let data: &mut FnHeroData = unsafe { &mut (*data) };
        data.direction = direction;
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_score_get(score: *const FnHeroScore) -> u64 {
        assert!(!score.is_null());
        let score: &FnHeroScore = unsafe { &(*score) };
        score.count
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
    pub extern "C" fn fn_hero_firepower_increase(
        firepower: *mut FnHeroFirepower,
        amount: u8,
    ) {
        assert!(!firepower.is_null());
        let firepower: &mut FnHeroFirepower = unsafe { &mut (*firepower) };
        firepower.increase(amount);
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

    #[no_mangle]
    pub extern "C" fn fn_hero_fetched_letter_state_picked(
        ptr: *mut FnHeroFetchedLetterState,
        letter: c_char,
    ) {
        assert!(!ptr.is_null());
        let state: &mut FnHeroFetchedLetterState = unsafe { &mut (*ptr) };
        use std::convert::TryFrom;
        let picked =
            super::FetchedLetter::try_from(letter as u8 as char).unwrap();
        state.picked(picked);
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_fetched_letter_state_reset(
        ptr: *mut FnHeroFetchedLetterState,
    ) {
        assert!(!ptr.is_null());
        let state: &mut FnHeroFetchedLetterState = unsafe { &mut (*ptr) };
        state.reset();
    }

    #[no_mangle]
    pub extern "C" fn fn_hero_fetched_letter_state_succeeded(
        ptr: *const FnHeroFetchedLetterState,
    ) -> bool {
        assert!(!ptr.is_null());
        let state: &FnHeroFetchedLetterState = unsafe { &(*ptr) };
        state.succeeded()
    }
}
