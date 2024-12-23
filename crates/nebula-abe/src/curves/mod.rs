pub mod bls24479;
pub mod bls48556;
pub mod bn462;

#[macro_use]
pub mod miracl;

use rand_core::{CryptoRng, RngCore};
use serde::{Deserialize, Serialize};
#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::random::Random;

pub trait Pow<Rhs = Self> {
    type Output;

    fn pow(self, x: &Rhs) -> Self::Output;
}

pub trait Inv {
    type Output;

    fn inverse(self) -> Self::Output;
}

pub trait RefAdd<Rhs = Self> {
    type Output;

    fn ref_add(&self, x: &Rhs) -> Self::Output;
}

pub trait RefMul<Rhs = Self> {
    type Output;

    fn ref_mul(&self, x: &Rhs) -> Self::Output;
}

pub trait RefSub<Rhs = Self> {
    type Output;

    fn ref_sub(&self, x: &Rhs) -> Self::Output;
}

pub trait RefDiv<Rhs = Self> {
    type Output;

    fn ref_div(&self, x: &Rhs) -> Self::Output;
}

pub trait RefNeg {
    type Output;

    fn ref_neg(&self) -> Self::Output;
}

pub trait RefPow<Rhs = Self> {
    type Output;

    fn ref_pow(&self, x: &Rhs) -> Self::Output;
}

pub trait FieldWithOrder: Field {
    fn order() -> Self;
    fn random_within_order(rng: &mut <Self as Random>::Rng) -> Self;
}

#[cfg(not(feature = "zeroize"))]
pub trait Field:
    Sized
    + From<u64>
    + Clone
    + PartialEq
    + RefNeg<Output = Self>
    + Neg<Output = Self>
    + RefAdd<Output = Self>
    + Add<Output = Self>
    + RefSub<Output = Self>
    + Sub<Output = Self>
    + RefMul<Output = Self>
    + Mul<Output = Self>
    + RefDiv<Output = Self>
    + Div<Output = Self>
    + Pow<Self, Output = Self>
    + RefPow<Self, Output = Self>
    + Serialize
    + Random
    + Sum<Self>
where
    Self: for<'de> Deserialize<'de>,
{
    type Chunk: Copy;

    fn new() -> Self;
    fn one() -> Self;
    fn new_int(x: Self::Chunk) -> Self;
    fn new_ints(x: &[Self::Chunk]) -> Self;
}

#[cfg(feature = "zeroize")]
pub trait Field:
    Sized
    + From<u64>
    + Clone
    + PartialEq
    + RefNeg<Output = Self>
    + Neg<Output = Self>
    + RefAdd<Output = Self>
    + Add<Output = Self>
    + RefSub<Output = Self>
    + Sub<Output = Self>
    + RefMul<Output = Self>
    + Mul<Output = Self>
    + RefDiv<Output = Self>
    + Div<Output = Self>
    + Pow<Self, Output = Self>
    + RefPow<Self, Output = Self>
    + Serialize
    + Random
    + Sum<Self>
    + Zeroize
    + ZeroizeOnDrop
where
    Self: for<'de> Deserialize<'de>,
{
    type Chunk: Copy;

    fn new() -> Self;
    fn one() -> Self;
    fn new_int(x: Self::Chunk) -> Self;
    fn new_ints(x: &[Self::Chunk]) -> Self;
}

#[cfg(not(feature = "zeroize"))]
pub trait GroupG1:
    Clone
    + Sized
    + Neg<Output = Self>
    + RefAdd<Output = Self>
    + Add<Output = Self>
    + RefMul<Self::Field, Output = Self>
    + Mul<Self::Field, Output = Self>
    + Serialize
where
    Self: for<'de> Deserialize<'de>,
{
    type Field: Field;

    fn new(x: &Self::Field) -> Self;
    fn zero() -> Self;
    fn generator() -> Self;
}

#[cfg(feature = "zeroize")]
pub trait GroupG1:
    Clone
    + Sized
    + Neg<Output = Self>
    + RefAdd<Output = Self>
    + Add<Output = Self>
    + RefMul<Self::Field, Output = Self>
    + Mul<Self::Field, Output = Self>
    + Serialize
    + Zeroize
    + ZeroizeOnDrop
where
    Self: for<'de> Deserialize<'de>,
{
    type Field: Field;

    fn new(x: &Self::Field) -> Self;
    fn zero() -> Self;
    fn generator() -> Self;
}

#[cfg(not(feature = "zeroize"))]
pub trait GroupG2:
    Clone
    + Sized
    + RefAdd<Output = Self>
    + Add<Output = Self>
    + RefMul<Self::Field, Output = Self>
    + Mul<Self::Field, Output = Self>
    + Serialize
where
    Self: for<'de> Deserialize<'de>,
{
    type Field: Field;

    fn new(x: &Self::Field) -> Self;
    fn generator() -> Self;
}

#[cfg(feature = "zeroize")]
pub trait GroupG2:
    Clone
    + Sized
    + RefAdd<Output = Self>
    + Add<Output = Self>
    + RefMul<Self::Field, Output = Self>
    + Mul<Self::Field, Output = Self>
    + Serialize
    + Zeroize
    + ZeroizeOnDrop
where
    Self: for<'de> Deserialize<'de>,
{
    type Field: Field;

    fn new(x: &Self::Field) -> Self;
    fn generator() -> Self;
}

#[cfg(not(feature = "zeroize"))]
pub trait GroupGt:
    Sized
    + Inv<Output = Self>
    + RefMul<Output = Self>
    + Mul<Output = Self>
    + RefPow<Self::Field, Output = Self>
    + Pow<Self::Field, Output = Self>
    + Clone
    + Serialize
    + Random
    + Into<Vec<u8>>
where
    Self: for<'de> Deserialize<'de>,
    Self: for<'a> From<&'a [u8]>,
    Self: for<'a> Mul<&'a Self, Output = Self>,
{
    type Field: Field;

    fn one() -> Self;
}

#[cfg(feature = "zeroize")]
pub trait GroupGt:
    Sized
    + Inv<Output = Self>
    + RefMul<Output = Self>
    + Mul<Output = Self>
    + RefPow<Self::Field, Output = Self>
    + Pow<Self::Field, Output = Self>
    + Clone
    + Serialize
    + Random
    + Into<Vec<u8>>
    + Zeroize
    + ZeroizeOnDrop
where
    Self: for<'de> Deserialize<'de>,
    Self: for<'a> From<&'a [u8]>,
    Self: for<'a> Mul<&'a Self, Output = Self>,
{
    type Field: Field;

    fn one() -> Self;
}

pub trait PairingCurve: Sized + Clone {
    type Rng: CryptoRng + RngCore;
    type Field: FieldWithOrder<Rng = Self::Rng>;
    type G1: GroupG1<Field = Self::Field>;
    type G2: GroupG2<Field = Self::Field>;
    type Gt: GroupGt<Field = Self::Field, Rng = Self::Rng>;

    fn pair(e1: &Self::G1, e2: &Self::G2) -> Self::Gt;
    fn hash_to_g1(msg: &[u8]) -> Self::G1;
    fn hash_to_g2(msg: &[u8]) -> Self::G2;
}
