type Foo = "foo" | "foo_";
type Bar = "bar" | "bar_";

type Baz = `${Foo | Bar}_id`;
