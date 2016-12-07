use std::ops::Deref;
use std::fmt;

use libc::c_char;
use std::ffi::CStr;

use gobject;

/// Represents an owned gobject `T`
pub struct Ptr<T> {
    data: *mut T
}

impl<T> Ptr<T> {
    pub unsafe fn new(data: *mut T) -> Ptr<T> {
        Ptr { data: data }
    }

    pub fn set_property(&self,
                        prop: *const i8,
                        value: *const gobject::GValue) {
        unsafe {gobject::g_object_set_property(self.data as *mut gobject::GObject,
                                       prop, value)};
    }
}

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        unsafe {
            gobject::g_object_ref(self.data as *mut gobject::GObject);
            Ptr { data: self.data }
        }
    }
}

impl<T> Deref for Ptr<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            &(*self.data)
        }
    }
}

impl<T> Drop for Ptr<T> {
    fn drop(&mut self) {
        unsafe {
            gobject::g_object_unref(self.data as *mut gobject::GObject);
        }
    }
}

impl<T> fmt::Display for Ptr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c_buf: *const c_char = unsafe {
            gobject::g_type_name_from_instance(
                self.data as *mut gobject::GTypeInstance)
        };

        let c_str: &CStr = unsafe { CStr::from_ptr(c_buf)  };
        let typename = c_str.to_str().unwrap();

        write!(f, "{}<{:p}>", typename, self.data)
    }
}
