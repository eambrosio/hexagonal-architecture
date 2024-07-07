#[derive(PartialEq, Clone)]
pub struct CarNumber(u16);

impl TryFrom<u16> for CarNumber {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value > 0 && value < 1000 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

impl From<CarNumber> for u16 {
    fn from(value: CarNumber) -> Self {
        value.0
    }
}

pub struct CarName(String);

impl TryFrom<String> for CarName {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

pub struct CarTypes(Vec<CarType>);

impl TryFrom<Vec<String>> for CarTypes {
    type Error = ();

    fn try_from(values: Vec<String>) -> Result<Self, Self::Error> {
        if values.is_empty() {
            Err(())
        } else {
            let mut types = vec![];
            for v in values.iter() {
                match CarType::try_from(String::from(v)) {
                    Ok(car_type) => types.push(car_type),
                    _ => return Err(()),
                }
            }
            Ok(Self(types))
        }
    }
}

enum CarType {
    Electric,
    Hybrid,
}

impl TryFrom<String> for CarType {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Electric" => Ok(Self::Electric),
            "Hybrid" => Ok(Self::Hybrid),
            _ => Err(()),
        }
    }
}

pub struct Car {
    pub number: CarNumber,
    name: CarName,
    types: CarTypes,
}

impl Car {
    pub fn new(number: CarNumber, name: CarName, types: CarTypes) -> Self {
        Self {
            number,
            name,
            types,
        }
    }
}
