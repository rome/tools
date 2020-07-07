import {Number0, ob1Add, ob1Get} from "@romejs/ob1";

export function getChar(index: Number0, input: string, offset = 0): string {
	const targetIndex = ob1Get(index) + offset;
	return input[targetIndex];
}

export function consumeComment(index: Number0, input: string): [Number0, string] {
	let value = "";
	while (getChar(index, input) !== ">") {
		const char = getChar(index, input);
		if (char !== "-") {
			value += getChar(index, input);
		}
		index = ob1Add(index, 1);
	}

	return [ob1Add(index, 1), value.trim()];
}
