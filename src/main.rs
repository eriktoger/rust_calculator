mod calculate;
use calculate::calculate;
#[macro_use]
extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "http://localhost:4001",
        ));
    }
}

#[post("/", data = "<input>")]
fn get_answer(input: String) -> String {
    calculate(input)
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(CORS).mount("/", routes![get_answer])
}
