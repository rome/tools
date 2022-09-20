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