class A {
  say() {
    print "A";
  }
}

class B < A {
  getClosure() {
    fn closure() {
      super.say();
    }
    return closure;
  }

  say() {
    print "B";
  }
}

class C < B {
  say() {
    print "C";
  }
}

C().getClosure()(); // expect: A
