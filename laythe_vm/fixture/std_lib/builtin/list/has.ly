let x = [1, 2, nil, false, "example"];

assertEq(x.has(2), true);
assertEq(x.has("example"), true);
assertEq(x.has(false), true);

assertEq(x.has(true), false);
assertEq(x.has("no present"), false);