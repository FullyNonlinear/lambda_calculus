//! **lambda_calculus** is a simple implementation of the untyped lambda calculus in Rust.

#![feature(test)]

extern crate test;

pub mod term;
pub mod reduction;
pub mod arithmetic;
pub mod booleans;
pub mod pair;
pub mod list;
pub mod combinators;
pub mod parser;
