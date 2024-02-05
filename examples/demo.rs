use std::fmt::Display;

use shader_layout::{vec3, Align16};

struct StructLayout {
    name: &'static str,
    fields: Vec<FieldLayout>,
}

impl Display for StructLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "struct {} {{", self.name)?;
        for field in &self.fields {
            writeln!(
                f,
                "    {}: {} @ {}..{} | {},",
                field.name,
                field.ty,
                field.offset,
                field.offset + field.size,
                field.size
            )?;
        }
        writeln!(f, "}}")
    }
}

struct FieldLayout {
    name: &'static str,
    ty: &'static str,
    offset: usize,
    size: usize,
}

macro_rules! offset_of {
    ($field:ident in $struct:ty) => {{
        let s = ::std::mem::MaybeUninit::<$struct>::uninit().as_ptr();
        let f = unsafe { ::std::ptr::addr_of!((*s).$field) };
        (f as usize - s as usize)
    }};
}

macro_rules! struct_layout {
    ($s:ty {$($fn:ident: $ft:ty),* $(,)?}) => {
        StructLayout {
            name: stringify!($s),
            fields: ::std::vec![
                $(
                    FieldLayout {
                        name: stringify!($fn),
                        ty: stringify!($ft),
                        offset: offset_of!($fn in $s),
                        size: ::std::mem::size_of::<$ft>(),
                    }
                ),*
            ],
        }
    };
}

#[repr(C)]
struct Foo {
    a: f32,
    _b: Align16,
    b: vec3,
    c: f32,
    _d: Align16,
    d: vec3,
    _e: Align16,
    e: vec3,
}

fn main() {
    println!(
        "{}",
        struct_layout!(Foo {
            a: f32,
            b: vec3,
            c: f32,
            d: vec3,
            e: vec3
        })
    );
}
