// Tests that we correctly track the line info across multiline strings.
let a = "1
2
3
";

let b = '1
2
3
';

err; // // expect runtime error: Undefined variable 'err'.