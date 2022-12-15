// invalid

{
  function baz() { ; }
}

{
  function buzz() {
    const x = 10;;
  }
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