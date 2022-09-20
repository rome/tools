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