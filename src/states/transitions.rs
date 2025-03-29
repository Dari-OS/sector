//! # Sector State Transitions
//!
//! This module provides methods to convert a [`Sector`] from one state to another. Each state—such as
//! `Normal`, `Dynamic`, `Fixed`, `Locked`, `Manual`, and `Tight`—defines a different behavior regarding
//! how the underlying buffer is managed (for example, with respect to growth, shrinkage, or capacity
//! constraints). These conversion functions allow you to change the state of a sector without reallocating
//! its buffer; they simply change the type-level marker that governs its behavior.
//!
//! ## How It Works
//!
//! The conversion is implemented using the generic method [`to_custom`]. This method performs a raw
//! copy of the sector's internal buffer and length to a new sector of the target state, then uses
//! [`core::mem::forget`] to prevent the old sector from running its destructor. This way, the transition
//! is efficient and does not involve memory reallocation.
//!
//! ## Safety and Invariants
//!
//! These conversions assume that the target state's invariants are compatible with the current sector's
//! contents. The conversion functions are safe as long as these invariants hold; for example, converting
//! from a dynamically resizing sector to a fixed-capacity sector should be done only if the current
//! buffer satisfies the fixed capacity constraints.
//!
//! ## Usage Example
//!
//! ```rust
//! # use sector::Sector;
//! # use sector::states::*;
//! let normal_sector: Sector<Normal, i32> = Sector::new();
//! // Convert from a Normal sector to a Dynamic sector
//! let dynamic_sector: Sector<Dynamic, i32> = normal_sector.to_dynamic();
//! ```
//!
//! The following implementations provide state-specific conversion methods. Each method is an inline wrapper
//! around the generic [`to_custom`] method.
use crate::Sector;

impl<T, State> Sector<State, T> {
    /// Generic conversion method to transform the current sector into one with a new state.
    ///
    /// This method performs a bitwise copy of the internal buffer (`buf`), current length (`len`), and
    /// state marker, then transfers ownership to a new sector of type `Sector<Target, T>`. The original
    /// sector is forgotten to avoid running its destructor.
    ///
    /// # Safety
    ///
    /// The conversion is safe as long as the invariants of the target state are compatible with the
    /// current sector. No reallocation or modification of the buffer occurs.
    pub fn to_custom<Target>(self) -> Sector<Target, T> {
        let new_sector = Sector {
            buf: unsafe { core::ptr::read(&self.buf) },
            len: self.len,
            _state: core::marker::PhantomData,
        };
        core::mem::forget(self);
        new_sector
    }
}

impl<T> Sector<super::Normal, T> {
    /// Converts a `Normal` sector to a `Dynamic` sector.
    pub fn to_dynamic(self) -> Sector<super::Dynamic, T> {
        Self::to_custom(self)
    }

    /// Converts a `Normal` sector to a `Fixed` sector.
    pub fn to_fixed(self) -> Sector<super::Fixed, T> {
        Self::to_custom(self)
    }

    /// Converts a `Normal` sector to a `Tight` sector.
    pub fn to_tight(self) -> Sector<super::Tight, T> {
        Self::to_custom(self)
    }

    /// Converts a `Normal` sector to a `Locked` sector.
    pub fn to_locked(self) -> Sector<super::Locked, T> {
        Self::to_custom(self)
    }

    /// Converts a `Normal` sector to a `Manual` sector.
    pub fn to_manual(self) -> Sector<super::Manual, T> {
        Self::to_custom(self)
    }
}

impl<T> Sector<super::Dynamic, T> {
    /// Converts a `Dynamic` sector to a `Normal` sector.
    pub fn to_normal(self) -> Sector<super::Normal, T> {
        Self::to_custom(self)
    }

    /// Converts a `Dynamic` sector to a `Fixed` sector.
    pub fn to_fixed(self) -> Sector<super::Fixed, T> {
        Self::to_custom(self)
    }

    /// Converts a `Dynamic` sector to a `Tight` sector.
    pub fn to_tight(self) -> Sector<super::Tight, T> {
        Self::to_custom(self)
    }

