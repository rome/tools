import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"react no access state in set state",
	async (t) => {
		await testLint(
			t,
			{
				// INVALID
				invalid: [
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
						value: !this.state.value
					});
				`,
					`
					this.setState({
						value: !!this.state.value
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
				],
				valid: [
					`
					this.setState({
						foo: bar
					});
				`,
					`
					function increment() {
						this.setState(prevState => ({value: prevState.value + 1}));
					}
				`,
				],
			},
			{category: "lint/react/noAccessStateInSetState"},
		);
	},
);
