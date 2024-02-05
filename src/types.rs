#![allow(non_camel_case_types)]

/// 16-bit floating point type compatible with shader languages.
#[derive(Clone, Copy, Debug, Default)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct f16(pub u16);

/// Vector type compatible with shader languages.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct vec<T, const N: usize>(pub [T; N]);

/// Matrix type compatible with shader languages.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct mat<T, const N: usize, const M: usize>(pub [[T; M]; N]);

/// Handy type aliases for two-component vector.
pub type vec2<T> = vec<T, 2>;

/// Handy type aliases for three-component vector.
pub type vec3<T> = vec<T, 3>;

/// Handy type aliases for four-component vector.
pub type vec4<T> = vec<T, 4>;

/// Handy type aliases for 2x2 matrix.
pub type mat2x2<T> = mat<T, 2, 2>;

/// Handy type aliases for 2x3 matrix.
pub type mat2x3<T> = mat<T, 2, 3>;

/// Handy type aliases for 2x4 matrix.
pub type mat2x4<T> = mat<T, 2, 4>;

/// Handy type aliases for 3x2 matrix.
pub type mat3x2<T> = mat<T, 3, 2>;

/// Handy type aliases for 3x3 matrix.
pub type mat3x3<T> = mat<T, 3, 3>;

/// Handy type aliases for 3x4 matrix.
pub type mat3x4<T> = mat<T, 3, 4>;

/// Handy type aliases for 4x2 matrix.
pub type mat4x2<T> = mat<T, 4, 2>;

/// Handy type aliases for 4x3 matrix.
pub type mat4x3<T> = mat<T, 4, 3>;

/// Handy type aliases for 4x4 matrix.
pub type mat4x4<T> = mat<T, 4, 4>;

/// Short-hand for `mat2x2`.
pub type mat2<T> = mat2x2<T>;

/// Short-hand for `mat3x3`.
pub type mat3<T> = mat3x3<T>;

/// Short-hand for `mat4x4`.
pub type mat4<T> = mat4x4<T>;

/// Handy type aliases for two-component vector.
pub type vec2b = vec<bool, 2>;

/// Handy type aliases for three-component vector.
pub type vec3b = vec<bool, 3>;

/// Handy type aliases for four-component vector.
pub type vec4b = vec<bool, 4>;

/// Handy type aliases for 2x2 matrix.
pub type mat2x2b = mat<bool, 2, 2>;

/// Handy type aliases for 2x3 matrix.
pub type mat2x3b = mat<bool, 2, 3>;

/// Handy type aliases for 2x4 matrix.
pub type mat2x4b = mat<bool, 2, 4>;

/// Handy type aliases for 3x2 matrix.
pub type mat3x2b = mat<bool, 3, 2>;

/// Handy type aliases for 3x3 matrix.
pub type mat3x3b = mat<bool, 3, 3>;

/// Handy type aliases for 3x4 matrix.
pub type mat3x4b = mat<bool, 3, 4>;

/// Handy type aliases for 4x2 matrix.
pub type mat4x2b = mat<bool, 4, 2>;

/// Handy type aliases for 4x3 matrix.
pub type mat4x3b = mat<bool, 4, 3>;

/// Handy type aliases for 4x4 matrix.
pub type mat4x4b = mat<bool, 4, 4>;

/// Short-hand for `mat2x2`.
pub type mat2b = mat2x2b;

/// Short-hand for `mat3x3`.
pub type mat3b = mat3x3b;

/// Short-hand for `mat4x4`.
pub type mat4b = mat4x4b;

/// Handy type aliases for two-component vector.
pub type vec2h = vec<f16, 2>;

/// Handy type aliases for three-component vector.
pub type vec3h = vec<f16, 3>;

/// Handy type aliases for four-component vector.
pub type vec4h = vec<f16, 4>;

/// Handy type aliases for 2x2 matrix.
pub type mat2x2h = mat<f16, 2, 2>;

/// Handy type aliases for 2x3 matrix.
pub type mat2x3h = mat<f16, 2, 3>;

/// Handy type aliases for 2x4 matrix.
pub type mat2x4h = mat<f16, 2, 4>;

/// Handy type aliases for 3x2 matrix.
pub type mat3x2h = mat<f16, 3, 2>;

/// Handy type aliases for 3x3 matrix.
pub type mat3x3h = mat<f16, 3, 3>;

/// Handy type aliases for 3x4 matrix.
pub type mat3x4h = mat<f16, 3, 4>;

/// Handy type aliases for 4x2 matrix.
pub type mat4x2h = mat<f16, 4, 2>;

