#![allow(non_camel_case_types)]
#![feature(never_type)]

//! libsix is the main part of six lang. It must be used inside rust.
//! Example:
//! ```no_run
//! extern crate libsix;
//! use libsix::*;
//! 
//! six!{
//! 
//! entry {
//!     print!("Hello, World!");
//! }
//! 
//! }
//! ```

/// Six lang experimental parser.
#[macro_export]
macro_rules! six {
    () => {};

    (rust $body:block$($rest:tt)*) => {$body$crate::six!{$($rest)*}};

    (var $i:ident: $type:ty;$($rest:tt)*) => {let mut $i: $type;$crate::six!{$($rest)*}};

    (var $i:ident: $type:ty = $body:expr;$($rest:tt)*) => {let mut $i: $type = $body;$crate::six!{$($rest)*}};

    (var $i:ident = $body:expr;$($rest:tt)*) => {let mut $i = $body;$crate::six!{$($rest)*}};

    (entry -> $type:ty {$($body:tt)*}$($rest:tt)*) => {fn main() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (entry($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {fn main($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (entry {$($body:tt)*}$($rest:tt)*) => {fn main() {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (fn $i:ident($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {fn $i($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (fn $i:ident() => $type:ty {$($body:tt)*}$($rest:tt)*) => {fn $i() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (proc $i:ident($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[no_mangle] extern "C" fn $i($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (proc $i:ident() => $type:ty {$($body:tt)*}$($rest:tt)*) => {#[no_mangle] extern "C" fn $i() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (label $i:ident($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[naked] #[no_mangle] extern "C" fn $i($(args:tt)*) {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (label $i:ident($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[naked] #[no_mangle] extern "C" fn $i($(args:tt)*) {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    ({$($body:tt)*}$($rest:tt)*) => {{$crate::six!{$($body)*}}$crate::six!{$($rest)*}};

    ($name:ident($($args:tt)*)$($rest:tt)*) => {$name($($args)*);$crate::six!{$($rest)*}};

    ($name:ident()($rest:tt)*) => {$name();$crate::six!($($rest)*)};

    ($name:ident!($($args:tt)*)$($rest:tt)*) => {$name!($($args)*);$crate::six!{$($rest)*}};

    ($name:ident!()($rest:tt)*) => {$name!();$crate::six!{$($rest)*}};

    ($name:ident!{$($args:tt)*}$($rest:tt)*) => {$name!{$($args)*};$crate::six!{$($rest)*}};

    ($name:ident!{}($rest:tt)*) => {$name!{};$crate::six!{$($rest)*}};

    ($name:ident![$($args:tt)*]$($rest:tt)*) => {$name![$($args)*];$crate::six!{$($rest)*}};

    ($name:ident![]($rest:tt)*) => {$name![];$crate::six!{$($rest)*}};

    (;$($rest:tt)*) => {;$crate::six!{$($rest)*}};

    ($i:ident = $x:expr;$($rest:tt)*) => {$i = $x;$($rest)*};

    //(cond $i:expr $x:block$($rest:tt)*) => {if $i {$crate::six!{$x}} $($rest)*};

    //(cond $i:expr $x:block or $y:block$($rest:tt)*) => {if $i {$crate::six!{$x}} else {$crate::six!{$y}} $($rest)*};

    //(cond $i:expr $x:block $(or cond $z:expr $y:block)*$($rest:tt)*) => {if $i {$crate::six!{$x}} $(else if $z {$crate::six!{$y}})* $($rest)*};

    //(cond $i:expr $x:block $(or cond $z:expr $y:block)* or $m:block$($rest:tt)*) => {if $i {$crate::six{$x}} $(else if $z {$crate::six!{$y}})* else {$crate::six!{$m}} $($rest)*};

    //(loop $x:block$($rest:tt)*) => {loop {$crate::six!{$x}}$($rest)*};

    //(break;$($rest:tt)*) => {break;$($rest)*};

    //(asm {$($line:tt);* $($args:expr),*}$($rest:tt)*) => {core::arch::asm!($(stringify!($line)),* $($args),*)$($rest)*};
}

pub type unit = ();
pub type never = !;