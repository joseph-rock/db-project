#[derive(Debug)]
pub struct Measurement {
    pub name: MeasurementName,
    pub amount: f32,
}

#[derive(Debug)]
pub enum MeasurementName {
    Gallon,
    Cup,
    Ounce,
}

impl std::string::ToString for MeasurementName {
    fn to_string(&self) -> String {
        match self {
            MeasurementName::Gallon => "Gallon".to_string(),
            MeasurementName::Cup => "Cup".to_string(),
            MeasurementName::Ounce => "Ounce".to_string(),
        }
    }
}

impl MeasurementName {
    pub fn from_string(measurement: &str) -> Option<MeasurementName> {
        match measurement {
            "Gallon" => Some(MeasurementName::Gallon),
            "Cup" => Some(MeasurementName::Cup),
            "Ounce" => Some(MeasurementName::Ounce),
            _  => None,
        }
    }
}

// pub fn convert_gallon_and_cup(measurement: Measurement) -> Measurement {
//     match measurement {
//         Measurement::Gallon(amount) => Measurement::Cup(amount * 16.0),
//         Measurement::Cup(amount) => Measurement::Gallon(amount / 16.0),
//         Measurement::Ounce(amount) => todo!(),
//     }
// }
