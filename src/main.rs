extern crate expass;
extern crate hyper;
extern crate clap;
extern crate num_cpus;

use hyper::server::{
    Server,
    Request,
    Response,
    Handler,
};
use hyper::uri::RequestUri;


static DEFAULT_SERVER_BIND: &'static str = "0.0.0.0:8000";
static DEFAULT_DATABASE_PATH: &'static str = "list_of_expired_passports.csv";

struct Args {
    database_path: String,
    server_bind: String,
    num_threads: usize,
}

fn parse_args() -> Args {
    let default_num_threads: String = (num_cpus::get() * 4).to_string();
    let args_matches = clap::App::new("Expass")
                                 .about("Check expared passports")
                                 .arg(
                                     clap::Arg::with_name("database_path")
                                               .long("database")
                                               .short("db")
                                               .value_name("FILE_PATH")
                                               .help("Path to database")
                                               .takes_value(true)
                                               .default_value(DEFAULT_DATABASE_PATH)
                                 )
                                 .arg(
                                     clap::Arg::with_name("server_bind")
                                               .long("bind")
                                               .short("b")
                                               .value_name("IP_WITH_PORT")
                                               .help("Server bind to IP_WITH_PORT")
                                               .takes_value(true)
                                               .default_value(DEFAULT_SERVER_BIND)
                                 )
                                 .arg(
                                     clap::Arg::with_name("num_threads")
                                               .long("threads")
                                               .short("t")
                                               .value_name("NUM_THREADS")
                                               .help("Number of threads (default <num_cpu>*2")
                                               .takes_value(true)
                                               .default_value(&default_num_threads)
                                 )
                                 .get_matches();
    Args {
        database_path: args_matches.value_of("database_path")
                                   .unwrap()
                                   .to_string(),
        server_bind: args_matches.value_of("server_bind")
                                 .unwrap()
                                 .to_string(),
        num_threads: args_matches.value_of("num_threads")
                                 .unwrap()
                                 .to_string()
                                 .parse()
                                 .unwrap(),
    }
}

struct ExpassHandler {
    shared_database: std::sync::Arc<expass::Database>,
}

impl ExpassHandler {
    fn new(database: expass::Database) -> Self {
        ExpassHandler {
            shared_database: std::sync::Arc::new(database),
        }
    }
}

impl Handler for ExpassHandler {
    fn handle(&self, req: Request, res: Response) {
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
            let exists = self.shared_database.is_exist(series, number);
            println!("{:04} {:06} is {}", series, number, exists);
            res.send(format!("{}", exists).as_bytes()).unwrap();
        } else {
            println!("{} is not AbsolutePath", req.uri);
        }

    }
}

fn main() {
    let args = parse_args();

    let database = expass::Database::new(&args.database_path);

    println!(
        "Start server on {} with {} threads",
        args.server_bind,
        args.num_threads,
    );

    let expass_handler = ExpassHandler::new(database);


    Server::http(&*args.server_bind)
           .unwrap()
           .handle_threads(expass_handler, args.num_threads)
           .unwrap();
}
