{
  let local = "local";
  fn f() {
    assertEq(local, "local"); // expect: local
  }
  f();
}
