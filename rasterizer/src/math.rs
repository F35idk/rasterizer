use std::convert::From;
use std::ops::{Add, Div, Mul, MulAssign, Sub};

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

impl Mul<Vec4<f32>> for Mat4 {
    type Output = Vec4<f32>;

    fn mul(self, rhs: Vec4<f32>) -> Vec4<f32> {
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
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

pub const fn vec4<T>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4 { x, y, z, w }
}

pub fn vec4_from_3<T>(vec3: Vec3<T>, w: T) -> Vec4<T> {
    Vec4 {
        x: vec3.x,
        y: vec3.y,
        z: vec3.z,
        w,
    }
}

impl<T> Add for Vec4<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Vec4<T>) -> Self::Output {
        vec4(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl<T> Sub for Vec4<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Vec4<T>) -> Self::Output {
        vec4(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl<T> Div<T> for Vec4<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        vec4(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl Mul<Mat4> for Vec4<f32> {
    type Output = Self;
    fn mul(self, rhs: Mat4) -> Self {
        rhs * self
    }
}

impl MulAssign<Mat4> for Vec4<f32> {
    fn mul_assign(&mut self, rhs: Mat4) {
        *self = rhs * *self;
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub const fn vec3<T>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3 { x, y, z }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Vec3<T>) -> Self {
        vec3(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Vec3<T>) -> Self {
        vec3(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        vec3(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

pub fn vec2<T>(x: T, y: T) -> Vec2<T> {
    Vec2 { x, y }
}

impl<T> Sub for Vec2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Vec2<T>) -> Self {
        vec2(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        vec2(self.x * rhs, self.y * rhs)
    }
}

impl<T> From<Vec3<T>> for Vec2<T> {
    fn from(f: Vec3<T>) -> Self {
        vec2(f.x, f.y)
    }
}

impl From<Vec2<f32>> for Vec2<i32> {
    fn from(f: Vec2<f32>) -> Self {
        vec2::<i32>(f.x as i32, f.y as i32)
    }
}
