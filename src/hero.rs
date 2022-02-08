// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::{
    actor::{ActorType, SingleAnimationType},
    game::GameCommands,
    level::tiles::LevelTiles,
    rendering::Renderer,
    HorizontalDirection, KeyColor, Result, Sizes, SoundIndex,
    HERO_FALLING_LEFT, HERO_FALLING_RIGHT, HERO_JUMPING_LEFT,
    HERO_JUMPING_LEFT_SOMERSAULT, HERO_JUMPING_RIGHT,
    HERO_JUMPING_RIGHT_SOMERSAULT, HERO_NUM_FALLING, HERO_NUM_JUMPING,
    HERO_NUM_STANDING, HERO_NUM_WALKING, HERO_SKELETON_LEFT,
    HERO_SKELETON_RIGHT, HERO_STANDING_LEFT, HERO_STANDING_RIGHT,
    HERO_WALKING_LEFT, HERO_WALKING_RIGHT, LEVEL_HEIGHT, LEVEL_WIDTH,
};
use sdl2::rect::{Point, Rect};
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    NotMoving,
    Walking,
}

#[derive(Debug)]
pub struct Hero {
    pub position: Position,
    pub score: Score,
    pub health: Health,
    pub firepower: Firepower,
    pub inventory: Inventory,
    pub fetched_letter_state: FetchedLetterState,
    pub immunity: Immunity,
    pub hidden: bool,
    pub direction: HorizontalDirection,
    just_turned_around: bool,
    pub motion: Motion,
    is_in_the_air: bool,
    pub is_shooting: bool,
    counter: usize,
    somersault: Option<usize>,
    base_tile_number: usize,
    current_frame: usize,
    num_frames: usize,
    vertical_speed: i32,
    pub gets_hurt: bool,
}

impl Hero {
    pub fn new(sizes: &dyn Sizes) -> Self {
        Hero {
            position: Position::new(sizes),
            score: Score::new(),
            health: Health::new(),
            firepower: Firepower::new(),
            inventory: Inventory::new(),
            fetched_letter_state: FetchedLetterState::new(),
            immunity: Immunity::new(),
            hidden: false,
            direction: HorizontalDirection::Right,
            just_turned_around: false,
            motion: Motion::NotMoving,
            is_in_the_air: false,
            is_shooting: false,
            counter: 0,
            somersault: None,
            base_tile_number: HERO_STANDING_RIGHT,
            current_frame: 0,
            num_frames: 1,
            vertical_speed: 0i32,
            gets_hurt: false,
        }
    }

    pub fn reset(&mut self, sizes: &dyn Sizes) {
        self.position.reset(sizes);
        self.score.reset();
        self.health.reset();
        self.firepower.reset();
        self.inventory.reset();
        self.fetched_letter_state.reset();
        self.immunity.reset();
        self.hidden = false;
        self.direction = HorizontalDirection::Right;
        self.just_turned_around = false;
        self.motion = Motion::NotMoving;
        self.is_in_the_air = false;
        self.is_shooting = false;
        self.counter = 0;
        self.base_tile_number = HERO_STANDING_RIGHT;
        self.current_frame = 0;
        self.num_frames = 1;
        self.vertical_speed = 0i32;
        self.gets_hurt = false;
    }

    pub fn reset_for_level(&mut self) {
        self.direction = HorizontalDirection::Right;
        self.inventory.unset(InventoryItem::Key(KeyColor::Red));
        self.inventory.unset(InventoryItem::Key(KeyColor::Green));
        self.inventory.unset(InventoryItem::Key(KeyColor::Blue));
        self.inventory.unset(InventoryItem::Key(KeyColor::Pink));
        self.fetched_letter_state.reset();
        self.immunity.reset();
        self.hidden = false;
        self.just_turned_around = false;
        self.motion = Motion::NotMoving;
        self.is_in_the_air = false;
        self.is_shooting = false;
        self.counter = 0;
        self.base_tile_number = HERO_STANDING_RIGHT;
        self.current_frame = 0;
        self.num_frames = 1;
        self.vertical_speed = 0i32;
        self.gets_hurt = false;
    }

