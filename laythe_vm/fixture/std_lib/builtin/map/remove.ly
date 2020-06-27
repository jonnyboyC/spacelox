let map = {
  1: 1,
  2: 2,
  3: 3,
  false: false,
  "stuff": "stuff"
};

assertEq(map.remove(1), 1);
assertEq(map.remove(2), 2);
assertEq(map.remove(3), 3);
assertEq(map.remove(false), false);
assertEq(map.remove("stuff"), "stuff");
assertEq(map.size(), 0);