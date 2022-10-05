function MyComponent() {
    const local = 1;
    useEffect(() => {
      console.log(local);
    });
  }