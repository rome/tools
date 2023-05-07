class Foo {
	// one decorator without a newline
	@decorator.method(value) property;
	@decorator.method(value) method() {}
	@decorator.method(value) async method() {}
	@decorator.method(value) *method() {}
	@decorator.method(value) get getter() {}
	@decorator.method(value) set setter(val) {}

	// two decorators without a newline
	@decorator.method(value) @decorator property;
	@decorator.method(value) @decorator method() {}
	@decorator.method(value) @decorator async method() {}
	@decorator @decorator.method(value) *method() {}
	@decorator @decorator.method(value) get getter() {}
	@decorator @decorator.method(value) set setter(val) {}

	// one decorator with a newline
	@decorator.method(value)
	property;
	@decorator
	method() {}
	@decorator.method(value)
	async method() {}
	@decorator
	*method() {}
	@decorator.method(value)
	get getter() {}
	@decorator.method(value)
	set setter(val) {}

	// two decorators without a newline
	@decorator
	@decorator.method(value) property;
	@decorator.method(value)
	@decorator method() {}
	@decorator.method(value)
	@decorator.method(value) async method() {}
	@decorator.method(value)
	@decorator *method() {}
	@decorator.method(value)
	@decorator.method(value) get getter() {}
	@decorator
	@decorator.method(value) set setter(val) {}
}


class Foo {
	// one decorator without a newline
	/*before*/ @decorator /*after*/ property;
	/*before*/ @decorator.method(value) /*after*/ method() {}
	/*before*/ @decorator.method(value) /*after*/ async method() {}
	/*before*/ @decorator /*after*/ *method() {}
	/*before*/ @decorator.method(value) /*after*/ get getter() {}
	/*before*/ @decorator /*after*/ set setter(val) {}

	// two decorators without a newline
	/*before*/ @decorator /*middle*/ @decorator.method(value) /*after*/ property;
	/*before*/ @decorator.method(value) /*middle*/ @decorator /*after*/ method() {}
	/*before*/ @decorator.method(value) /*middle*/ @decorator.method(value) /*after*/ async method() {}
	/*before*/ @decorator /*middle*/ @decorator.method(value) /*after*/ *method() {}
	/*before*/ @decorator /*middle*/ @decorator.method(value) /*after*/ get getter() {}
	/*before*/ @decorator.method(value) /*middle*/ @decorator /*after*/ set setter(val) {}

	// one decorator with a newline
	@decorator.method(value) /*before*/
		/*after*/ property;
	@decorator /*before*/
	/*after*/ method() {}
	@decorator.method(value) /*before*/
	/*after*/ async method() {}
	@decorator /*before*/
	/*after*/ *method() {}
	@decorator.method(value) /*before*/
	/*after*/ get getter() {}
	@decorator /*before*/
	/*after*/ set setter(val) {}

	// two decorators without a newline
	@decorator.method(value) /*before*/
	/*middle*/ @decorator /*after*/ property;
	@decorator.method(value) /*before*/
	/*middle*/ @decorator /*after*/ method() {}
	@decorator.method(value) /*before*/
	/*middle*/ @decorator /*after*/ async method() {}
	@decorator.method(value) /*before*/
	/*middle*/ @decorator /*after*/ *method() {}
	@decorator /*before*/
	/*middle*/ @decorator.method(value) /*after*/ get getter() {}
	@decorator.method(value) /*before*/
	/*middle*/ @decorator /*after*/ set setter(val) {}
}
