fn f() {
  if (true) return "ok";
}

assertEq(f(), "ok"); // expect: ok
