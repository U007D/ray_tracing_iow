use crate::vec3::Vec3;
use float_cmp::Ulps;
use interpol::write;
use std::{
    fmt::{Display, Formatter},
    ops::{Deref, DerefMut},
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color(Vec3);

impl Color {
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

impl Deref for Color {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let r = (256.0.prev() * self.0.x()).trunc();
        let g = (256.0.prev() * self.0.y()).trunc();
        let b = (256.0.prev() * self.0.z()).trunc();
        write!(f, "{r} {g} {b}")
    }
}
