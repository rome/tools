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

const THIS_IS_OK = 1;
