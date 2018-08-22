use std::path::PathBuf;

use stdweb::unstable::TryInto;
use stdweb::Reference;

/// `Process` is an interface to an object that looks like `global.process`.
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Object")]
pub struct Process(Reference);

impl Process {
    /// Process ID of Process
    pub fn pid(&self) -> i32 {
        (js! {
            return @{&self}.pid;
        }).try_into()
            .expect("PID not number!")
    }

    /// Current working directory
    pub fn cwd(&self) -> PathBuf {
        PathBuf::from(js! { return @{&self}.cwd(); }.into_string().unwrap())
    }
}
