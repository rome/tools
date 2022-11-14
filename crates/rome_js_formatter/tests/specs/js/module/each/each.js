describe.each`
a|b|expected
${11   } | ${  1  }|${222}
${1-1}|${2+2}|${ 3333}
${2+1+2}|${1111}|${3}
`('$a + $b', ({a, b, expected}) => {
	test(`returns ${expected}`, () => {
		expect(a + b).toBe(expected);
	});

	test(`returned value not be greater than ${expected}`, () => {
		expect(a + b).not.toBeGreaterThan(expected);
	});

	test(`returned value not be less than ${expected}`, () => {
		expect(a + b).not.toBeLessThan(expected);
	});
});

describe.only.each`
a|b|expected
${11   } | ${  1  }|${222}|${'unknown column 1'}|${'unknown column 2'}
${1-1}|${2+2}|${ 3333}
${2+1+2}|${1111}|${3}          |${'unknown column xyz'}
`

describe.only.each`
||
${11   } | ${  1  }|${222}|${'unknown column 1'}|${'unknown column 2'}
${1-1}|${2+2}|${ 3333}
${2+1+2}|${1111}|${3}          |${'unknown column xyz'}
`

describe.each`a    | b    | expected
${1} | ${1} | ${2}
${1} | ${2} | ${3}
${2} | ${1} | ${3}`

// an example to demo multiline quasi
describe.each`a    | b    | expected
${11111111111} | ${a().b(x => x).c().d()} | ${2}
${1} | ${2} | ${3}
${2} | ${1} | ${3}`

test.each`
a | b         | c
${1}      | ${[{ start: 5, end: 15 }]} | ${[1,2,3,4,5,6,7,8]}
${1}| ${[{ start: 5, end: 15 }]} | ${["test", "string", "for", "prettier"]}
${3}      | ${[{ start: 5, end: 15 }]} | ${[]}
${4} | ${[{ start: 1, end: 3 },{ start: 15, end: 20 },]} | ${[]}
`("example test", ({a, b, c}) => {})


test.each`
a |
${[{ a: 1, b: 3 },{ c: 15, d: 20 }]}|
${[{ start: 1, end: 3 },{ start: 15, end: 20 }, { start: 15, end: 20 },]}|
`("example test", ({a, b, c}) => {})


// here poor layout because the first element isn't header
describe.each`${1}a    | b    | expected
${11111111111} | ${2} | ${2}
${1} | ${2} | ${3}
${2} | ${1} | ${3}`

