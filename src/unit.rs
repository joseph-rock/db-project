#[derive(Debug)]
pub struct Unit {
    pub name: UnitName,
    pub amount: f64,
}

#[derive(Debug)]
pub enum UnitName {
    Gallon,
    Cup,
    Ounce,
}

impl std::string::ToString for UnitName {
    fn to_string(&self) -> String {
        match self {
            UnitName::Gallon => "Gallon".to_string(),
            UnitName::Cup => "Cup".to_string(),
            UnitName::Ounce => "Ounce".to_string(),
        }
    }
}

impl UnitName {
    pub fn from_string(unit: &str) -> Option<UnitName> {
        match unit {
            "Gallon" => Some(UnitName::Gallon),
            "Cup" => Some(UnitName::Cup),
            "Ounce" => Some(UnitName::Ounce),
            _  => None,
        }
    }
}

// pub fn convert_gallon_and_cup(unit: Unit) -> Unit {
//     match unit {
//         Unit::Gallon(amount) => Unit::Cup(amount * 16.0),
//         Unit::Cup(amount) => Unit::Gallon(amount / 16.0),
//         Unit::Ounce(amount) => todo!(),
//     }
// }
