#![allow(dead_code)] // Water tile er public API til fremtidige zoner.

//! Procedurale sprites — generering af detaljerede karakter- og køretøj-sprites.
//!
//! I stedet for bare farvede firkanter, genererer vi sprites med:
//! - Karakterer: hoved, krop, skuldre, retning-indikator, våben
//! - Køretøjer: chassis, ruder, hjul, detaljer
//! - Politiskilt, skygger

use image::{ImageBuffer, Rgba, RgbaImage};

/// Helper: sæt en pixel hvis indenfor bounds.
fn put(img: &mut RgbaImage, x: u32, y: u32, c: Rgba<u8>) {
    if x < img.width() && y < img.height() {
        img.put_pixel(x, y, c);
    }
}

/// Helper: sæt en pixel hvis indenfor bounds og pixelen er transparent.
fn put_if_empty(img: &mut RgbaImage, x: u32, y: u32, c: Rgba<u8>) {
    if x < img.width() && y < img.height() {
        let p = img.get_pixel(x, y);
        if p[3] == 0 {
            img.put_pixel(x, y, c);
        }
    }
}

/// Helper: tegn en udfyldt rektangel.
fn fill_rect(img: &mut RgbaImage, x0: u32, y0: u32, w: u32, h: u32, c: Rgba<u8>) {
    for y in y0..y0 + h {
        for x in x0..x0 + w {
            put(img, x, y, c);
        }
    }
}

/// Helper: tegn en outline-rektangel.
fn outline_rect(img: &mut RgbaImage, x0: u32, y0: u32, w: u32, h: u32, c: Rgba<u8>) {
    for x in x0..x0 + w {
        put(img, x, y0, c);
        put(img, x, y0 + h - 1, c);
    }
    for y in y0..y0 + h {
        put(img, x0, y, c);
        put(img, x0 + w - 1, y, c);
    }
}

/// Helper: tegn en cirkel (Bresenham).
fn fill_circle(img: &mut RgbaImage, cx: u32, cy: u32, r: u32, c: Rgba<u8>) {
    let r = r as i32;
    let cx = cx as i32;
    let cy = cy as i32;
    for y in -r..=r {
        for x in -r..=r {
            if x * x + y * y <= r * r {
                put(img, (cx + x) as u32, (cy + y) as u32, c);
            }
        }
    }
}

/// Helper: tegn en outline-cirkel.
fn outline_circle(img: &mut RgbaImage, cx: u32, cy: u32, r: u32, c: Rgba<u8>) {
    let r = r as i32;
    let cx = cx as i32;
    let cy = cy as i32;
    for y in -r..=r {
        for x in -r..=r {
            let d2 = x * x + y * y;
            if d2 >= (r - 1) * (r - 1) && d2 <= r * r {
                put(img, (cx + x) as u32, (cy + y) as u32, c);
            }
        }
    }
}

/// Shadow farve (sort, halv-transparent).
const SHADOW: Rgba<u8> = Rgba([0, 0, 0, 60]);

