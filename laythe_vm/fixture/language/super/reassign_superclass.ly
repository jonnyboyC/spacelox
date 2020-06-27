class Base {
  method() {
    return "Base.method()";
  }
}

class Derived < Base {
  method() {
    return super.method();
  }
}

class OtherBase {
  method() {
    return "OtherBase.method()";
  }
}

let derived = Derived();
assertEq(derived.method(), "Base.method()"); // expect: Base.method()
Base = OtherBase;
assertEq(derived.method(), "Base.method()"); // expect: Base.method()
