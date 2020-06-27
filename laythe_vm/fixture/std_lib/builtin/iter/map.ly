let iter = [1, 2, 3, 4].iter().map(|x| x / 2);

assertEq(iter.next(), true);
assertEq(iter.current, 0.5);

assertEq(iter.next(), true);
assertEq(iter.current, 1);

assertEq(iter.next(), true);
assertEq(iter.current, 1.5);

assertEq(iter.next(), true);
assertEq(iter.current, 2.0);

assertEq(iter.next(), false);