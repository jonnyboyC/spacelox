fn f() {
  while (true) {
    let i = "i";
    return i;
  }
}

assertEq(f(), "i"); // expect: i
