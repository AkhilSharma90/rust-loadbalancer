use std::sync::{Arc, Mutex};
use std::thread;

struct LoadBalancer {
    servers: Vec<String>,
    current_index: Mutex<usize>,
}

impl LoadBalancer {
    fn new(servers: Vec<String>) -> Self {
        LoadBalancer {
            servers,
            current_index: Mutex::new(0),
        }
    }

    fn next_server(&self) -> String {
        let mut current_index = self.current_index.lock().unwrap();
        let server = self.servers[*current_index].clone();
        *current_index = (*current_index + 1) % self.servers.len();
        server
    }
}

fn main() {
    let servers = vec![
        "http://server1.com".to_string(),
        "http://server2.com".to_string(),
        "http://server3.com".to_string(),
    ];
    let load_balancer = Arc::new(LoadBalancer::new(servers));
    for _ in 0..10 {
        let load_balancer = load_balancer.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                let server = load_balancer.next_server();
                println!("Request sent to server: {}", server);
            }
        });
    }
    thread::sleep(std::time::Duration::from_secs(1));
}
