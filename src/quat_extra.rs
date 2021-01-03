trait HasIdentity {
    fn identity() -> Self;
}

/*trait UnitNormSquared {
    fn norm_squared(&self) -> N::SimdRealField;
}

trait RotationBetween: HasIdentity {
    fn rotation_between(a: &Vec3, b: &Vec3) -> Option<Self> {
        Self::scaled_rotation_between(a, b, 1.0)
    }
    fn scaled_rotation_between(a: &Vec3, b: &Vec3, s: f32) -> Option<Self> {
        if let (Some(na), Some(nb)) = () {
            Self::scaled_rotation_between_axis(&na, &nb, s)
        } else {
            Some(Self::identity())
        }
    }
    fn scaled_rotation_between_axis(a: &Vec3, b: &Vec3, s: f32) -> Option<Self> {

    }
}*/

trait ExtraDot {
    fn dotc(&self, rhs: &Self) -> f32;
    fn dotx(&self, rhs: &Self, conjugate: impl Fn(f32) -> f32);
}

/*impl ExtraDot for Vec4 {
    fn dotc(&self, rhs: &Vec4) -> f32 {
        self.dotx(rhs, )
    }
}*/

impl HasIdentity for Quat {
    fn identity() -> Self {
        Quat::identity()
    }
}

/*impl UnitNormSquared for Quat {
    fn norm_squared(&self) -> f32 {
        let mut res = 0.0;

        res += self.0.dotc(&self).simd_real();

        res
    }
}*/