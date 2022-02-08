type A = typeof import("test");
type B = import("test");
type C = typeof import("test").a.b.c.d.e.f;
