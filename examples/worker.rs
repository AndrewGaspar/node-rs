extern crate node_rs;
extern crate stdweb;

use node_rs::cluster;

fn main() {
    stdweb::initialize();

    let worker = cluster::worker().expect("This module should only be instantiated as a worker");

    let p = worker.process();

    println!("I'm a worker! pid = {}", p.pid());

    node_rs::set_timeout(
        move || {
            worker.disconnect();
        },
        1000,
    );

    stdweb::event_loop();
}
