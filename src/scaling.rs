use super::*;

macro_rules! impl_scale_for {
  ($m:ident<$elem_t:ty> { $($field:ident),+ }) => {
    // MatMxN * Element
    impl core::ops::Mul<$elem_t> for $m {
      type Output = Self;
      fn mul(self, rhs: $elem_t) -> Self {
        $m {
          $( $field: self.$field * rhs ),+
        }
      }
    }
    // Element * MatMxN
    impl core::ops::Mul<$m> for $elem_t {
      type Output = $m;
      fn mul(self, rhs: $m) -> $m {
        $m {
          $( $field: self * rhs.$field ),+
        }
      }
    }
  }
}

impl_scale_for!(Mat2x2<f32> { r0c0, r1c0, r0c1, r1c1 });
impl_scale_for!(Mat2x3<f32> { r0c0, r1c0, r0c1, r1c1, r0c2, r1c2 });
impl_scale_for!(Mat2x4<f32> { r0c0, r1c0, r0c1, r1c1, r0c2, r1c2, r0c3, r1c3 });
impl_scale_for!(Mat3x2<f32> { r0c0, r1c0, r2c0, r0c1, r1c1, r2c1 });
impl_scale_for!(Mat3x3<f32> { r0c0, r1c0, r2c0, r0c1, r1c1, r2c1, r0c2, r1c2, r2c2 });
impl_scale_for!(Mat3x4<f32> { r0c0, r1c0, r2c0, r0c1, r1c1, r2c1, r0c2, r1c2, r2c2, r0c3, r1c3, r2c3 });
impl_scale_for!(Mat4x2<f32> { r0c0, r1c0, r2c0, r3c0, r0c1, r1c1, r2c1, r3c1 });
impl_scale_for!(Mat4x3<f32> { r0c0, r1c0, r2c0, r3c0, r0c1, r1c1, r2c1, r3c1, r0c2, r1c2, r2c2, r3c2 });
impl_scale_for!(Mat4x4<f32> { r0c0, r1c0, r2c0, r3c0, r0c1, r1c1, r2c1, r3c1, r0c2, r1c2, r2c2, r3c2, r0c3, r1c3, r2c3, r3c3 });
impl_scale_for!(Vec2<f32> { x, y });
impl_scale_for!(Vec3<f32> { x, y, z });
impl_scale_for!(Vec4<f32> { x, y, z, w });
