use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShipType {
    Mercury,
    Mars,
    Venus,
    Earth,
}

impl ShipType {
    
}