    pub fn next_frame(&mut self) {
        self.current_frame += 1;
        self.current_frame %= self.num_frames;
    }

    pub fn render(
        &self,
        renderer: &mut dyn Renderer,
        sizes: &dyn Sizes,
        tiles: &LevelTiles,
        draw_collision_bounds: bool,
    ) -> Result<()> {
        if self.hidden || self.immunity.hero_invisible() {
            return Ok(());
        }

        let mut point = self.position.geometry.top_left();
        point.x -= sizes.half_width() as i32;

        let base_tile_number = if self.immunity.hero_skeleton() {
            match self.direction {
                HorizontalDirection::Left => HERO_SKELETON_LEFT,
                HorizontalDirection::Right => HERO_SKELETON_RIGHT,
            }
        } else {
            self.base_tile_number
        };

        renderer.place_tile(base_tile_number, point)?;
        point.x += sizes.width() as i32;
        renderer.place_tile(base_tile_number + 1, point)?;
        point.x -= sizes.width() as i32;
        point.y += sizes.height() as i32;
        renderer.place_tile(base_tile_number + 2, point)?;
        point.x += sizes.width() as i32;
        renderer.place_tile(base_tile_number + 3, point)?;

        if draw_collision_bounds {
            let color = crate::collision_bounds_color();
            let g = self.position.geometry;

            renderer.draw_rect(g, color)?;

            for i in (g.x / sizes.width() as i32) - 1
                ..(g.x / sizes.width() as i32) + 2
            {
                for j in (g.y / sizes.height() as i32) - 1
                    ..(g.y / sizes.height() as i32) + 3
                {
                    if i > 0
                        && j > 0
                        && tiles
                            .get(i, j)
                            .map(|t| t.solid)
                            .unwrap_or(false)
                    {
                        let obstacle = Rect::new(
                            i * sizes.width() as i32,
                            j * sizes.height() as i32,
                            sizes.width(),
                            sizes.height(),
                        );
                        renderer.draw_rect(obstacle, color)?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn enter_level(&mut self, x: i32, y: i32) {
        self.position.geometry.x = x;
        self.position.geometry.y = y;
        self.reset_for_level();
    }

    pub fn would_collide(
        &self,
        sizes: &dyn Sizes,
        tiles: &LevelTiles,
        x: i32,
        y: i32,
    ) -> bool {
        let mut destination = self.position.geometry;
        destination.x = x;
        destination.y = y;
        tiles.collides(sizes, destination)
    }

    pub fn update_animation(&mut self) {
        self.base_tile_number = if self.is_in_the_air {
            // hero is jumping or falling
            if self.counter > 0 {
                // hero is jumping
                self.num_frames = HERO_NUM_JUMPING;
                if let Some(frame) = self.somersault {
                    self.direction.map(
                        HERO_JUMPING_LEFT_SOMERSAULT,
                        HERO_JUMPING_RIGHT_SOMERSAULT,
                    ) + 4 * (frame / 2)
                } else {
                    self.direction
                        .map(HERO_JUMPING_LEFT, HERO_JUMPING_RIGHT)
                }
            } else {
                // hero is falling
                self.num_frames = HERO_NUM_FALLING;
                if let Some(frame) = self.somersault {
                    self.direction.map(
                        HERO_JUMPING_LEFT_SOMERSAULT,
                        HERO_JUMPING_RIGHT_SOMERSAULT,
                    ) + 4 * (frame / 2)
                } else {
                    self.direction
                        .map(HERO_FALLING_LEFT, HERO_FALLING_RIGHT)
                }
            }
        } else {
            // hero is standing or walking on ground
            if self.motion == Motion::NotMoving {
                // hero is standing
                self.num_frames = HERO_NUM_STANDING;
                if self.is_shooting {
                    self.direction
                        .map(HERO_WALKING_LEFT, HERO_WALKING_RIGHT)
                        + 12
                } else {
                    self.direction
                        .map(HERO_STANDING_LEFT, HERO_STANDING_RIGHT)
                }
            } else {
                // hero is walking
                self.num_frames = HERO_NUM_WALKING;
                self.direction.map(HERO_WALKING_LEFT, HERO_WALKING_RIGHT)
                    + 4 * self.current_frame
            }
        };
    }

    pub fn jump(&mut self, game_commands: &mut dyn GameCommands) {
        if !self.is_in_the_air {
            let (vertical_speed, somersault) =
                if self.inventory.is_set(InventoryItem::Boot) {
                    let somersault = {
                        use rand::Rng;
                        let mut rng = rand::thread_rng();
                        if self.motion == Motion::Walking
                            && rng.gen_range(0u8..5u8) == 0u8
                        {
                            Some(0)
                        } else {
                            None
                        }
                    };
                    (JUMP_PROFILE_HIGH[0], somersault)
                } else {
                    (JUMP_PROFILE_DEFAULT[0], None)
                };
            self.counter = JUMP_PROFILE_DEFAULT.len();
            self.somersault = somersault;
            self.vertical_speed = vertical_speed;
            self.is_in_the_air = true;
            game_commands.add_sound(SoundIndex::PLAYERJUMP);
        }
    }

    pub fn land(&mut self) {
        self.vertical_speed = 0i32;
        self.is_in_the_air = false;
        self.somersault = None;
        self.counter = 0;
    }

    fn fall(&mut self) {
        self.is_in_the_air = true;
        self.counter = 0;
    }

    /// Returns the remaining health
    pub fn act(
        &mut self,
        sizes: &dyn Sizes,
        tiles: &LevelTiles,
        game_commands: &mut dyn GameCommands,
    ) -> Result<()> {
        self.immunity.count_down();
        if !self.immunity.hero_is_protected() && self.gets_hurt {
            self.immunity.enable();
            self.health.decrease(1);
            game_commands.add_sound(SoundIndex::PLAYERHIT);
            // when jumping, this jump should be interrupted
            // just as if the hero had bumped against a ceiling
            self.counter = 0;
        }

        self.somersault = match self.somersault {
            Some(frame) if frame == 13 => None,
            Some(frame) => Some(frame + 1),
            None => None,
        };

        if self.motion == Motion::Walking {
            // the hero is moving
            if self.just_turned_around {
                self.just_turned_around = false;
            } else {
                let mut new_position = self.position.geometry;
                match self.direction {
                    HorizontalDirection::Left => {
                        new_position.x -= sizes.half_width() as i32;
                    }
                    HorizontalDirection::Right => {
                        new_position.x += sizes.half_width() as i32;
                    }
                }
                if !self.would_collide(
                    sizes,
                    tiles,
                    new_position.x,
                    new_position.y,
                ) {
                    if !self.is_in_the_air {
                        game_commands.add_sound(SoundIndex::WALKING);
                    }
                    self.position.move_to(
                        sizes,
                        new_position.x(),
                        new_position.y(),
                    );
                }
            }
        }

        if !self.is_in_the_air {
            // the hero is standing or walking
            self.vertical_speed = 0i32;
        } else {
            // the hero is jumping or falling
            if self.counter > 0 {
                // the hero is jumping
                self.counter -= 1;
                let index = JUMP_PROFILE_DEFAULT.len() - 1 - self.counter;
                self.vertical_speed =
                    if self.inventory.is_set(InventoryItem::Boot) {
                        JUMP_PROFILE_HIGH[index]
                    } else {
                        JUMP_PROFILE_DEFAULT[index]
                    };

                for _ in 0..self.vertical_speed {
                    let geometry = self.position.geometry;
                    if !self.would_collide(
                        sizes,
                        tiles,
                        geometry.x,
                        geometry.y - 1,
                    ) {
                        self.position.move_y_to(sizes, geometry.y() - 1);
                    } else {
                        // hero bumped against the ceiling
                        self.counter = 0;
                        game_commands.add_sound(SoundIndex::HITHEAD);
                    }
                }
            } else {
                // the hero is falling
                self.vertical_speed = 8;

                for _ in 0..self.vertical_speed {
                    let geometry = self.position.geometry;
                    if !self.would_collide(
                        sizes,
                        tiles,
                        geometry.x,
                        geometry.y + 1,
                    ) {
                        self.position.move_y_to(sizes, geometry.y() + 1);
                    }
                }
            }
        }

        let geometry = self.position.geometry;
        if self.would_collide(
            sizes,
            tiles,
            geometry.x,
            geometry.y + sizes.half_height() as i32,
        ) {
            if self.is_in_the_air {
                game_commands.add_actor(
                    ActorType::SingleAnimation(
                        SingleAnimationType::DustCloud,
                    ),
                    Point::new(
                        self.position.geometry.x(),
                        self.position.geometry.y() + sizes.height() as i32,
                    ),
                );
                game_commands.add_sound(SoundIndex::PLAYERLAND);
            }
            // the hero is standing on solid ground
            self.land();
        } else {
            // the hero is falling down
            if self.counter == 0 {
                self.fall();
            }
        }

        Ok(())
    }
}

const JUMP_PROFILE_DEFAULT: [i32; 9] = [13, 11, 9, 7, 5, 3, 0, 0, 0];
const JUMP_PROFILE_HIGH: [i32; 9] = [15, 13, 11, 9, 7, 5, 3, 1, 0];

#[derive(Debug)]
pub struct Position {
    pub geometry: Rect,
}

impl Position {
    pub fn new(sizes: &dyn Sizes) -> Self {
        Self {
            geometry: Self::default_geometry(sizes),
        }
    }

    pub fn reset(&mut self, sizes: &dyn Sizes) {
        self.geometry = Self::default_geometry(sizes);
    }

    fn default_geometry(sizes: &dyn Sizes) -> Rect {
        Rect::new(0, 0, sizes.width(), sizes.height() * 2)
    }

    pub fn move_to(&mut self, sizes: &dyn Sizes, x: i32, y: i32) {
        self.geometry.x =
            std::cmp::min(x, LEVEL_WIDTH as i32 * sizes.width() as i32);
        self.geometry.y =
            std::cmp::min(y, LEVEL_HEIGHT as i32 * sizes.height() as i32);
    }

    pub fn move_x_to(&mut self, sizes: &dyn Sizes, x: i32) {
        self.move_to(sizes, x, self.geometry.y())
    }

    pub fn move_y_to(&mut self, sizes: &dyn Sizes, y: i32) {
        self.move_to(sizes, self.geometry.x(), y)
    }

    pub fn push_vertically(
        &mut self,
        sizes: &dyn Sizes,
        tiles: &LevelTiles,
        offset: i32,
    ) -> i32 {
        if offset == 0 {
            return 0;
        }
        let mut geometry = self.geometry;
        geometry.y += offset;

        if !tiles.collides(sizes, geometry) {
            self.move_y_to(sizes, geometry.y());
            return offset;
        }

        let offset_absolute = offset.abs();
        let direction = offset / offset_absolute;

        for i in 0..offset_absolute {
            geometry.offset(0, -direction);
            if !tiles.collides(sizes, geometry) {
                self.move_y_to(sizes, geometry.y());
                return i * direction;
            }
        }
        0
    }

    pub fn push_horizontally(
        &mut self,
        sizes: &dyn Sizes,
        tiles: &LevelTiles,
        offset: i32,
    ) -> i32 {
        if offset == 0 {
            return 0;
        }
        let mut geometry = self.geometry;
        geometry.offset(offset, 0);

        if !tiles.collides(sizes, geometry) {
            self.move_x_to(sizes, geometry.x());
            return offset;
        }

        let offset_absolute = offset.abs();
        let direction = offset / offset_absolute;

        for i in 0..offset_absolute {
            geometry.offset(-direction, 0);
            if !tiles.collides(sizes, geometry) {
                self.move_x_to(sizes, geometry.x());
                return i * direction;
            }
        }
        0
    }
}

#[derive(Debug)]
pub struct Immunity {
    countdown: usize,
}

impl Immunity {
    const DURATION: usize = 16;

    fn new() -> Self {
        Immunity { countdown: 0 }
    }

    fn reset(&mut self) {
        self.countdown = 0;
    }

    fn enable(&mut self) {
        self.countdown = Self::DURATION;
    }

    fn hero_invisible(&self) -> bool {
        self.countdown % 2 > 0
    }

    fn hero_skeleton(&self) -> bool {
        self.countdown == Self::DURATION
    }

    fn hero_is_protected(&self) -> bool {
        self.countdown > 0
    }

    fn count_down(&mut self) {
        if self.countdown > 0 {
            self.countdown -= 1;
        }
    }
}

#[derive(Debug)]
pub struct Score {
    count: u128,
}

impl Default for Score {
    fn default() -> Self {
        Self::new()
    }
}

impl Score {
    pub fn new() -> Self {
        Score { count: 0 }
    }

    pub fn add(&mut self, amount: u128) {
        self.count = self.count.saturating_add(amount);
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }

    pub fn set_value(&mut self, value: u128) {
        self.count = value;
    }

    pub fn value(&self) -> u128 {
        self.count
    }
}

#[derive(Debug)]
pub struct Health {
    life: Option<u8>,
}

impl Default for Health {
    fn default() -> Self {
        Health { life: Some(8u8) }
    }
}

impl Health {
    pub const MAX: u8 = 8;

    pub fn new() -> Self {
        Health::default()
    }

    pub fn reset(&mut self) {
        self.life = Some(Self::MAX);
    }

    pub fn set(&mut self, count: u8) {
        self.life = Some(std::cmp::min(Self::MAX, count));
    }

    pub fn increase(&mut self, count: u8) {
        self.life = match self.life {
            Some(life) => Some(std::cmp::min(Self::MAX, life + count)),
            None => None,
        }
    }

    pub fn decrease(&mut self, count: u8) {
        self.life = match self.life {
            Some(life) if life < count => None,
            Some(life) => Some(life - count),
            None => None,
        };
    }

    pub fn fill_max(&mut self) {
        self.life = Some(Self::MAX);
    }

    pub fn kill(&mut self) {
        self.life = None
    }

    pub fn life(&self) -> Option<u8> {
        self.life
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
    }

    pub fn reset(&mut self) {
        self.shots = 1;
    }

    pub fn num_shots(&self) -> u8 {
        self.shots
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum InventoryItem {
    Key(KeyColor),
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
    }

    pub fn get_items(&self) -> std::collections::BTreeSet<InventoryItem> {
        self.items.clone()
    }

    pub fn set_items(
        &mut self,
        items: std::collections::BTreeSet<InventoryItem>,
    ) {
        self.items = items;
    }

    pub fn set(&mut self, item: InventoryItem) {
        self.items.insert(item);
    }

    pub fn unset(&mut self, item: InventoryItem) {
        self.items.remove(&item);
    }

    pub fn is_set(&self, item: InventoryItem) -> bool {
        self.items.contains(&item)
    }
}

#[derive(Debug)]
pub struct FetchedLetterState {
    last_fetched: Option<FetchedLetter>,
}

impl Default for FetchedLetterState {
    fn default() -> Self {
        Self::new()
    }
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
