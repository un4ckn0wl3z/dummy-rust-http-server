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