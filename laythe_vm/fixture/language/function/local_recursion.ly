{
  fn fib(n) {
    if (n < 2) return n;
    return fib(n - 1) + fib(n - 2);
  }

  assertEq(fib(8), 21); // expect: 21
}
