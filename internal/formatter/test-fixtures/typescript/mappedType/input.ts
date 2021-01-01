type Mapped<T> = {
	[K in keyof T ]: T[K]
}
type KeyRemapped<T> = {
	[K in keyof T as  NewKeyType ]: T[K]
}