/// Generer en karakter-sprite (top-down, 32x32 for player, 24x24 for NPC).
/// `body_color`: kropsfarve. `head_color`: hudfarve. `accent`: våben/tilbehør farve.
/// `armed`: hvis true, tegn våben (pistol) i hånden.
/// `size`: 24 eller 32.
pub fn generate_character(
    body_color: [u8; 3],
    head_color: [u8; 3],
    accent_color: [u8; 3],
    armed: bool,
    size: u32,
) -> RgbaImage {
    let mut img = ImageBuffer::new(size, size);
    let cx = size / 2;
    let cy = size / 2;

    // Skygge (oval under karakteren).
    fill_circle(&mut img, cx, cy + size / 4, size / 6, SHADOW);

    // Krop: rektangel i midten.
    let body_w = size / 3;
    let body_h = size / 2;
    let body_x = cx - body_w / 2;
    let body_y = cy - body_h / 3;
    let body_c = Rgba([body_color[0], body_color[1], body_color[2], 255]);
    fill_rect(&mut img, body_x, body_y, body_w, body_h, body_c);
    // Mørkere kant på krop.
    let dark_body = Rgba([
        (body_color[0] as f32 * 0.6) as u8,
        (body_color[1] as f32 * 0.6) as u8,
        (body_color[2] as f32 * 0.6) as u8,
        255,
    ]);
    outline_rect(&mut img, body_x, body_y, body_w, body_h, dark_body);

    // Hoved: cirkel.
    let head_r = size / 6;
    let head_c = Rgba([head_color[0], head_color[1], head_color[2], 255]);
    fill_circle(&mut img, cx, body_y - head_r / 2, head_r, head_c);
    // Mørkere kant på hoved.
    let dark_head = Rgba([
        (head_color[0] as f32 * 0.5) as u8,
        (head_color[1] as f32 * 0.5) as u8,
        (head_color[2] as f32 * 0.5) as u8,
        255,
    ]);
    outline_circle(&mut img, cx, body_y - head_r / 2, head_r, dark_head);

    // Skuldre: små rektangler på siden af krop.
    let shoulder_w = size / 8;
    let shoulder_h = size / 4;
    let shoulder_c = Rgba([
        (body_color[0] as f32 * 0.8) as u8,
        (body_color[1] as f32 * 0.8) as u8,
        (body_color[2] as f32 * 0.8) as u8,
        255,
    ]);
    fill_rect(&mut img, body_x - shoulder_w / 2, body_y + size / 16, shoulder_w, shoulder_h, shoulder_c);
    fill_rect(&mut img, body_x + body_w - shoulder_w / 2, body_y + size / 16, shoulder_w, shoulder_h, shoulder_c);

    // Våben: pistol = lille mørk rektangel foran krop.
    if armed {
        let gun_c = Rgba([40, 40, 40, 255]);
        let gun_w = size / 6;
        let gun_h = size / 10;
        fill_rect(&mut img, cx + body_w / 4, cy - gun_h / 2, gun_w, gun_h, gun_c);
        // Pistol greb.
        fill_rect(&mut img, cx + body_w / 4, cy, gun_w / 2, gun_h, gun_c);
    }

    // Accent: lille farve på brystet (faction farve eller badge).
    let accent_c = Rgba([accent_color[0], accent_color[1], accent_color[2], 255]);
    fill_rect(&mut img, cx - size / 12, body_y + size / 10, size / 6, size / 8, accent_c);

    img
}

/// Generer en politi-sprite (top-down, 24x32).
pub fn generate_police() -> RgbaImage {
    // Mørkeblå uniform, hud-farvet hoved, gul badge.
    generate_character(
        [30, 50, 120],   // body: mørkeblå
        [200, 180, 150], // head: hud
        [255, 200, 0],   // accent: gul badge
        true,             // armed
        32,
    )
}

/// Generer en player-sprite (top-down, 32x32).
pub fn generate_player() -> RgbaImage {
    // Mørkegrå jakke, hud-farvet hoved, blå accent.
    generate_character(
        [60, 70, 90],     // body: mørkegrå jakke
        [210, 180, 140], // head: hud
        [80, 140, 220],   // accent: blå
        true,             // armed (viser pistol)
        32,
    )
}

/// Generer en NPC-sprite (top-down, 24x24).
pub fn generate_npc(body_color: [u8; 3]) -> RgbaImage {
    generate_character(
        body_color,
        [200, 180, 150], // head: hud
        [120, 120, 120], // accent: grå
        false,            // ikke armed
        24,
    )
}

