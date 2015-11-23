use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use translate::*;

/// Wrapper implementations for Boxed types. See `glib_wrapper!`.
#[macro_export]
macro_rules! glib_boxed_wrapper {
    ([$($attr:meta)*] $name:ident, $ffi_name:path, @copy $copy_arg:ident $copy_expr:expr,
     @free $free_arg:ident $free_expr:expr) => {
        $(#[$attr])*
        pub struct $name($crate::boxed::Boxed<$ffi_name, MemoryManager>);

        glib_boxed_wrapper!(@INTERNALS $name, $ffi_name, @copy $copy_arg $copy_expr,
            @free $free_arg $free_expr);
    };
    ([$($attr:meta)*] $name:ident, $ffi_name:path,
     $(@fields $fld_accessor_type:ident $fld_conv:ident $func_name:ident $fld_name:ident $fld_type:ty,)+
     @copy $copy_arg:ident $copy_expr:expr,
     @free $free_arg:ident $free_expr:expr) => {
        $(#[$attr])*
            pub struct $name($crate::boxed::Boxed<$ffi_name, MemoryManager>);

        impl $name{
            glib_boxed_wrapper!(@DECLARE_ACCESSORS $(@fields $fld_accessor_type $fld_conv $func_name $fld_name $fld_type),+);
        }

        glib_boxed_wrapper!(@INTERNALS $name, $ffi_name, @copy $copy_arg $copy_expr,
            @free $free_arg $free_expr);
    };
    (@INTERNALS $name:ident, $ffi_name:path, @copy $copy_arg:ident $copy_expr:expr,
     @free $free_arg:ident $free_expr:expr) => {
        #[doc(hidden)]
        pub struct MemoryManager;

        impl $crate::boxed::BoxedMemoryManager<$ffi_name> for MemoryManager {
            #[inline]
            unsafe fn copy($copy_arg: *const $ffi_name) -> *mut $ffi_name {
                $copy_expr
            }

            #[inline]
            unsafe fn free($free_arg: *mut $ffi_name) {
                $free_expr
            }
        }

        impl $crate::translate::Uninitialized for $name {
            #[inline]
            unsafe fn uninitialized() -> Self {
                $name($crate::boxed::Boxed::uninitialized())
            }
        }

        impl<'a> $crate::translate::ToGlibPtr<'a, *const $ffi_name> for &'a $name {
            type Storage = &'a $crate::boxed::Boxed<$ffi_name, MemoryManager>;

            #[inline]
            fn to_glib_none(&self) -> $crate::translate::Stash<'a, *const $ffi_name, Self> {
                let stash = (&self.0).to_glib_none();
                $crate::translate::Stash(stash.0, stash.1)
            }
        }

        impl<'a> $crate::translate::ToGlibPtrMut<'a, *mut $ffi_name> for $name {
            type Storage = &'a mut $crate::boxed::Boxed<$ffi_name, MemoryManager>;

            #[inline]
            fn to_glib_none_mut(&'a mut self) -> $crate::translate::StashMut<'a, *mut $ffi_name, Self> {
                let stash = self.0.to_glib_none_mut();
                $crate::translate::StashMut(stash.0, stash.1)
            }
        }

        impl $crate::translate::FromGlibPtr<*mut $ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_none(ptr: *mut $ffi_name) -> Self {
                $name($crate::translate::from_glib_none(ptr))
            }

            #[inline]
            unsafe fn from_glib_full(ptr: *mut $ffi_name) -> Self {
                $name($crate::translate::from_glib_full(ptr))
            }

            #[inline]
            unsafe fn from_glib_borrow(ptr: *mut $ffi_name) -> Self {
                $name($crate::translate::from_glib_borrow(ptr))
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                $name(self.0.clone())
            }
        }
    };
    (@DECLARE_ACCESSORS @fields $fld_accessor_type:ident $fld_conv:ident $func_name:ident $fld_name:ident $fld_type:ty,
     $(@fields $rest_fld_accessor_type:ident $rest_fld_conv:ident $rest_func_name:ident $rest_fld_name:ident $rest_fld_type:ty),+
     ) => {
        glib_boxed_wrapper!(@DECLARE_ACCESSOR $fld_accessor_type $fld_conv $func_name $fld_name $fld_type);
        glib_boxed_wrapper!(@DECLARE_ACCESSORS $(@fields $rest_fld_accessor_type $rest_fld_conv $rest_func_name $rest_fld_name $rest_fld_type),+);
    };
    (@DECLARE_ACCESSORS @fields $fld_accessor_type:ident $fld_conv:ident $func_name:ident $fld_name:ident $fld_type:ty
     ) => {
        glib_boxed_wrapper!(@DECLARE_ACCESSOR $fld_accessor_type $fld_conv $func_name $fld_name $fld_type);
    };
    (@DECLARE_ACCESSOR get direct $func_name:ident $fld_name:ident $fld_type:ty
     ) => {
        pub fn $func_name(&self) -> $fld_type {
            (self.0).$fld_name
        }
    };
    (@DECLARE_ACCESSOR put direct $func_name:ident $fld_name:ident $fld_type:ty
     ) => {
        pub fn $func_name(&mut self, val: $fld_type) {
            (self.0).$fld_name = val;
        }
    };
    (@DECLARE_ACCESSOR get pointer $func_name:ident $fld_name:ident $fld_type:ty
     ) => {
        pub fn $func_name(&self) -> $fld_type {
            unsafe {
                from_glib_none((self.0).$fld_name)
            }
        }
    };
}

