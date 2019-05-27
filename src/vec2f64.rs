use std;
use axgeom;
///A 2d point made up of f64's with a way to get the value on a particular axis easily.
#[repr(transparent)]
#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct Vec2(pub [f64;2]);

impl Vec2 {
    
    pub fn new(a:f64,b:f64)->Vec2{
        Vec2([a,b])
    }

    pub fn get_axis(&self,axis:impl axgeom::AxisTrait)->&f64{
        if axis.is_xaxis(){
            &self.0[0]
        }else{
            &self.0[1]
        }
    }

    pub fn get_axis_mut(&mut self,axis:impl axgeom::AxisTrait)->&mut f64{
        if axis.is_xaxis(){
            &mut self.0[0]
        }else{
            &mut self.0[1]
        }
    }

    pub fn set_zero(&mut self){
        self.0[0]=0.0;
        self.0[1]=0.0;
    }
    ///Calculates the dot product.
    #[inline(always)]
    pub fn inner_product(self, b: Vec2) -> f64 {
        let a=&self.0;
        let b=&b.0;
        a[0] * b[0] + a[1] * b[1]
    }

    ///Force the length of the vec to of max length nlen.
    ///If the length of the vec is zero, this will panic.
    #[inline(always)]
    pub fn truncate(&mut self, nlen: f64) {
        if self.dis_sqr()<nlen.powi(2){
            *self /= self.dis();
            *self *= nlen;
        }
    }

    #[inline(always)]
    pub fn rotate90(self) -> Vec2 {
        self.rotate_by(Vec2([0.0, 1.0]))
    }

    #[inline(always)]
    pub fn rotate_by(self, b: Vec2) -> Vec2 {
        let a=&self.0;
        let b=&b.0;
        Vec2([a[0] * b[0] - a[1] * b[1],
                  a[0] * b[1] + a[1] * b[0]])

    }

    ///Calculates len using sqrt().
    #[inline(always)]
    pub fn dis(self) -> f64 {
        self.dis_sqr().sqrt()
    }


    #[inline(always)]
    pub fn dis_sqr(self) -> f64 {
        let a=&self.0;
        a[0]*a[0]+a[1]*a[1]
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn add(self, other: Vec2) -> Vec2 {
        let a=&self.0;
        let b=&other.0;
        Vec2([a[0]+b[0],a[1]+b[1]])
    }
}

impl std::ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn mul(self, other: f64) -> Vec2 {
        let a=&self.0;
        Vec2([a[0]*other,a[1]*other])
    }
}

impl std::ops::Div<f64> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn div(self, other: f64) -> Vec2 {
        let a=&self.0;
        Vec2([a[0] / other, a[1] / other])
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn neg(self) -> Vec2 {
        let a=&self.0;
        Vec2([-a[0], -a[1]])
    }
}

impl std::ops::MulAssign<f64> for Vec2 {

    #[inline(always)]
    fn mul_assign(&mut self, rhs: f64) {
        let a=&mut self.0;
        a[0]*=rhs;
        a[1]*=rhs;
    }
}

impl std::ops::DivAssign<f64> for Vec2 {

    #[inline(always)]
    fn div_assign(&mut self, rhs: f64) {
        let a=&mut self.0;
        a[0]/=rhs;
        a[1]/=rhs;
    }
}

impl std::ops::AddAssign for Vec2 {

    #[inline(always)]
    fn add_assign(&mut self, other: Vec2) {
        let a=&mut self.0;
        let b=&other.0;
        a[0]+=b[0];
        a[1]+=b[1];
    }
}

impl std::ops::SubAssign for Vec2 {

    #[inline(always)]
    fn sub_assign(&mut self, other: Vec2) {
        let a=&mut self.0;
        let b=&other.0;
        a[0]-=b[0];
        a[1]-=b[1];
    }
}
impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn sub(self, other: Vec2) -> Vec2 {
        let a=&self.0;
        let b=&other.0;
        Vec2([a[0]-b[0],a[1]-b[1]])
    }
}