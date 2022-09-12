// base cases

// chained members
foo && foo.bar
foo.bar && foo.bar.baz
foo && foo()
foo.bar && foo.bar()
foo && foo.bar && foo.bar.baz && foo.bar.baz.buzz
foo.bar && foo.bar.baz && foo.bar.baz.buzz

// case with a jump (i.e. a non-nullish prop)
foo && foo.bar && foo.bar.baz.buzz
foo.bar && foo.bar.baz.buzz

// case where for some reason there is a doubled up expression
foo && foo.bar && foo.bar.baz && foo.bar.baz && foo.bar.baz.buzz
foo.bar && foo.bar.baz && foo.bar.baz && foo.bar.baz.buzz

// chained members with element access
foo && foo[bar] && foo[bar].baz && foo[bar].baz.buzz

// case with a jump (i.e. a non-nullish prop)
foo && foo[bar].baz && foo[bar].baz.buzz

// chained calls
foo && foo.bar && foo.bar.baz && foo.bar.baz.buzz()
foo && foo.bar && foo.bar.baz && foo.bar.baz.buzz && foo.bar.baz.buzz()
foo.bar && foo.bar.baz && foo.bar.baz.buzz && foo.bar.baz.buzz()

// case with a jump (i.e. a non-nullish prop)
foo && foo.bar && foo.bar.baz.buzz()
foo.bar && foo.bar.baz.buzz()

// case with a jump (i.e. a non-nullish prop)
foo && foo.bar && foo.bar.baz.buzz && foo.bar.baz.buzz()

// case with a call expr inside the chain for some inefficient reason
foo && foo.bar() && foo.bar().baz && foo.bar().baz.buzz && foo.bar().baz.buzz()

// chained calls with element access
foo && foo.bar && foo.bar.baz && foo.bar.baz[buzz]()
foo && foo.bar && foo.bar.baz && foo.bar.baz[buzz] && foo.bar.baz[buzz]()

// (partially) pre-optional chained
foo && foo?.bar && foo?.bar.baz && foo?.bar.baz[buzz] && foo?.bar.baz[buzz]()
foo && foo?.bar.baz && foo?.bar.baz[buzz]
foo && foo?.() && foo?.().bar
foo.bar && foo.bar?.() && foo.bar?.().baz


// it should ignore parts of the expression that aren't part of the expression chain

// chained members
foo && foo.bar && bing
foo.bar && foo.bar.baz && bing
foo && foo() && bing
foo.bar && foo.bar() && bing
foo && foo.bar && foo.bar.baz && foo.bar.baz.buzz && bing
foo.bar && foo.bar.baz && foo.bar.baz.buzz && bing

// case with a jump (i.e. a non-nullish prop)
foo && foo.bar && foo.bar.baz.buzz && bing
foo.bar && foo.bar.baz.buzz && bing

// case where for some reason there is a doubled up expression
foo && foo.bar && foo.bar.baz && foo.bar.baz && foo.bar.baz.buzz && bing
foo.bar && foo.bar.baz && foo.bar.baz && foo.bar.baz.buzz && bing

// chained members with element access
foo && foo[bar] && foo[bar].baz && foo[bar].baz.buzz && bing

// case with a jump (i.e. a non-nullish prop)
foo && foo[bar].baz && foo[bar].baz.buzz && bing

// chained calls
foo && foo.bar && foo.bar.baz && foo.bar.baz.buzz() && bing
foo && foo.bar && foo.bar.baz && foo.bar.baz.buzz && foo.bar.baz.buzz() && bing
foo.bar && foo.bar.baz && foo.bar.baz.buzz && foo.bar.baz.buzz() && bing

// case with a jump (i.e. a non-nullish prop)
foo && foo.bar && foo.bar.baz.buzz() && bing
foo.bar && foo.bar.baz.buzz() && bing

// case with a jump (i.e. a non-nullish prop)
foo && foo.bar && foo.bar.baz.buzz && foo.bar.baz.buzz() && bing

// case with a call expr inside the chain for some inefficient reason
foo && foo.bar() && foo.bar().baz && foo.bar().baz.buzz && foo.bar().baz.buzz() && bing

