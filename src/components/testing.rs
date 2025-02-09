#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct ZeroSizedType;
impl PartialEq for ZeroSizedType {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

/// Repeats the given expression _n_ times.
///
/// # Example
///
/// This:
/// ```
/// use sector::Sector;
/// use sector::states::Normal;
///
/// macro_rules! repeat {
///     ($ele:expr, $times:expr) => {{
///         for _ in 0..$times {
///             $ele;
///         }
///     }};
/// }
///
/// let mut sector: Sector<Normal, i32> = Sector::new();
/// repeat!(sector.push(123), 3);
///
/// assert_eq!(sector.len(), 3);
///
///
/// ```
///
/// is equivalent to:
/// ```
/// use sector::Sector;
/// use sector::states::Normal;
///
///
/// let mut sector: Sector<Normal, i32> = Sector::new();
///
/// sector.push(123);
/// sector.push(123);
/// sector.push(123);
///
/// assert_eq!(sector.len(), 3)
/// ```
#[allow(unused_macros)]
macro_rules! repeat {
    ($ele:expr, $times:expr) => {{
        for _ in 0..$times {
            $ele;
        }
    }};
}

#[allow(unused_imports)]
pub(crate) use repeat;
/// A helper struct that increments a shared counter when dropped
/// Only used for testing purposes
#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct DropCounter<'a> {
    /// Shared counter to increment on drop
    pub(crate) counter: &'a std::cell::Cell<i32>,
}

impl Drop for DropCounter<'_> {
    /// Increments the counter when an instance is dropped
    fn drop(&mut self) {
        self.counter.set(self.counter.get() + 1);
    }
}
