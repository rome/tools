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

class Foo {
	constructor(
		@inject(Bar)
		private readonly bar: IBar,

		@inject(MyProcessor)
		private readonly myProcessor: IMyProcessor,

		@inject(InjectionTypes.AnotherThing)

		private readonly anotherThing: IAnotherThing | undefined,
	) { }
}

export class TestTextFileService {
	constructor(
		@ILifecycleService lifecycleService,
	) {
	}
}

@commonEditorContribution
export class TabCompletionController {
}

@Component({
	selector: 'angular-component',
})
class AngularComponent {
	@Input() myInput: string;
}

class Class {
	method(
		@Decorator
			{ prop1, prop2 }: Type
	) {
		doSomething();
	}
}

class Class2 {
	method(
		@Decorator1
		@Decorator2
			{ prop1, prop2 }: Type
	) {
		doSomething();
	}
}

class Class3 {
	method(
		@Decorator
			{ prop1_1, prop1_2 }: Type,
		{ prop2_1, prop2_2 }: Type
	) {
		doSomething();
	}
}

class Class4 {
	method(
		param1,
		@Decorator
			{ prop1, prop2 }: Type
	) {}
}

class Class5 {
	method(
		@Decorator { prop1 }: Type
	) {}
}

class Class6 {
	method(
		@Decorator({}) { prop1 }: Type
	) {}
	method(
		@Decorator(
			{}) { prop1 }: Type
	) {}
	method(
		@Decorator([]) { prop1 }: Type
	) {}
	method(
		@Decorator(
			[]) { prop1 }: Type
	) {}
}


@d1
@d2(foo)
@d3.bar
@d4.baz()
class Class1 {}

class Class2 {
	@d1
	@d2(foo)
	@d3.bar
	@d4.baz()
	method1() {}

	@d1
	method2() {}

	@d2(foo)
	method3() {}

	@d3.bar
	method4() {}
}

class Class3 {
	@d1 fieldA;
	@d2(foo) fieldB;
	@d3.bar fieldC;
	@d4.baz() fieldD;

	constructor (
		@d1 private x: number,
		@d2(foo) private y: number,
		@d3('foo') private z: number,
		@d4({
			x: string
		}) private a: string,
	) {}
}

@decorated class Foo {}

class Bar {
	@decorated method() {}
}

class MyContainerComponent {
	@ContentChildren(MyComponent) components: QueryListSomeBigName<MyComponentThat>;
}

@sealed
class Test {
	@readonly
	prop: string;

	constructor(@param test, @readonly private other, @aVeryLongDecoratorNameLetsSeeWhatHappensWithIt last) {}

	method(@param a) {}

	get getter() {}

	set setter(@param param) {}
}

class Class4 {
	method(
		@Decorator
			{ prop1_1, prop1_2 }: Type = {},
		{ prop2_1, prop2_2 }: Type
	) {
		doSomething();
	}
}

class Class {
	method(@aVeryLongDecoratorNameLetsSeeWhatHappensWithIt last, @d2(foo) y: number,) {

	}
}


class Class2 {
	constructor(@param test, @aVeryLongDecoratorNameLetsSeeWhatHappensWithIt last, @d2(foo) y: number,) {}

	method(@aVeryLongDecoratorNameLetsSeeWhatHappensWithIt last, @d2(foo) y: number,) {

	}
}

class Class3 {
	constructor (
		private aaaaaaaaaaaaaaaaa: {aaaaaaaaaaaaa: number; bbbbbbbbbbb: string; cccccccccccc: Type},
		@d1 private readonly x: number,
		@d2(foo) private y: number,
		@d3('foo') private z: number,

	) {}

}

// comments
class Foo {
	constructor(
		//leading own line
		/*leading same line*/ @Decorator /*trailing*/
		//leading own line between
		/*leading same line between*/ @dec //trailing
			/*leading parameter*/
			parameter: number
	) {}

	method(
	//leading own line
	/*leading same line*/ @Decorator /*trailing*/
	//leading own line between
	/*leading same line between*/ @dec //trailing
	/*leading parameter*/
	parameter
	) {}
}
