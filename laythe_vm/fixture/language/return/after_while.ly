fn f() {
  while (true) return "ok";
}

assertEq(f(), "ok"); // expect: ok
