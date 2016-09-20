extern crate iron;
extern crate router;
#[macro_use]

extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let mut router = Router::new();

    router.get("/", hello_world, "index");
    router.get("/random", random_number, "random_number");

    Iron::new(router).http("localhost:3000").unwrap();
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}

fn random_number(_: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let my_number = calc(15);

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(format!("My number is: {\n}", my_number));

    Ok(response)
}

fn calc(x: i32) -> i32 {
    let y;
    match x {
        1...40 => y = 34,
        _ => y = 2,
    }
    y
}
