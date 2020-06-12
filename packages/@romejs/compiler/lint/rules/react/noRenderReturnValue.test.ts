import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"react no render return value",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"const foo = ReactDOM.render(<div />, document.body);",
					"const foo = bar && ReactDOM.render(<div />, document.body);",
					"const foo = bar ? ReactDOM.render(<div />, document.body) : null",
					"const foo = () => ReactDOM.render(<div />, document.body);",
					`
					const foo = {
						react: ReactDOM.render(<div />, document.body)
					};`,
					`
					let foo;
					foo = ReactDOM.render(<div />, document.body);
					`,
					`
					function render () {
						return ReactDOM.render(<div />, document.body)
					}`,
				],
				valid: [
					"ReactDOM.render(<div />, document.body);",
					`
					function render () {
						ReactDOM.render(<div />, document.body);
					}`,
					`
					const render = () => {
						ReactDOM.render(<div />, document.body);
					}`,
				],
			},
			{category: "lint/react/noRenderReturnValue"},
		);
	},
);
