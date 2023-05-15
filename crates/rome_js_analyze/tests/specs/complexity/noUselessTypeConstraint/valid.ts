interface FooAny0<T> {
	field: T;
}

interface FooNotAny0<T extends string> {
	field: T;
}
