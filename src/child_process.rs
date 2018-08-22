use stdweb::{unstable::TryInto, Reference};

/// Explicitly loads the [Child Processes](https://nodejs.org/api/child_process.html) module,
/// returning `None` if 'child_process' could not be found.
///
/// This typically does not need to be called explicitly.
pub fn initialize() -> Option<&'static Reference> {
    static mut CHILD_PROCESS: Option<Reference> = None;

    if unsafe { CHILD_PROCESS == None } {
        unsafe {
            CHILD_PROCESS = js! {
                return @{ super::js_private() }.child_process = require("child_process");
            }.into_reference()
        };
    }

    unsafe { CHILD_PROCESS.as_ref() }
}

/// A type representing the ChildProcess prototype.
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "RUST_NODEJS_PRIVATE.child_process.ChildProcess")]
pub struct ChildProcess(Reference);

impl ChildProcess {
    /// Returns the Process ID of the child process.
    pub fn pid(&self) -> i32 {
        (js! {
            return @{&self}.pid;
        }).try_into()
            .expect("PID not number!")
    }
}
