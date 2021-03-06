use std::mem;

pub struct Any {
    ptr: *mut (),
    // Can't get a real TypeId because we don't have 'static bounds on the
    // Serializer associates types, so settle for checking size_of and align_of.
    fingerprint: Fingerprint,
}

impl Any {
    pub fn new<T>(t: T) -> Self {
        let ptr: *mut T = Box::into_raw(Box::new(t));
        Any {
            ptr: ptr as *mut (),
            fingerprint: Fingerprint::of::<T>(),
        }
    }

    pub fn view<T>(&mut self) -> &mut T {
        if self.fingerprint != Fingerprint::of::<T>() {
            panic!("invalid cast");
        }
        let ptr = self.ptr as *mut T;
        unsafe { &mut *ptr }
    }

    pub fn take<T>(self) -> T {
        if self.fingerprint != Fingerprint::of::<T>() {
            panic!("invalid cast");
        }
        let ptr = self.ptr as *mut T;
        let box_t = unsafe { Box::from_raw(ptr) };
        *box_t
    }
}

#[derive(Eq, PartialEq)]
struct Fingerprint {
    size: usize,
    align: usize,
}

impl Fingerprint {
    fn of<T>() -> Fingerprint {
        Fingerprint {
            size: mem::size_of::<T>(),
            align: mem::align_of::<T>(),
        }
    }
}
