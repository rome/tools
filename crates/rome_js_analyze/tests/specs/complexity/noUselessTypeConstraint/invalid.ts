interface FooAny1<T extends any> {
	field: T;
}

interface FooAny2<T extends unknown> {
	field: T;
}
