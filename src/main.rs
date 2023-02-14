fn main() {
    let server = server::Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {
    pub struct Server {
        addr: String,    
    }
    
    impl Server {
        pub fn new(addr: String) -> Self { // Associate function for create new instance of Server struct
            Self {
                addr
            }
        }
    
        pub fn run(self) { // Method of server instance
            println!("Listening on {}", self.addr)
        }
    }
}


mod http {
    mod request {
        struct Request {
            path: String,
            query_string: Option<String>,
            method: super::method::Method,
        }
    }

    mod method {
        pub enum Method {
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
    }

    

}

