fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,    
}

impl Server {
    fn new(addr: String) -> Self { // Associate function for create new instance of Server struct
        Self {
            addr
        }
    }

    fn run(self) { // Method of server instance
        println!("Listening on {}", self.addr)
    }
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

enum Method {
    GET,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
    DELETE
}