// chained calls with element access
foo && foo.bar && foo.bar.baz && foo.bar.baz[buzz]() && bing
foo && foo.bar && foo.bar.baz && foo.bar.baz[buzz] && foo.bar.baz[buzz]() && bing

// (partially) pre-optional chained
foo && foo?.bar && foo?.bar.baz && foo?.bar.baz[buzz] && foo?.bar.baz[buzz]() && bing
foo && foo?.bar.baz && foo?.bar.baz[buzz] && bing
foo && foo?.() && foo?.().bar && bing
foo.bar && foo.bar?.() && foo.bar?.().baz && bing

// chained members
foo && foo.bar && bing.bong
foo.bar && foo.bar.baz && bing.bong
foo && foo() && bing.bong
foo.bar && foo.bar() && bing.bong
foo && foo.bar && foo.bar.baz && foo.bar.baz.buzz && bing.bong
foo.bar && foo.bar.baz && foo.bar.baz.buzz && bing.bong

// case with a jump (i.e. a non-nullish prop)
foo && foo.bar && foo.bar.baz.buzz && bing.bong
foo.bar && foo.bar.baz.buzz && bing.bong

// case where for some reason there is a doubled up expression
foo && foo.bar && foo.bar.baz && foo.bar.baz && foo.bar.baz.buzz && bing.bong
foo.bar && foo.bar.baz && foo.bar.baz && foo.bar.baz.buzz && bing.bong

// chained members with element access
foo && foo[bar] && foo[bar].baz && foo[bar].baz.buzz && bing.bong

// case with a jump (i.e. a non-nullish prop)
foo && foo[bar].baz && foo[bar].baz.buzz && bing.bong

// chained calls
foo && foo.bar && foo.bar.baz && foo.bar.baz.buzz() && bing.bong
foo && foo.bar && foo.bar.baz && foo.bar.baz.buzz && foo.bar.baz.buzz() && bing.bong
foo.bar && foo.bar.baz && foo.bar.baz.buzz && foo.bar.baz.buzz() && bing.bong

// case with a jump (i.e. a non-nullish prop)
foo && foo.bar && foo.bar.baz.buzz() && bing.bong
foo.bar && foo.bar.baz.buzz() && bing.bong

// case with a jump (i.e. a non-nullish prop)
foo && foo.bar && foo.bar.baz.buzz && foo.bar.baz.buzz() && bing.bong

// case with a call expr inside the chain for some inefficient reason
foo && foo.bar() && foo.bar().baz && foo.bar().baz.buzz && foo.bar().baz.buzz() && bing.bong

// chained calls with element access
foo && foo.bar && foo.bar.baz && foo.bar.baz[buzz]() && bing.bong
foo && foo.bar && foo.bar.baz && foo.bar.baz[buzz] && foo.bar.baz[buzz]() && bing.bong

// (partially) pre-optional chained
foo && foo?.bar && foo?.bar.baz && foo?.bar.baz[buzz] && foo?.bar.baz[buzz]() && bing.bong
foo && foo?.bar.baz && foo?.bar.baz[buzz] && bing.bong
foo && foo?.() && foo?.().bar && bing.bong
foo.bar && foo.bar?.() && foo.bar?.().baz && bing.bong

// strict nullish equality checks x !== null && x.y !== null
// chained members
foo !== null && foo.bar
foo.bar !== null && foo.bar.baz
foo !== null && foo()
foo.bar !== null && foo.bar()
foo !== null && foo.bar !== null && foo.bar.baz !== null && foo.bar.baz.buzz
foo.bar !== null && foo.bar.baz !== null && foo.bar.baz.buzz

// case with a jump (i.e. a non-nullish prop)
foo !== null && foo.bar !== null && foo.bar.baz.buzz
foo.bar !== null && foo.bar.baz.buzz

// case where for some reason there is a doubled up expression
foo !== null && foo.bar !== null && foo.bar.baz !== null && foo.bar.baz !== null && foo.bar.baz.buzz
foo.bar !== null && foo.bar.baz !== null && foo.bar.baz !== null && foo.bar.baz.buzz

