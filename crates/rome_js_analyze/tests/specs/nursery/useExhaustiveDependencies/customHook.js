function MyComponent() {
    let a = 1;
    useEffect(() => {
        console.log(a);
    });
    useMyEffect(() => {
        console.log(a);
    });
}