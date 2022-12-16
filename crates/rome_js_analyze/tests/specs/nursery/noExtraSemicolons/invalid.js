// invalid

{
  const x = 10;
  if (x > 5) {
    x = 5;
  };
}

{
  while(true);;
}

{
  while(true) {};
}

{
  loop1:
  for (let i = 0; i < 5; i++) {
    str = str + i;;
  };
}

{
  loop1:
  for (let i = 0; i < 5; i++) {
    if (i === 1) {
      continue loop1;
    }
    str = str + i;
  };
}

{
  function baz() { ; }
}

{
  function buzz() {
    const x = 10;;
  }
}

{
  for(;true;);;
}

{
  for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);;
}

{
  const x = 5;;
}

{
  function foo() {
      // code
  };
}

{
  class C {
      field;;
  
      method() {
          // code
      }
  
      static {
          // code
      }
  }
}

{
  class C {
      field;
  
      method() {
          // code
      };
  
      static {
          // code
      }
  }
}

{
  class C {
      field;
  
      method() {
          // code
      }
  
      static {
          // code
      };
  }
}

{
  class C {
      field;
  
      method() {
          // code
      }
  
      static {
          // code
      }
  };
}