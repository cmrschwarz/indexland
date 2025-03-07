use indexland::Idx;

fn main() {
    #[derive(Idx)]
    #[indexland(omit(Default), only(Add))]
    pub enum Bar {
        A,
        B,
    }
}