// chained members with element access
foo !== null && foo[bar] !== null && foo[bar].baz !== null && foo[bar].baz.buzz

// case with a jump (i.e. a non-nullish prop)
foo !== null && foo[bar].baz !== null && foo[bar].baz.buzz

// chained calls
foo !== null && foo.bar !== null && foo.bar.baz !== null && foo.bar.baz.buzz()
foo !== null && foo.bar !== null && foo.bar.baz !== null && foo.bar.baz.buzz !== null && foo.bar.baz.buzz()
foo.bar !== null && foo.bar.baz !== null && foo.bar.baz.buzz !== null && foo.bar.baz.buzz()

// case with a jump (i.e. a non-nullish prop)
foo !== null && foo.bar !== null && foo.bar.baz.buzz()
foo.bar !== null && foo.bar.baz.buzz()

// case with a jump (i.e. a non-nullish prop)
foo !== null && foo.bar !== null && foo.bar.baz.buzz !== null && foo.bar.baz.buzz()

// case with a call expr inside the chain for some inefficient reason
foo !== null && foo.bar() !== null && foo.bar().baz !== null && foo.bar().baz.buzz !== null && foo.bar().baz.buzz()

// chained calls with element access
foo !== null && foo.bar !== null && foo.bar.baz !== null && foo.bar.baz[buzz]()
foo !== null && foo.bar !== null && foo.bar.baz !== null && foo.bar.baz[buzz] !== null && foo.bar.baz[buzz]()

// (partially) pre-optional chained
foo !== null && foo?.bar !== null && foo?.bar.baz !== null && foo?.bar.baz[buzz] !== null && foo?.bar.baz[buzz]()
foo !== null && foo?.bar.baz !== null && foo?.bar.baz[buzz]
foo !== null && foo?.() !== null && foo?.().bar
foo.bar !== null && foo.bar?.() !== null && foo.bar?.().baz

// chained members
foo !== undefined && foo.bar
foo.bar !== undefined && foo.bar.baz
foo !== undefined && foo()
foo.bar !== undefined && foo.bar()
foo !== undefined && foo.bar !== undefined && foo.bar.baz !== undefined && foo.bar.baz.buzz
foo.bar !== undefined && foo.bar.baz !== undefined && foo.bar.baz.buzz

// case with a jump (i.e. a non-nullish prop)
foo !== undefined && foo.bar !== undefined && foo.bar.baz.buzz
foo.bar !== undefined && foo.bar.baz.buzz

// case where for some reason there is a doubled up expression
foo !== undefined && foo.bar !== undefined && foo.bar.baz !== undefined && foo.bar.baz !== undefined && foo.bar.baz.buzz
foo.bar !== undefined && foo.bar.baz !== undefined && foo.bar.baz !== undefined && foo.bar.baz.buzz

// chained members with element access
foo !== undefined && foo[bar] !== undefined && foo[bar].baz !== undefined && foo[bar].baz.buzz

// case with a jump (i.e. a non-nullish prop)
foo !== undefined && foo[bar].baz !== undefined && foo[bar].baz.buzz

// chained calls
foo !== undefined && foo.bar !== undefined && foo.bar.baz !== undefined && foo.bar.baz.buzz()
foo !== undefined && foo.bar !== undefined && foo.bar.baz !== undefined && foo.bar.baz.buzz !== undefined && foo.bar.baz.buzz()
foo.bar !== undefined && foo.bar.baz !== undefined && foo.bar.baz.buzz !== undefined && foo.bar.baz.buzz()

// case with a jump (i.e. a non-nullish prop)
foo !== undefined && foo.bar !== undefined && foo.bar.baz.buzz()
foo.bar !== undefined && foo.bar.baz.buzz()

// case with a jump (i.e. a non-nullish prop)
foo !== undefined && foo.bar !== undefined && foo.bar.baz.buzz !== undefined && foo.bar.baz.buzz()

// case with a call expr inside the chain for some inefficient reason
foo !== undefined && foo.bar() !== undefined && foo.bar().baz !== undefined && foo.bar().baz.buzz !== undefined && foo.bar().baz.buzz()