enum AnyBox<T> {
    Native(Box<T>),
    ForeignOwned(*mut T),
    ForeignBorrowed(*mut T),
}

/// Memory management functions for a boxed type.
pub trait BoxedMemoryManager<T>: 'static {
    /// Makes a copy.
    unsafe fn copy(ptr: *const T) -> *mut T;
    /// Frees the object.
    unsafe fn free(ptr: *mut T);
}

/// Encapsulates memory management logic for boxed types.
pub struct Boxed<T: 'static, MM: BoxedMemoryManager<T>> {
    inner: AnyBox<T>,
    _dummy: PhantomData<MM>,
}

impl<T: 'static, MM: BoxedMemoryManager<T>> Deref for Boxed<T, MM> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        use self::AnyBox::*;
        match self.inner {
            Native(ref b) => b,
            ForeignOwned(p) | ForeignBorrowed(p) => unsafe{&*p},
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> DerefMut for Boxed<T, MM> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        use self::AnyBox::*;
        match self.inner {
            Native(ref mut b) => b,
            ForeignOwned(p) | ForeignBorrowed(p) => unsafe{&mut*p},
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> Boxed<T, MM> {
    #[inline]
    pub unsafe fn uninitialized() -> Self {
        Boxed {
            inner: AnyBox::Native(Box::new(mem::uninitialized())),
            _dummy: PhantomData,
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> Uninitialized for Boxed<T, MM> {
    #[inline]
    unsafe fn uninitialized() -> Self {
        Boxed { 
            inner: AnyBox::Native(Box::new(mem::uninitialized())),
            _dummy: PhantomData,
        }
    }
}

impl<'a, T: 'static, MM: BoxedMemoryManager<T>> ToGlibPtr<'a, *const T> for &'a Boxed<T, MM> {
    type Storage = Self;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *const T, Self> {
        use self::AnyBox::*;
        let ptr = match self.inner {
            Native(ref b) => &**b as *const T,
            ForeignOwned(p) | ForeignBorrowed(p) => p as *const T,
        };
        Stash(ptr, *self)
    }
}

impl<'a, T: 'static, MM: BoxedMemoryManager<T>> ToGlibPtrMut<'a, *mut T> for Boxed<T, MM> {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut T, Self> {
        use self::AnyBox::*;
        let ptr = match self.inner {
            Native(ref mut b) => &mut **b as *mut T,
            ForeignOwned(p) | ForeignBorrowed(p) => p,
        };
        StashMut(ptr, self)
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> FromGlibPtr<*mut T> for Boxed<T, MM> {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        let ptr = MM::copy(ptr);
        from_glib_full(ptr)
    }

    #[inline]
    unsafe fn from_glib_full(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        Boxed {
            inner: AnyBox::ForeignOwned(ptr),
            _dummy: PhantomData,
        }
    }

    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        Boxed {
            inner: AnyBox::ForeignBorrowed(ptr),
            _dummy: PhantomData,
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> Drop for Boxed<T, MM> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if let AnyBox::ForeignOwned(ptr) = self.inner {
                MM::free(ptr);
            }
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> Clone for Boxed<T, MM> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            from_glib_none(self.to_glib_none().0 as *mut T)
        }
    }
}
