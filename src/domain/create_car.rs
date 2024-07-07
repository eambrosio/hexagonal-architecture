use crate::repositories::car::{Insert, Repository};

use super::entities::{CarName, CarNumber, CarTypes};

struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

enum Response {
    Ok(u16),
    BadRequest,
    Conflict,
    Error,
}

fn execute(repo: &mut dyn Repository, req: Request) -> Response {
    match (
        CarNumber::try_from(req.number),
        CarName::try_from(req.name),
        CarTypes::try_from(req.types),
    ) {
        (Ok(number), Ok(name), Ok(types)) => match repo.insert(number.clone(), name, types) {
            Insert::Ok(_) => Response::Ok(u16::from(number)),
            Insert::Conflict => Response::Conflict,
            Insert::Error => Response::Error,
        },
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod tests {
    use crate::repositories::car::InMemoryRepository;

    use super::*;

    #[test]
    fn it_should_return_the_car_number() {
        let number = 10;
        let mut repo = InMemoryRepository::new();

        let req = Request {
            number,
            name: String::from("Tesla"),
            types: vec![String::from("Electric")],
        };

        let result = execute(&mut repo, req);

        match result {
            Response::Ok(result_number) => assert_eq!(result_number, number),
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let req = Request {
            number: 10,
            name: String::from(""),
            types: vec![String::from("Electrict")],
        };
        let mut repo = InMemoryRepository::new();

        let result = execute(&mut repo, req);

        match result {
            Response::BadRequest => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_conflict_when_the_car_number_already_exists() {
        let number = CarNumber::try_from(10).unwrap();
        let name = CarName::try_from(String::from("Tesla")).unwrap();
        let types = CarTypes::try_from(vec![String::from("Electric")]).unwrap();

        let mut repo = InMemoryRepository::new();
        repo.insert(number, name, types);

        let req = Request {
            number: 10,
            name: String::from("Audi"),
            types: vec![String::from("Hybrid")],
        };

        let result = execute(&mut repo, req);

        match result {
            Response::Conflict => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_an_error_when_unexpected_error_happens() {
        let mut repo = InMemoryRepository::new().with_error();

        let req = Request {
            number: 10,
            name: String::from("my_car"),
            types: vec![String::from("Electric")],
        };

        let result = execute(&mut repo, req);

        match result {
            Response::Error => {}
            _ => unreachable!(),
        }
    }
}