// chained calls with element access
foo !== undefined && foo.bar !== undefined && foo.bar.baz !== undefined && foo.bar.baz[buzz]()
foo !== undefined && foo.bar !== undefined && foo.bar.baz !== undefined && foo.bar.baz[buzz] !== undefined && foo.bar.baz[buzz]()

// (partially) pre-optional chained
foo !== undefined && foo?.bar !== undefined && foo?.bar.baz !== undefined && foo?.bar.baz[buzz] !== undefined && foo?.bar.baz[buzz]()
foo !== undefined && foo?.bar.baz !== undefined && foo?.bar.baz[buzz]
foo !== undefined && foo?.() !== undefined && foo?.().bar
foo.bar !== undefined && foo.bar?.() !== undefined && foo.bar?.().baz

// chained members
foo != null && foo.bar
foo.bar != null && foo.bar.baz
foo != null && foo()
foo.bar != null && foo.bar()
foo != null && foo.bar != null && foo.bar.baz != null && foo.bar.baz.buzz
foo.bar != null && foo.bar.baz != null && foo.bar.baz.buzz

// case with a jump (i.e. a non-nullish prop)
foo != null && foo.bar != null && foo.bar.baz.buzz
foo.bar != null && foo.bar.baz.buzz

// case where for some reason there is a doubled up expression
foo != null && foo.bar != null && foo.bar.baz != null && foo.bar.baz != null && foo.bar.baz.buzz
foo.bar != null && foo.bar.baz != null && foo.bar.baz != null && foo.bar.baz.buzz

// chained members with element access
foo != null && foo[bar] != null && foo[bar].baz != null && foo[bar].baz.buzz

// case with a jump (i.e. a non-nullish prop)
foo != null && foo[bar].baz != null && foo[bar].baz.buzz

// chained calls
foo != null && foo.bar != null && foo.bar.baz != null && foo.bar.baz.buzz()
foo != null && foo.bar != null && foo.bar.baz != null && foo.bar.baz.buzz != null && foo.bar.baz.buzz()
foo.bar != null && foo.bar.baz != null && foo.bar.baz.buzz != null && foo.bar.baz.buzz()

// case with a jump (i.e. a non-nullish prop)
foo != null && foo.bar != null && foo.bar.baz.buzz()
foo.bar != null && foo.bar.baz.buzz()

// case with a jump (i.e. a non-nullish prop)
foo != null && foo.bar != null && foo.bar.baz.buzz != null && foo.bar.baz.buzz()

// case with a call expr inside the chain for some inefficient reason
foo != null && foo.bar() != null && foo.bar().baz != null && foo.bar().baz.buzz != null && foo.bar().baz.buzz()

// chained calls with element access
foo != null && foo.bar != null && foo.bar.baz != null && foo.bar.baz[buzz]()
foo != null && foo.bar != null && foo.bar.baz != null && foo.bar.baz[buzz] != null && foo.bar.baz[buzz]()

// (partially) pre-optional chained
foo != null && foo?.bar != null && foo?.bar.baz != null && foo?.bar.baz[buzz] != null && foo?.bar.baz[buzz]()
foo != null && foo?.bar.baz != null && foo?.bar.baz[buzz]
foo != null && foo?.() != null && foo?.().bar
foo.bar != null && foo.bar?.() != null && foo.bar?.().baz

// chained members
foo != undefined && foo.bar
foo.bar != undefined && foo.bar.baz
foo != undefined && foo()
foo.bar != undefined && foo.bar()
foo != undefined && foo.bar != undefined && foo.bar.baz != undefined && foo.bar.baz.buzz
foo.bar != undefined && foo.bar.baz != undefined && foo.bar.baz.buzz

// case with a jump (i.e. a non-nullish prop)
foo != undefined && foo.bar != undefined && foo.bar.baz.buzz
foo.bar != undefined && foo.bar.baz.buzz

// case where for some reason there is a doubled up expression
foo != undefined && foo.bar != undefined && foo.bar.baz != undefined && foo.bar.baz != undefined && foo.bar.baz.buzz
foo.bar != undefined && foo.bar.baz != undefined && foo.bar.baz != undefined && foo.bar.baz.buzz

