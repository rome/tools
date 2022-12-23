type ShortUnion =
    | A
    | B

type LongUnion = A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z

type Comments =
    // leading separator
    |
    // leading type
    A | B /*
trailing type */

type A = [
    /*leading comment with new line*/
    A | B,
    ];

type RemoveLeadingSeparatorIfNotBreak = /*a*/ | /*b*/ A | B;

type BreakLongTypeAddedLeadingSeparator = BBBBBBBBBBBBBBBBBBBBBBBB | CCCCCCCCCCCCCCCCCCCCCCCCCCC | DDDDDDDDDDDDDDDDDDDDDDDDDDDDD;
type BreakLongTypeWithLeadingComment = /*leading comment*/ BBBBBBBBBBBBBBBBBBBBBBBB | CCCCCCCCCCCCCCCCCCCCCCCCCCC | DDDDDDDDDDDDDDDDDDDDDDDDDDDDD;

<B | C | D> someLongLongObject.longlongmember;

<BBBBBBBBBBBBBBBBBBBBBBBB | CCCCCCCCCCCCCCCCCCCCCCCCCCC | DDDDDDDDDDDDDDDDDDDDDDDDDDDDD> someLongLongObject.longlongmember;
(<BBBBBBBBBBBBBBBBBBBBBBBB | CCCCCCCCCCCCCCCCCCCCCCCCCCC | DDDDDDDDDDDDDDDDDDDDDDDDDDDDD> someLongLongObject.longlongmember) += 1

type FunctionTypeWithReturnUnion1 = () => /*1*/|/*2*/ A | B | C;

type FunctionTypeWithReturnUnion2 = () => BBBBBBBBBBBBBBBBBBBBBBBB | CCCCCCCCCCCCCCCCCCCCCCCCCCC | DDDDDDDDDDDDDDDDDDDDDDDDDDDDD;

type InlineType = TypeName | null | void;
type InlineTypeWithLongName = TypeNameTypeNameTypeNameTypeNameTypeNameTypeNameTypeName | null | void;

type TypeWithTypleInsideShort = [
    A | [A, B, C] | C,
    A | [A, B, C] | C,
  ];

type TypeWithTypleInsideLong = [
    AAAAAAAAAAAAAAAAA | [AAAAAAAAAAAAAAAAA, BBBBBBBBBBBB, CCCCCCCCCCCCC] | CCCCCCCCCCCCCCCCCCCC,
    AAAAAAAAAAAAAAAAA | [AAAAAAAAAAAAAAAAA, BBBBBBBBBBBB, CCCCCCCCCCCCC] | CCCCCCCCCCCCCCCCCCCC,
  ];

type TypeWithUnionInsideIntersactionAddParenthesesShort = B & (C | A) & D;

type TypeWithUnionInsideIntersactionAddParenthesesLong = BBBBBBBBBBBB & (CCCCCCCCCCCCC | AAAAAAAAAAAAAAAAA) & DDDDDDDDDDDDDDDDDDDDDDDDDDDDD;

const fooo: SomeThingWithShortMappedType<{
      [P in A | B | C | string]: number;
    }> = {};

const fooo: SomeThingWithLongMappedType<{
    [P in AAAAAAAAAAAAAAAAA | BBBBBBBBBBBB | CCCCCCCCCCCCCCCCCCCCC | DDDDDDDDDDDDDDDDDDDDDDDDDDDDD]: number;
    }> = {};

    export type A =
	| aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
	| bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;

export type B =
	| aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
	| bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;

export type C =
	| aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
	| bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;

export type D =
	| aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
	| bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;

export type Multi = (string | number)[];

function f(): string | number {}

var x: string | number;
var y: string | number;

class Foo<T extends string | number> {}

interface Interface {
	i: (X | Y) & Z;
	j: Partial<X | Y>;
}

type State = {
	sharedProperty: any;
} & (
	| { discriminant: "FOO"; foo: any }
	| { discriminant: "BAR"; bar: any }
	| { discriminant: "BAZ"; baz: any }
);

const foo1 = [abc, def, ghi, jkl, mno, pqr, stu, vwx, yz] as (
	| string
	| undefined
)[];

const foo2: (
	| AAAAAAAAAAAAAAAAAAAAAA
	| BBBBBBBBBBBBBBBBBBBBBB
	| CCCCCCCCCCCCCCCCCCCCCC
	| DDDDDDDDDDDDDDDDDDDDDD
)[] = [];

const foo3: keyof (
	| AAAAAAAAAAAAAAAAAAAAAA
	| BBBBBBBBBBBBBBBBBBBBBB
	| CCCCCCCCCCCCCCCCCCCCCC
	| DDDDDDDDDDDDDDDDDDDDDD
) = bar;

const foo4:
	| foo
	| (
			| AAAAAAAAAAAAAAAAAAAAAA
			| BBBBBBBBBBBBBBBBBBBBBB
			| CCCCCCCCCCCCCCCCCCCCCC
			| DDDDDDDDDDDDDDDDDDDDDD
	  ) = bar;

let a1: C;
let a2: C;
let a3: C;
let a4: C;
let a5: C;
let a6: /*1*/ C;
let a7: /*1*/ C;
let a8: /*1*/ C;
let a9: /*1*/ C;
let a10: /*1*/ /*2*/ C;
let a11: /*1*/ /*2*/ C;

