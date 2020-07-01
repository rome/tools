import {test} from "rome";
import {testLint} from "../../utils/testing";
import {dedent} from "@romejs/string-utils";

test(
	"react state in constructor",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						class Foo extends React.Component {
				            state = { bar: 0 }
				            render() {
				                return <div>Foo</div>
				            }
				        }
					`,
				],
				valid: [
					dedent`
						class Foo extends React.Component {
                            constructor(props) {
                                super(props)
                                this.state = { bar: 0 }
                            }
                            render() {
                                return <div>Foo</div>
                            }
                        }
					`,
					dedent`
						class Foo extends Component {
							constructor() {
								this.state = { text: "foo" }
							}

							render() {
							const { state } = this;
							const { text } = state;
							return (
								<span>{text}</span>
							)
							}	
						}
					`,
				],
			},
			{category: "lint/react/stateInConstructor"},
		);
	},
);
