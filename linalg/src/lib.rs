use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone)]
pub struct Vector<T>
where
    T: Add + Sub + Mul + Div + Clone + Copy,
{
    inner: Vec<T>,
}

impl<T> Vector<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Clone + Copy + Sum,
{
    pub fn new(size: usize, item: T) -> Vector<T> {
        Vector::<T> {
            inner: vec![item; size],
        }
    }

    pub fn dot(&self, rhs: &Vector<T>) -> T {
        self.inner
            .iter()
            .enumerate()
            .map(|(idx, &x)| x * rhs.inner[idx])
            .sum()
    }
}

impl<T> Add<Vector<T>> for Vector<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Clone + Copy,
{
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Vector::<T> {
            inner: self
                .inner
                .iter()
                .enumerate()
                .map(|(idx, &x)| x + rhs.inner[idx])
                .collect(),
        }
    }
}

impl<T> Sub<Vector<T>> for Vector<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Clone + Copy,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Vector::<T> {
            inner: self
                .inner
                .iter()
                .enumerate()
                .map(|(idx, &x)| x - rhs.inner[idx])
                .collect(),
        }
    }
}

impl<T> Mul<Vector<T>> for Vector<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Clone + Copy,
{
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        Vector::<T> {
            inner: self
                .inner
                .iter()
                .enumerate()
                .map(|(idx, &x)| x * rhs.inner[idx])
                .collect(),
        }
    }
}
impl<T> Div<Vector<T>> for Vector<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Clone + Copy,
{
    type Output = Vector<T>;

    fn div(self, rhs: Vector<T>) -> Self::Output {
        Vector::<T> {
            inner: self
                .inner
                .iter()
                .enumerate()
                .map(|(idx, &x)| x / rhs.inner[idx])
                .collect(),
        }
    }
}
