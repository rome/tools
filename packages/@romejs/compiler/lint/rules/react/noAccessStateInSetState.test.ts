import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"react no access state in set state",
	async (t) => {
		await testLint(
			t,
			{
				// INVALID
				invalid: [
					`
					class MyComponent extends Component {
						function increment() {
							this.setState({value: this.state.value + 1});
						}
					}
				`,
					`
					class MyComponent extends Component {
						function increment() {
							this.setState({value: 1 + this.state.value});
						}
					}
				`,
					`
					class MyComponent extends Component {
						function toggle() {
							this.setState({
								value: !this.state.value
							});
						}
					}
				`,
					`
					class MyComponent extends Component {
						function toggle() {
							this.setState({
								value: !!this.state.value
							});
						}
					}
				`,
					`
					class MyComponent extends Component {
						function update() {
							this.setState({
								foo: bar,
								value: 1 + this.state.value
							});
						}
					}
				`,
					`
					class MyComponent extends Component {
						function update() {
							this.setState({
								foo: bar,
								value: this.state.value + 1
							});
						}
					}
				`,
					`
					class MyComponent extends Component {
						function update() {
							this.setState({
								value: this.state.value
							});
						}
					}
				`,
					`
					class MyComponent extends Component {
						function update() {
							this.setState({
								foo: bar,
								value: this.state.value
							});
						}
					}
				`,
				],
				valid: [
					`
					class MyComponent extends Component {
						function update() {
							this.setState({
								foo: bar
							});
						}
					}
				`,
					`
					class MyComponent extends Component {
						function increment() {
							this.setState(prevState => ({value: prevState.value + 1}));
						}
					}
				`,
				],
			},
			{category: "lint/react/noAccessStateInSetState"},
		);
	},
);
