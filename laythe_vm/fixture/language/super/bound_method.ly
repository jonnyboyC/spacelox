class A {
  method(arg) {
    return "A.method(" + arg + ")";
  }
}

class B < A {
  getClosure() {
    return super.method;
  }

  method(arg) {
    return "B.method(" + arg + ")";
  }
}


let closure = B().getClosure();
assertEq(closure("arg"), "A.method(arg)"); // expect: A.method(arg)
