use std::ops::*;

pub trait MaxMin {
    fn max(&self, other: Self) -> Self;
    fn min(&self, other: Self) -> Self;
}

impl MaxMin for f32 {
    fn max(&self, other: Self) -> Self {
        f32::max(*self, other)
    }
    fn min(&self, other: Self) -> Self {
        f32::min(*self, other)
    }
}

impl MaxMin for f64 {
    fn max(&self, other: Self) -> Self {
        f64::max(*self, other)
    }
    fn min(&self, other: Self) -> Self {
        f64::min(*self, other)
    }
}

pub trait One {
    fn one() -> Self;
}

impl One for f32 {
    fn one() -> Self {
        1.0
    }
}

impl One for f64 {
    fn one() -> Self {
        1.0
    }
}

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for f32 {
    fn zero() -> Self {
        0.0
    }
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0
    }
}

pub trait Clamp {
    fn clamp(&self, low: Self, high: Self) -> Self;
}

impl Clamp for f32 {
    fn clamp(&self, low: Self, high: Self) -> Self {
        self.max(low).min(high)
    }
}

impl Clamp for f64 {
    fn clamp(&self, low: Self, high: Self) -> Self {
        self.max(low).min(high)
    }
}

pub trait Vec<T>: Sized + Neg<Output=Self> + Mul<T, Output=Self> + Add<Self, Output=Self> + Sub<Self, Output=Self> + MaxMin + Zero + Clamp {
    fn dot(&self, other: Self) -> T;
    fn magnitude(&self) -> T;
    fn abs(&self) -> Self;
    fn normalized(&self) -> Self;
}

pub trait Vec3<T>: Vec<T> {
    fn new(x: T, y: T, z: T) -> Self;
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn z(&self) -> T;
}

pub trait Vec2<T>: Vec<T> {
    fn new(x: T, y: T) -> Self;
    fn x(&self) -> T;
    fn y(&self) -> T;
}