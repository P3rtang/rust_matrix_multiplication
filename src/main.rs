#![allow(unstable_features)]
#![feature(slice_flatten)]
use std::{ops::{self, Mul}, fmt::Display};

#[derive(Debug, Clone, Copy)]
struct Vector<const S: usize> {
    vector: [f32; S]
}

impl<const S: usize> Vector<S> {
    fn dot(self, rhs: Self) -> f32 {
        let mut rtn = 0.0;
        (0..S).into_iter().for_each(|i| rtn += self[i] * rhs[i]);
        return rtn
    }
}

impl<const S: usize> Default for Vector<S> {
    fn default() -> Self {
        Self { vector: [0.0; S] }
    }
}

impl<const S: usize> ops::Add for Vector<S> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let rtn: Self = [0.0; S].into_iter().enumerate().map(|(i, _)| {
            self.vector[i] + rhs.vector[i]
        }).collect();
        return rtn
    }
}

impl<const S: usize> ops::Sub for Vector<S> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let rtn: Self = [0.0; S].into_iter().enumerate().map(|(i, _)| {
            self.vector[i] - rhs.vector[i]
        }).collect();
        return rtn
    }
}

impl<const S: usize> ops::Mul for Vector<S> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let rtn: Self = [0.0; S].into_iter().enumerate().map(|(i, _)| {
            self.vector[i] * rhs.vector[i]
        }).collect();
        return rtn
    }
}

impl<const S: usize> ops::Index<usize> for Vector<S> {
    type Output = f32;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.vector[idx]
    }
}

impl<const S: usize> ops::IndexMut<usize> for Vector<S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vector[index]
    }
}

impl<const S: usize> FromIterator<f32> for Vector<S> {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        let vector: Vec<f32> = iter.into_iter().collect();
        assert_eq!(vector.len(), S);

        return Vector::<S>{ vector: vector.try_into().unwrap() }
    }
}

impl<const S: usize> Display for Vector<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.vector)
    }
}

#[derive(Debug, Clone)]
struct Matrix<const S: usize, const T: usize> {
    matrix: [Vector<S>; T]
}

impl<const S: usize, const T: usize> Matrix<S, T> {
    pub const SIZE: usize = S * T;
    fn new(matrix: [[f32; S]; T]) -> Self {
        matrix.flatten().into_iter().map(|v| *v).collect()
    }

    fn diag_mirror(self) -> Matrix<T, S> {
        let mut mirror: Matrix<T, S> = Matrix::default();
        let size = Matrix::<T, S>::SIZE;
        self.into_iter().enumerate().for_each(|(i, v)| {
            if i != Self::SIZE - 1 {
                mirror[i*T % (size - 1)] = v
            } else {
                mirror[Self::SIZE - 1] = v
            }
        } );
        return mirror
    }
}

impl<const S: usize, const T: usize> Default for Matrix<S, T> {
    fn default() -> Self {
        return Self { matrix: [Vector::<S>::default(); T] }
    }
}

impl<const S: usize, const T: usize> Display for Matrix<S, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.matrix.into_iter().for_each(|v| writeln!(f, "{}", v).unwrap());
        Ok(())
    }
}

impl<const S: usize, const T: usize> ops::Add for Matrix<S, T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.into_iter().enumerate().map(|(i, v)| v + rhs[i]).collect()
    }
}

impl<const S: usize, const T: usize, const U: usize> Mul<Matrix<U, S>> for Matrix<S, T> {
    type Output = Matrix<U, T>;

    fn mul(self, rhs: Matrix<U, S>) -> Self::Output {
        let rhs = rhs.diag_mirror();
        let size = Matrix::<U, T>::SIZE;
        (0..size).into_iter().map(|i| self.matrix[i / U].dot(rhs.matrix[i % U])).collect()
    }
}

impl<const S: usize, const T: usize> ops::Index<usize> for Matrix<S, T> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < Self::SIZE);
        let t = index / S;
        let s = index % S;
        return &self.matrix[t][s]
    }
}

impl<const S: usize, const T: usize> ops::IndexMut<usize> for Matrix<S, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < Self::SIZE);
        let t = index / S;
        let s = index % S;
        return &mut self.matrix[t][s]
    }
}

impl<const S: usize, const T: usize> IntoIterator for Matrix<S, T> {
    type Item = f32;

    type IntoIter = std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.matrix.into_iter().map(|v| v.vector).flatten().collect::<Vec<f32>>().into_iter()
    }
}

impl<const S: usize, const T: usize> FromIterator<f32> for Matrix<S, T> {
    fn from_iter<U: IntoIterator<Item = f32>>(iter: U) -> Self {
        let mut matrix = Matrix::default();
        iter.into_iter().enumerate().for_each(|(i, v)| matrix[i] = v);
        return matrix
    }
}

fn main() {
    let matrix1 = Matrix::new([
        [3.0, 3.0],
        [1.0, 2.0],
        [0.0, 6.0],
    ]);

    let matrix2 = Matrix::new([
        [2.0, 7.0, 3.0, 2.0],
        [4.0, 3.0, 2.0, 3.0],
    ]);

    println!("{}", matrix1 * matrix2)
}
