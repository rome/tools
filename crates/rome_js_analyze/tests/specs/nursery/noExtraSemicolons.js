// invalid

{
  function baz() { ; }
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

// valid

{
  function baz() {}
}

{
  for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);
}

{
  const x = 5;
}

{
  const foo = function() {
      // code
  };
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
  }
}