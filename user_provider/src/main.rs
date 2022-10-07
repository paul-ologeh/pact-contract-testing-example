mod lib;

#[macro_use]
extern crate rouille;
extern crate serde;

use crate::lib::{UserIn, UserOut};
use chrono::Utc;
use rouille::{input::json_input, Request, Response};
use uuid::Uuid;

fn main() {
    println!("Now listening on localhost:8000");

    rouille::start_server("localhost:8000", handle_routes);
}

fn log_request_response(request: &Request, response: &Response) {
    println!("Received {:?}", request);
    println!("Sent {:?}", response);
}

// This function actually handles the request.
fn handle_routes(request: &Request) -> Response {
    router!(request,
        (GET) (/user/{id: Uuid}) => {
            let res = UserOut {
                id,
                username: "paulologeh".to_string(),
                name: "Paul".to_string(),
                surname: "Ologeh".to_string(),
                age: 25,
                timestamp: Utc::now().naive_utc(),
                job_title: Some("Software Engineer".to_string()),
                phone_number: Some("12345789".to_string())
            };

            let res = Response::json(&res);

            log_request_response(request, &res);

            res
        },

        (POST) (/user) => {
            let new_user: UserIn = try_or_400!(json_input(request));
            let res = UserOut {
                id: Uuid::new_v4(),
                username: new_user.username,
                name: new_user.name,
                surname: new_user.surname,
                age: new_user.age,
                timestamp: Utc::now().naive_utc(),
                job_title: new_user.job_title,
                phone_number: new_user.phone_number
            };

            let res = Response::json(&res).with_status_code(201);

            log_request_response(request, &res);

            res
        },

        _ => Response::empty_404()
    )
}
