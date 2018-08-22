use stdweb::{self, unstable::TryInto, Reference};

/// node.js Promise implementation
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Promise")]
pub struct Promise(Reference);

impl Promise {
    /// Creates a new Promise. Caller provided callback determines success or failure using
    /// `resolve` and `reject` callbacks.
    pub fn new<F>(user_callback: F) -> Self
    where
        F: 'static + FnOnce(PromiseCallback, PromiseCallback),
    {
        let callback = move |resolve: stdweb::Value, reject: stdweb::Value| {
            user_callback(
                PromiseCallback(resolve.into_reference().unwrap()),
                PromiseCallback(reject.into_reference().unwrap()),
            );
        };

        (js! {
            return new Promise(function(resolve, reject) {
                let callback = @{stdweb::Once(callback)};
                callback(resolve, reject);
            });
        }).try_into()
            .expect("new Promise did not return a Promise!")
    }

    /// Creates a new Promise that completes when all child promises complete
    pub fn all(promises: &[Promise]) -> Promise {
        (js! {
            return Promise.all(@{promises});
        }).try_into()
            .expect("Promise.all did not return a Promise!")
    }

    /// Creates a new Promise that completes when both this promise and the Promise returned by
    /// `callback` complete. `callback` is not called until this promise completes.
    pub fn then<F: 'static + FnOnce(stdweb::Value) -> stdweb::Value>(
        &self,
        callback: F,
    ) -> Promise {
        (js! {
            let callback = @{stdweb::Once(callback)};
            return @{self}.then(callback);
        }).try_into()
            .expect("Promise.then did not return a Promise!")
    }
}

/// A callback object that is passed to the `user_callback` in `Promise::new`. This is not a `Fn()`
/// due to language limitations.
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Function")]
pub struct PromiseCallback(Reference);

impl PromiseCallback {
    /// Resolves or rejects the Promise with no value.
    pub fn complete(self) {
        js! {
            @(no_return)
            let completion = @{self};
            completion();
        }
    }

    /// Resovles or rejects the Promise with a value.
    pub fn with<JS: stdweb::JsSerialize>(self, value: JS) {
        js! {
            @(no_return)
            let completion = @{self};
            completion(@{value});
        }
    }
}
