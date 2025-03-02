//@ revisions: stable2021 classic2024 structural2024
//@[stable2021] edition: 2021
//@[classic2024] edition: 2024
//@[structural2024] edition: 2024
//! Tests for pattern errors not handled by the pattern typing rules, but by borrowck.
#![allow(incomplete_features)]
#![cfg_attr(classic2024, feature(ref_pat_eat_one_layer_2024))]
#![cfg_attr(structural2024, feature(ref_pat_eat_one_layer_2024_structural))]

/// These patterns additionally use `&` to match a `&mut` reference type, which causes compilation
/// to fail in HIR typeck on stable. As such, they need to be separate from the other tests.
fn errors_caught_in_hir_typeck_on_stable() {
    let [&x] = &[&mut 0];
    //[stable2021]~^ mismatched types
    //[stable2021]~| types differ in mutability
    //[classic2024]~^^^ ERROR: cannot move out of type
    let _: &u32 = x;

    let [&x] = &mut [&mut 0];
    //[stable2021]~^ mismatched types
    //[stable2021]~| types differ in mutability
    //[classic2024]~^^^ ERROR: cannot move out of type
    let _: &u32 = x;
}

pub fn main() {
    if let Some(&Some(x)) = Some(&Some(&mut 0)) {
        //~^ ERROR: cannot move out of a shared reference [E0507]
        let _: &u32 = x;
    }

    let &ref mut x = &0;
    //~^ cannot borrow data in a `&` reference as mutable [E0596]

    if let &Some(Some(x)) = &Some(&mut Some(0)) {
        //[stable2021,classic2024]~^ ERROR: cannot borrow data in a `&` reference as mutable
        let _: &u32 = x;
    }

    let &[x] = &&mut [0];
    //[stable2021,classic2024]~^ ERROR: cannot borrow data in a `&` reference as mutable
    let _: &u32 = x;

    let [&mut x] = &mut [&mut 0];
    //[classic2024]~^ ERROR: cannot move out of type
    #[cfg(stable2021)] let _: u32 = x;
    #[cfg(structural2024)] let _: &mut u32 = x;
}
