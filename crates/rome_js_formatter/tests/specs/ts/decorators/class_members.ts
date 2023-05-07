class Foo {
	// one decorator without a newline
	@dec public property;
	@dec public method() {}
	@dec public async method() {}
	@dec public *method() {}
	@dec public get getter() {}
	@dec public set setter(val) {}

	// two decorators without a newline
	@dec @dec public property;
	@dec @dec public method() {}
	@dec @dec public async method() {}
	@dec @dec public *method() {}
	@dec @dec public get getter() {}
	@dec @dec public set setter(val) {}

	// one decorator with a newline
	@dec
	public property;
	@dec
	public method() {}
	@dec
	public async method() {}
	@dec
	public *method() {}
	@dec
	public get getter() {}
	@dec
	public set setter(val) {}

	// two decorators without a newline
	@dec
	@dec public property;
	@dec
	@dec public method() {}
	@dec
	@dec public async method() {}
	@dec
	@dec public *method() {}
	@dec
	@dec public get getter() {}
	@dec
	@dec public set setter(val) {}
}


class Foo {
	// one decorator without a newline
	/*before*/ @dec /*after*/ public property;
	/*before*/ @dec /*after*/ public method() {}
	/*before*/ @dec /*after*/ public async method() {}
	/*before*/ @dec /*after*/ public *method() {}
	/*before*/ @dec /*after*/ public get getter() {}
	/*before*/ @dec /*after*/ public set setter(val) {}

	// two decorators without a newline
	/*before*/ @dec /*middle*/ @dec /*after*/ public property;
	/*before*/ @dec /*middle*/ @dec /*after*/ public method() {}
	/*before*/ @dec /*middle*/ @dec /*after*/ public async method() {}
	/*before*/ @dec /*middle*/ @dec /*after*/ public *method() {}
	/*before*/ @dec /*middle*/ @dec /*after*/ public get getter() {}
	/*before*/ @dec /*middle*/ @dec /*after*/ public set setter(val) {}

	// one decorator with a newline
	@dec /*before*/
		/*after*/ public property;
	@dec /*before*/
	/*after*/ public method() {}
	@dec /*before*/
	/*after*/ public async method() {}
	@dec /*before*/
	/*after*/ public *method() {}
	@dec /*before*/
	/*after*/ public get getter() {}
	@dec /*before*/
	/*after*/ public set setter(val) {}

	// two decorators without a newline
	@dec /*before*/
	/*middle*/ @dec /*after*/ public property;
	@dec /*before*/
	/*middle*/ @dec /*after*/ public method() {}
	@dec /*before*/
	/*middle*/ @dec /*after*/ public async method() {}
	@dec /*before*/
	/*middle*/ @dec /*after*/ public *method() {}
	@dec /*before*/
	/*middle*/ @dec /*after*/ public get getter() {}
	@dec /*before*/
	/*middle*/ @dec /*after*/ public set setter(val) {}
}
