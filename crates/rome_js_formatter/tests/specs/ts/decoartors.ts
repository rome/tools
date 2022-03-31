@sealed
class Test {
	@readonly
	prop: string;

	constructor(@param test, @readonly private other, @aVeryLongDecoratorNameLetsSeeWhatHappensWithIt last) {}

	method(@param a) {}

	get getter() {}

	set setter(@param param) {}
}

@sealed
export default class {}

@sealed
export class Test {}

// Leading comment before decorator
@test // first decorator
// Leading comment before class
class Test2 {





	/*
	 * Leading multiline comment
	 */



	@test /* trailing multiline comment
	 for decorator */ @anotherDecorator()




		// leading comment
	prop: string;

}

