use std::fmt;
use unit_conversions::{ volume, mass };

#[derive(Debug)]
pub enum UnitName {
    Gallon,
    Cup,
    Ounce,
    Pound,
    Gram,
}

impl fmt::Display for UnitName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitName::Gallon => write!(f, "{}", "Gallon".to_string()),
            UnitName::Cup => write!(f, "{}", "Cup".to_string()),
            UnitName::Ounce => write!(f, "{}", "Ounce".to_string()),
            UnitName::Pound => write!(f, "{}", "Pound".to_string()),
            UnitName::Gram => write!(f, "{}", "Gram".to_string()),
        }
    }
}

impl UnitName {
    // TODO: consider cleaning input (make lowercase, remove plural 's', etc.)
    pub fn from_string(unit: &str) -> Option<UnitName> {
        match unit {
            "Gallon" => Some(UnitName::Gallon),
            "Cup" => Some(UnitName::Cup),
            "Ounce" => Some(UnitName::Ounce),
            "Pound" => Some(UnitName::Pound),
            "Gram" => Some(UnitName::Gram),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Unit {
    pub name: UnitName,
    pub amount: f64,
}

impl Unit {
    // TODO: consider using result to return Err message (cannot convert from x to y)
    pub fn convert(&self, to: &UnitName) -> Option<f64> {
        match self.name {
            UnitName::Gallon => from_gallon(self.amount, to),
            UnitName::Cup => from_cup(self.amount, to),
            UnitName::Ounce => from_ounce(self.amount, to),
            UnitName::Pound => from_pound(self.amount, to),
            UnitName::Gram => from_gram(self.amount, to),
        }
    }

    // TODO: error handling, maybe return why this failed instead of non-specific None
    pub fn subtract(&self, unit: &Unit) -> Option<f64> {
        if let Some(converted) = unit.convert(&self.name) {
            return Some(&self.amount - converted);
        }
        None
    }
}

fn from_gallon(amount: f64, to_unit: &UnitName) -> Option<f64> {
    match to_unit {
        UnitName::Gallon => Some(amount),
        UnitName::Cup => Some(volume::gallons::to_u_s_cups(amount)),
        _ => None,
    }
}

fn from_cup(amount: f64, to_unit: &UnitName) -> Option<f64> {
    match to_unit {
        UnitName::Gallon => Some(volume::u_s_cups::to_gallons(amount)),
        UnitName::Cup => Some(amount),
        _ => None,
    }
}

fn from_ounce(amount: f64, to_unit: &UnitName) -> Option<f64> {
    match to_unit {
        UnitName::Ounce => Some(amount),
        UnitName::Pound => Some(mass::ounces::to_pounds(amount)),
        UnitName::Gram => Some(mass::ounces::to_grams(amount)),
        _ => None,
    }
}

fn from_pound(amount: f64, to_unit: &UnitName) -> Option<f64> {
    match to_unit {
        UnitName::Ounce => Some(mass::pounds::to_ounces(amount)),
        UnitName::Pound => Some(amount),
        UnitName::Gram => Some(mass::pounds::to_grams(amount)),
        _ => None,
    }
}

fn from_gram(amount: f64, to_unit: &UnitName) -> Option<f64> {
    match to_unit {
        UnitName::Ounce => Some(mass::grams::to_ounces(amount)),
        UnitName::Pound => Some(mass::grams::to_pounds(amount)),
        UnitName::Gram => Some(amount),
        _ => None,
    }
}
