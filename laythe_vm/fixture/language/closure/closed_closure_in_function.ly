let f;

{
  let local = "local";
  fn f_() {
    assertEq(local, "local");
  }
  f = f_;
}

f(); // expect: local
