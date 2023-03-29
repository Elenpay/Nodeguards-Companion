use std::any::Any;
use std::cell::RefCell;
use std::thread_local;

#[derive(Debug)]
pub struct State {
    data: Box<dyn Any>,
}

impl State {
    pub fn new<T: Any>(data: T) -> Self {
        Self {
            data: Box::new(data),
        }
    }

    #[must_use]
    pub fn get_ref<T: Any>(&self) -> Option<&T> {
        self.data.downcast_ref::<T>()
    }
}
type KeyValue = Vec<(String, Box<dyn Fn(State)>)>;
struct GlobalState {
    mutable_data: RefCell<KeyValue>,
}

thread_local! {
    static STATE: GlobalState = GlobalState {
        mutable_data: RefCell::new(vec![]),
    };
}

pub struct EventManager {}

impl EventManager {
    pub fn register_callback<F: Fn(State) + 'static>(name: &str, callback: F) {
        STATE.with(|state| {
            state
                .mutable_data
                .borrow_mut()
                .push((name.to_string(), Box::new(callback)));
        });
    }

    pub fn call(method: &str, state: State) {
        STATE.with(|gstate| {
            let data = gstate.mutable_data.borrow_mut();
            let kv = data.iter().find(|(key, _)| key.eq(method));

            if let Some((_, m)) = kv {
                m(state);
            }
        });
    }
}
