use std::{marker::PhantomPinned, pin::Pin};

#[test]
fn pinning_but_still_moveable() {
  struct Foo {
    _pin: PhantomPinned,
  }

  let foo = Foo {
    _pin: PhantomPinned,
  };

  let bar = foo;
}

#[test]
fn pinning_with_unpin_value() {
  struct Foo {}

  let mut foo = Foo {};
  let mut p = Pin::new(&mut foo);
  let p = &mut *p;
  *p = Foo {};
}

#[test]
fn pinning_with_pin_value_directly_get() {
  struct Foo {
    s: String,
    _pin: PhantomPinned,
  }

  let mut foo = Foo {
    s: "Hello World!".to_owned(),
    _pin: PhantomPinned,
  };
  let p = unsafe { Pin::new_unchecked(&mut foo) };
  let p = unsafe { p.get_unchecked_mut() };
  p.s = "ByteDance".to_owned();
}

#[test]
fn pinning_with_pin_value_mapped() {
  struct Foo {
    s: String,
    _pin: PhantomPinned,
  }

  let mut foo = Foo {
    s: "Hello World!".to_owned(),
    _pin: PhantomPinned,
  };
  let p = unsafe { Pin::new_unchecked(&mut foo) };
  let mut p = unsafe { p.map_unchecked_mut(|p| &mut p.s) };
  *p = "ByteDance".to_owned();
}
