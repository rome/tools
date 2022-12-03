/* does not generate diagnostics */

function Component1({ a }) {
    useEffect();

    {
        useEffect();
    }
}

// Hook called indirectly
function helper() {
    useEffect();
}

function Component2({a}) {
    helper();
}