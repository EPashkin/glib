// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Quarks - a 2-way association between a string and a unique integer identifier.
//!
//! Quark can lead to undefined behavior in gtk\glib code build with G_DISABLE_CHECKS
//! (see https://github.com/GNOME/glib/blob/master/gio/gsettings.c#L381-L383),
//! so it better not used in public interfaces.

use std::convert::From;
use std::ffi::{CStr, CString};
use std::fmt::{Display, Error, Formatter};
use glib_ffi::{self, GQuark};
use translate::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Quark(GQuark);

impl Display for Quark {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unsafe {
            let ptr = glib_ffi::g_quark_to_string(self.0);
            if ptr.is_null() {
                Ok(())
            } else {
                CStr::from_ptr(ptr).to_string_lossy().fmt(f)
            }
        }
    }
}

impl<'a> From<&'a str> for Quark {
    fn from(s: &'a str) -> Quark {
        let tmp = CString::new(s).unwrap();
        let quark = unsafe {
            glib_ffi::g_quark_from_string(tmp.as_ptr())
        };
        Quark(quark)
    }
}

impl ToGlib for Quark {
    type GlibType = GQuark;

    #[inline]
    fn to_glib(&self) -> GQuark {
        self.0
    }
}

impl FromGlib<GQuark> for Quark {
    #[inline]
    fn from_glib(val: GQuark) -> Quark {
        Quark(val)
    }
}
