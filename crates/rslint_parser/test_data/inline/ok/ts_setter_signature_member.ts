type A = { set a(b: number) }
type B = { set a(b) }
// members that look like setters but aren't setters
type C = { set(a) }
type D = { set: number }
type E = { set }
