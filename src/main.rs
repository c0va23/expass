extern crate expass;
extern crate hyper;

use hyper::server::{Server, Request, Response};
use hyper::uri::RequestUri;

fn main() {
    let database = expass::Database::new("expass.csv");
    let series = "0500";
    let number = 128883_u32;
    println!(
        "Exists {} {}: {}",
        series.to_string(),
        number,
        database.is_exist(
            series.to_string(),
            number,
        ),
    );

    Server::http("0.0.0.0:8000").unwrap().handle(move |req: Request, res: Response| {
        if let RequestUri::AbsolutePath(path) = req.uri {
            let mut path_parts = path.splitn(3, '/').skip(1); // Skip empty part
            let series = path_parts.next().expect("Series not found");
            let number = path_parts.next().expect("Number not found")
                                   .parse::<u32>().expect("Invalid number");
            let exists = database.is_exist(series.to_string(), number);
            println!("{} {} is {}", series, number, exists);
            res.send(format!("{}", exists).as_bytes()).unwrap();
        } else {
            println!("{} is not AbsolutePath", req.uri);
        }

    }).unwrap();
}
