let a: String;

let e: Object;

let b: { c: String };

function foo(a: String) {}

'a' as String;

class Foo<T = String> extends Bar<String> implements Baz<Object> {
  constructor(foo: String | Object) {}

  exit(): Array<String> {
    const foo: String = 1 as String;
  }
}

let baz: [boolean, Boolean] = [true, false];

let z = true as Boolean;

type Props = {};

const abc: {} = [{}]

let fn: Function = () => true

