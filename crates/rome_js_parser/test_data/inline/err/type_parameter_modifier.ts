type Foo<i\\u006E T> = T
type Foo<ou\\u0074 T> = T
type Foo<in in> = T
type Foo<out in> = T
type Foo<out in T> = T
type Foo<public T> = T
type Foo<in out in T> = T
type Foo<in out out T> = T
function foo<in T>() {}
function foo<out T>() {}
