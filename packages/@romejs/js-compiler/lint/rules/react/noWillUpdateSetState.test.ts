import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"no this.setState in componentWillUpdate",
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
          UNSAFE_componentWillUpdate() {
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
          componentWillUpdate() {
						foo()
          }
        }
				`,
				`
        class Hello extends React.Component {
          UNSAFE_componentWillUpdate() {
						foo()
          }
        }
        `,
			],
			{category: "lint/noWillUpdateSetState"},
		);
	},
);
