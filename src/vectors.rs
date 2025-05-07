use num::{
    cast::AsPrimitive, traits::{
        identities, Float, Num, NumAssignOps, One, Signed 
    }, FromPrimitive, Zero 
};
use std::{fmt::Debug, ops};
use ggez::{
    glam, 
    mint::{Point2, Vector2}
};

pub trait Identity {
    fn identity() -> Self;
}

#[derive(Clone, Copy, Debug)]
pub struct Vector<T: Copy> {
    pub x: T,
    pub y: T
}
impl<T: Copy> Vector<T> {
    pub fn mintify<P>(&self) -> Vector2<P> where T: AsPrimitive<P>, P: Copy + 'static {
        Vector2 { x: self.x.as_(), y: self.y.as_() }
    }
    pub fn pointify<P>(&self) -> Point2<P> where T: AsPrimitive<P>, P: Copy + 'static {
        Point2 { x: self.x.as_(), y: self.y.as_() }
    }
}
impl<T: AsPrimitive<f32>+Copy> Vector<T> {
    pub fn glamify(&self) -> glam::Vec2 {
        glam::Vec2 { x: self.x.as_(), y: self.y.as_() }
    }
}
impl<T> Vector<T>
where 
    T: One+Copy+Debug
{
    pub fn multiplicative_identity() -> Self {
        Self {
            x: T::one(),
            y: T::one()
        }
    }
}
impl<T> Vector<T>
where 
    T: Zero+Copy+Debug
{
    pub fn identity() -> Self {
        Self {
            x: T::zero(),
            y: T::zero()
        }
    }
}

impl<T> Vector<T> 
where 
    T: One+ops::Neg<Output = T>+PartialOrd+Copy+Debug
{
    pub fn clamp(&self, min: Self, max: Self) -> Self {
        let x_sign = if min.x < max.x {
            T::one()
        } else { -T::one() };
        let y_sign = if min.y < max.y {
            T::one()
        } else { -T::one() };
        let mut output = self.clone();
        if output.x * x_sign > max.x * x_sign {
            output.x = max.x;
        } 
        if output.x * x_sign < min.x * x_sign {
            output.x = min.x;
        } 
        if output.y * y_sign > max.y * y_sign {
            output.y = max.y;
        } 
        if output.y * y_sign < min.y * y_sign {
            output.y = min.y;
        } 
        output
    }
}

