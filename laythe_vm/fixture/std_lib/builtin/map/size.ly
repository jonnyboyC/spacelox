assertEq({"hi": 10}.size(), 1);

let x = {
  1: 1,
  2: 2,
  3: 3,
  4: 4,
  5: 5,
};
assertEq(x.size(), 5);
x[6] = 6;
assertEq(x.size(), 6);