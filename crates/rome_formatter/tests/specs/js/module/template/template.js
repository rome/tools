`something`;

tag`something`
`something ${ " hello" }`;

`something ${ () => { var hey; const looooooooooong_expression = "loooooooooong_expression" }} something else ${ ehy }`;
    `something ${ () => { var hey; const looooooooooong_expression = "loooooooooong_expression"; return hey; }} something else ${ ehy }`;


`test
  abcd ${input}
output
`;

`test
  abcd ${ () => { var hey; const looooooooooong_expression = "loooooooooong_expression"; return hey; }}
output
`;