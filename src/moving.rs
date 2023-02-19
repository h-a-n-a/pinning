#[test]
fn moving() {
  struct Foo {}

  let foo = Foo {};
  println!("{:p}", &foo);
  let bar = foo;
  println!("{:p}", &bar);
}

#[test]
fn moving_with_ptr() {
  struct Foo(Box<String>);

  let foo = Foo(Box::new("Hello World!".to_owned()));
  let foo_ptr = &foo as *const Foo;
  println!("{}", &foo.0);
  let bar = foo;
  println!("{}", &bar.0);
  drop(bar);
  let foo = unsafe { &*foo_ptr };
  println!("{}", &foo.0);
}
