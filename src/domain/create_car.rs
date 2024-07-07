use super::entities::{CarName, CarNumber, CarTypes};

struct Request {
    id: u16,
    name: String,
    products: Vec<String>,
}

enum Response {
    Ok(u16),
    BadRequest,
}

fn execute(req: Request) -> Response {
    match (
        CarNumber::try_from(req.id),
        CarName::try_from(req.name),
        CarTypes::try_from(req.products),
    ) {
        (Ok(number), Ok(_), Ok(_)) => Response::Ok(u16::from(number)),
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_the_car_number() {
        let number = 10;

        let req = Request {
            id: number,
            name: String::from("Tesla"),
            products: vec![String::from("Electric")],
        };

        let result = execute(req);

        match result {
            Response::Ok(result_number) => assert_eq!(result_number, number),
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let req = Request {
            id: 10,
            name: String::from(""),
            products: vec![String::from("Electrict")],
        };

        let result = execute(req);

        match result {
            Response::BadRequest => {}
            _ => unreachable!(),
        }
    }
}
