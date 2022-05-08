type Foo<in T> = T
type Foo<out T> = T
type Foo<in out> = T
type Foo<out out> = T
type Foo<in out out> = T
type Foo<in X, out Y> = [X, Y]
type Foo<out X, in Y> = [X, Y]
type Foo<out X, out Y extends keyof X> = [X, Y]
