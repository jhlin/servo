/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Computed percentages.

use properties::animated_properties::Animatable;
use std::fmt;
use style_traits::ToCss;
use values::CSSFloat;
use values::animated::ToAnimatedZero;

/// A computed percentage.
#[derive(Clone, ComputeSquaredDistance, Copy, Debug, Default, HasViewportPercentage, PartialEq)]
#[cfg_attr(feature = "servo", derive(Deserialize, HeapSizeOf, Serialize))]
pub struct Percentage(pub CSSFloat);

impl Percentage {
    /// 0%
    #[inline]
    pub fn zero() -> Self {
        Percentage(0.)
    }

    /// 100%
    #[inline]
    pub fn hundred() -> Self {
        Percentage(1.)
    }
}

/// https://drafts.csswg.org/css-transitions/#animtype-percentage
impl Animatable for Percentage {
    #[inline]
    fn add_weighted(&self, other: &Self, self_portion: f64, other_portion: f64) -> Result<Self, ()> {
        Ok(Percentage((self.0 as f64 * self_portion + other.0 as f64 * other_portion) as f32))
    }
}

impl ToAnimatedZero for Percentage {
    #[inline]
    fn to_animated_zero(&self) -> Result<Self, ()> {
        Ok(Percentage(0.))
    }
}

impl ToCss for Percentage {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        write!(dest, "{}%", self.0 * 100.)
    }
}