/// Handy type aliases for 4x3 matrix.
pub type mat4x3h = mat<f16, 4, 3>;

/// Handy type aliases for 4x4 matrix.
pub type mat4x4h = mat<f16, 4, 4>;

/// Short-hand for `mat2x2`.
pub type mat2h = mat2x2h;

/// Short-hand for `mat3x3`.
pub type mat3h = mat3x3h;

/// Short-hand for `mat4x4`.
pub type mat4h = mat4x4h;

/// Handy type aliases for two-component vector.
pub type vec2u = vec<u32, 2>;

/// Handy type aliases for three-component vector.
pub type vec3u = vec<u32, 3>;

/// Handy type aliases for four-component vector.
pub type vec4u = vec<u32, 4>;

/// Handy type aliases for 2x2 matrix.
pub type mat2x2u = mat<u32, 2, 2>;

/// Handy type aliases for 2x3 matrix.
pub type mat2x3u = mat<u32, 2, 3>;

/// Handy type aliases for 2x4 matrix.
pub type mat2x4u = mat<u32, 2, 4>;

/// Handy type aliases for 3x2 matrix.
pub type mat3x2u = mat<u32, 3, 2>;

/// Handy type aliases for 3x3 matrix.
pub type mat3x3u = mat<u32, 3, 3>;

/// Handy type aliases for 3x4 matrix.
pub type mat3x4u = mat<u32, 3, 4>;

/// Handy type aliases for 4x2 matrix.
pub type mat4x2u = mat<u32, 4, 2>;

/// Handy type aliases for 4x3 matrix.
pub type mat4x3u = mat<u32, 4, 3>;

/// Handy type aliases for 4x4 matrix.
pub type mat4x4u = mat<u32, 4, 4>;

/// Short-hand for `mat2x2`.
pub type mat2u = mat2x2u;

/// Short-hand for `mat3x3`.
pub type mat3u = mat3x3u;

/// Short-hand for `mat4x4`.
pub type mat4u = mat4x4u;

/// Handy type aliases for two-component vector.
pub type vec2i = vec<i32, 2>;

/// Handy type aliases for three-component vector.
pub type vec3i = vec<i32, 3>;

/// Handy type aliases for four-component vector.
pub type vec4i = vec<i32, 4>;

/// Handy type aliases for 2x2 matrix.
pub type mat2x2i = mat<i32, 2, 2>;

/// Handy type aliases for 2x3 matrix.
pub type mat2x3i = mat<i32, 2, 3>;

/// Handy type aliases for 2x4 matrix.
pub type mat2x4i = mat<i32, 2, 4>;

/// Handy type aliases for 3x2 matrix.
pub type mat3x2i = mat<i32, 3, 2>;

/// Handy type aliases for 3x3 matrix.
pub type mat3x3i = mat<i32, 3, 3>;

/// Handy type aliases for 3x4 matrix.
pub type mat3x4i = mat<i32, 3, 4>;

/// Handy type aliases for 4x2 matrix.
pub type mat4x2i = mat<i32, 4, 2>;

/// Handy type aliases for 4x3 matrix.
pub type mat4x3i = mat<i32, 4, 3>;

/// Handy type aliases for 4x4 matrix.
pub type mat4x4i = mat<i32, 4, 4>;

/// Short-hand for `mat2x2`.
pub type mat2i = mat2x2i;

/// Short-hand for `mat3x3`.
pub type mat3i = mat3x3i;

/// Short-hand for `mat4x4`.
pub type mat4i = mat4x4i;

/// Handy type aliases for two-component vector.
pub type vec2f = vec<f32, 2>;

/// Handy type aliases for three-component vector.
pub type vec3f = vec<f32, 3>;

/// Handy type aliases for four-component vector.
pub type vec4f = vec<f32, 4>;

/// Handy type aliases for 2x2 matrix.
pub type mat2x2f = mat<f32, 2, 2>;

/// Handy type aliases for 2x3 matrix.
pub type mat2x3f = mat<f32, 2, 3>;

/// Handy type aliases for 2x4 matrix.
pub type mat2x4f = mat<f32, 2, 4>;

/// Handy type aliases for 3x2 matrix.
pub type mat3x2f = mat<f32, 3, 2>;

/// Handy type aliases for 3x3 matrix.
pub type mat3x3f = mat<f32, 3, 3>;

/// Handy type aliases for 3x4 matrix.
pub type mat3x4f = mat<f32, 3, 4>;

/// Handy type aliases for 4x2 matrix.
pub type mat4x2f = mat<f32, 4, 2>;

/// Handy type aliases for 4x3 matrix.
pub type mat4x3f = mat<f32, 4, 3>;

