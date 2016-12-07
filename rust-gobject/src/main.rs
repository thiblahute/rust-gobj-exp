#![allow(dead_code)]

extern crate libc;
extern crate gobject_2_0_sys as gobject;
extern crate gtypes as gtypes;
#[macro_use]
extern crate lazy_static;

use gtypes::gpointer;
use std::cell::Cell;
use std::mem;
use std::ptr;

mod pointer;
use self::pointer::Ptr;

#[repr(C)]
enum TestFooProperties {
    PROP_FOO_NONE,
    PROP_FOO_BAR
}

#[repr(C)]
struct TestFoo {
    parent_instance: gobject::GObject,
    private: Box<TestFooPrivate>
}

impl TestFoo {
    fn private(&self) -> &TestFooPrivate { &self.private }
}

#[repr(C)]
struct TestFooPrivate {
    some: Cell<usize>,
}

#[repr(C)]
struct TestFooClass {
    parent_class: gobject::GObjectClass
}

extern "C" fn test_foo_class_init(klass: *mut TestFooClass) {
    unsafe {
        gobject::g_type_class_add_private(klass as g::gpointer, mem::size_of::<TestFooPrivate>());
        let g_object_class = klass as *mut gobject::GObjectClass;

        let finalize: gobject::GObjectFinalizeFunc = Some(test_foo_finalize);
        let set: gobject::GObjectSetPropertyFunc = Some(test_foo_set_property);
        let get: gobject::GObjectGetPropertyFunc = Some(test_foo_get_property);

        (*g_object_class).finalize = finalize;
        (*g_object_class).set_property = set;
        (*g_object_class).get_property = get;

        gobject::g_object_class_install_property (
            g_object_class, TestFooProperties::PROP_FOO_BAR as u32,
            gobject::g_param_spec_string (b"foo-bar"  as *const u8 as *const i8,
                                           b"Foo bar"  as *const u8 as *const i8,
                                           b"Just a fooish bargy"  as *const u8 as *const i8,
                                           0 as *const i8,
                                           gobject::GParamFlags::from_bits(3).unwrap()));
    }
}

#[allow(unused_variables)]
unsafe extern "C" fn test_foo_get_property(object: *mut gobject::GObject,
                                           property_id: u32,
                                           value: *mut gobject::GValue,
                                           pspec: *mut gobject::GParamSpec) {
    println!("Getting {}", property_id);
}

#[allow(unused_variables)]
unsafe extern "C" fn test_foo_set_property(object: *mut gobject::GObject,
                                    property_id: libc::c_uint,
                                    value: *const gobject::GValue,
                                    pspec: *mut gobject::GParamSpec)
{
    println!("Setting {}", property_id);
}

unsafe extern "C" fn test_foo_finalize(object: *mut gobject::GObject) {
        let foo = object as *mut TestFoo;
        mem::drop(&mut (*foo).private);

        // FIXME -- g_class field of `GTypeInstance` ought to be `pub`
        let xxx = object as *mut *mut gobject::GTypeClass;
        let object_class = *xxx;
        let parent_class = gobject::g_type_class_peek_parent(object_class as g::gpointer);
        let g_object_class = parent_class as *mut gobject::GObjectClass;
        ((*g_object_class).finalize.unwrap())(object);
}

#[allow(dead_code)]
extern "C" fn test_foo_instance_init(obj: *mut TestFoo) {
    let private = Box::new(TestFooPrivate {
        some: Cell::new(5)
    });

    unsafe {ptr::write(&mut (*obj).private, private)};
}

fn test_foo_construct(object_type: g::GType) -> Ptr<TestFoo> {
    unsafe {
        let this: *mut TestFoo = gobject::g_object_new(object_type, ptr::null_mut()) as *mut TestFoo;
        Ptr::new(this)
    }
}

fn test_foo_new() -> Ptr<TestFoo> {
    test_foo_construct(*TEST_TYPE_FOO)
}

lazy_static! {
    pub static ref TEST_TYPE_FOO: g::GType = {
        unsafe {
            gobject::g_type_register_static_simple(
                gobject::g_object_get_type(),
                "TestFoo".to_glib_none().0,
                mem::size_of::<TestFooClass>() as u32,
                mem::transmute(test_foo_class_init as extern "C" fn(*mut TestFooClass)),
                mem::size_of::<TestFoo>() as u32,
                mem::transmute(test_foo_instance_init as extern "C" fn(*mut TestFoo)),
                mem::transmute(0)) // FIXME GTypeFlags should not be an enum
       }
    };
}

fn main() {
    let bar = test_foo_new();

    bar.set_property(b"foo-bar" as *const u8 as *const i8,
                     glib::Value::from("HELLO!").to_glib_none().0);
    println!("Bar is: {}", bar);
}
