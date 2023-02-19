use std::marker::PhantomPinned;

#[test]
fn boxing_with_unpin_value() {
  let mut b = Box::pin(123);
  *b = 456;
}

#[test]
fn boxing_with_pin_value() {
  struct Foo {
    s: String,
    _pin: PhantomPinned,
  }

  let foo = Foo {
    s: "Hello World!".to_owned(),
    _pin: PhantomPinned,
  };

  let mut b = Box::pin(foo);
  // Cannot derefMut
  // b.as_mut().s = "ByteDance".to_owned();

  // Looks good
  unsafe { b.as_mut().get_unchecked_mut() }.s = "ByteDance".to_owned();

  // Don't do that! We move the original value and the Pin<T> contract is broken!
  let _ = std::mem::replace(
    unsafe { b.as_mut().get_unchecked_mut() },
    Foo {
      s: "Hello World!".to_owned(),
      _pin: PhantomPinned,
    },
  );
}
