#![allow(non_camel_case_types, unused_imports)]
#![feature(never_type, decl_macro, try_blocks, box_syntax, panic_info_message)]

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

extern crate unmem;

pub use unmem::{cstr, AsCStr, DeOwn, IsCopy, fsize};

/// Six lang experimental parser.
#[macro_export]
macro_rules! six {
    () => {};

    //(rust {$($body:tt)*}$($rest:tt)*) => {$($body)*$crate::six!{$($rest)*}};

    (var $i:ident: $type:ty;$($rest:tt)*) => {let mut $i: $type;$crate::six!{$($rest)*}};

    (var $i:ident: $type:ty = $body:tt;$($rest:tt)*) => {let mut $i: $type = $body;$crate::six!{$($rest)*}};

    (var $i:ident = $body:tt;$($rest:tt)*) => {let mut $i = $body;$crate::six!{$($rest)*}};

    (let $i:ident: $type:ty;$($rest:tt)*) => {let $i: $type;$crate::six!{$($rest)*}};

    (let $i:ident: $type:ty = $body:tt;$($rest:tt)*) => {let $i: $type = $body;$crate::six!{$($rest)*}};

    (let $i:ident = $body:tt;$($rest:tt)*) => {let $i = $body;$crate::six!{$($rest)*}};

    (entry -> $type:ty {$($body:tt)*}$($rest:tt)*) => {fn main() -> $type {
    #[cfg(not(debug_assertions))]
    #[cfg(panic = "unwind")]
    std::panic::set_hook(box |info| {eprint!("{msg}", msg = match info.message() {
        None => "Program panicked!".to_owned(),
        Some(x) => x.to_string()
    });});
    $crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (entry: $args:ident -> $type:ty {$($body:tt)*}$($rest:tt)*) => {fn main() -> $type {
    #[cfg(not(debug_assertions))]
    #[cfg(panic = "unwind")]
    std::panic::set_hook(box |info| {eprint!("{msg}", msg = match info.message() {
        None => "Program panicked!".to_owned(),
        Some(x) => x.to_string()
    });});
    let $args = $crate::heart::rt::args();$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (entry: $args:ident {$($body:tt)*}$($rest:tt)*) => {fn main() {
    #[cfg(not(debug_assertions))]
    #[cfg(panic = "unwind")]
    std::panic::set_hook(box |info| {eprint!("{msg}", msg = match info.message() {
        None => "Program panicked!".to_owned(),
        Some(x) => x.to_string()
    });});
    let $args = $crate::heart::rt::args();$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (entry {$($body:tt)*}$($rest:tt)*) => {fn main() {
    #[cfg(not(debug_assertions))]
    #[cfg(panic = "unwind")]
    std::panic::set_hook(box |info| {eprint!("{msg}", msg = match info.message() {
        None => "Program panicked!".to_owned(),
        Some(x) => x.to_string()
    });});
    $crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (fn $i:ident($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {fn $i($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (fn $i:ident() -> $type:ty {$($body:tt)*}$($rest:tt)*) => {fn $i() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (proc $i:ident($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[no_mangle] extern "C" fn $i($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (proc $i:ident() -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[no_mangle] extern "C" fn $i() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (unsafe fn $i:ident($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {unsafe fn $i($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (unsafe fn $i:ident() -> $type:ty {$($body:tt)*}$($rest:tt)*) => {unsafe fn $i() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (unsafe proc $i:ident($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[no_mangle] unsafe extern "C" fn $i($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (unsafe proc $i:ident() -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[no_mangle] unsafe extern "C" fn $i() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (fn $i:ident<$($generics:tt)*>($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {fn $i<$($generics)*>($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (fn $i:ident<$($generics:tt)*>() -> $type:ty {$($body:tt)*}$($rest:tt)*) => {fn $i<$($generics)*>() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (proc $i:ident<$generics:tt>($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[no_mangle] extern "C" fn $i<$generics>($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (proc $i:ident<$($generics:tt)*>() -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[no_mangle] extern "C" fn $i<$($generics)*>() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (unsafe fn $i:ident<$($generics:tt)*>($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {unsafe fn $i<$($generics)*>($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (unsafe fn $i:ident<$($generics:tt)*>() -> $type:ty {$($body:tt)*}$($rest:tt)*) => {unsafe fn $i<$($generics)*>() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (unsafe proc $i:ident<$($generics:tt)*>($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[no_mangle] unsafe extern "C" fn $i<$($generics)*>($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (unsafe proc $i:ident<$($generics:tt)*>() -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[no_mangle] unsafe extern "C" fn $i<$($generics)*>() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (label $i:ident($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[naked] #[no_mangle] extern "C" fn $i($(args:tt)*) {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (label $i:ident($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[naked] #[no_mangle] extern "C" fn $i($(args:tt)*) {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (label $i:ident: $t:ty = $body:tt;$($rest:tt)*) => {#[no_mangle] static $i: $t = $body;$crate::six!{$($rest)*}};

    (pub fn $i:ident($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {pub fn $i($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (pub fn $i:ident() -> $type:ty {$($body:tt)*}$($rest:tt)*) => {pub fn $i() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (pub proc $i:ident($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[no_mangle] pub extern "C" fn $i($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (pub proc $i:ident() -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[no_mangle] pub extern "C" fn $i() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (pub unsafe fn $i:ident($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {pub unsafe fn $i($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (pub unsafe fn $i:ident() -> $type:ty {$($body:tt)*}$($rest:tt)*) => {pub unsafe fn $i() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (pub unsafe proc $i:ident($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[no_mangle] pub unsafe extern "C" fn $i($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (pub unsafe proc $i:ident() -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[no_mangle] pub unsafe extern "C" fn $i() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (pub fn $i:ident<$($generics:tt)*>($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {pub fn $i<$($generics)*>($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (pub fn $i:ident<$($generics:tt)*>() -> $type:ty {$($body:tt)*}$($rest:tt)*) => {pub fn $i<$($generics)*>() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (pub proc $i:ident<$($generics:tt)*>($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[no_mangle] pub extern "C" fn $i<$($generics)*>($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (pub proc $i:ident<$($generics:tt)*>() -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[no_mangle] pub extern "C" fn $i<$($generics)*>() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (pub unsafe fn $i:ident<generics:tt>($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {pub unsafe fn $i<$($generics)*>($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (pub unsafe fn $i:ident<$($generics:tt)*>() -> $type:ty {$($body:tt)*}$($rest:tt)*) => {pub unsafe fn $i<$($generics)*>() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (pub unsafe proc $i:ident<$($generics:tt)*>($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[no_mangle] pub unsafe extern "C" fn $i<$($generics)*>($($args)*) -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (pub unsafe proc $i:ident<$($generics:tt)*>() -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[no_mangle] pub unsafe extern "C" fn $i<$($generics)*>() -> $type {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (pub label $i:ident($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[naked] #[no_mangle] pub extern "C" fn $i($(args:tt)*) {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (pub label $i:ident($($args:tt)*) -> $type:ty {$($body:tt)*}$($rest:tt)*) => {#[naked] #[no_mangle] pub extern "C" fn $i($(args:tt)*) {$crate::six!{{{$($body)*}}}}$crate::six!{$($rest)*}};

    (pub label $i:ident: $t:ty = $body:tt;$($rest:tt)*) => {#[no_mangle] pub static $i: $t = $body;$crate::six!{$($rest)*}};

    (const $i:ident: $t:ty = $body:tt;$($rest:tt)*) => {const $i: $t = $body;$crate::six!{$($rest)*}};

    (pub const $i:ident: $t:ty = $body:tt;$($rest:tt)*) => {pub const $i: $t = $body;$crate::six!{$($rest)*}};

    ({$($body:tt)*}$($rest:tt)*) => {{$crate::six!{$($body)*}}$crate::six!{$($rest)*}};

    ($name:tt($($args:tt)*);$($rest:tt)*) => {$name($($args)*);$crate::six!{$($rest)*}};

    ($name:tt();($rest:tt)*) => {$name();$crate::six!($($rest)*)};

    ($name:tt($($args:tt)*).$($rest:tt)*) => {$name($($args)*).$crate::six!{$($rest)*}};

    ($name:tt().($rest:tt)*) => {$name().$crate::six!($($rest)*)};

    ($name:tt!($($args:tt)*);$($rest:tt)*) => {$name!($($args)*);$crate::six!{$($rest)*}};

    ($name:tt!();($rest:tt)*) => {$name!();$crate::six!{$($rest)*}};

    ($name:tt!{$($args:tt)*};$($rest:tt)*) => {$name!{$($args)*};$crate::six!{$($rest)*}};

    ($name:tt!{};($rest:tt)*) => {$name!{};$crate::six!{$($rest)*}};

    ($name:tt![$($args:tt)*];$($rest:tt)*) => {$name![$($args)*];$crate::six!{$($rest)*}};

    ($name:tt![];($rest:tt)*) => {$name![];$crate::six!{$($rest)*}};

    (;$($rest:tt)*) => {;$crate::six!{$($rest)*}};

    ($i:tt = $x:tt;$($rest:tt)*) => {$i = $x;$crate::six!{$($rest)*}};

    (*$i:tt = $x:tt;$($rest:tt)*) => {*$i = $x;$crate::six!{$($rest)*}};

    (extern {$($body:tt)*}$($rest:tt)*) => {extern {$($body)*}$crate::six!{$($rest)*}};

    (extern "C" {$($body:tt)*}$($rest:tt)*) => {extern "C" {$($body)*}$crate::six!{$($rest)*}};

    (extern "Rust" {$($body:tt)*}$($rest:tt)*) => {extern "Rust" {$($body)*}$crate::six!{$($rest)*}};

    (extern "system" {$($body:tt)*}$($rest:tt)*) => {extern "system" {$($body)*}$crate::six!{$($rest)*}};

    (extern "stdcall" {$($body:tt)*}$($rest:tt)*) => {extern "stdcall" {$($body)*}$crate::six!{$($rest)*}};

    (extern "cdecl" {$($body:tt)*}$($rest:tt)*) => {extern "cdecl" {$($body)*}$crate::six!{$($rest)*}};

    (extern "win64" {$($body:tt)*}$($rest:tt)*) => {extern "win64" {$($body)*}$crate::six!{$($rest)*}};

    (extern "sysv64" {$($body:tt)*}$($rest:tt)*) => {extern "sysv64" {$($body)*}$crate::six!{$($rest)*}};

    (extern "aapcs" {$($body:tt)*}$($rest:tt)*) => {extern "aapcs" {$($body)*}$crate::six!{$($rest)*}};

    (extern "fastcall" {$($body:tt)*}$($rest:tt)*) => {extern "fastcall" {$($body)*}$crate::six!{$($rest)*}};

    (extern "vectorcall" {$($body:tt)*}$($rest:tt)*) => {extern "vectorcall" {$($body)*}$crate::six!{$($rest)*}};

    (extern "rust-call" {$($body:tt)*}$($rest:tt)*) => {extern "rust-call" {$($body)*}$crate::six!{$($rest)*}};

    (extern "rust-cold" {$($body:tt)*}$($rest:tt)*) => {extern "rust-cold" {$($body)*}$crate::six!{$($rest)*}};

    (extern "rust-intrinsics" {$($body:tt)*}$($rest:tt)*) => {extern "rust-intrinsics" {$($body)*}$crate::six!{$($rest)*}};

    //(macro $i:ident {$(($args:tt) => {$($body:tt)*};)*}$($rest:tt)*) => {macro_rules! $i {$(($args) => {$crate::six!{{$body}}})*}$($rest)*};

    (unsafe {$($body:tt)*}$($rest:tt)*) => {unsafe {$crate::six!{$($body)*}}$crate::six!{$($rest)*}};

    (import $p:path;$($rest:tt)*) => {use $p;$crate::six!{$($rest)*}};

    (pub import $p:path;$($rest:tt)*) => {pub use $p;$crate::six!{$($rest)*}};

    (include $p:expr;$($rest:tt)*) => {$crate::six!{include!($p)$($rest)*}};

    (pub $($rest:tt)*) => {pub $crate::six!{$($rest)*}};

    (ast $i:ident {$($body:tt)*}$($rest:tt)*) => {macro_rules! $i {$($body)*}$crate::six!{$($rest)*}};

    (pub ast $i:ident {$($body:tt)*}$($rest:tt)*) => {#[macro_export] macro_rules! $i {$($body)*}$crate::six!{$($rest)*}};

    (@i:meta$($rest:tt)*) => {#[$i]$crate::six!{$($rest)*}};

    (#i:meta$($rest:tt)*) => {#![$i]$crate::six!{$($rest)*}};

    ($i:ident++;$($rest:tt)*) => {$i += 1;$crate::six!{$($rest)*}};

    ($i:ident--;$($rest:tt)*) => {$i -= 1;$crate::six!{$($rest)*}};

    (type $i:ident = $t:ty;$($rest:tt)*) => {type $i = $t;$crate::six!{$($rest)*}};

    (pub type $i:ident = $t:ty;$($rest:tt)*) => {pub type $i = $t;$crate::six!{$($rest)*}};

    //(asm {global, $($body:expr),*}$($rest:tt)*) => {core::arch::global_asm!{$($body),*}$crate::six!{$($body)*}};

    //(asm {local, $($body:expr),*};$($rest:tt)*) => {core::arch::asm!($($body),*);$crate::six!{$($body)*}};

    //(try {$($tb:tt)*} except $x {$($eb:tt)*}$($rest:tt)*) => {if let Err($x) = try {$crate::six!{$($tb)*}} {$crate::six!{$($eb)*}}$crate::six!{$($rest)*}};

    (cond $i:tt {$($body:tt)*}$($rest:tt)*) => {if $i {$crate::six!{$($body)*}}$crate::six!{$($rest)*}};

    (cond $i:tt {$($body:tt)*} $(or $x:tt {$($xbody:tt)*})*$($rest:tt)*) => {if $i {$crate::six!{$($body)*}} $(else if $x {$crate::six!{$($xbody)*}})*$crate::six!{$($rest)*}};

    (cond $i:tt {$($body:tt)*} else {$($y:tt)*}$($rest:tt)*) => {if $i {$crate::six!{$($body)*}} else {$crate::six!{$($y)*}}$crate::six!{$($rest)*}};

    (cond $i:tt {$($body:tt)*} $(or $x:tt {$($xbody:tt)*})* else {$($y:tt)*}$($rest:tt)*) => {if $i {$crate::six!{$($body)*}} $(else if $x {$crate::six!{$($xbody)*}})* else {$crate::six!{$($y)*}}$crate::six!{$($rest)*}};

    (asm {$($body:tt)*} $($rest:tt)*) => {core::arch::asm!($($body)*);$crate::six!{$($rest)*}};

    (global_asm {$($body:tt)*} $($rest:tt)*) => {core::arch::global_asm!($($body)*)$crate::six!{$($rest)*}};
    
    (rust {$($body:tt)*} $($rest:tt)*) => {$($body)*$crate::six!{$($rest)*}};

    (loop {$($body:tt)*}$($rest:tt)*) => {loop {$crate::six!{$($body)*}}$crate::six!{$($rest)*}};

    (break;$($rest:tt)*) => {break;$crate::six!{$($rest)*}};

    ($i:lifetime: loop {$($body:tt)*}$($rest:tt)*) => {$i: loop {$crate::six!{$($body)*}}$crate::six!{$($rest)*}};

    (break;$($rest:tt)*) => {break;$crate::six!{$($rest)*}};

    (break $i:lifetime;$($rest:tt)*) => {break $i;$crate::six!{$($rest)*}};

    (continue;$($rest:tt)*) => {continue;$crate::six!{$($rest)*}};

    (match $i:tt {$($x:pat => $y:tt),*}$($rest:tt)*) => {match $i {$($x => $crate::six!{$y}),*}$crate::six!{$($rest)*}};

    (assert $i:tt;$($rest:tt)*) => {assert!($i);$crate::six!{$($rest)*}};

    (assert $i:tt, $($x:expr),*;$($rest:tt)*) => {assert!($i, $($x),*);$crate::six!{$($rest)*}};

    (for $i:tt {$($body:tt)*}$($rest:tt)*) => {for $i {$crate::six!{$($body)*}}$crate::six!{$($rest)*}};

    //(_expr{$i:tt + $($rest:tt)*}) => {$i + $($rest)*};

    //(_expr{$i:tt - $($rest:tt)*}) => {$i - $($rest)*};

    //(_expr{$i:tt * $($rest:tt)*}) => {$i * $($rest)*};

    //(_expr{$i:tt / $($rest:tt)*}) => {$i / $($rest)*};

    /*(_expr{match $($rest:tt)*}) => {$crate::six!{match $($rest)*}};

    (_expr{$i:expr}) => {$i};

    (_expr{cond $($rest:tt)*}) => {$crate::six!{cond $($rest:tt)*}};

    (_expr{unsafe $($rest:tt)*}) => {$crate::six!{unsafe $($rest:tt)*}};

    (_expr{rust $($rest:tt)*}) => {$crate::six!{asm $($rest:tt)*}};

    (_expr{{$($rest:tt)*}}) => {{$crate::six!{$($rest:tt)*}}};*/
}

pub type unit = ();
pub type never = !;

pub mod heart;
pub mod mind;
pub use heart::io::writef;