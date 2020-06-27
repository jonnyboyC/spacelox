class Foo {
  inFoo() {
    return "in foo";
  }
}

class Bar < Foo {
  inBar() {
    return "in bar";
  }
}

class Baz < Bar {
  inBaz() {
    return "in baz";
  }
}

let baz = Baz();
assertEq(baz.inFoo(), "in foo"); // expect: in foo
assertEq(baz.inBar(), "in bar"); // expect: in bar
assertEq(baz.inBaz(), "in baz"); // expect: in baz
