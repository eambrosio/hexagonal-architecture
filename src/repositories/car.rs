use core::num;

use crate::domain::entities::{Car, CarName, CarNumber, CarTypes};

pub enum Insert {
    Ok(CarNumber),
    Conflict,
    Error,
}

pub trait Repository {
    fn insert(&mut self, number: CarNumber, name: CarName, types: CarTypes) -> Insert;
}

pub struct InMemoryRepository {
    error: bool,
    cars: Vec<Car>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self {
            error: false,
            cars: vec![],
        }
    }

    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}
impl Repository for InMemoryRepository {
    fn insert(&mut self, number: CarNumber, name: CarName, types: CarTypes) -> Insert {
        if self.error {
            return Insert::Error;
        }

        if self.cars.iter().any(|car| car.number == number) {
            return Insert::Conflict;
        }

        self.cars.push(Car::new(number.clone(), name, types));
        Insert::Ok(number)
    }
}
