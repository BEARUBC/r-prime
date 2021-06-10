use std::sync::{
    Mutex,
    MutexGuard,
    PoisonError,
};

use crate::component::Identifier;

pub(crate) type MutexError<'a> = PoisonError<MutexGuard<'a, Identifier>>;

lazy_static! {
    static ref ID_STORE: Mutex<usize> = Mutex::new(0usize);
}

pub(crate) fn get_new_id<'a>() -> Result<usize, MutexError<'a>> {
    ID_STORE.lock().map(|mut ref_id| {
        let id = *ref_id;
        *ref_id += 1usize;

        id
    })
}
