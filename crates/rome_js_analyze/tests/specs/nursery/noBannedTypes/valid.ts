let f = Object();

let foo: { x: number; y: number } = { x: 1, y: 1 };

let g = Object.create(null);

let h = String(false);

let b: undefined;

let c: null;

let a: [];

let tuple: [boolean, string] = [true, "hello"];

type Props = {
	foo: string;
}

namespace X {
	// Allow user aliases
	type Number = number
	function f(): Number {
		return 0;
	}
}

type PhoneNumber = number | null | undefined;
type NonNullablePhoneNumber = PhoneNumber & {};

function consumeNonNullableValue<T extends {}>(value: T) {}
