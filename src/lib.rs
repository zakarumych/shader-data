//! This crate provides a simple way to convert a value into same value
//! with layout expected by the shader.
//!
//! The primary trait is `ShaderRepr` which is implemented for
//! fundamental types that can be sent to the shaders.
//! And derive macro to implement it for user-defined structures and enums (with only unit variants).
#![cfg_attr(not(feature = "codegen"), no_std)]

#[macro_export]
macro_rules! ident_or_ident {
    ($ident:ident) => {
        $ident
    };
    ($_:ident $ident:ident) => {
        $model
    };
}

/// Implement traits for scalar types in the shader languages.
///
/// This implements `ShaderRepr` and `ShaderScalar` for the type.
/// And if `codegen` feature is enabled it also implements `CodeGen`.
#[macro_export]
macro_rules! shader_scalar {
    (in $lang:ident => $t:ident $(as $st:ident)? | x1 = $align:ident, x2 = $align_x2:ident, x4 = $align_x4:ident) => {
        /// Repr is the same in all layouts for scalars.
        impl<Layout> $crate::ShaderRepr<$lang, Layout> for $t {
            type Align = $align;
            type Repr = Self;

            #[inline(always)]
            fn repr(&self) -> Self::Repr {
                *self as _
            }
        }

        impl $crate::ShaderScalar<$lang> for $t {
            type AlignX2 = $align_x2;
            type AlignX4 = $align_x4;
        }

        #[cfg(feature = "codegen")]
        impl $crate::CodeGen<$lang> for $t {
            #[inline(always)]
            fn name() -> ::std::borrow::Cow<'static, str> {
                ::std::borrow::Cow::Borrowed({::core::stringify!($t) $(;::core::stringify!($st))?})
            }

            #[inline(always)]
            fn definition() -> ::std::option::Option<::std::string::String> {
                ::std::option::Option::None
            }
        }
    };
    (in $lang:ident as $model:ident => {
        $($t:ident $(as $st:ident)? | x1 = $align:ident, x2 = $align_x2:ident, x4 = $align_x4:ident;)+
    }) => {
        $(shader_scalar!(in $lang as $model => $t $(as $st)? | x1 = $align, x2 = $align_x2, x4 = $align_x4);)+
    };
    (in $lang:ident => {
        $($t:ident $(as $st:ident)? | x1 = $align:ident, x2 = $align_x2:ident, x4 = $align_x4:ident;)+
    }) => {
        $(shader_scalar!(in $lang => $t $(as $st)? | x1 = $align, x2 = $align_x2, x4 = $align_x4);)+
    };
}

#[macro_export]
macro_rules! align {
    (in $lang:ident $(as $model:ident)?, $t:ty) => {
        <$t as $crate::ShaderRepr<$lang $(, $model)?>>::Align
    };
    (in $lang:ident, $t:ty, x2) => {
        <$t as $crate::ShaderScalar<$lang>>::AlignX2
    };
    (in $lang:ident $(as $model:ident)?, $t:ty, x4) => {
        <$t as $crate::ShaderScalar<$lang>>::AlignX4
    };
}

/// Implement `CodeGen` for the built-in types.
#[cfg(feature = "codegen")]
#[macro_export]
macro_rules! codegen_builtin {
    (in $lang:ident => $t:ty) => {
        impl CodeGen<$lang> for $t {
            fn name() -> ::std::borrow::Cow<'static, str> {
                // Built-in types
                ::std::borrow::Cow::Borrowed(::core::stringify!($t))
            }

            #[inline(always)]
            fn definition() -> ::std::option::Option<::std::string::String> {
                // Built-in types need no definition.
                ::std::option::Option::None
            }
        }
    };
    (in $lang:ident => $t:ty as $st:ident) => {
        impl CodeGen<$lang> for $t {
            fn name() -> ::std::borrow::Cow<'static, str> {
                // Built-in types
                ::std::borrow::Cow::Borrowed(::core::stringify!($st))
            }

            #[inline(always)]
            fn definition() -> ::std::option::Option<::std::string::String> {
                // Built-in types need no definition.
                ::std::option::Option::None
            }
        }
    };
    (in $lang:ident => { $($t:ty $(as $st:ident)?),+ $(,)? }) => {
        $(codegen_builtin!(in $lang => $t $(as $st)?);)+
    };
}

mod types;

pub use self::types::*;

#[cfg(feature = "glsl")]
mod glsl;

#[cfg(feature = "wgsl")]
mod wgsl;

#[cfg(feature = "hlsl")]
mod hlsl;

#[cfg(feature = "msl")]
mod msl;

pub enum DefaultLayout {}

/// Trait for types that can be sent to the shaders.
/// It provides repr-types for different shader languages (like GLSL, HLSL, etc.)
/// and methods to create repr-values from `&self`.
///
/// Each shader language must be enabled with a feature flag.
pub trait ShaderRepr<Lang, Layout = DefaultLayout>: 'static {
    /// Alignment ZST for the type.
    ///
    /// In Rust size of the type is always a multiple of its alignment.
    /// So it's not possible to make `Repr` to have the alignment required by shader language.
    /// Instead we use this type to align the value in composite types.
    type Align: Default + Copy + 'static;

    /// Lang representation of the type.
    type Repr: Copy + 'static;

    /// Create a repr-value from `&self`.
    fn repr(&self) -> Self::Repr;
}

/// Trait for scalar types in the shader languages.
/// They must be represented by self in the shader languages in any layout.
pub trait ShaderScalar<Lang>: ShaderRepr<Lang, Repr = Self> + Copy {
    /// Doubled alignment ZST for the type.
    type AlignX2: Default + Copy + 'static;

    /// Quadrupled alignment ZST for the type.
    type AlignX4: Default + Copy + 'static;
}

/// ZST with alignment of 1.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C, align(1))]
pub struct Align1;

/// ZST with alignment of 2.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C, align(2))]
pub struct Align2;

/// ZST with alignment of 4.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C, align(4))]
pub struct Align4;

/// ZST with alignment of 8.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C, align(8))]
pub struct Align8;

/// ZST with alignment of 16.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C, align(16))]
pub struct Align16;

/// ZST with alignment of 32.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C, align(32))]
pub struct Align32;

/// Combines alignment and type.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Aligned<A, T> {
    align: A,
    value: T,
}

impl<A, T> Aligned<A, T>
where
    A: Default,
{
    /// Create a new `Aligned` from `value`.
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Aligned {
            align: Default::default(),
            value,
        }
    }
}

#[cfg(feature = "codegen")]
pub trait CodeGen<Lang> {
    /// Name of the type in the shader language.
    ///
    /// When using code-generation returned string will be used
    /// to refer to the type in the generated code.
    fn name() -> std::borrow::Cow<'static, str>;

    /// Shader language specific definition of the type.
    ///
    /// When using code-generation returned string will be
    /// added once to the generated code if some.
    fn definition() -> Option<String>;
}
