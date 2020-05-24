import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"react no will update set state",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				`
        class Hello extends React.Component {
          componentWillUpdate() {
            this.setState({
              name: 'John'
            });
          }
        }
        `,
				`
        class Hello extends React.Component {
          componentWillUpdate() {
            foo();
            this.setState({
              name: 'John'
            });
          }
        }
        `,
				`
        class Hello extends Component {
          componentWillUpdate() {
            this.setState({
              name: 'John'
            });
          }
        }
        `,
				`
        class Hello extends Component {
          componentWillUpdate() {
            foo();
            this.setState({
              name: 'John'
            });
          }
        }
        `,
				`
        class Hello extends React.Component {
          UNSAFE_componentWillUpdate() {
            this.setState({
              name: 'John'
            });
          }
        }
        `,
				`
        class Hello extends Component {
          UNSAFE_componentWillUpdate() {
            this.setState({
              name: 'John'
            });
          }
        }
        `,
				// VALID
				`
        class Hello extends React.Component {
          componentWillUpdate() {
            if (condition) {
              this.setState({
                name: 'John'
              });
            }
          }
        }
				`,
				`
        class Hello extends React.Component {
          componentWillUpdate() {
            condition && this.setState({
							name: 'John'
						});
          }
        }
        `,
				`
        class Hello extends React.Component {
          componentWillUpdate() {
            condition ? this.setState({
							name: 'John'
						}) : undefined;
          }
        }
        `,
			],
			{category: "lint/react/noWillUpdateSetState"},
		);
	},
);
