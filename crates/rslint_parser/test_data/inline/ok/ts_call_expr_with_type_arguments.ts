function a<A, B, C>() {}
a<A, B, C>();
(() => { a }).a<A, B, C>()
(() => a)<A, B, C>();
