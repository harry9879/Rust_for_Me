// The Drop Check

#[derive(Debug)]
pub struct Boks<T> {
    p: *mut T,
}

impl<T> Drop for Boks<T> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.p);
        }
        // Box from the inner type, this is probably the easiest way to do it because it both calls the destructor of the T and it deallocates the box
        // std::ptr::drop_in_place(self.p);
    }
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Boks {
            p: Box::into_raw(Box::new(t)),
        }
    }
}

impl<T> std::ops::Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers, and hasn't been freed, since self is alive
        unsafe {
            &*self.p
        }
    }
}

impl<T> std::ops::DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers, and hasn't been freed, since self is alive
        // Also, since we have &mut self, no other mutable reference has benn given out to p
        unsafe {
            &mut *self.p
        }
    }
}

fn main() {
    let x = 42;
    let b = Boks::ny(x);
    println!("{:?}", *b);
    y = 43;
    drop(b); // read from &mut y
}
