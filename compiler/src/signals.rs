use std::{
    cell::{Cell, RefCell},
    rc::{Rc, Weak},
};

// Thread-local storage to track the current effect
thread_local! {
    static CURRENT_EFFECT: RefCell<Option<Effect>> = RefCell::new(None);
}

// Effect is just a wrapper around a reactive function
#[derive(Clone)]
struct Effect(Rc<dyn Fn()>);

type Callback = Rc<dyn Fn()>;

/// A reactive signal holding a value and a list of subscribers
#[derive(Clone)]
pub struct Signal<T: Copy> {
    value: Rc<Cell<T>>,
    subscribers: Rc<RefCell<Vec<Weak<dyn Fn()>>>>,
}

impl<T: Copy> Signal<T> {
    /// Create a new signal with an initial value
    pub fn new(initial: T) -> Self {
        Self {
            value: Rc::new(Cell::new(initial)),
            subscribers: Rc::new(RefCell::new(Vec::with_capacity(4))),
        }
    }

    /// Get the signal's value and track the dependency if inside an effect
    pub fn get(&self) -> T {
        CURRENT_EFFECT.with(|ctx| {
            if let Some(effect) = ctx.borrow().as_ref() {
                // Subscribe the current effect (via Weak to avoid memory leaks)
                self.subscribers
                    .borrow_mut()
                    .push(Rc::downgrade(&effect.0));
            }
        });
        self.value.get()
    }

    /// Set the signal's value and notify all subscribers if the value changed
    pub fn set(&self, new_value: T) {
        if self.value.get() != new_value {
            self.value.set(new_value);
            self.notify_subscribers();
        }
    }

    /// Call all subscribed effects
    fn notify_subscribers(&self) {
        let mut subs = self.subscribers.borrow_mut();
        subs.retain(|weak| {
            if let Some(callback) = weak.upgrade() {
                callback();
                true
            } else {
                false
            }
        });
    }
}

/// Register a reactive effect: it auto-runs and re-runs on signal changes
pub fn effect<F>(f: F)
where
    F: Fn() + 'static,
{
    let callback: Callback = Rc::new({
        let mut f_opt = Some(f);
        move || {
            CURRENT_EFFECT.with(|ctx| {
                *ctx.borrow_mut() = Some(Effect(callback.clone()));
                if let Some(ref user_fn) = f_opt {
                    user_fn();
                }
                *ctx.borrow_mut() = None;
            });
        }
    });

    callback(); // run immediately once
}
