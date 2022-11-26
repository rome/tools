/* Options: {"hooks": [["useMyEffect", 0, 1]]} */

function MyComponent() {
    let a = 1;
    useMyEffect(() => {
        console.log(a);
    });
}