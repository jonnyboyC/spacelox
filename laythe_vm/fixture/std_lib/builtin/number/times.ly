let results = [0, 1, 2, 3, 4].iter();

5.times().each(|i| {
  results.next();
  assertEq(i, results.current);
});