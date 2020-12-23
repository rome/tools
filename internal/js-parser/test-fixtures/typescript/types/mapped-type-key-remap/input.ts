type MappedType<T> = {
	[K in keyof T as NewKeyType]: T[K]
}
