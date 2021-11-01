mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/org.eclipse.tahu.protobuf.rs"));
}
mod config;
mod device;
mod mqtt;
mod node;
mod sparkplug;
mod util;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

fn main() {
    let config = config::load_config();

    let running = Arc::new(AtomicBool::new(true));

    let signal = running.clone();
    ctrlc::set_handler(move || {
        signal.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut tasks = Vec::new();
    for nodeconfig in config.nodes {
        let group = config.group.clone();
        let broker = config.broker.clone();
        let signal = running.clone();
        let handler = std::thread::spawn(move || {
            let mut node = crate::node::Node::new(&broker, &nodeconfig, group);
            node.run(signal);
        });
        tasks.push(handler);
    }

    for task in tasks {
        task.join().unwrap();
    }
}
