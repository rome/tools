
function lint_problems() {
    let a = { b: 1 };
    delete a.b;
    console.log(arguments);

    let arguments = 1;
    console.log(arguments);
}