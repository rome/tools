class Test {
	method() {}
	async asyncMethod() {}
	async* asyncGeneratorMethod() {}
	* generatorMethod() {}
	"foo"() {}
	["foo" + "bar"]() {}
	5() {}
	#private() {}
}
class ContextualKeywords {
	// Methods called static
	static() {}
	async static() {}
	* static() {}
	async* static() {}
