import ReactDom, { render } from "react-dom";

ReactDom.render(<div />, document.body);
function render1() {
    ReactDom.render(<div />, document.body);
}
const render2 = () => {
    ReactDom.render(<div />, document.body);
}

render(<div />, document.body);
function render1() {
    render(<div />, document.body);
}
const render2 = () => {
    render(<div />, document.body);
}
