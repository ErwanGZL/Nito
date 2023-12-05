use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    Air,
    Water,
    Sand,
    Wood,
    Fire,
    Smoke,
    Acid,
    Lava,
    Gas,
    Stone,
}

pub trait Physics {
    fn density(&self) -> f64;
    fn flammability(&self) -> f64;
}

impl Physics for Element {
    fn density(&self) -> f64 {
        match self {
            Self::Air => 1.0,
            Self::Water => 2.0,
            Self::Sand => 3.0,
            Self::Wood => 10.0,
            Self::Fire => 0.0,
            Self::Smoke => 0.5,
            Self::Acid => 1.5,
            Self::Lava => 4.0,
            Self::Gas => 0.5,
            Self::Stone => 20.0,
        }
    }
    fn flammability(&self) -> f64 {
        match self {
            Self::Air => 0.0,
            Self::Water => 0.0,
            Self::Sand => 0.0,
            Self::Wood => 1.0,
            Self::Fire => 0.0,
            Self::Smoke => 0.0,
            Self::Acid => 0.0,
            Self::Lava => 0.0,
            Self::Gas => 2.0,
            Self::Stone => 0.0,
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Air => {
                write!(f, "·").unwrap();
            }
            Self::Water => {
                write!(f, "~").unwrap();
            }
            Self::Sand => {
                write!(f, "¤").unwrap();
            }
            Self::Wood => {
                write!(f, "▓").unwrap();
            }
            _ => {}
        }
        Ok(())
    }
}

impl Element {
    pub fn from_byte(byte: u8) -> Result<Self, ()> {
        match byte {
            0 => Ok(Self::Air),
            1 => Ok(Self::Water),
            2 => Ok(Self::Sand),
            3 => Ok(Self::Wood),
            4 => Ok(Self::Fire),
            5 => Ok(Self::Smoke),
            6 => Ok(Self::Acid),
            7 => Ok(Self::Lava),
            8 => Ok(Self::Gas),
            9 => Ok(Self::Stone),
            _ => Err(()),
        }
    }
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::Air => 0,
            Self::Water => 1,
            Self::Sand => 2,
            Self::Wood => 3,
            Self::Fire => 4,
            Self::Smoke => 5,
            Self::Acid => 6,
            Self::Lava => 7,
            Self::Gas => 8,
            Self::Stone => 9,
        }
    }
}
