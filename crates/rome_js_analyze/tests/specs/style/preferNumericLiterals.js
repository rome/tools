// Invalid
parseInt("111110111", 2) === 503;
parseInt(`111110111`, 2) === 503;
parseInt("767", 8) === 503;
parseInt("1F7", 16) === 503;
Number.parseInt("111110111", 2) === 503;
Number.parseInt("767", 8) === 503;
Number.parseInt("1F7", 16) === 503;

// Invalid, No fix
Number.parseInt("ZZZ", 16) === 503;

// Valid
parseInt(1);
parseInt(1, 3);
Number.parseInt(1);
Number.parseInt(1, 3);

0b111110111 === 503;
0o767 === 503;
0x1f7 === 503;

a[parseInt](1, 2);

parseInt(foo);
parseInt(foo, 2);
Number.parseInt(foo);
Number.parseInt(foo, 2);
