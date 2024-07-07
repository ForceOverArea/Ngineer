use std::f64::consts::PI;

/// A trait for objects that represent the cross section 
/// of a beam in a 2-dimensional structures problem.
pub trait Beam2DCrossSection 
{
    /// Returns the area moment of inertia w.r.t. the bending 
    /// axis in a 2-dimensional structures problem. 
    fn ix(&self) -> f64;

    /// Returns the cross-sectional area of the beam cross section.
    fn area(&self) -> f64;
}

pub struct Square
{
    pub width: f64,
    pub height: f64,
}
impl Beam2DCrossSection for Square
{
    fn ix(&self) -> f64 
    {
        self.width * self.height.powi(3) / 12.0
    }

    fn area(&self) -> f64 
    {
        self.width * self.height    
    }
}

pub struct SquareTube
{
    width: f64, 
    height: f64, 
    inner_width: f64, 
    inner_height: f64,
}
impl Beam2DCrossSection for SquareTube
{
    fn ix(&self) -> f64
    {
        let pos = Square 
        { 
            width: self.width, 
            height: self.height 
        }; 

        let neg = Square
        {
            width: self.inner_width,
            height: self.inner_height,
        };

        pos.ix() - neg.ix()
    }

    fn area(&self) -> f64 
    {
        let pos = Square 
        { 
            width: self.width, 
            height: self.height 
        }; 

        let neg = Square
        {
            width: self.inner_width,
            height: self.inner_height,
        };

        pos.area() - neg.area()   
    }
}

pub struct Round
{
    od: f64,
}
impl Beam2DCrossSection for Round
{
    fn ix(&self) -> f64 
    {
        (PI * self.od.powi(4)) / 64.0
    }

    fn area(&self) -> f64 
    {
        (self.od / 2.0).powi(4) * PI
    }
}

pub struct RoundTube
{
    od: f64,
    id: f64,
}
impl Beam2DCrossSection for RoundTube
{
    fn ix(&self) -> f64
    {
        let pos = Round { od: self.od };
        let neg = Round { od: self.id };

        pos.ix() - neg.ix()
    }

    fn area(&self) -> f64 
    {
        let pos = Round { od: self.od };
        let neg = Round { od: self.id };

        pos.area() - neg.area()   
    }
}

pub struct IBeam
{
    width: f64,
    height: f64,
    inner_height: f64,
    flange_length: f64,
}
impl Beam2DCrossSection for IBeam
{
    fn ix(&self) -> f64 
    {
        (1.0 / 12.0) * (self.width * self.height.powi(3) - (2.0 * self.flange_length * self.inner_height.powi(3)))
    }

    fn area(&self) -> f64 
    {
        self.width * self.height - (2.0 * self.flange_length * self.inner_height)
    }
}