//! Axect's Blog
//!
//! # Quick Link
//! * [Github](https://github.com/Axect)
//! * [Lab](http://nexus.yonsei.ac.kr)
//! * [Address](http://nexus.yonsei.ac.kr/contact)
//! * [Address(Google Map)](http://bit.ly/ShYonsei)

pub mod introduce;
pub mod post;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tag {
    Numerical,
    Rust,
}

pub trait Tagged {
    fn tag() -> Vec<Tag>;
}
