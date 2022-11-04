use std::any::Any;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

use super::*;

/// A container with a horizontal and vertical component.
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Axes<T> {
    /// The horizontal component.
    pub x: T,
    /// The vertical component.
    pub y: T,
}

impl<T> Axes<T> {
    /// Create a new instance from the two components.
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Create a new instance with two equal components.
    pub fn splat(v: T) -> Self
    where
        T: Clone,
    {
        Self { x: v.clone(), y: v }
    }

    /// Map the individual fields with `f`.
    pub fn map<F, U>(self, mut f: F) -> Axes<U>
    where
        F: FnMut(T) -> U,
    {
        Axes { x: f(self.x), y: f(self.y) }
    }

    /// Convert from `&Axes<T>` to `Axes<&T>`.
    pub fn as_ref(&self) -> Axes<&T> {
        Axes { x: &self.x, y: &self.y }
    }

    /// Convert from `&Axes<T>` to `Axes<&<T as Deref>::Target>`.
    pub fn as_deref(&self) -> Axes<&T::Target>
    where
        T: Deref,
    {
        Axes { x: &self.x, y: &self.y }
    }

    /// Convert from `&mut Axes<T>` to `Axes<&mut T>`.
    pub fn as_mut(&mut self) -> Axes<&mut T> {
        Axes { x: &mut self.x, y: &mut self.y }
    }

    /// Zip two instances into an instance over a tuple.
    pub fn zip<U>(self, other: Axes<U>) -> Axes<(T, U)> {
        Axes { x: (self.x, other.x), y: (self.y, other.y) }
    }

    /// Whether a condition is true for at least one of fields.
    pub fn any<F>(self, mut f: F) -> bool
    where
        F: FnMut(&T) -> bool,
    {
        f(&self.x) || f(&self.y)
    }

    /// Whether a condition is true for both fields.
    pub fn all<F>(self, mut f: F) -> bool
    where
        F: FnMut(&T) -> bool,
    {
        f(&self.x) && f(&self.y)
    }

    /// Filter the individual fields with a mask.
    pub fn filter(self, mask: Axes<bool>) -> Axes<Option<T>> {
        Axes {
            x: if mask.x { Some(self.x) } else { None },
            y: if mask.y { Some(self.y) } else { None },
        }
    }
}

impl<T: Default> Axes<T> {
    /// Create a new instance with y set to its default value.
    pub fn with_x(x: T) -> Self {
        Self { x, y: T::default() }
    }

    /// Create a new instance with x set to its default value.
    pub fn with_y(y: T) -> Self {
        Self { x: T::default(), y }
    }
}

impl<T: Ord> Axes<T> {
    /// The component-wise minimum of this and another instance.
    pub fn min(self, other: Self) -> Self {
        Self { x: self.x.min(other.x), y: self.y.min(other.y) }
    }

    /// The component-wise minimum of this and another instance.
    pub fn max(self, other: Self) -> Self {
        Self { x: self.x.max(other.x), y: self.y.max(other.y) }
    }
}

impl<T> Get<Axis> for Axes<T> {
    type Component = T;

    fn get(self, axis: Axis) -> T {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
        }
    }

    fn get_mut(&mut self, axis: Axis) -> &mut T {
        match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
        }
    }
}

impl<T> Debug for Axes<T>
where
    T: Debug + 'static,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Axes { x: Some(x), y: Some(y) } =
            self.as_ref().map(|v| (v as &dyn Any).downcast_ref::<Align>())
        {
            write!(f, "{:?}-{:?}", x, y)
        } else if (&self.x as &dyn Any).is::<Abs>() {
            write!(f, "Size({:?}, {:?})", self.x, self.y)
        } else {
            write!(f, "Axes({:?}, {:?})", self.x, self.y)
        }
    }
}

/// The two layouting axes.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Axis {
    /// The horizontal axis.
    X,
    /// The vertical  axis.
    Y,
}

impl Axis {
    /// The direction with the given positivity for this axis.
    pub fn dir(self, positive: bool) -> Dir {
        match (self, positive) {
            (Self::X, true) => Dir::LTR,
            (Self::X, false) => Dir::RTL,
            (Self::Y, true) => Dir::TTB,
            (Self::Y, false) => Dir::BTT,
        }
    }

    /// The other axis.
    pub fn other(self) -> Self {
        match self {
            Self::X => Self::Y,
            Self::Y => Self::X,
        }
    }
}

impl Debug for Axis {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad(match self {
            Self::X => "horizontal",
            Self::Y => "vertical",
        })
    }
}

impl<T> Axes<Option<T>> {
    /// Unwrap the individual fields.
    pub fn unwrap_or(self, other: Axes<T>) -> Axes<T> {
        Axes {
            x: self.x.unwrap_or(other.x),
            y: self.y.unwrap_or(other.y),
        }
    }
}

impl Axes<bool> {
    /// Select `t.x` if `self.x` is true and `f.x` otherwise and same for `y`.
    pub fn select<T>(self, t: Axes<T>, f: Axes<T>) -> Axes<T> {
        Axes {
            x: if self.x { t.x } else { f.x },
            y: if self.y { t.y } else { f.y },
        }
    }
}

impl Not for Axes<bool> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self { x: !self.x, y: !self.y }
    }
}

impl BitOr for Axes<bool> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self { x: self.x | rhs.x, y: self.y | rhs.y }
    }
}

impl BitOr<bool> for Axes<bool> {
    type Output = Self;

    fn bitor(self, rhs: bool) -> Self::Output {
        Self { x: self.x | rhs, y: self.y | rhs }
    }
}

impl BitAnd for Axes<bool> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self { x: self.x & rhs.x, y: self.y & rhs.y }
    }
}

impl BitAnd<bool> for Axes<bool> {
    type Output = Self;

    fn bitand(self, rhs: bool) -> Self::Output {
        Self { x: self.x & rhs, y: self.y & rhs }
    }
}

impl BitOrAssign for Axes<bool> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.x |= rhs.x;
        self.y |= rhs.y;
    }
}

impl BitAndAssign for Axes<bool> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.x &= rhs.x;
        self.y &= rhs.y;
    }
}