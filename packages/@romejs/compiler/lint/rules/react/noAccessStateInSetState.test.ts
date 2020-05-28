import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"react no access state in set state",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				`
					function increment() {
						this.setState({value: this.state.value + 1});
					}
				`,
				`
					function increment() {
						this.setState({value: 1 + this.state.value});
					}
				`,
				`
					this.setState({
						value: this.state.value + 1
					});
				`,
				`
					this.setState({
						value: 1 + this.state.value
					});
				`,
				`
					this.setState({
						foo: bar,
						value: 1 + this.state.value
					});
				`,
				`
					this.setState({
						foo: bar,
						value: this.state.value + 1
					});
				`,
				`
					this.setState({
						value: this.state.value
					});
				`,
				`
					this.setState({
						foo: bar,
						value: this.state.value
					});
				`,
				// VALID
				`
					this.setState({
						foo: bar
					});
				`,
				`
					function increment() {
						this.setState(prevState => ({value: prevState.value + 1}));
					}
				`
			],
			{category: "lint/react/noAccessStateInSetState"},
		);
	},
);
