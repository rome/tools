switch (foo) {
    case true:
    case false:
        let foo = '';
        foo;
}

switch (foo) {
    // comment
    case false :
        let foo = '';
        foo;
}

switch (foo) {
    case false : // comment
        let foo = '';
        foo;
}

switch (foo) {
    case false
    /* comment */ :
        let foo = '';
        foo;
}

switch (foo) {
    case true:
    case false:
        'yes';
}

switch (foo) {
    case true: {
        // empty
    }
}

switch (foo) {
    case true:
}
