pub type A = for<'a> fn(&'a bool);

pub type B = fn(&bool);

pub type C<'a, 'b> = (&'a u8, &'b u16);

pub type D<T: IntoIterator> = T::Item;

pub type E<T: IntoIterator> = T::Item;

pub fn abc(_: bool) { }

pub fn def(_: &bool) { }

pub fn efg(_: &'static str) { }

pub fn fgh(_: &str) { }
