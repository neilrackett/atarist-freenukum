// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: Wolfgang Silbermayr <wolfgang@silbermayr.at>

use crate::Result;
use crate::TileProvider;
use anyhow::Error;
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, RenderTarget, TextureCreator},
    surface::Surface,
};

pub type TileIndex = usize;

pub trait Renderer {
    fn place_surface(
        &mut self,
        surface: &Surface,
        rect: Rect,
    ) -> Result<()>;

    fn place_tile(
        &mut self,
        tile: TileIndex,
        destination: Point,
    ) -> Result<()>;
    fn fill_rect(&mut self, rect: Rect, color: Color) -> Result<()>;
    fn fill(&mut self, color: Color) -> Result<()>;
    fn draw_rect(&mut self, rect: Rect, color: Color) -> Result<()>;
}

pub struct MovePositionRenderer<'a> {
    pub offset_x: i32,
    pub offset_y: i32,
    pub upstream: &'a mut dyn Renderer,
}

impl<'a> Renderer for MovePositionRenderer<'a> {
    fn place_surface(
        &mut self,
        surface: &Surface,
        mut rect: Rect,
    ) -> Result<()> {
        rect.offset(self.offset_x, self.offset_y);
        self.upstream.place_surface(surface, rect)
    }

    fn place_tile(
        &mut self,
        tile: TileIndex,
        destination: Point,
    ) -> Result<()> {
        self.upstream.place_tile(
            tile,
            destination.offset(self.offset_x, self.offset_y),
        )
    }

    fn fill_rect(&mut self, mut rect: Rect, color: Color) -> Result<()> {
        rect.offset(self.offset_x, self.offset_y);
        self.upstream.fill_rect(rect, color)
    }

    fn fill(&mut self, color: Color) -> Result<()> {
        self.upstream.fill(color)
    }

    fn draw_rect(&mut self, mut rect: Rect, color: Color) -> Result<()> {
        rect.offset(self.offset_x, self.offset_y);
        self.upstream.draw_rect(rect, color)
    }
}

pub struct CanvasRenderer<'a, RT: RenderTarget, T> {
    pub canvas: &'a mut Canvas<RT>,
    pub texture_creator: &'a TextureCreator<T>,
    pub tileprovider: &'a dyn TileProvider,
}

impl<'a, RT: RenderTarget, T> Renderer for CanvasRenderer<'a, RT, T> {
    fn place_surface(
        &mut self,
        surface: &Surface,
        rect: Rect,
    ) -> Result<()> {
        self.canvas
            .copy(&surface.as_texture(&self.texture_creator)?, None, rect)
            .map_err(Error::msg)?;
        Ok(())
    }

    fn place_tile(
        &mut self,
        tile: TileIndex,
        destination: Point,
    ) -> Result<()> {
        let tile = self.tileprovider.get_tile(tile).unwrap();
        let rect = Rect::new(
            destination.x,
            destination.y,
            tile.width(),
            tile.height(),
        );
        self.canvas
            .copy(&tile.as_texture(&self.texture_creator)?, None, rect)
            .map_err(Error::msg)?;
        Ok(())
    }

    fn fill_rect(&mut self, rect: Rect, color: Color) -> Result<()> {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(rect).map_err(Error::msg)?;
        Ok(())
    }

    fn fill(&mut self, color: Color) -> Result<()> {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
        Ok(())
    }

    fn draw_rect(&mut self, rect: Rect, color: Color) -> Result<()> {
        self.canvas.set_draw_color(color);
        self.canvas.draw_rect(rect).map_err(Error::msg)?;
        Ok(())
    }
}
