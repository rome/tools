import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"react no did update set state",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				`
        class Hello extends React.Component {
          componentDidUpdate() {
            this.setState({
              name: 'John'
            });
          }
        }
        `,
				`
        class Hello extends React.Component {
          componentDidUpdate() {
            foo();
            this.setState({
              name: 'John'
            });
          }
        }
        `,
				`
        class Hello extends Component {
          componentDidUpdate() {
            this.setState({
              name: 'John'
            });
          }
        }
        `,
				`
        class Hello extends Component {
          componentDidUpdate() {
            foo();
            this.setState({
              name: 'John'
            });
          }
        }
        `,
				// VALID
				`
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
				`
        class Hello extends React.Component {
          componentDidUpdate() {
            condition && this.setState({
							name: 'John'
						});
          }
        }
        `,
				`
        class Hello extends React.Component {
          componentDidUpdate() {
            condition ? this.setState({
							name: 'John'
						}) : undefined;
          }
        }
        `,
			],
			{category: "lint/react/noDidUpdateSetState"},
		);
	},
);
