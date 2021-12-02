let { ... } = a;
let { ...c = "default" } = a;
let { ...{a} } = b;
let { ...rest, other_assignment } = a;
let { ...rest, } = a;
async function test() {
  let { ...await } = a;
}
