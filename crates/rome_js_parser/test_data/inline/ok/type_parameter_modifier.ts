type Foo<in T> = T
type Foo<out> = out
type Foo<out T> = T
type Foo<in out> = T
type Foo<out out> = T
type Foo<in out out> = T
type Foo<in X, out Y> = [X, Y]
type Foo<out X, in Y> = [X, Y]
type Foo<out X, out Y extends keyof X> = [X, Y]
class Foo<in T> {}
class Foo<out T> {}
export default class Foo<in T> {}
class Foo<out T> {}
interface Foo<in T> {}
interface Foo<out T> {}
declare class Foo<in T> {}
declare class Foo<out T> {}
declare interface Foo<in T> {}
declare interface Foo<out T> {}
