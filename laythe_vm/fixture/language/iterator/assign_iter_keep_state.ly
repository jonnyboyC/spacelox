let list = [1, 2, 3, 4];
let iter1 = list.iter();

iter1.next();
iter1.next();

let iter2 = iter1;
iter2.next();
iter2.next();

assertEq(iter1.current, 4);
assertEq(iter2.current, 4);