let aa1: /*1*/ /*2*/ C | D;
let aa2: /*1*/ /*2*/ C | /*3*/ D;
let aa3: /*1*/ /*2*/ C | /*3*/ D /*4*/;

type A1 = C;
type A2 = C;
type A3 = C;
type A4 = C;
type A5 = C;
type A6 = /*1*/ C;
type A7 = /*1*/ C;
type A8 = /*1*/ C;
type A9 = /*1*/ C;
type A10 = /*1*/ /*2*/ C;
type A11 = /*1*/ /*2*/ C;
type A12 = /*1*/ C;
type A13 = /*1*/ C;

type Aa1 = /*1*/ /*2*/ C | D;
type Aa2 = /*1*/ /*2*/ C | /*3*/ D;
type Aa3 = /*1*/ /*2*/ C | /*3*/ D /*4*/;

type C1 = /*1*/ a | b;
type C2 = /*1*/ a | b;
type C3 = /*1*/ a | b;
type C4 = /*1*/ a | b;
type C5 = /*1*/ a | b;
type C6 /*0*/ = /*1*/ a | b;

type Ctor = (new () => X) | Y;

type A = [AAAAAAAAAAAAAAAAAAAAAA | BBBBBBBBBBBBBBBBBBBBBB | CCCCCCCCCCCCCCCCCCCCCC | DDDDDDDDDDDDDDDDDDDDDD]

type B = [
  | AAAAAAAAAAAAAAAAAAAAAA
  | BBBBBBBBBBBBBBBBBBBBBB
  | CCCCCCCCCCCCCCCCCCCCCC
  | DDDDDDDDDDDDDDDDDDDDDD
]

type B1 = [
  (
    | AAAAAAAAAAAAAAAAAAAAAA
    | BBBBBBBBBBBBBBBBBBBBBB
    | CCCCCCCCCCCCCCCCCCCCCC
    | DDDDDDDDDDDDDDDDDDDDDD
  )
]

type C = [
  | [AAAAAAAAAAAAAAAAAAAAAA | BBBBBBBBBBBBBBBBBBBBBB | CCCCCCCCCCCCCCCCCCCCCC | DDDDDDDDDDDDDDDDDDDDDD]
  | [AAAAAAAAAAAAAAAAAAAAAA | BBBBBBBBBBBBBBBBBBBBBB | CCCCCCCCCCCCCCCCCCCCCC | DDDDDDDDDDDDDDDDDDDDDD]
]

type D = [
  (AAAAAAAAAAAAAAAAAAAAAA | BBBBBBBBBBBBBBBBBBBBBB | CCCCCCCCCCCCCCCCCCCCCC | DDDDDDDDDDDDDDDDDDDDDD),
  (AAAAAAAAAAAAAAAAAAAAAA | BBBBBBBBBBBBBBBBBBBBBB | CCCCCCCCCCCCCCCCCCCCCC | DDDDDDDDDDDDDDDDDDDDDD)
]

type D1 = [
  (
    | AAAAAAAAAAAAAAAAAAAAAA
    | BBBBBBBBBBBBBBBBBBBBBB
    | CCCCCCCCCCCCCCCCCCCCCC
    | DDDDDDDDDDDDDDDDDDDDDD
  ),
  (
    | AAAAAAAAAAAAAAAAAAAAAA
    | BBBBBBBBBBBBBBBBBBBBBB
    | CCCCCCCCCCCCCCCCCCCCCC
    | DDDDDDDDDDDDDDDDDDDDDD
  )
]

type D2 = [
  | AAAAAAAAAAAAAAAAAAAAAA
  | BBBBBBBBBBBBBBBBBBBBBB
  | CCCCCCCCCCCCCCCCCCCCCC
  | DDDDDDDDDDDDDDDDDDDDDD,
  | AAAAAAAAAAAAAAAAAAAAAA
  | BBBBBBBBBBBBBBBBBBBBBB
  | CCCCCCCCCCCCCCCCCCCCCC
  | DDDDDDDDDDDDDDDDDDDDDD
]

type E = [ AA | BB, AA | BB ]

type F = [
  | AAAAAAAAAAAAAAAAAAAAAA
  | BBBBBBBBBBBBBBBBBBBBBB
  | CCCCCCCCCCCCCCCCCCCCCC
  | DDDDDDDDDDDDDDDDDDDDDD,
  | AAAAAAAAAAAAAAAAAAAAAA
  | BBBBBBBBBBBBBBBBBBBBBB
]

type GetChatsSagaEffects =
  | CallEffect
  | PutEffect<
      | GetUsersRequestedAction
      | GetChatsSucceededAction
      | GetChatsFailedAction
      | GetChatsStartedAction
    >
  | SelectEffect

//https://github.com/prettier/prettier/issues/13153
type SuperLongTypeNameLoremIpsumLoremIpsumBlaBlaBlaBlaBlaBlaBlaBlaBlaBlaBlaBla =
| Fooo1000
| Baz2000
| BarLoooooooooooooooooooooooooooooooooooooooooooooooooLong;
