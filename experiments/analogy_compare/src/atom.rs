// IMPORTANT NOTE: In this experiment, we are using string in lieu of unique identifier.
// Different allegations which would normally both be associated to the same artifact "Cat" should be differentiated with a number
// like "Cat1" and "Cat2" to signify that they are different instances of "Cat"
#[derive(Debug, Clone)]
pub struct Particle(pub(crate) String);

#[derive(Debug, Clone)]
pub enum Spin {
    Up,
    Down,
}

#[derive(Debug, Clone)]
pub enum Charge {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Atom {
    id:     Particle,
    spin:   Spin,
    charge: Charge,
}
