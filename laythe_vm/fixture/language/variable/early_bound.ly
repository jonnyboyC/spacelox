let a = "outer";
{
  fn foo() {
    return a;
  }

  assertEq(foo(), "outer"); // expect: outer
  let a = "inner";
  assertEq(foo(), "outer"); // expect: outer
}
