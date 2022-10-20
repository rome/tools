function MyComponent() {
  let a = 1;
  useEffect(() => {}, [a]);
}

// multiple extra dependencies

function MyComponent2() {
  let a = 1, b = 1;
  useEffect(() => {}, [a, b]);
}
