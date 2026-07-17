#![allow(dead_code)] // Text API er public; HUD rendering er WIP.

//! Text rendering — tegn tekst via font atlas med custom UV per tegn.
//!
//! Bruger Sprite struct med custom UV's per character sprite.
//! Fordi SpriteBatch allerede understøtter multi-texture batching,
//! kan tekst-sprites blandes med andre sprites i samme batch.

use heat_core::{Sprite, TextureHandle, Vec2, Color};

use crate::font::{char_uv, CHAR_W, CHAR_H};

/// Tekst-renderer: konverterer strenge til sprites.
pub struct TextRenderer {
    pub font_texture: Option<TextureHandle>,
}

impl TextRenderer {
    pub fn new() -> Self {
        Self { font_texture: None }
    }

    pub fn set_font(&mut self, tex: TextureHandle) {
        self.font_texture = Some(tex);
    }

    /// Tilføj en tekst-streng til en sprite-batch.
    /// `pos`: top-left position i world/screen coords.
    /// `scale`: pixel-scale per tegn (1.0 = 8x8 px).
    /// `color`: tekst farve.
    /// `layer`: render layer.
    pub fn add_text(
        &self,
        batch: &mut Vec<Sprite>,
        pos: Vec2,
        text: &str,
        scale: f32,
        color: Color,
        layer: i32,
    ) {
        let Some(tex) = self.font_texture else { return };
        let char_w = CHAR_W as f32 * scale;
        let char_h = CHAR_H as f32 * scale;
        let mut x = pos.x;
        let y = pos.y;
        for c in text.chars() {
            if c == ' ' {
                x += char_w;
                continue;
            }
            if let Some((u0, v0, u1, v1)) = char_uv(c) {
                batch.push(Sprite {
                    texture: tex,
                    position: Vec2::new(x + char_w * 0.5, y + char_h * 0.5),
                    size: Vec2::new(char_w, char_h),
                    rotation: 0.0,
                    color,
                    layer,
                    uv_rect: Some([u0, v0, u1, v1]),
                });
            }
            x += char_w;
        }
    }

    /// Beregn tekst-bredde i pixels.
    pub fn text_width(text: &str, scale: f32) -> f32 {
        text.len() as f32 * CHAR_W as f32 * scale
    }

    /// Beregn tekst-højde i pixels.
    pub fn text_height(scale: f32) -> f32 {
        CHAR_H as f32 * scale
    }
}