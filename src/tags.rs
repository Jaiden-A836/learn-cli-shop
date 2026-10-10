//! Tags system which provides tag definitions and containers for storage.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tag(&'static str);

impl Tag {
    pub const fn new(name: &'static str) -> Self {
        Tag(name)
    }
}
