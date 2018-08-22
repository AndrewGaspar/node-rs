extern crate node_rs;
extern crate stdweb;

use node_rs::{cluster, Promise};

fn main() {
    stdweb::initialize();

    assert!(
        !cluster::is_worker(),
        "This module should not be instantiated as a worker!"
    );

    cluster::setup_master(
        cluster::ClusterSettingsBuilder::new()
            .exec(node_rs::dirname().join("worker.js"))
            .build(),
    );

    println!("I'm the master!");

    let args: Vec<_> = std::env::args().collect();

    let num_procs = if args.len() >= 2 {
        args[1].parse().expect("First argument must be an integer.")
    } else {
        4
    };

    let workers: Vec<_> = (0..num_procs).map(|_| cluster::fork()).collect();

    let promises: Vec<_> = workers
        .iter()
        .cloned()
        .map(|worker| {
            Promise::new(move |resolve, _| {
                worker.on_exit(move |_, _| {
                    resolve.complete();
                });
            })
        })
        .collect();

    Promise::all(&promises).then(|_| {
        println!("Master exiting...");
        stdweb::Value::Undefined
    });

    stdweb::event_loop();
}
