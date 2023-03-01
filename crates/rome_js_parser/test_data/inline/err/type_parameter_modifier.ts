type Foo<i\\u006E T> = {}
type Foo<ou\\u0074 T> = {}
type Foo<in in> = {}
type Foo<out in> = {}
type Foo<out in T> = {}
type Foo<public T> = {}
type Foo<in out in T> = {}
type Foo<in out out T> = {}
function foo<in T>() {}
function foo<out T>() {}
