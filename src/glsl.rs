use core::{
    any::TypeId,
    mem::{size_of, MaybeUninit},
};

use crate::*;

pub enum Glsl {}

pub type Std140 = DefaultLayout;

pub enum Std430 {}

shader_scalar!(in Glsl => {
    bool | x1 = Align1, x2 = Align2, x4 = Align4;
    i32 as int | x1 = Align4, x2 = Align8, x4 = Align16;
    u32 as uint | x1 = Align4, x2 = Align8, x4 = Align16;
    f32 as float | x1 = Align4, x2 = Align8, x4 = Align16;
    f64 as double | x1 = Align8, x2 = Align16, x4 = Align32;
});

macro_rules! vec_mat_repr {
    ($($t:ty)+) => {$(
        impl<Std> ShaderRepr<Glsl, Std> for vec<$t, 2>
        where
            $t: ShaderScalar<Glsl>,
        {
            type Align = align!(in Glsl, $t, x2);
            type Repr = Self;

            #[inline(always)]
            fn repr(&self) -> Self {
                *self
            }
        }

        impl<Std> ShaderRepr<Glsl, Std> for vec<$t, 3>
        where
            $t: ShaderScalar<Glsl>,
        {
            type Align = align!(in Glsl, $t, x4);
            type Repr = Self;

            #[inline(always)]
            fn repr(&self) -> Self {
                *self
            }
        }

        impl<Std> ShaderRepr<Glsl, Std> for vec<$t, 4>
        where
            $t: ShaderScalar<Glsl>,
        {
            type Align = align!(in Glsl, $t, x4);
            type Repr = Self;

            #[inline(always)]
            fn repr(&self) -> Self {
                *self
            }
        }

        impl<const N: usize> ShaderRepr<Glsl, Std140> for mat<$t, N, 2> {
            type Repr = [Aligned<Self::Align, vec<$t, 2>>; N];
            type Align = (Align16, align!(in Glsl, $t, x2));

            #[inline(always)]
            fn repr(&self) -> Self::Repr {
                self.0.map(|v| Aligned::new(vec(v)))
            }
        }

        impl<const N: usize> ShaderRepr<Glsl, Std140> for mat<$t, N, 3> {
            type Align = (Align16, align!(in Glsl, $t, x4));
            type Repr = [Aligned<Self::Align, vec<$t, 3>>; N];

            #[inline(always)]
            fn repr(&self) -> Self::Repr {
                self.0.map(|v| Aligned::new(vec(v)))
            }
        }

        impl<const N: usize> ShaderRepr<Glsl, Std140> for mat<$t, N, 4> {
            type Align = (Align16, align!(in Glsl, $t, x4));
            type Repr = [Aligned<Self::Align, vec<$t, 4>>; N];

            #[inline(always)]
            fn repr(&self) -> Self::Repr {
                self.0.map(|v| Aligned::new(vec(v)))
            }
        }

        impl<const N: usize> ShaderRepr<Glsl, Std430> for mat<$t, N, 2> {
            type Align = align!(in Glsl, $t, x2);
            type Repr = [Aligned<Self::Align, vec<$t, 2>>; N];

            #[inline(always)]
            fn repr(&self) -> Self::Repr {
                self.0.map(|v| Aligned::new(vec(v)))
            }
        }

        impl<const N: usize> ShaderRepr<Glsl, Std430> for mat<$t, N, 3> {
            type Align = align!(in Glsl, $t, x4);
            type Repr = [Aligned<Self::Align, vec<$t, 3>>; N];

            #[inline(always)]
            fn repr(&self) -> Self::Repr {
                self.0.map(|v| Aligned::new(vec(v)))
            }
        }

        impl<const N: usize> ShaderRepr<Glsl, Std430> for mat<$t, N, 4> {
            type Align = align!(in Glsl, $t, x4);
            type Repr = [Aligned<Self::Align, vec<$t, 4>>; N];

            #[inline(always)]
            fn repr(&self) -> Self::Repr {
                self.0.map(|v| Aligned::new(vec(v)))
            }
        }
    )*};
}

vec_mat_repr! { bool i32 u32 f32 f64 }

