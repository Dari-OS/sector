use crate::Sector;

impl<T, State> Sector<State, T> {
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
    pub fn to_dynamic(self) -> Sector<super::Dynamic, T> {
        Self::to_custom(self)
    }

    pub fn to_fixed(self) -> Sector<super::Fixed, T> {
        Self::to_custom(self)
    }

    pub fn to_tight(self) -> Sector<super::Fixed, T> {
        Self::to_custom(self)
    }

    pub fn to_locked(self) -> Sector<super::Locked, T> {
        Self::to_custom(self)
    }

    pub fn to_manual(self) -> Sector<super::Manual, T> {
        Self::to_custom(self)
    }
}

impl<T> Sector<super::Dynamic, T> {
    pub fn to_normal(self) -> Sector<super::Normal, T> {
        Self::to_custom(self)
    }

    pub fn to_fixed(self) -> Sector<super::Fixed, T> {
        Self::to_custom(self)
    }

    pub fn to_tight(self) -> Sector<super::Fixed, T> {
        Self::to_custom(self)
    }

    pub fn to_locked(self) -> Sector<super::Locked, T> {
        Self::to_custom(self)
    }

    pub fn to_manual(self) -> Sector<super::Manual, T> {
        Self::to_custom(self)
    }
}

impl<T> Sector<super::Fixed, T> {
    pub fn to_normal(self) -> Sector<super::Normal, T> {
        Self::to_custom(self)
    }

    pub fn to_dynamic(self) -> Sector<super::Dynamic, T> {
        Self::to_custom(self)
    }

    pub fn to_tight(self) -> Sector<super::Tight, T> {
        Self::to_custom(self)
    }

    pub fn to_locked(self) -> Sector<super::Locked, T> {
        Self::to_custom(self)
    }

    pub fn to_manual(self) -> Sector<super::Manual, T> {
        Self::to_custom(self)
    }
}

impl<T> Sector<super::Locked, T> {
    pub fn to_normal(self) -> Sector<super::Normal, T> {
        Self::to_custom(self)
    }

    pub fn to_dynamic(self) -> Sector<super::Dynamic, T> {
        Self::to_custom(self)
    }

    pub fn to_tight(self) -> Sector<super::Fixed, T> {
        Self::to_custom(self)
    }

    pub fn to_fixed(self) -> Sector<super::Fixed, T> {
        Self::to_custom(self)
    }

    pub fn to_manual(self) -> Sector<super::Manual, T> {
        Self::to_custom(self)
    }
}

impl<T> Sector<super::Manual, T> {
    pub fn to_normal(self) -> Sector<super::Normal, T> {
        Self::to_custom(self)
    }

    pub fn to_dynamic(self) -> Sector<super::Dynamic, T> {
        Self::to_custom(self)
    }

    pub fn to_tight(self) -> Sector<super::Fixed, T> {
        Self::to_custom(self)
    }

    pub fn to_fixed(self) -> Sector<super::Fixed, T> {
        Self::to_custom(self)
    }

    pub fn to_locked(self) -> Sector<super::Locked, T> {
        Self::to_custom(self)
    }
}
