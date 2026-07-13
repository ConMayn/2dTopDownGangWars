//! Math — game-specific math types bygget på glam.
//!
//! Re-exports glam's Vec2/Mat4 og definerer game-specifikke typer:
//! AABB (axis-aligned bounding box), Transform (position+rotation+scale),
//! Rect (integer bounds for zones/tilemaps), Color.

pub use glam::{Mat3, Mat4, Quat, Vec2, Vec3, Vec4};

/// Axis-aligned bounding box i 2D.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub min: Vec2,
    pub max: Vec2,
}

impl Aabb {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn from_center(center: Vec2, half_extents: Vec2) -> Self {
        Self {
            min: center - half_extents,
            max: center + half_extents,
        }
    }

    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    pub fn half_extents(&self) -> Vec2 {
        self.size() * 0.5
    }

    pub fn contains_point(&self, p: Vec2) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }

    pub fn intersects(&self, other: &Aabb) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }
}

/// 2D transform: position, rotation (radians), scale.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
}

impl Transform {
    pub fn at(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            ..Default::default()
        }
    }

    pub fn to_mat4(&self) -> Mat4 {
        Mat4::from_translation(self.position.extend(0.0))
            * Mat4::from_rotation_z(self.rotation)
            * Mat4::from_scale(self.scale.extend(1.0))
    }
}

/// Integer rect — bruges til zone-bounds, tilemap-områder.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    pub fn min(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }

    pub fn max(&self) -> Vec2 {
        Vec2::new((self.x + self.w) as f32, (self.y + self.h) as f32)
    }

    pub fn contains(&self, p: Vec2) -> bool {
        p.x >= self.x as f32
            && p.x <= (self.x + self.w) as f32
            && p.y >= self.y as f32
            && p.y <= (self.y + self.h) as f32
    }
}

/// RGBA color, 0.0-1.0 range.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub [f32; 4]);

impl Color {
    pub const WHITE: Color = Color([1.0, 1.0, 1.0, 1.0]);
    pub const BLACK: Color = Color([0.0, 0.0, 0.0, 1.0]);
    pub const RED: Color = Color([1.0, 0.0, 0.0, 1.0]);
    pub const GREEN: Color = Color([0.0, 1.0, 0.0, 1.0]);
    pub const BLUE: Color = Color([0.0, 0.0, 1.0, 1.0]);
    pub const CYAN: Color = Color([0.0, 1.0, 1.0, 1.0]);
    pub const YELLOW: Color = Color([1.0, 1.0, 0.0, 1.0]);
    pub const MAGENTA: Color = Color([1.0, 0.0, 1.0, 1.0]);

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self([r, g, b, a])
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self([r, g, b, 1.0])
    }
}