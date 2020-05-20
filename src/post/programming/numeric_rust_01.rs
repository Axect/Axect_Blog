//! Numerical Computing with Rust
//!
//! # 1. Rust 소개
//!
//! Rust는 2006년 Mozilla Research의 연구원이었던 Graydon Hoare의 개인 프로젝트로 시작하여 2009년
//! Mozilla가 참여하여 2012년에 발표된 프로그래밍 언어입니다.

use crate::{Tag, Tagged};

pub struct NumericRust01 {}

/// Tag for NumericRust01
///
/// `Rust`, `Numerical`
impl Tagged for NumericRust01 {
    fn tag() -> Vec<Tag> {
        vec![Tag::Rust, Tag::Numerical]
    }
}
