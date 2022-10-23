// valid
console.log("foo" + "bar");
console.log(foo() + "\n");
// invalid
const foo = "bar";
console.log(foo + "baz");
console.log(1 * 2 + "foo");
console.log(1 + "foo" + 2 + "bar" + "baz" + 3);
console.log((1 + "foo") * 2);
console.log(1 * (2 + "foo") + "bar");
console.log("foo" + 1);
console.log("foo" + `bar${`baz${"bat" + "bam"}`}` + "boo");
console.log("foo" + 1 + 2);
1 + "2" - 3;
foo() + " bar";

1 * /**leading*/"foo"    /**trailing */                   + "bar"

console.log("${foo." + bar + ".baz}");
