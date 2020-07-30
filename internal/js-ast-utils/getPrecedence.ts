import {BinaryOperator, LogicalOperator} from "@internal/ast";

const PRECEDENCE = {
	"||": 0,
	"&&": 1,
	"??": 1,
	"|": 2,
	"^": 3,
	"&": 4,
	"==": 5,
	"===": 5,
	"!=": 5,
	"!==": 5,
	"<": 6,
	">": 6,
	"<=": 6,
	">=": 6,
	in: 6,
	instanceof: 6,
	">>": 7,
	"<<": 7,
	">>>": 7,
	"+": 8,
	"-": 8,
	"*": 9,
	"/": 9,
	"%": 9,
	"**": 10,
};

export function getPrecedence(
	operator: BinaryOperator | LogicalOperator,
): number {
	return PRECEDENCE[operator];
}
