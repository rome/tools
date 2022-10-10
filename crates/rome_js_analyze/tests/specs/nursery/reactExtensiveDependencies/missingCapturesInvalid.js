function MyComponent() {
    const local = 1;
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