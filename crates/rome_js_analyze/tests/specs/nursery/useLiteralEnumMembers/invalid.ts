enum InvalidObject {
  A = {},
}


enum InvalidArray {
  A = [],
}


enum InvalidTemplateLiteral {
  A = `foo ${0}`,
}


enum InvalidConstructor {
  A = new Set(),
}


enum InvalidExpression {
  A = 2 + 2,
}

enum InvalidExpression {
  A = delete 2,
  B = -a,
  C = void 2,
  D = ~2,
  E = !0,
}


const variable = 'Test';
enum InvalidVariable {
  A = 'TestStr',
  B = 2,
  C,
  V = variable,
}


enum InvalidEnumMember {
  A = 'TestStr',
  B = A,
}


const Valid = { A: 2 };
enum InvalidObjectMember {
  A = 'TestStr',
  B = Valid.A,
}


enum Valid {
  A,
}
enum InvalidEnumMember {
  A = 'TestStr',
  B = Valid.A,
}


const obj = { a: 1 };
enum InvalidSpread {
  A = 'TestStr',
  B = { ...a },
}


const x = 1;
enum Foo {
  A = x << 0,
  B = x >> 0,
  C = x >>> 0,
  D = x | 0,
  E = x & 0,
  F = x ^ 0,
  G = ~x,
}

