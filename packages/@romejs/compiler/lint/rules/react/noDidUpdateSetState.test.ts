import {test} from "rome";
import {testLint} from "../testHelpers";
import {dedent} from "@romejs/string-utils";

test(
	"react no did update set state",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						class Hello extends React.Component {
							componentDidUpdate() {
								this.setState({
									name: 'John'
								});
							}
						}
					`,
					dedent`
						class Hello extends React.Component {
							componentDidUpdate() {
								foo();
								this.setState({
									name: 'John'
								});
							}
						}
					`,
					dedent`
						class Hello extends Component {
							componentDidUpdate() {
								this.setState({
									name: 'John'
								});
							}
						}
					`,
					dedent`
						class Hello extends Component {
							componentDidUpdate() {
								foo();
								this.setState({
									name: 'John'
								});
							}
						}
					`,
				],
				valid: [
					dedent`
						class Hello extends React.Component {
							componentDidUpdate() {
								if (condition) {
									this.setState({
										name: 'John'
									});
								}
							}
						}
					`,
					dedent`
						class Hello extends React.Component {
							componentDidUpdate() {
								condition && this.setState({
									name: 'John'
								});
							}
						}
					`,
					dedent`
						class Hello extends React.Component {
							componentDidUpdate() {
								condition ? this.setState({
									name: 'John'
								}) : undefined;
							}
						}
					`,
				],
			},
			{category: "lint/react/noDidUpdateSetState"},
		);
	},
);
