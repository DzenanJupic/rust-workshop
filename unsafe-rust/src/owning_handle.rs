use std::mem::ManuallyDrop;

pub struct OwningHandle<O, H> {
    owner: ManuallyDrop<Box<O>>,
    handle: ManuallyDrop<H>,
}

impl<O: 'static, H> OwningHandle<O, H> {
    pub fn mapped<F: Fn(&'static O) -> H>(
        owner: O,
        map: F
    ) -> Self {
        let boxed = Box::new(owner);

        let boxed_ref = &*boxed;
        let boxed_ptr = boxed_ref as *const O;
        let owner_ref: &'static O = unsafe { &*boxed_ptr };

        let handle = map(owner_ref);

        Self {
            owner: ManuallyDrop::new(boxed),
            handle: ManuallyDrop::new(handle),
        }
    }

    pub fn owner(&self) -> &O {
        &self.owner
    }

    pub fn as_ref(&self) -> &H {
        &self.handle
    }

    pub fn as_mut(&mut self) -> &mut H {
        &mut self.handle
    }
}

impl<O, H> Drop for OwningHandle<O, H> {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.handle);
            ManuallyDrop::drop(&mut self.owner);
        }
    }
}
