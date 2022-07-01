function no_delete() {
    let a = { b: 1 };
    delete a.b;
}

function no_arguments_1() {
    console.log(arguments);
}

function no_arguments_2() {
    let arguments = 1;
    console.log(arguments);
}