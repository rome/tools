function MyComponent1() {
    useEffect(() => {
    });
}

function MyComponent2() {
    const local = 1;
    useEffect(() => {
        console.log(local);
    }, [local]);
}