    /// Converts a `Dynamic` sector to a `Locked` sector.
    pub fn to_locked(self) -> Sector<super::Locked, T> {
        Self::to_custom(self)
    }

    /// Converts a `Dynamic` sector to a `Manual` sector.
    pub fn to_manual(self) -> Sector<super::Manual, T> {
        Self::to_custom(self)
    }
}

impl<T> Sector<super::Fixed, T> {
    /// Converts a `Fixed` sector to a `Normal` sector.
    pub fn to_normal(self) -> Sector<super::Normal, T> {
        Self::to_custom(self)
    }

    /// Converts a `Fixed` sector to a `Dynamic` sector.
    pub fn to_dynamic(self) -> Sector<super::Dynamic, T> {
        Self::to_custom(self)
    }

    /// Converts a `Fixed` sector to a `Tight` sector.
    pub fn to_tight(self) -> Sector<super::Tight, T> {
        Self::to_custom(self)
    }

    /// Converts a `Fixed` sector to a `Locked` sector.
    pub fn to_locked(self) -> Sector<super::Locked, T> {
        Self::to_custom(self)
    }

    /// Converts a `Fixed` sector to a `Manual` sector.
    pub fn to_manual(self) -> Sector<super::Manual, T> {
        Self::to_custom(self)
    }
}

impl<T> Sector<super::Locked, T> {
    /// Converts a `Locked` sector to a `Normal` sector.
    pub fn to_normal(self) -> Sector<super::Normal, T> {
        Self::to_custom(self)
    }

    /// Converts a `Locked` sector to a `Dynamic` sector.
    pub fn to_dynamic(self) -> Sector<super::Dynamic, T> {
        Self::to_custom(self)
    }

    /// Converts a `Locked` sector to a `Tight` sector.
    pub fn to_tight(self) -> Sector<super::Tight, T> {
        Self::to_custom(self)
    }

    /// Converts a `Locked` sector to a `Fixed` sector.
    pub fn to_fixed(self) -> Sector<super::Fixed, T> {
        Self::to_custom(self)
    }

    /// Converts a `Locked` sector to a `Manual` sector.
    pub fn to_manual(self) -> Sector<super::Manual, T> {
        Self::to_custom(self)
    }
}

impl<T> Sector<super::Manual, T> {
    /// Converts a `Manual` sector to a `Normal` sector.
    pub fn to_normal(self) -> Sector<super::Normal, T> {
        Self::to_custom(self)
    }

    /// Converts a `Manual` sector to a `Dynamic` sector.
    pub fn to_dynamic(self) -> Sector<super::Dynamic, T> {
        Self::to_custom(self)
    }

    /// Converts a `Manual` sector to a `Tight` sector.
    pub fn to_tight(self) -> Sector<super::Tight, T> {
        Self::to_custom(self)
    }

    /// Converts a `Manual` sector to a `Fixed` sector.
    pub fn to_fixed(self) -> Sector<super::Fixed, T> {
        Self::to_custom(self)
    }

    /// Converts a `Manual` sector to a `Locked` sector.
    pub fn to_locked(self) -> Sector<super::Locked, T> {
        Self::to_custom(self)
    }
}
impl<T> Sector<super::Tight, T> {
    /// Converts a `Tight` sector to a `Normal` sector.
    pub fn to_normal(self) -> Sector<super::Normal, T> {
        Self::to_custom(self)
    }

    /// Converts a `Tight` sector to a `Dynamic` sector.
    pub fn to_dynamic(self) -> Sector<super::Dynamic, T> {
        Self::to_custom(self)
    }

    /// Converts a `Tight` sector to a `Fixed` sector.
    pub fn to_fixed(self) -> Sector<super::Fixed, T> {
        Self::to_custom(self)
    }

    /// Converts a `Tight` sector to a `Locked` sector.
    pub fn to_locked(self) -> Sector<super::Locked, T> {
        Self::to_custom(self)
    }

    /// Converts a `Tight` sector to a `Manual` sector.
    pub fn to_manual(self) -> Sector<super::Manual, T> {
        Self::to_custom(self)
    }
}
