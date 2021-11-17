class Getters {
	get foo() {}
	get static() {}
	static get bar() {}

	get "baz"() {}

	get ["a" + "b"]() {}

	get 5() {}
}

class NotGetters {
	get() {}
	async get() {}
	static get() {}
}

