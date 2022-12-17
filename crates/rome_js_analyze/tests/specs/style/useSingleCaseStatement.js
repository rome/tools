switch (foo) {
    case true:
    case false:
        let foo = '';
        foo;
}

switch (foo) { case false: let foo = ''; foo; }

switch (foo) {
    // comment
    case false:
        let foo = '';
        foo;
}

switch (foo) {
    case false: // comment
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

switch (foo) {
    case true:
    default:
        let foo = '';
        foo;
}

switch (foo) {
    // comment
    default:
        let foo = '';
        foo;
}

switch (foo) {
    default: // comment
        let foo = '';
        foo;
}

switch (foo) {
    default
    /* comment */ :
        let foo = '';
        foo;
}

switch (foo) {
    case true:
    default:
        'yes';
}

switch (foo) {
    default: {
        // empty
    }
}

switch (foo) {
    default:
}