// chained members with element access
foo != undefined && foo[bar] != undefined && foo[bar].baz != undefined && foo[bar].baz.buzz

// case with a jump (i.e. a non-nullish prop)
foo != undefined && foo[bar].baz != undefined && foo[bar].baz.buzz

// chained calls
foo != undefined && foo.bar != undefined && foo.bar.baz != undefined && foo.bar.baz.buzz()
foo != undefined && foo.bar != undefined && foo.bar.baz != undefined && foo.bar.baz.buzz != undefined && foo.bar.baz.buzz()
foo.bar != undefined && foo.bar.baz != undefined && foo.bar.baz.buzz != undefined && foo.bar.baz.buzz()

// case with a jump (i.e. a non-nullish prop)
foo != undefined && foo.bar != undefined && foo.bar.baz.buzz()
foo.bar != undefined && foo.bar.baz.buzz()

// case with a jump (i.e. a non-nullish prop)
foo != undefined && foo.bar != undefined && foo.bar.baz.buzz != undefined && foo.bar.baz.buzz()

// case with a call expr inside the chain for some inefficient reason
foo != undefined && foo.bar() != undefined && foo.bar().baz != undefined && foo.bar().baz.buzz != undefined && foo.bar().baz.buzz()

// chained calls with element access
foo != undefined && foo.bar != undefined && foo.bar.baz != undefined && foo.bar.baz[buzz]()
foo != undefined && foo.bar != undefined && foo.bar.baz != undefined && foo.bar.baz[buzz] != undefined && foo.bar.baz[buzz]()

// (partially) pre-optional chained
foo != undefined && foo?.bar != undefined && foo?.bar.baz != undefined && foo?.bar.baz[buzz] != undefined && foo?.bar.baz[buzz]()
foo != undefined && foo?.bar.baz != undefined && foo?.bar.baz[buzz]
foo != undefined && foo?.() != undefined && foo?.().bar
foo.bar != undefined && foo.bar?.() != undefined && foo.bar?.().baz

//private static member name
foo && foo.#bar
foo.#bar && foo.#bar.#baz
foo.#bar && foo.#bar()
foo && foo.#bar && foo.#bar.#baz && foo.#bar.#baz.#buzz
foo.#bar && foo.#bar.#baz && foo.#bar.#baz.#buzz

// two  errors
foo && foo.bar && foo.bar.baz || baz && baz.bar && baz.bar.foo

// case with inconsistent checks
foo && foo.bar != null && foo.bar.baz !== undefined && foo.bar.baz.buzz;

foo.bar && foo.bar.baz != null && foo.bar.baz.qux !== undefined && foo.bar.baz.qux.buzz;

// ensure essential whitespace isn't removed
foo && foo.bar(baz => <This Requires Spaces />);
foo && foo.bar(baz => typeof baz);
foo && foo["some long string"] && foo["some long string"].baz
foo && foo[`some long string`] && foo[`some long string`].baz
foo && foo['some long string'] && foo['some long string'].baz;

// other literal expressions
foo && foo[123] && foo[123].baz;
foo && foo[true] && foo[true].baz;
foo && foo[null] && foo[null].baz;
foo && foo[12n] && foo[12n].baz;
foo && foo[/\w+/] && foo[/\w+/].baz;


// should preserve comments in a call expression
foo && foo.bar(/* comment */a,
	// comment2
	b, );

// ensure binary expressions that are the last expression do not get removed
foo && foo.bar != null;
foo && foo.bar != undefined;
foo && foo.bar != null && baz;

// other weird cases
foo && foo?.();
foo.bar && foo.bar?.();

// comments
foo && foo.bar && /*0*/foo/*1*/./*2*/bar/*3*/./*4*/baz/*5*/;
foo && foo[bar] && /*0*/foo/*1*/[/*2*/bar/*3*/]/*4*/[/*5*/baz/*6*/]/*7*/;

foo && foo[bar] && /*0*/foo/*1*/?./*2*/[/*3*/bar/*4*/]/*5*/?./*6*/[/*7*/baz/*8*/]/*9*/;