impl<T> Vector<T> 
where 
    T: Float+NumAssignOps+FromPrimitive+Copy+Debug+'static,
    f64: AsPrimitive<T>
{
    pub fn move_towards(&self, rhs: Self, delta: T) -> Self {
        let mut x_speed: T = T::one();
        let mut y_speed: T = T::one();
        if (self.x - rhs.x).abs() > (self.y - rhs.y).abs() { y_speed = (self.y - rhs.y).abs() / (self.x - rhs.x).abs(); if y_speed.is_nan() { y_speed = T::zero(); } }
        else { x_speed = (self.x - rhs.x).abs() / (self.y - rhs.y).abs(); if x_speed.is_nan() { x_speed = T::zero(); }}
        let mut output = self.clone();
        let x_sign = (rhs - *self).x.signum();
        let y_sign = (rhs - *self).y.signum();
        let delta = Vector {
            x: x_speed * x_sign,
            y: y_speed * y_sign
        }.with_magnitude(delta);
        output += delta;
        output.clamp(*self, rhs)
    }
    pub fn distance(self, other: Vector<T>) -> T {
        (self - other).magnitude()
    }
    pub fn clamp_magnitude(&self, magnitude: T) -> Self {
        if self.magnitude() > magnitude { return self.with_magnitude(magnitude) }
        else { return *self; }
    }
    pub fn normalise(&self) -> Self {
        let magnitude = self.magnitude();
        Self { x: self.x / magnitude, y: self.y / magnitude }
    }
    pub fn with_magnitude(&self, magnitude: T) -> Self {
        if self.magnitude() == T::zero() { return *self }
        let scale = magnitude / self.magnitude();
        Self { x: self.x * scale, y: self.y * scale }
    }
    pub fn magnitude(&self) -> T {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    pub fn lerp(&self, start: Self, end: Self, speed: T) -> Self {
        if start == end { return end }
        /*let mut t_x = Self::find_t_from_x((self.x - start.x) / (end.x - start.x));
        if t_x.is_nan() {t_x = T::from_f64(0.0).unwrap()}
        let mut t_y = Self::find_t_from_x((self.y - start.y) / (end.y - start.y));
        if t_y.is_nan() {t_y = T::from_f64(0.0).unwrap()}
        //if t_x < T::from_f64(0.00).unwrap() || t_y < T::from_f64(0.00).unwrap() { println!("{:?} {:?} {:?} {:?} {:?}", t_x, t_y, self, start, end) };
        let mut output = Self::identity();
        if T::one() - t_x <= T::from_f64(0.01).unwrap() && t_x < T::one() { 
            //println!("approximated one");
            output.x = end.x;
        } else {//else if t_x <= T::from_f64(0.001).unwrap() && t_x != 0.0 { output.x = start.x; } else { 
            if t_x > T::one() { t_x -= 0.1 * speed }
            else { t_x += 0.1 * speed; }
            let progress_x = Self::lerp_value(t_x);
            output.x = start.x + (end.x - start.x) * progress_x;
        };*/
        let x = ((*self - start) * (end - start)) / ((end - start).magnitude().powi(2));
        let mut t = Self::find_t_from_x(x);
        let mut output = end.clone();
        if !(T::one() - t <= T::from_f64(0.01).unwrap() && t < T::one()) {//if t_y <= T::from_f64(0.001).unwrap() { output.y = start.y; } else { 
            if t > T::one() { t -= T::from_f64(0.01).unwrap() * speed; }
            else { t += T::from_f64(0.01).unwrap() * speed; }
            let progress = Self::lerp_value(t);

            output = start + (end - start) * progress;
        };
        //println!("{:?}", self);
        //println!("{:?}", end);
        //println!("{:?}", start);
        //println!("TTTTTTTTTTTTTTTTTT {:?} {:?}", t_x, t_y);
        output
    }
    fn lerp_value(t: T) -> T {
        if t > T::one() {return T::one()}
        //println!("{:?}", t);
        T::from_f64(2.25).unwrap() * t - (T::from_f64(1.5).unwrap() * t.powi(2)) + T::from_f64(0.25).unwrap() * t.powi(3)
    }
    fn lerp_value_derivative(t: T) -> T {
        //if t < T::zero() {return T::one()}
        if t > T::one() { return T::zero() }
        T::from_f64(2.25).unwrap() - T::from_f64(3.0).unwrap() * t + T::from_f64(0.75).unwrap() * t.powi(2)
    }
    fn find_t_from_x(x: T) -> T {
        let mut t = T::from_f64(0.5).unwrap();
        let tolerance: T = T::from_f64(1e-64).unwrap();
        //println!("{:?}", tolerance);
        //println!("{:?}", t);
        //println!("{:?}", x);
        
        for _ in 0..5000 {
            let value = Self::lerp_value(t) - x;
            let derivative = Self::lerp_value_derivative(t);
            //println!("{:?}", value);

            if value.abs() < tolerance { break }   
            if derivative == T::zero() { t = T::one(); break }
            t -= value / derivative;
        }
        t
    }
    pub fn angle(&self) -> T {
        self.y.atan2(self.x)
    }
    pub fn magnitude_angle(angle: T, magnitude: T) -> Self {
        Self {
            x: magnitude * (angle).sin(), // + PI.as_()
            y: magnitude * (angle).cos(),
        }
    }
}
impl<T> Vector<T>
where 
    T: identities::ConstZero+Copy 
{
    pub const fn const_identity() -> Self {
        Self {
            x: T::ZERO,
            y: T::ZERO,            
        }
    }
}
impl<T> ops::Add for Vector<T> 
where 
    T: Copy+num::traits::Num
{
    type Output = Self;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}
impl<T> ops::Sub for Vector<T> 
where 
    T: Copy+num::traits::Num
{
    type Output = Self;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}
impl<T> ops::AddAssign for Vector<T> 
where 
    T: Copy+num::traits::NumAssignOps
{
    fn add_assign(&mut self, rhs: Vector<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl<T> ops::SubAssign for Vector<T> 
where 
    T: Copy+num::traits::NumAssignOps
{
    fn sub_assign(&mut self, rhs: Vector<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl<T> ops::Mul for Vector<T> 
where 
    T: Copy+num::traits::Num
{
    type Output = T;
    fn mul(self, rhs: Vector<T>) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}
impl<T> ops::Neg for Vector<T> 
where 
    T: Copy+num::traits::Num+ops::Neg<Output = T>
{
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y 
        }
    }
}
impl<T, M> ops::Mul<M> for Vector<T> 
where 
    T: Copy+num::traits::Num+ops::Mul<M, Output = T>,
    M: Num+Copy
{
    type Output = Vector<T>;
    fn mul(self, rhs: M) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}
impl<T: PartialEq+Copy> PartialEq for Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
    fn ne(&self, other: &Self) -> bool {
        self.x != other.x && self.y != other.y
    }
}
impl<T: Eq+Copy> Eq for Vector<T> {}
impl<T: Copy+AsPrimitive<P>, P: Copy + 'static> Into<Point2<P>> for Vector<T> {
    fn into(self) -> Point2<P> {
        self.pointify()
    }
}
impl<T: Copy+AsPrimitive<P>, P: Copy + 'static> Into<Vector2<P>> for Vector<T> {
    fn into(self) -> Vector2<P> {
        self.mintify()
    }
}
impl<T: Copy+AsPrimitive<f32>> Into<glam::Vec2> for Vector<T> {
    fn into(self) -> glam::Vec2 {
        self.glamify()
    }
}
impl<T: Copy+PartialOrd> PartialOrd for Vector<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.x > other.x && self.y > other.y { Some(std::cmp::Ordering::Greater) }
        else if self.x < other.x && self.y < other.y { Some(std::cmp::Ordering::Less) }
        else if self.x == other.x && self.y == other.y { Some(std::cmp::Ordering::Equal) }
        else { None }
    }
}
impl<T: Copy + 'static, M: AsPrimitive<T>> From<Point2<M>> for Vector<T> {
    fn from(value: Point2<M>) -> Self {
        Self {
            x: value.x.as_(),
            y: value.y.as_()
        }
    }
}
/*impl<T: PartialOrd> PartialOrd for Vector<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.x.partial_cmp(&other.x).unwrap().
    }
}*/