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