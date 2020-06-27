fn greeter(name) {
  print "hi! " + name.str();
}

let x = ["cat", "dog", greeter, false, [10]];
let iter = x.iter();

assertEq(iter.current, nil);

assertEq(iter.next(), true);
assertEq(iter.current, "cat");

assertEq(iter.next(), true);
assertEq(iter.current, "dog");

assertEq(iter.next(), true);
assertEq(iter.current, greeter);

assertEq(iter.next(), true);
assertEq(iter.current, false);

assertEq(iter.next(), true);
assertEq(iter.current[0], 10);

assertEq(iter.next(), false);