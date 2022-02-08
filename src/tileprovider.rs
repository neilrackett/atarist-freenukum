// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use sdl2::surface::Surface;

pub type TileIndex = usize;

pub trait TileProvider {
    fn get_tile(&self, index: TileIndex) -> Option<&Surface>;
}
