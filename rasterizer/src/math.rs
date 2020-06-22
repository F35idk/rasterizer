use std::convert::From;
use std::ops::{Add, Div, Mul, MulAssign, Shl, Sub};

pub fn clamp_f(min: f32, max: f32, val: f32) -> f32 {
    min.max(val.min(max))
}

pub fn clamp_i(min: i32, max: i32, val: i32) -> i32 {
    min.max(val.min(max))
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Mat4 {
    pub x4: [f32; 4],
    pub y4: [f32; 4],
    pub z4: [f32; 4],
    pub w4: [f32; 4],
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        let basis_x = vec4(
            rhs.x * self.x4[0],
            rhs.x * self.y4[0],
            rhs.x * self.z4[0],
            rhs.x * self.w4[0],
        );

        let basis_y = vec4(
            rhs.y * self.x4[1],
            rhs.y * self.y4[1],
            rhs.y * self.z4[1],
            rhs.y * self.w4[1],
        );

        let basis_z = vec4(
            rhs.z * self.x4[2],
            rhs.z * self.y4[2],
            rhs.z * self.z4[2],
            rhs.z * self.w4[2],
        );

        let basis_w = vec4(
            rhs.w * self.x4[3],
            rhs.w * self.y4[3],
            rhs.w * self.z4[3],
            rhs.w * self.w4[3],
        );

        basis_x + basis_y + basis_z + basis_w
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Mat4) -> Self {
        let mut col_vec_x = vec4(rhs.x4[0], rhs.y4[0], rhs.z4[0], rhs.w4[0]);
        let mut col_vec_y = vec4(rhs.x4[1], rhs.y4[1], rhs.z4[1], rhs.w4[1]);
        let mut col_vec_z = vec4(rhs.x4[2], rhs.y4[2], rhs.z4[2], rhs.w4[2]);
        let mut col_vec_w = vec4(rhs.x4[3], rhs.y4[3], rhs.z4[3], rhs.w4[3]);

        col_vec_x *= self;
        col_vec_y *= self;
        col_vec_z *= self;
        col_vec_w *= self;

        Mat4 {
            x4: [col_vec_x.x, col_vec_y.x, col_vec_z.x, col_vec_w.x],
            y4: [col_vec_x.y, col_vec_y.y, col_vec_z.y, col_vec_w.y],
            z4: [col_vec_x.z, col_vec_y.z, col_vec_z.z, col_vec_w.z],
            w4: [col_vec_x.w, col_vec_y.w, col_vec_z.w, col_vec_w.w],
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4 { x, y, z, w }
}

pub const fn vec4_from_3(vec3: Vec3, w: f32) -> Vec4 {
    Vec4 {
        x: vec3.x,
        y: vec3.y,
        z: vec3.z,
        w,
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Vec4) -> Self {
        vec4(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Sub for Vec4 {
    type Output = Self;
    fn sub(self, rhs: Vec4) -> Self {
        vec4(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        vec4(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl Mul<Mat4> for Vec4 {
    type Output = Self;
    fn mul(self, rhs: Mat4) -> Self {
        rhs * self
    }
}

impl MulAssign<Mat4> for Vec4 {
    fn mul_assign(&mut self, rhs: Mat4) {
        *self = rhs * *self;
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self {
        vec3(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self {
        vec3(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        vec3(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Uvec2 {
    pub x: u32,
    pub y: u32,
}

pub fn uvec2(x: u32, y: u32) -> Uvec2 {
    Uvec2 { x, y }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Ivec2 {
    pub x: i32,
    pub y: i32,
}

pub fn ivec2(x: i32, y: i32) -> Ivec2 {
    Ivec2 { x, y }
}

impl Sub for Ivec2 {
    type Output = Self;
    fn sub(self, rhs: Ivec2) -> Self {
        ivec2(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add for Ivec2 {
    type Output = Self;
    fn add(self, rhs: Ivec2) -> Self {
        ivec2(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Shl<u8> for Ivec2 {
    type Output = Self;
    fn shl(self, rhs: u8) -> Self {
        ivec2(self.x << rhs, self.y << rhs)
    }
}

impl From<Vec2> for Ivec2 {
    fn from(f: Vec2) -> Self {
        ivec2(f.x as i32, f.y as i32)
    }
}

impl From<Vec3> for Ivec2 {
    fn from(f: Vec3) -> Self {
        ivec2(f.x as i32, f.y as i32)
    }
}

#[derive(Debug, Copy, Clone, Default)]
// a vector of 'long' ints (i64)
pub struct Lvec2 {
    pub x: i64,
    pub y: i64,
}

pub fn lvec2(x: i64, y: i64) -> Lvec2 {
    Lvec2 { x, y }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self {
        vec2(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        vec2(self.x * rhs, self.y * rhs)
    }
}

impl From<Vec3> for Vec2 {
    fn from(f: Vec3) -> Self {
        vec2(f.x, f.y)
    }
}
