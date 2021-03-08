use super::*;

macro_rules! impl_add_for {
  ($m:ident { $($f:ident),+ }) => {
    impl core::ops::Add for $m {
      type Output = Self;
      fn add(self, rhs: Self) -> Self {
        $m {
          $( $f: self.$f + rhs.$f ),+
        }
      }
    }
    impl core::ops::Add<&$m> for $m {
      type Output = Self;
      fn add(self, rhs: &Self) -> Self {
        self + *rhs
      }
    }
    //
    impl core::ops::AddAssign for $m {
      fn add_assign(&mut self, rhs: Self) {
        $( self.$f += rhs.$f; )+
      }
    }
    impl core::ops::AddAssign<&$m> for $m {
      fn add_assign(&mut self, rhs: &Self) {
        $( self.$f += rhs.$f; )+
      }
    }
  }
}

impl_add_for!(Mat2x2 { r0c0, r1c0, r0c1, r1c1 });
impl_add_for!(Mat2x3 { r0c0, r1c0, r0c1, r1c1, r0c2, r1c2 });
impl_add_for!(Mat2x4 { r0c0, r1c0, r0c1, r1c1, r0c2, r1c2, r0c3, r1c3 });
impl_add_for!(Mat3x2 { r0c0, r1c0, r2c0, r0c1, r1c1, r2c1 });
impl_add_for!(Mat3x3 { r0c0, r1c0, r2c0, r0c1, r1c1, r2c1, r0c2, r1c2, r2c2 });
impl_add_for!(Mat3x4 {
  r0c0,
  r1c0,
  r2c0,
  r0c1,
  r1c1,
  r2c1,
  r0c2,
  r1c2,
  r2c2,
  r0c3,
  r1c3,
  r2c3
});
impl_add_for!(Mat4x2 { r0c0, r1c0, r2c0, r3c0, r0c1, r1c1, r2c1, r3c1 });
impl_add_for!(Mat4x3 {
  r0c0,
  r1c0,
  r2c0,
  r3c0,
  r0c1,
  r1c1,
  r2c1,
  r3c1,
  r0c2,
  r1c2,
  r2c2,
  r3c2
});
impl_add_for!(Mat4x4 {
  r0c0,
  r1c0,
  r2c0,
  r3c0,
  r0c1,
  r1c1,
  r2c1,
  r3c1,
  r0c2,
  r1c2,
  r2c2,
  r3c2,
  r0c3,
  r1c3,
  r2c3,
  r3c3
});
impl_add_for!(Vec2 { x, y });
impl_add_for!(Vec3 { x, y, z });
impl_add_for!(Vec4 { x, y, z, w });
