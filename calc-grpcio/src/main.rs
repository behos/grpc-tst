use std::{sync::Arc, thread, io::{self, Read}};

use futures::{channel::oneshot, executor::block_on};
use grpcio::{Environment, ServerBuilder, ServerCredentials};

use crate::api::{create_calculator, CalculatorService};

mod api;

fn main() {
    let addr = "[::1]:10000";
    let env = Arc::new(Environment::new(1));
    let instance = CalculatorService {};
    let service = create_calculator(instance);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .build()
        .unwrap();
    server
        .add_listening_port(addr, ServerCredentials::insecure())
        .unwrap();
    server.start();
    println!("Calculator listening on {}", addr);
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    block_on(rx).unwrap();
    block_on(server.shutdown()).unwrap();
}
