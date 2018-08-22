extern crate node_rs;
extern crate stdweb;

use node_rs::{cluster, Promise};

fn main() {
    stdweb::initialize();

    if let Some(worker) = cluster::worker() {
        assert!(cluster::is_worker());

        let p = worker.process();

        println!("I'm a worker! pid = {}", p.pid());

        node_rs::set_timeout(
            move || {
                worker.disconnect();
            },
            1000,
        );
    } else {
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
            println!("Master exiting after a second...");
            stdweb::Value::Reference(
                Promise::new(|resolve, _| {
                    node_rs::set_timeout(
                        move || {
                            resolve.complete();
                        },
                        1000,
                    );
                }).into(),
            )
        });
    }

    stdweb::event_loop();
}
