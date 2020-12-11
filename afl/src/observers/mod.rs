extern crate num;

use alloc::boxed::Box;
use alloc::string::String;
use core::any::Any;
use serde::{Deserialize, Serialize};

use crate::serde_anymap::{ArrayMut, SerdeAny};
use crate::AflError;

// TODO register each observer in the Registry in new()

/// Observers observe different information about the target.
/// They can then be used by various sorts of feedback.
pub trait Observer: SerdeAny + 'static {
    /// The testcase finished execution, calculate any changes.
    #[inline]
    fn flush(&mut self) -> Result<(), AflError> {
        Ok(())
    }

    fn reset(&mut self) -> Result<(), AflError>;

    #[inline]
    fn post_exec(&mut self) -> Result<(), AflError> {
        Ok(())
    }

    fn name(&self) -> &String;
}

crate::create_serde_registry_for_trait!(observer_serde, crate::observers::Observer);

/// A MapObserver observes the static map, as oftentimes used for afl-like coverage information
pub trait MapObserver<T>
where
    T: Default + Copy,
{
    /// Get the map
    fn map(&self) -> &[T];

    /// Get the map (mutable)
    fn map_mut(&mut self) -> &mut [T];

    /// Get the initial value for reset()
    fn initial(&self) -> T;

    /// Get the initial value for reset()
    fn initial_mut(&mut self) -> &mut T;

    /// Set the initial value for reset()
    fn set_initial(&mut self, initial: T);

    /// Reset the map
    #[inline]
    fn reset_map(&mut self) -> Result<(), AflError> {
        // Normal memset, see https://rust.godbolt.org/z/Trs5hv
        let initial = self.initial();
        for i in self.map_mut().iter_mut() {
            *i = initial;
        }
        Ok(())
    }
}

/// The Map Observer retrieves the state of a map,
/// that will get updated by the target.
/// A well-known example is the AFL-Style coverage map.
#[derive(Serialize, Deserialize)]
#[serde(bound = "T: serde::de::DeserializeOwned")]
pub struct StdMapObserver<T>
where
    T: Default + Copy + 'static + serde::Serialize + serde::de::DeserializeOwned,
{
    map: ArrayMut<T>,
    initial: T,
    name: String,
}

impl<T> Observer for StdMapObserver<T>
where
    T: Default + Copy + 'static + serde::Serialize + serde::de::DeserializeOwned,
{
    #[inline]
    fn reset(&mut self) -> Result<(), AflError> {
        self.reset_map()
    }

    #[inline]
    fn name(&self) -> &String {
        &self.name
    }
}

impl<T> SerdeAny for StdMapObserver<T>
where
    T: Default + Copy + 'static + serde::Serialize + serde::de::DeserializeOwned,
{
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<T> MapObserver<T> for StdMapObserver<T>
where
    T: Default + Copy + 'static + serde::Serialize + serde::de::DeserializeOwned,
{
    #[inline]
    fn map(&self) -> &[T] {
        self.map.as_slice()
    }

    #[inline]
    fn map_mut(&mut self) -> &mut [T] {
        self.map.as_mut_slice()
    }

    #[inline]
    fn initial(&self) -> T {
        self.initial
    }

    #[inline]
    fn initial_mut(&mut self) -> &mut T {
        &mut self.initial
    }

    #[inline]
    fn set_initial(&mut self, initial: T) {
        self.initial = initial
    }
}

impl<T> StdMapObserver<T>
where
    T: Default + Copy + 'static + serde::Serialize + serde::de::DeserializeOwned,
{
    /// Creates a new MapObserver
    pub fn new(name: &'static str, map: &'static mut [T]) -> Self {
        observer_serde::RegistryBuilder::register::<Self>();
        let initial = if map.len() > 0 { map[0] } else { T::default() };
        Self {
            map: ArrayMut::Cptr((map.as_mut_ptr(), map.len())),
            initial: initial,
            name: name.into(),
        }
    }

    /// Creates a new MapObserver from a raw pointer
    pub fn new_from_ptr(name: &'static str, map_ptr: *mut T, len: usize) -> Self {
        observer_serde::RegistryBuilder::register::<Self>();
        unsafe {
            let initial = if len > 0 { *map_ptr } else { T::default() };
            StdMapObserver {
                map: ArrayMut::Cptr((map_ptr, len)),
                initial: initial,
                name: name.into(),
            }
        }
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {

    use crate::observers::{Observer, StdMapObserver};
    static mut MAP: [u32; 4] = [0; 4];

    #[test]
    fn test_observer_serde() {
        let o: Box<dyn Observer> =
            Box::new(StdMapObserver::<u32>::new("test", unsafe { &mut MAP }));
        let s = serde_json::to_string(&o).unwrap();
        println!("{}", s);
        let d: Box<dyn Observer> = serde_json::from_str(&s).unwrap();
        assert_eq!(d.name(), o.name());
    }
}