/// Generer en køretøj-sprite (top-down, width x height).
/// Tegner: chassis, ruder (mørkere), hjul (mørk), farve baseret på input.
pub fn generate_vehicle(width: u32, height: u32, color: [f32; 3]) -> RgbaImage {
    let mut img = ImageBuffer::new(width, height);
    let body_c = Rgba([
        (color[0] * 255.0) as u8,
        (color[1] * 255.0) as u8,
        (color[2] * 255.0) as u8,
        255,
    ]);
    let dark_body = Rgba([
        (color[0] * 255.0 * 0.5) as u8,
        (color[1] * 255.0 * 0.5) as u8,
        (color[2] * 255.0 * 0.5) as u8,
        255,
    ]);
    let window_c = Rgba([40, 50, 70, 255]);
    let wheel_c = Rgba([25, 25, 30, 255]);
    let highlight_c = Rgba([
        (color[0] * 255.0 * 1.15).min(255.0) as u8,
        (color[1] * 255.0 * 1.15).min(255.0) as u8,
        (color[2] * 255.0 * 1.15).min(255.0) as u8,
        255,
    ]);

    // Skygge under bilen.
    fill_circle(&mut img, width / 2, height / 2, width / 3, SHADOW);

    // Chassis: hovedrektangel (lidt mindre end fuld størrelse for kant).
    let margin = 2u32;
    fill_rect(&mut img, margin, margin, width - margin * 2, height - margin * 2, body_c);
    // Highlight stripe på toppen (lysning).
    fill_rect(&mut img, margin, margin, width - margin * 2, (height / 8).max(2), highlight_c);
    // Mørkere kant.
    outline_rect(&mut img, margin, margin, width - margin * 2, height - margin * 2, dark_body);

    // Ruder: mørkere rektangel i midten.
    let win_w = (width - margin * 2) * 3 / 5;
    let win_h = (height - margin * 2) * 2 / 5;
    let win_x = (width - win_w) / 2;
    let win_y = (height - win_h) / 2;
    fill_rect(&mut img, win_x, win_y, win_w, win_h, window_c);
    // Vindue kant (lysere).
    outline_rect(&mut img, win_x, win_y, win_w, win_h, Rgba([60, 70, 90, 255]));

    // Hjul: 4 mørke rektangler i hjørnerne.
    let wheel_w = (width / 6).max(3);
    let wheel_h = (height / 8).max(3);
    fill_rect(&mut img, 0, margin, wheel_w, wheel_h, wheel_c);
    fill_rect(&mut img, width - wheel_w, margin, wheel_w, wheel_h, wheel_c);
    fill_rect(&mut img, 0, height - margin - wheel_h, wheel_w, wheel_h, wheel_c);
    fill_rect(&mut img, width - wheel_w, height - margin - wheel_h, wheel_w, wheel_h, wheel_c);

    // Motorhjelm: mørkere rektangel foran (top).
    fill_rect(&mut img, margin, margin, width - margin * 2, (height / 6).max(3), dark_body);
    // Baghjelm: mørkere rektangel bag (bottom).
    fill_rect(&mut img, margin, height - margin - (height / 6).max(3), width - margin * 2, (height / 6).max(3), dark_body);

    // Kofanger: lille hvid/stribe foran.
    let bumper_c = Rgba([200, 200, 200, 255]);
    fill_rect(&mut img, width / 3, margin / 2, width / 3, 2, bumper_c);

    img
}

/// Generer en tile-textur for asfalt (med gade-streger).
pub fn generate_asphalt_tile(tile_size: u32) -> RgbaImage {
    let mut img = ImageBuffer::new(tile_size, tile_size);
    // Mørkegrå base.
    let base = Rgba([30, 30, 35, 255]);
    fill_rect(&mut img, 0, 0, tile_size, tile_size, base);
    // Støj (subtil tekstur).
    for y in 0..tile_size {
        for x in 0..tile_size {
            let v = ((x * 7 + y * 13) % 7) as u8;
            let r = (30 + v / 3).min(255);
            let g = (30 + v / 3).min(255);
            let b = (35 + v / 3).min(255);
            put(&mut img, x, y, Rgba([r, g, b, 255]));
        }
    }
    // Gul linje i midten (gade-streg).
    let yellow = Rgba([180, 160, 40, 255]);
    fill_rect(&mut img, tile_size / 2 - 1, 0, 2, tile_size, yellow);
    img
}

