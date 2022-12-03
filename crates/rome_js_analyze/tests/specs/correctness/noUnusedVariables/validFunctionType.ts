/* should not generate diagnostics */

// `title` and 'args' are not used, but they are ok because they are inside
// of function type definitions

function f(fn: (title: string) => boolean) {
	console.log(fn);
}
f((x) => x == "");

export type Command = (...args: any[]) => unknown;


// https://github.com/rome/tools/issues/3669
// `a` and `b` are fine because they are inside a function type definition
interface Props {
	fn: ({ a, b }: { a: number; b: string }) => void;
}

console.log({
	fn: ({ a, b }: { a: number; b: string }) => console.log(a, b),
} as Props);
