import doSomething from 'a';

// No captures
function MyComponent1() {
    useEffect(() => {
    });
}

// All needed captures in the dependency list
function MyComponent2() {
    let a = 1;
    const b = 1;
    const c = a + 1;
    useEffect(() => {
        console.log(a, b, c);
    }, [a, c]);
}

// capturing declarations
function doSomething() { }
class A {}

function MyComponent3() {
    useEffect(() => {
        doSomething();
        console.log(new A ());
    }, []);
}

// interaction with other react hooks

function MyComponent4() {
    const [name, setName] = useState(0);
    const ref = useRef();
    const theme = useContext();
    const [state, dispatch] = useReducer();
    const memoizedCallback = useCallback();
    const memoizedValue = useMemo();
    const [isPending, startTransition] = useTransition();
    const id = useId();
    const externalStore = useSyncExternalStore();
    useEffect(() => {
        console.log(name);
        setName(1);

        console.log(ref);

        console.log(theme);

        console.log(state);
        dispatch(1)

        memoizedCallback();
        console.log(memoizedValue);

        console.log(isPending);
        startTransition();

        console.log(id);

        console.log(externalStore);
    }, [name, state, memoizedCallback, memoizedValue, isPending]);
}

// all hooks with dependencies

function MyComponent5() {
    let a = 1;
    useEffect(() => console.log(a), [a]);
    useCallback(() => console.log(a), [a]);
    useMemo(() => console.log(a), [a]);
    useImperativeHandle(ref, () => console.log(a), [a]);
    useLayoutEffect(() => console.log(a), [a]);
    useInsertionEffect(() => console.log(a), [a]);
}

// inner closures

function MyComponent5() {
    let a = 1;
    useEffect(() => {
        let b = 2;
        return () => console.log(a, b)
    }, [a]);
}

// from import 

function MyComponent6() {
    useEffect(() => {
        doSomething();
    });
}

