let iter = [1, 2, 3, "false"].iter();

assertEq(iter.next(), true);
assertEq(iter.current, 1);

assertEq(iter.next(), true);
assertEq(iter.current, 2);

assertEq(iter.next(), true);
assertEq(iter.current, 3);

assertEq(iter.next(), true);
assertEq(iter.current, "false");

assertEq(iter.next(), false);