/// Handy type aliases for 4x4 matrix.
pub type mat4x4f = mat<f32, 4, 4>;

/// Short-hand for `mat2x2`.
pub type mat2f = mat2x2f;

/// Short-hand for `mat3x3`.
pub type mat3f = mat3x3f;

/// Short-hand for `mat4x4`.
pub type mat4f = mat4x4f;

#[inline(always)]
pub fn vec2<T>(x: T, y: T) -> vec2<T> {
    vec([x, y])
}

#[inline(always)]
pub fn vec3<T>(x: T, y: T, z: T) -> vec3<T> {
    vec([x, y, z])
}

#[inline(always)]
pub fn vec4<T>(x: T, y: T, z: T, w: T) -> vec4<T> {
    vec([x, y, z, w])
}

#[inline(always)]
pub fn mat2x2<T>(m00: T, m01: T, m10: T, m11: T) -> mat2x2<T> {
    mat([[m00, m01], [m10, m11]])
}

#[inline(always)]
pub fn mat2x3<T>(m00: T, m01: T, m02: T, m10: T, m11: T, m12: T) -> mat2x3<T> {
    mat([[m00, m01, m02], [m10, m11, m12]])
}

#[inline(always)]
pub fn mat2x4<T>(m00: T, m01: T, m02: T, m03: T, m10: T, m11: T, m12: T, m13: T) -> mat2x4<T> {
    mat([[m00, m01, m02, m03], [m10, m11, m12, m13]])
}

#[inline(always)]
pub fn mat3x2<T>(m00: T, m01: T, m10: T, m11: T, m20: T, m21: T) -> mat3x2<T> {
    mat([[m00, m01], [m10, m11], [m20, m21]])
}

#[inline(always)]
pub fn mat3x3<T>(
    m00: T,
    m01: T,
    m02: T,
    m10: T,
    m11: T,
    m12: T,
    m20: T,
    m21: T,
    m22: T,
) -> mat3x3<T> {
    mat([[m00, m01, m02], [m10, m11, m12], [m20, m21, m22]])
}

#[inline(always)]
pub fn mat3x4<T>(
    m00: T,
    m01: T,
    m02: T,
    m03: T,
    m10: T,
    m11: T,
    m12: T,
    m13: T,
    m20: T,
    m21: T,
    m22: T,
    m23: T,
) -> mat3x4<T> {
    mat([
        [m00, m01, m02, m03],
        [m10, m11, m12, m13],
        [m20, m21, m22, m23],
    ])
}

#[inline(always)]
pub fn mat4x2<T>(m00: T, m01: T, m10: T, m11: T, m20: T, m21: T, m30: T, m31: T) -> mat4x2<T> {
    mat([[m00, m01], [m10, m11], [m20, m21], [m30, m31]])
}

#[inline(always)]
pub fn mat4x3<T>(
    m00: T,
    m01: T,
    m02: T,
    m10: T,
    m11: T,
    m12: T,
    m20: T,
    m21: T,
    m22: T,
    m30: T,
    m31: T,
    m32: T,
) -> mat4x3<T> {
    mat([
        [m00, m01, m02],
        [m10, m11, m12],
        [m20, m21, m22],
        [m30, m31, m32],
    ])
}

#[inline(always)]
pub fn mat4x4<T>(
    m00: T,
    m01: T,
    m02: T,
    m03: T,
    m10: T,
    m11: T,
    m12: T,
    m13: T,
    m20: T,
    m21: T,
    m22: T,
    m23: T,
    m30: T,
    m31: T,
    m32: T,
    m33: T,
) -> mat4x4<T> {
    mat([
        [m00, m01, m02, m03],
        [m10, m11, m12, m13],
        [m20, m21, m22, m23],
        [m30, m31, m32, m33],
    ])
}

#[inline(always)]
pub fn mat2<T>(m00: T, m01: T, m10: T, m11: T) -> mat2<T> {
    mat2x2(m00, m01, m10, m11)
}

#[inline(always)]
pub fn mat3<T>(m00: T, m01: T, m02: T, m10: T, m11: T, m12: T, m20: T, m21: T, m22: T) -> mat3<T> {
    mat3x3(m00, m01, m02, m10, m11, m12, m20, m21, m22)
}

#[inline(always)]
pub fn mat4<T>(
    m00: T,
    m01: T,
    m02: T,
    m03: T,
    m10: T,
    m11: T,
    m12: T,
    m13: T,
    m20: T,
    m21: T,
    m22: T,
    m23: T,
    m30: T,
    m31: T,
    m32: T,
    m33: T,
) -> mat4<T> {
    mat4x4(
        m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23, m30, m31, m32, m33,
    )
}