/// Generer en tile-textur for fortov.
pub fn generate_sidewalk_tile(tile_size: u32) -> RgbaImage {
    let mut img = ImageBuffer::new(tile_size, tile_size);
    // Lysegrå base.
    let base = Rgba([90, 90, 95, 255]);
    fill_rect(&mut img, 0, 0, tile_size, tile_size, base);
    // Flise-mønster (linjer).
    let line_c = Rgba([70, 70, 75, 255]);
    for y in (0..tile_size).step_by(8) {
        for x in 0..tile_size {
            put(&mut img, x, y, line_c);
        }
    }
    for x in (0..tile_size).step_by(8) {
        for y in 0..tile_size {
            put(&mut img, x, y, line_c);
        }
    }
    img
}

/// Generer en tile-textur for bygning (mursten).
pub fn generate_building_tile(tile_size: u32) -> RgbaImage {
    let mut img = ImageBuffer::new(tile_size, tile_size);
    // Mørkebrun base.
    let base = Rgba([50, 45, 40, 255]);
    fill_rect(&mut img, 0, 0, tile_size, tile_size, base);
    // Mursten (mønstre).
    let brick_c = Rgba([70, 55, 45, 255]);
    let mortar_c = Rgba([35, 30, 25, 255]);
    let brick_h = 8usize;
    let brick_w = 16usize;
    for y in (0..tile_size as usize).step_by(brick_h) {
        let offset = if (y / brick_h) % 2 == 0 { 0 } else { brick_w / 2 };
        for x in (0..tile_size as usize).step_by(brick_w) {
            fill_rect(&mut img, (x + offset) as u32, y as u32, (brick_w - 1) as u32, (brick_h - 1) as u32, brick_c);
        }
        for x in 0..tile_size {
            put(&mut img, x, y as u32, mortar_c);
        }
    }
    // Vinduer (mørkeblå firkanter).
    let window_c = Rgba([40, 50, 70, 255]);
    let win_s = 6u32;
    fill_rect(&mut img, tile_size / 4, tile_size / 4, win_s, win_s, window_c);
    fill_rect(&mut img, tile_size * 3 / 4 - win_s, tile_size / 4, win_s, win_s, window_c);
    fill_rect(&mut img, tile_size / 4, tile_size * 3 / 4 - win_s, win_s, win_s, window_c);
    fill_rect(&mut img, tile_size * 3 / 4 - win_s, tile_size * 3 / 4 - win_s, win_s, win_s, window_c);
    img
}

/// Generer en tile-textur for græs.
pub fn generate_grass_tile(tile_size: u32) -> RgbaImage {
    let mut img = ImageBuffer::new(tile_size, tile_size);
    // Grøn base.
    let base = Rgba([40, 80, 35, 255]);
    fill_rect(&mut img, 0, 0, tile_size, tile_size, base);
    // Græs-stænk (mørkere/lysere gråtoner).
    for y in 0..tile_size {
        for x in 0..tile_size {
            let v = ((x * 3 + y * 5) % 11) as u8;
            let g = (80 + v * 3).min(255);
            let r = (40 + v).min(255);
            let b = (35 + v / 2).min(255);
            put_if_empty(&mut img, x, y, Rgba([r, g, b, 255]));
        }
    }
    img
}

/// Generer en tile-textur for vand.
pub fn generate_water_tile(tile_size: u32) -> RgbaImage {
    let mut img = ImageBuffer::new(tile_size, tile_size);
    let base = Rgba([30, 50, 90, 255]);
    fill_rect(&mut img, 0, 0, tile_size, tile_size, base);
    for y in 0..tile_size {
        for x in 0..tile_size {
            let v = ((x + y) % 5) as u8;
            let b = (90 + v * 8).min(255);
            put_if_empty(&mut img, x, y, Rgba([30, 50, b, 255]));
        }
    }
    img
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn character_sprite_has_pixels() {
        let img = generate_player();
        assert!(img.width() == 32);
        assert!(img.height() == 32);
        let non_empty = img.pixels().filter(|p| p[3] > 0).count();
        assert!(non_empty > 50, "sprite should have visible pixels");
    }

    #[test]
    fn vehicle_has_windows() {
        let img = generate_vehicle(48, 32, [0.8, 0.2, 0.2]);
        // Find mørkeblå pixels (ruder).
        let windows = img.pixels().filter(|p| p[0] == 40 && p[1] == 50 && p[2] == 70).count();
        assert!(windows > 0);
    }
}