type Foo<in T> = {}
type Foo<out> = {}
type Foo<out T> = {}
type Foo<in out> = {}
type Foo<out out> = {}
type Foo<in out out> = {}
type Foo<in X, out Y> = {}
type Foo<out X, in Y> = {}
type Foo<out X, out Y extends keyof X> = {}
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
