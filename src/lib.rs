#![no_std]

//

use core::ops::{Add, Div, Mul, Range, Sub};
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};

//

/// Mapping a value from range `from`
/// to another range `to`.
///
/// # Panics
///
/// Panics if `from.end == from.start`,
/// if either range is empty with unsigned integers,
/// if `from.start > self` with unsigned integers,
/// or if any checked overflowing happens.
///
/// # Examples
///
/// ```
/// # use map_range::MapRange;
/// # use rand::random;
/// // Mapping from trig to f32 texture range
/// let initial_value = random::<f32>().sin();
///
/// let mapped = initial_value.map_range(-1.0..1.0, 0.0..1.0);
/// let expected = initial_value / 2.0 + 0.5;
///
/// assert!((mapped - expected).abs() <= f32::EPSILON);
/// ```
///
/// ```
/// # use map_range::MapRange;
/// // Mapping can happen outside of the ranges.
/// // Might panic with unsigned integers if `self - from.start`
/// // overflows or if either range is empty.
/// let x = 10_i32;
///
/// let y = x.map_range(0..5, -5..0);
/// let z = x.map_range(0..5, 0..-5);
///
/// assert_eq!(y, 5);
/// assert_eq!(z, -10);
/// ```
///
/// ```should_panic
/// # use map_range::MapRange;
/// // panics
/// let _ = 10_u32.map_range(0..5, 5..2);
/// ```
///
/// ```should_panic
/// # use map_range::MapRange;
/// // panics
/// let _ = 10_u32.map_range(20..30, 20..40);
/// ```
///
/// ```should_panic
/// # use map_range::MapRange;
/// // panics
/// let _ = 200_u8.map_range(0..10, 0..20);
/// ```
pub trait MapRange: Sized {
    #[must_use]
    fn map_range(self, from: Range<Self>, to: Range<Self>) -> Self;
}

/// Mapping a value from range `from`
/// to another range `to`.
///
/// This is a checked version of [`MapRange`]
///
/// # Examples
///
/// ```
/// # use map_range::CheckedMapRange;
/// let a = 10_u32.checked_map_range(0..5, 5..2);
/// let b = 10_u32.checked_map_range(0..5, 2..5);
///
/// assert_eq!(a, None);
/// assert_eq!(b, Some(8));
/// ```
pub trait CheckedMapRange: Sized {
    #[must_use]
    fn checked_map_range(self, from: Range<Self>, to: Range<Self>) -> Option<Self>;
}

//

impl<T> MapRange for T
where
    T: Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>,
{
    fn map_range(self, from: Range<Self>, to: Range<Self>) -> Self {
        // shamelessly stolen from my own code:
        // https://github.com/Overpeek/overpeek-engine/blob/3df11072378ba870033a19cd09fb332bcc4c466d/src/engine/utility/extra.hpp
        to.start + (self - from.start) * (to.end - to.start) / (from.end - from.start)
    }
}

impl<T> CheckedMapRange for T
where
    T: CheckedAdd<Output = Self>
        + CheckedSub<Output = Self>
        + CheckedMul<Output = Self>
        + CheckedDiv<Output = Self>,
{
    fn checked_map_range(self, from: Range<Self>, to: Range<Self>) -> Option<Self> {
        to.start.checked_add(
            &self
                .checked_sub(&from.start)?
                .checked_mul(&to.end.checked_sub(&to.start)?)?
                .checked_div(&from.end.checked_sub(&from.start)?)?,
        )
    }
}

//

#[cfg(test)]
mod tests {
    use crate::MapRange;

    #[test]
    fn test_f32_map() {
        assert!((5.0_f32.map_range(0.0..10.0, 0.0..20.0) - 10.0).abs() <= f32::EPSILON);
        assert!((5.0_f32.map_range(0.0..10.0, 10.0..0.0) - 5.0).abs() <= f32::EPSILON);
    }

    #[test]
    fn test_i32_map() {
        assert_eq!(5_i32.map_range(0..10, -10..10), 0);
    }
}
