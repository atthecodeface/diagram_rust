/*a Copyright

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

@file    active_mask.rs
@brief   Bit mask and vectors thereof
 */

//a Imports
use num::{Integer, NumCast, Unsigned};

//a Global constants for debug
// const DEBUG_      : bool = 1 == 0;

//a BitMask
//tt BitMask
pub trait BitMask: Default + Clone + std::fmt::Debug {
    fn new() -> Self;
    #[must_use]
    fn set_capacity(self, n: usize) -> Self;
    fn set(&mut self, n: usize);
    fn clear(&mut self, n: usize);
    fn is_set(&mut self, n: usize) -> bool;
}

//a BitMaskU
//tp BitMaskU - a u8/u16/u32/u64 that has the trait BitMask
/// This struct is a generic implementation of BitMask using a single u<>
#[derive(Default, Clone)]
pub struct BitMaskU<U>
where
    U: Maskable<U> + Default,
{
    mask: U,
}

//tt Maskable - the required trait for an BitMaskU subtype is complex, so alias it
/// This is effectively a trait alias for unsigned values that support
/// copy and the relevant bit ops/casts
pub trait Maskable<U>:
    Copy
    + Unsigned // from num
    + Integer // from num
    + NumCast // from num
    + std::fmt::LowerHex
    + std::ops::BitOr<Output = U>
    + std::ops::BitAnd<Output = U>
    + std::ops::Not<Output = U>
    + std::ops::Shr<usize, Output = U>
    + std::ops::Shl<usize, Output = U>
{
}

//ip std::fmt::Debug for BitMaskU
impl<U> std::fmt::Debug for BitMaskU<U>
where
    U: Maskable<U> + Default,
{
    //mp fmt
    /// Make it human-readable
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:0x}", self.mask)
    }

    //zz All done
}

//ip BitMask for BitMaskU
/// Implementation of BitMask for an unsigned value used as a bit mask
impl<U> BitMask for BitMaskU<U>
where
    U: Maskable<U> + Default,
{
    //fp new
    /// Create a new mask
    #[inline]
    fn new() -> Self {
        let mask = U::zero();
        Self { mask }
    }

    //fp set_capacity
    /// Set the capacity
    #[inline]
    fn set_capacity(self, n: usize) -> Self {
        assert!(n <= 8 * std::mem::size_of::<U>());
        self
    }

    #[inline]
    fn set(&mut self, n: usize) {
        self.mask = self.mask | (U::one().shl(n));
    }

    #[inline]
    fn clear(&mut self, n: usize) {
        self.mask = self.mask & !(U::one().shl(n));
    }

    #[inline]
    fn is_set(&mut self, n: usize) -> bool {
        (self.mask.shr(n)) & U::one() == U::one()
    }
}

//tp BitMaskU32, BitMaskU64
impl Maskable<u32> for u32 {}
pub type BitMaskU32 = BitMaskU<u64>;
impl Maskable<u64> for u64 {}
pub type BitMaskU64 = BitMaskU<u64>;

//a BitMaskX
//tp BitMaskX - an arbitrarily-sized vector
/// This supports arbitrarily-sized vectors
///
/// It utilizes a vector of u64, so it is should be used only for
/// where more than 64 bits are required
#[derive(Debug, Clone, Default)]
pub struct BitMaskX {
    num_bits: usize,
    mask: Vec<u64>,
}

//ip BitMask for BitMaskX
/// Implementation of BitMask for an array of unsigned values used as a bit mask
impl BitMask for BitMaskX {
    //fp new
    /// Create a new mask
    #[inline]
    fn new() -> Self {
        Self {
            num_bits: 0,
            mask: vec![],
        }
    }

    //fp set_capacity
    /// Set the capacity
    #[inline]
    fn set_capacity(mut self, n: usize) -> Self {
        let num_u64 = (n + 63) / 64;
        while self.mask.len() < num_u64 {
            self.mask.push(0);
        }
        self.num_bits = n;
        self
    }

    #[inline]
    fn set(&mut self, n: usize) {
        let b = n % 64;
        let n = n / 64;
        self.mask[n] |= 1 << b;
    }

    #[inline]
    fn clear(&mut self, n: usize) {
        let b = n % 64;
        let n = n / 64;
        self.mask[n] &= !(1 << b);
    }

    #[inline]
    fn is_set(&mut self, n: usize) -> bool {
        let b = n % 64;
        let n = n / 64;
        (self.mask[n] >> b) & 1 == 1
    }
}
