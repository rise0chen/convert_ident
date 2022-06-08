#![no_std]
extern crate alloc;

use alloc::format;
use alloc::string::*;

pub const RAW_IDENTIFIER: &str = "r#";

pub const RUST_KEYWORDS: [&str; 49] = [
    "as", "async", "await", "break", "const", "continue", "dyn", "else", "enum", "extern", "false",
    "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
    "return", "static", "struct", "super", "trait", "true", "type", "union", "unsafe", "use",
    "where", "while", "abstract", "become", "box", "do", "final", "macro", "override", "priv",
    "try", "typeof", "unsized", "virtual", "yield",
];

pub const RUST_SPECIAL_KEYWORDS: [&str; 3] = ["crate", "Self", "self"];

pub fn to_raw_ident(ident: &str) -> String {
    if RUST_KEYWORDS.iter().any(|s| s.eq(&ident)) {
        format!("r#{}", ident)
    } else if RUST_SPECIAL_KEYWORDS.iter().any(|s| s.eq(&ident)) {
        format!("{}_", ident)
    } else {
        ident.to_string()
    }
}
pub fn to_normal_ident(ident: &str) -> String {
    ident.trim_start_matches(RAW_IDENTIFIER).to_string()
}
