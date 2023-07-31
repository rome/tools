const foo = `
text
${a == b}
`;

// existing comment
a == b;

if (a == b) {
    false;
}

if (/** some weird comment
    **/ a == b) {

    }

let a = `Output of "rome rage":
  formatter enabled: ${formatter == true}
  linter: ${linter}
`;