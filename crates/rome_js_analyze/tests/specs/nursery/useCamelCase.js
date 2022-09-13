let snake_case;
snake_case = 1;
let _snake_case;
console.log(_snake_case);

function snake_function(snake_case, PascalCase) {}

class PascalCase {
	snake_property = 1;
	#private_snake_property = 2;

	snake_function() {}

	get snake_getter() {}
	set snake_setter(v) {
		console.log(v);
	}
}

console.log({
	snake_function() {},
	get snake_getter() {},
	set snake_setter(v) {
		console.log(v);
	},
});

let camelCase;
let longCamelCase;

let UPPER_CASE = 1;
let { UPPER_CASE } = env;
let [ UPPER_CASE ] = env;

const THIS_IS_OK = 1;
const { THIS_IS_OK } = env;
const [ THIS_IS_OK ] = env;

function PascalCaseOkBecauseNew() { }
console.log(new PascalCaseOkBecauseNew());

function PascalCaseOkBecauseExport() { }
export default PascalCaseOkBecauseExport;

function PascalCaseNOk() { }
console.log(PascalCaseNOk());
