use stdweb::{unstable::TryInto, Once, Reference};

/// `OneShotTimeout` object is a timeout that self-disposes when the timeout is called. It cannot be
/// requeued.
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Object")]
pub struct OneShotTimeout(Reference);

impl OneShotTimeout {
    /// Creates a `OneShotTimeout` object. `callback` will be called exactly once after
    /// approximately `delay` milliseconds.
    pub fn new<F: 'static + FnOnce()>(callback: F, delay: i32) -> Self {
        (js! {
            let callback = @{Once(callback)};
            let timeout = setTimeout(function() {
                callback();
                this.RUST_NODEJS_PRIVATE.callback.drop();
                this.RUST_NODEJS_PRIVATE.callback = null;
            }, @{delay});
            timeout.RUST_NODEJS_PRIVATE = {
                callback: callback
            };
            return timeout;
        }).try_into()
            .expect("setTimeout must return a Timeout object!")
    }

    /// Keeps node.js event loop alive. Timeouts are "ref'd" by default.
    pub fn reference(&self) -> &Self {
        js! {
            @(no_return)
            @{self}.ref();
        };
        self
    }

    /// Allows node.js event loop to exit even if this Timeout is pending.
    pub fn unreference(&self) -> &Self {
        js! {
            @(no_return)
            @{self}.unref();
        };
        self
    }

    /// Cancels the timer.
    pub fn clear(&self) -> &Self {
        js! {
            @(no_return)
            clearTimeout(@{self});
            if (this.RUST_NODEJS_PRIVATE.callback !== null) {
                this.RUST_NODEJS_PRIVATE.callback.drop();
                this.RUST_NODEJS_PRIVATE.callback = null;
            }
        };
        self
    }
}

/// `RepeatableTimeout` is a timeout object that can be refreshed, but must be manually disposed of
/// when no longer needed. Prefer `OneShotTimeout` if you don't need a refreshable timer.
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Object")]
pub struct RepeatableTimeout(Reference);

impl RepeatableTimeout {
    /// Creates a `RepeatableTimeout` object. `callback` will be called after `delay` ms and may be
    /// called multiple times if `RepeatableTimeout::refresh` is called.
    pub fn new<F: 'static + Fn()>(callback: F, delay: i32) -> Self {
        (js! {
            let callback = @{callback};
            let timeout = setTimeout(callback, @{delay});
            timeout.RUST_NODEJS_PRIVATE = {
                callback: callback
            };
            return timeout;
        }).try_into()
            .expect("setTimeout must return a Timeout object!")
    }

    /// Disposes of the underlying timeout callback. This should only be called when the Timeout is
    /// not scheduled.
    pub fn dispose(&self) {
        js! {
            @(no_return)
            @{self}.RUST_NODEJS_PRIVATE.callback.drop();
        };
    }

    /// Keeps node.js event loop alive. Timeouts are "ref'd" by default.
    pub fn reference(&self) -> &Self {
        js! {
            @(no_return)
            @{self}.ref();
        };
        self
    }

    /// Allows node.js event loop to exit even if this Timeout is pending.
    pub fn unreference(&self) -> &Self {
        js! {
            @(no_return)
            @{self}.unref();
        };
        self
    }

    /// Restarts the Timeout to be called again in `delay` ms.
    pub fn refresh(&self) -> &Self {
        js! {
            @(no_return)
            @{self}.refresh();
        };
        self
    }

    /// Cancels the current timeout. Does not dispose of the `RepeatableTimeout`. `self.dispose()`
    /// must be called since the `RepeatableTimeout` could still be refreshed.
    pub fn clear(&self) -> &Self {
        js! {
            @(no_return)
            clearTimeout(@{self});
        };
        self
    }
}

/// Creates a `OneShotTimeout` object. `callback` will be called exactly once in `delay` ms.
pub fn set_timeout<F: 'static + FnOnce()>(callback: F, delay: i32) -> OneShotTimeout {
    OneShotTimeout::new(callback, delay)
}

/// Clears the `timeout`.
pub fn clear_timeout<F: 'static + FnOnce()>(timeout: OneShotTimeout) {
    timeout.clear();
}
