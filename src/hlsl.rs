use core::{
    any::TypeId,
    mem::{size_of, MaybeUninit},
};

use crate::*;

pub enum Hlsl {}

shader_scalar!(in Hlsl => {
    bool | x1 = Align1, x2 = Align2, x4 = Align4;
    i32 as int | x1 = Align4, x2 = Align8, x4 = Align16;
    u32 as uint | x1 = Align4, x2 = Align8, x4 = Align16;
    f32 as float | x1 = Align4, x2 = Align8, x4 = Align16;
    f64 as double | x1 = Align8, x2 = Align16, x4 = Align32;
});

macro_rules! vec_mat_repr {
    ($($t:ty)+) => {$(
        impl<const N: usize> ShaderRepr<Hlsl> for vec<$t, N> {
            type Align = align!(in Hlsl, $t);
            type Repr = Self;

            #[inline(always)]
            fn repr(&self) -> Self {
                *self
            }
        }

        impl<const N: usize, const M: usize> ShaderRepr<Hlsl> for mat<$t, N, M> {
            type Align = align!(in Hlsl, $t);
            type Repr = Self;

            #[inline(always)]
            fn repr(&self) -> Self::Repr {
                *self
            }
        }
    )*};
}

vec_mat_repr! { bool i32 u32 f32 f64 }

impl<T, const N: usize> ShaderRepr<Hlsl> for [T; N]
where
    T: ShaderRepr<Hlsl>,
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
codegen_builtin!(in Hlsl => {
    vec2b as bool2,
    vec3b as bool3,
    vec4b as bool4,
    vec2<i32> as int2,
    vec3<i32> as int3,
    vec4<i32> as int4,
    vec2<u32> as uint2,
    vec3<u32> as uint3,
    vec4<u32> as uint4,
    vec2<f32> as float2,
    vec3<f32> as float3,
    vec4<f32> as float4,
    vec2<f64> as double2,
    vec3<f64> as double3,
    vec4<f64> as double4,
    mat2x2<bool> as bool2x2,
    mat2x3<bool> as bool2x3,
    mat2x4<bool> as bool2x4,
    mat3x2<bool> as bool3x2,
    mat3x3<bool> as bool3x3,
    mat3x4<bool> as bool3x4,
    mat4x2<bool> as bool4x2,
    mat4x3<bool> as bool4x3,
    mat4x4<bool> as bool4x4,
    mat2x2<i32> as int2x2,
    mat2x3<i32> as int2x3,
    mat2x4<i32> as int2x4,
    mat3x2<i32> as int3x2,
    mat3x3<i32> as int3x3,
    mat3x4<i32> as int3x4,
    mat4x2<i32> as int4x2,
    mat4x3<i32> as int4x3,
    mat4x4<i32> as int4x4,
    mat2x2<u32> as uint2x2,
    mat2x3<u32> as uint2x3,
    mat2x4<u32> as uint2x4,
    mat3x2<u32> as uint3x2,
    mat3x3<u32> as uint3x3,
    mat3x4<u32> as uint3x4,
    mat4x2<u32> as uint4x2,
    mat4x3<u32> as uint4x3,
    mat4x4<u32> as uint4x4,
    mat2x2<f32> as float2x2,
    mat2x3<f32> as float2x3,
    mat2x4<f32> as float2x4,
    mat3x2<f32> as float3x2,
    mat3x3<f32> as float3x3,
    mat3x4<f32> as float3x4,
    mat4x2<f32> as float4x2,
    mat4x3<f32> as float4x3,
    mat4x4<f32> as float4x4,
    mat2x2<f64> as double2x2,
    mat2x3<f64> as double2x3,
    mat2x4<f64> as double2x4,
    mat3x2<f64> as double3x2,
    mat3x3<f64> as double3x3,
    mat3x4<f64> as double3x4,
    mat4x2<f64> as double4x2,
    mat4x3<f64> as double4x3,
    mat4x4<f64> as double4x4,
});