impl<T, const N: usize> ShaderRepr<Glsl, Std140> for [T; N]
where
    T: ShaderRepr<Glsl, Std140>,
{
    type Align = (Align16, T::Align);
    type Repr = [Aligned<Self::Align, T::Repr>; N];

    #[inline(always)]
    fn repr(&self) -> Self::Repr {
        if TypeId::of::<T>() == TypeId::of::<T::Repr>()
            && size_of::<Self::Repr>() == size_of::<Self>()
        {
            // SAFETY: Element type matches layout.
            unsafe { core::mem::transmute_copy(self) }
        } else {
            // SAFETY: Array of `MaybeUninit` is always initialized.
            let mut array: [MaybeUninit<_>; N] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in self.iter().enumerate() {
                array[i] = MaybeUninit::new(Aligned::new(elem.repr()));
            }

            // SAFETY: Every element of `array` was initialized.
            array.map(|elem| unsafe { MaybeUninit::assume_init(elem) })
        }
    }
}

impl<T, const N: usize> ShaderRepr<Glsl, Std430> for [T; N]
where
    T: ShaderRepr<Glsl, Std430>,
{
    type Align = T::Align;
    type Repr = [Aligned<Self::Align, T::Repr>; N];

    #[inline(always)]
    fn repr(&self) -> Self::Repr {
        if TypeId::of::<T>() == TypeId::of::<T::Repr>()
            && size_of::<Self::Repr>() == size_of::<Self>()
        {
            // SAFETY: Element type matches layout.
            unsafe { core::mem::transmute_copy(self) }
        } else {
            // SAFETY: Array of `MaybeUninit` is always initialized.
            let mut array: [MaybeUninit<_>; N] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in self.iter().enumerate() {
                array[i] = MaybeUninit::new(Aligned::new(elem.repr()));
            }

            // SAFETY: Every element of `array` was initialized.
            array.map(|elem| unsafe { MaybeUninit::assume_init(elem) })
        }
    }
}

#[cfg(feature = "codegen")]
codegen_builtin!(in Glsl => {
    vec2b as bvec2,
    vec3b as bvec3,
    vec4b as bvec4,
    vec2i as ivec2,
    vec3i as ivec3,
    vec4i as ivec4,
    vec2u as uvec2,
    vec3u as uvec3,
    vec4u as uvec4,
    vec2f as vec2,
    vec3f as vec3,
    vec4f as vec4,
    vec2<f64> as dvec2,
    vec3<f64> as dvec3,
    vec4<f64> as dvec4,
    mat2x2b as bmat2x2,
    mat2x3b as bmat2x3,
    mat2x4b as bmat2x4,
    mat3x2b as bmat3x2,
    mat3x3b as bmat3x3,
    mat3x4b as bmat3x4,
    mat4x2b as bmat4x2,
    mat4x3b as bmat4x3,
    mat4x4b as bmat4x4,
    mat2x2i as imat2x2,
    mat2x3i as imat2x3,
    mat2x4i as imat2x4,
    mat3x2i as imat3x2,
    mat3x3i as imat3x3,
    mat3x4i as imat3x4,
    mat4x2i as imat4x2,
    mat4x3i as imat4x3,
    mat4x4i as imat4x4,
    mat2x2u as umat2x2,
    mat2x3u as umat2x3,
    mat2x4u as umat2x4,
    mat3x2u as umat3x2,
    mat3x3u as umat3x3,
    mat3x4u as umat3x4,
    mat4x2u as umat4x2,
    mat4x3u as umat4x3,
    mat4x4u as umat4x4,
    mat2x2f as mat2x2,
    mat2x3f as mat2x3,
    mat2x4f as mat2x4,
    mat3x2f as mat3x2,
    mat3x3f as mat3x3,
    mat3x4f as mat3x4,
    mat4x2f as mat4x2,
    mat4x3f as mat4x3,
    mat4x4f as mat4x4,
    mat2x2<f64> as dmat2x2,
    mat2x3<f64> as dmat2x3,
    mat2x4<f64> as dmat2x4,
    mat3x2<f64> as dmat3x2,
    mat3x3<f64> as dmat3x3,
    mat3x4<f64> as dmat3x4,
    mat4x2<f64> as dmat4x2,
    mat4x3<f64> as dmat4x3,
    mat4x4<f64> as dmat4x4,
});
