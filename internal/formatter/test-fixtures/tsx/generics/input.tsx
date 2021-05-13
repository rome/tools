const f1 = <T,>(arg: T) => false;
const f2 = <T extends any>(arg: T) => false;
const f3 = <T, S>(arg) => false;

function f4<T>() {
  return false;
}

f5<T>();
f6<T, S>();

interface Interface1<T> {
  one: "one";
}

interface Interface2 {
  two: Two<T>;
}

type Type1<T> = "type1";

type Type2 = Two<T>;
