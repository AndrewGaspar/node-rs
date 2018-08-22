use std::{collections::HashMap, path::PathBuf};
use stdweb::{self, unstable::TryInto, Reference, Value};

use Process;

/// Explicitly loads the [Cluster](https://nodejs.org/dist/latest/docs/api/cluster.html) module,
/// returning `None` if 'cluster' could not be found.
///
/// This typically does not need to be called explicitly.
pub fn initialize() -> Option<&'static Reference> {
    static mut CLUSTER: Option<Reference> = None;

    if unsafe { CLUSTER.is_none() } {
        unsafe {
            CLUSTER = js! {
                return @{ super::js_private() }.cluster = require("cluster");
            }.into_reference()
        };
    }

    unsafe { CLUSTER.as_ref() }
}

/// Returns a reference to the [Cluster](https://nodejs.org/dist/latest/docs/api/cluster.html)
/// module, panicing if 'cluster' does not exist.
pub fn cluster() -> &'static Reference {
    initialize().expect("cluster module could not be loaded")
}

/// Forks Master to create a Worker.
///
/// node.js docs:
/// [cluster.fork()](https://nodejs.org/dist/latest/docs/api/cluster.html#cluster_cluster_fork_env)
pub fn fork() -> Worker {
    (js! {
        return @{cluster()}.fork();
    }).try_into()
        .expect("fork() did not return a Worker!")
}

/// Returns true if the current process is a Master.
///
/// node.js docs:
/// [cluster.isMaster](https://nodejs.org/dist/latest/docs/api/cluster.html#cluster_cluster_ismaster)
pub fn is_master() -> bool {
    (js! {
        return @{cluster()}.isMaster;
    }).try_into()
        .expect("isMaster was not bool!")
}

/// Returns true if the process is not the master.
///
/// node.js docs:
/// [cluster.isWorker](https://nodejs.org/dist/latest/docs/api/cluster.html#cluster_cluster_isworker)
pub fn is_worker() -> bool {
    (js! {
        return @{cluster()}.isWorker;
    }).try_into()
        .expect("isWorker was not bool!")
}

/// Returns the worker object if the current process is a worker.
/// Returns None if the current process is a master.
///
/// node.js docs:
/// [cluster.worker](https://nodejs.org/dist/latest/docs/api/cluster.html#cluster_cluster_worker)
pub fn worker() -> Option<Worker> {
    (js! {
        return @{cluster()}.worker;
    }).try_into()
        .ok()
}

/// A builder for a ClusterSettings object.
///
/// node.js docs:
/// [cluster.settings](https://nodejs.org/dist/latest/docs/api/cluster.html#cluster_cluster_settings)
#[derive(Default)]
pub struct ClusterSettingsBuilder {
    settings: HashMap<&'static str, Value>,
}

impl ClusterSettingsBuilder {
    /// Creates an empty ClusterSettingsBuilder
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the executable that will be run by workers.
    pub fn exec(mut self, exec: PathBuf) -> Self {
        self.settings.insert(
            "exec",
            js! { return @{exec.into_os_string().into_string().unwrap()}; },
        );
        self
    }

    /// Constructs the ClusterSettings object.
    pub fn build(&self) -> ClusterSettings {
        (js! {
            return @{&self.settings};
        }).try_into()
            .unwrap()
    }
}

/// A collection of ClusterSettings.
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Object")]
pub struct ClusterSettings(Reference);

/// Changes the settings that will be used to fork workers.
///
/// node.js docs:
/// [cluster.setupMaster](https://nodejs.org/dist/latest/docs/api/cluster.html#cluster_cluster_setupmaster_settings)
pub fn setup_master(settings: ClusterSettings) {
    js! {
        @(no_return)
        @{cluster()}.setupMaster(@{settings});
    }
}

/// A Worker is an object that represents either a forked process from the master or, for a Worker,
/// its own state.
///
/// node.js docs: [Worker](https://nodejs.org/dist/latest/docs/api/cluster.html#cluster_class_worker)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "RUST_NODEJS_PRIVATE.cluster.Worker")]
pub struct Worker(Reference);

impl Worker {
    /// Returns an object without information on the process for the Worker.
    ///
    /// ## Note
    /// node.js API docs document this as being a ChildProcess object, but this is only the case
    /// from the Master. From the workers, this object is equivalent to global.process, which is not
    /// a ChildProcess. Hence, we just return Process here, which is a type that just represents an
    /// object that "looks like" a Process.
    pub fn process(&self) -> Process {
        (js! {
            return @{&self}.process;
        }).try_into()
            .expect("Worker did not have a process member!")
    }

    /// When called from the Worker, closes all servers, waits for the 'close' event and disconnects
    /// the IPC channel to the master.stdweb
    ///
    /// When called from the master, a message is sent to the Worker to tell it to disconnect.
    ///
    /// node.js docs:
    /// [worker.disconnect()](https://nodejs.org/dist/latest/docs/api/cluster.html#cluster_worker_disconnect)
    pub fn disconnect(&self) {
        js! {
            @{&self}.disconnect();
        };
    }

    /// Registers a callback for the 'exit' event. The callback will be called when this worker
    /// exits.
    ///
    /// node.js docs:
    /// ['exit'](https://nodejs.org/dist/latest/docs/api/cluster.html#cluster_event_exit)
    pub fn on_exit<F: 'static + FnOnce(i32, Option<&str>) -> ()>(&self, callback: F) {
        let on_exit_callback = move |code: stdweb::Value, signal: stdweb::Value| {
            let code = code.try_into().unwrap();
            let signal = (&signal).try_into().ok();
            callback(code, signal);
        };

        js! {
            @{&self.0}.on("exit", (code, signal) => {
                let on_exit_callback = @{stdweb::Once(on_exit_callback)};
                on_exit_callback(code, signal);
            });
        }
    }
}
