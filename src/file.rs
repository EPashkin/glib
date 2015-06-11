// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use libc::{c_char};
use object::{GenericObject, Ref, Upcast, Wrapper};
use translate::*;
use types;

pub type File = GenericObject<ffi::GFile>;

impl types::StaticType for File {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::g_file_get_type()) }
    }
}

pub trait FileExt{
    /// Constructs a File for a given path. This operation never fails,
    /// but the returned object might not support any I/O operation if path is malformed.  
    fn new_for_path(path: &str)-> Self;
    /// Constructs a GFile for a given URI. This operation never fails,
    /// but the returned object might not support any I/O operation if uri is malformed or if the uri type is not supported.
    fn new_for_uri(uri: &str)-> Self;
    /// Gets the local pathname for GFile, if one exists.
    /// This call does no blocking I/O.
    fn get_path(&mut self) -> Option<String>;
    /// Gets the URI for the file.
    /// This call does no blocking I/O.
    fn get_uri(&mut self) -> String;
}

impl<O: Upcast<File>> FileExt for O {
    fn new_for_path(path: &str)-> Self {
        unsafe{
            let str2 = path.to_glib_none().0;
            let res: *mut ffi::GFile = ffi::g_file_new_for_path(str2);
            let res2: Ref = Ref::from_glib_full(res as *mut ffi::GObject);
            let res3: Self = Self::wrap(res2);
            res3
        }
    }
    fn new_for_uri(uri: &str)-> Self {
        unsafe{
            let str2 = uri.to_glib_none().0;
            let res: *mut ffi::GFile = ffi::g_file_new_for_uri(str2);
            let res2: Ref = Ref::from_glib_full(res as *mut ffi::GObject);
            let res3: Self = Self::wrap(res2);
            res3
        }
    }
    fn get_path(&mut self) -> Option<String> {
        unsafe {
            let this = self.upcast().to_glib_none().0;
            from_glib_full(ffi::g_file_get_path(this) as *const c_char)
        }
    }
    fn get_uri(&mut self) -> String {
        unsafe {
            let this = self.upcast().to_glib_none().0;
            from_glib_full(ffi::g_file_get_uri(this) as *const c_char)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(unused_variables)]
    fn constructed_from_path() {
      let file: File = File::new_for_path("a.txt");
    }

    #[test]
    #[allow(unused_variables)]
    fn constructed_from_uri() {
      let file: File = File::new_for_uri("https://github.com/rust-gnome/glib");
    }

    #[test]
    fn get_path_works_when_from_path(){
      let mut file = File::new_for_path("a.txt");
      file.get_path().unwrap();
    }
    
    #[test]
    fn get_path_not_work_when_from_uri(){
      let mut file = File::new_for_uri("https://github.com/rust-gnome/glib");
      assert_eq!(file.get_path(), None);
    }

    #[test]
    fn get_uri_works_when_from_uri(){
      let mut file = File::new_for_uri("https://github.com/rust-gnome/glib");
      file.get_uri();
    }
    
    #[test]
    fn get_uri_works_when_from_path(){
      let mut file = File::new_for_path("a.txt");
      file.get_uri();
    }
}
