// This benchmark stresses instance creation and initializer calling.

class Foo {
  init() {}
}

let start = clock();
let i = 0;
while (i < 500000) {
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  Foo();
  i = i + 1;
}

print clock() - start;

