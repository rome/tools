function MyComponent() {
    let local = 1;
    useEffect(() => {
      console.log(local);
    });
  }

// interaction with other react hooks

function MyComponent4() {
  const [name, setName] = useState(0);
  const [state, dispatch] = useReducer();
  const memoizedCallback = useCallback();
  const memoizedValue = useMemo();
  const deferredValue = useDeferredValue(value);
  const [isPending, startTransition] = useTransition();
  useEffect(() => {
      console.log(name);
      setName(1);

      console.log(state);
      dispatch(1);

      console.log(memoizedCallback);
      console.log(memoizedValue);
      console.log(deferredValue);

      console.log(isPending);
      startTransition();
  }, []);
}

// all hooks with dependencies

function MyComponent5() {
  let a = 1;
  useEffect(() => console.log(a));
  useCallback(() => console.log(a));
  useMemo(() => console.log(a));
  useImperativeHandle(ref, () => console.log(a));
  useLayoutEffect(() => console.log(a));
  useInsertionEffect(() => console.log(a));
}

// inner closures

function MyComponent6() {
  let a = 1;
  useEffect(() => {
      return () => console.log(a)
  }, []);
}

// same variable multiple times

function MyComponent7() {
  let a = 1;
  useEffect(() => {
    console.log(a);
    return () => console.log(a);
  }, []);
}
