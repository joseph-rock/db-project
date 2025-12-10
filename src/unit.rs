use unit_conversions::{ volume, mass };

#[derive(Debug)]
pub enum UnitName {
    Gallon,
    Cup,
    Ounce,
    Pound,
}

impl std::string::ToString for UnitName {
    fn to_string(&self) -> String {
        match self {
            UnitName::Gallon => "Gallon".to_string(),
            UnitName::Cup => "Cup".to_string(),
            UnitName::Ounce => "Ounce".to_string(),
            UnitName::Pound => "Pound".to_string(),
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
    pub fn convert(&self, to: &UnitName) -> Option<f64> {
        match self.name {
            UnitName::Gallon => from_gallon(self.amount, to),
            UnitName::Cup => from_cup(self.amount, to),
            UnitName::Ounce => from_ounce(self.amount, to),
            UnitName::Pound => from_pound(self.amount, to),
        }
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
        UnitName::Cup => Some(amount),
        UnitName::Gallon => Some(volume::u_s_cups::to_gallons(amount)),
        _ => None,
    }
}

fn from_ounce(amount: f64, to_unit: &UnitName) -> Option<f64> {
    match to_unit {
        UnitName::Ounce => Some(amount),
        UnitName::Pound => Some(mass::ounces::to_pounds(amount)),
        _ => None,
    }
}

fn from_pound(amount: f64, to_unit: &UnitName) -> Option<f64> {
    match to_unit {
        UnitName::Pound => Some(amount),
        UnitName::Gallon => Some(mass::pounds::to_ounces(amount)),
        _ => None,
    }
}
