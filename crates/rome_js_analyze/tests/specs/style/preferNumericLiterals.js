// Invalid
parseInt("111110111", 2);
parseInt(`111110111`, 2);
parseInt("767", 8);
parseInt("1F7", 16);

Number.parseInt("111110111", 2);
Number.parseInt("767", 8);
Number.parseInt("1F7", 16);

Number["parseInt"]("1F7", 16);
Number['parseInt']("1F7", 16);
Number[`parseInt`]("1F7", 16);

// Invalid, No fix
Number.parseInt("ZZZ", 16);

// Valid
parseInt(1);
parseInt(1, 3);
Number.parseInt(1);
Number.parseInt(1, 3);

0b111110111;
0o767;
0x1f7;

a[parseInt](1, 2);

parseInt(foo);
parseInt(foo, 2);
Number.parseInt(foo);
Number.parseInt(foo, 2);
