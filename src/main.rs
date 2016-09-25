extern crate expass;
extern crate hyper;

use hyper::server::{Server, Request, Response};
use hyper::uri::RequestUri;

static SERVER_BIND: &'static str = "0.0.0.0:8000";

fn main() {
    let database = expass::Database::new("data.csv");
    let shared_database = std::sync::Arc::new(database);

    println!("Start server on {}", SERVER_BIND);
    Server::http(SERVER_BIND).unwrap().handle(move |req: Request, res: Response| {
        println!("Start process requeset: {} {}", req.method, req.uri);
        if let RequestUri::AbsolutePath(path) = req.uri {
            let mut path_parts = path.splitn(3, '/').skip(1); // Skip empty part
            let series = path_parts.next()
                                   .expect("Series not found")
                                   .parse()
                                   .expect("Invalid series format");
            let number = path_parts.next()
                                   .expect("Number not found")
                                   .parse::<u32>()
                                   .expect("Invalid number format");
            let exists = shared_database.is_exist(series, number);
            println!("{:04} {:06} is {}", series, number, exists);
            res.send(format!("{}", exists).as_bytes()).unwrap();
        } else {
            println!("{} is not AbsolutePath", req.uri);
        }

    }).unwrap();
}
