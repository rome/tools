use rome_formatter_test::check_reformat::CheckReformat;
use rome_js_formatter::context::{JsFormatOptions, Semicolons};
use rome_js_formatter::format_node;
use rome_js_parser::parse;
use rome_js_syntax::SourceType;

mod language {
    include!("language.rs");
}

#[ignore]
#[test]
// use this test check if your snippet prints as you wish, without using a snapshot
fn quick_test() {
    let src = r#"
import { LitElement, html } from '@polymer/lit-element';

class MyElement extends LitElement {
  static get properties() {
    return {
      mood: { type: String }
    };
  }

  constructor() {
    super();
    this.mood = 'happy';
  }

  render() {
    return html`
      <style


      >
                  .mood { color: green; }
      </style



      >

         Web            Components         are     <span


      class="mood"      >${
        this.mood

      }</span

           >!
    `;
  }
}

customElements.define('my-element', MyElement);

const someHtml1 = html`<div       > hello ${world} </div     >`;
const someHtml2 = /* HTML */ `<div      > hello ${world} </div     >`;

html``

html`<my-element obj=${obj}></my-element>`;

html`  <${Footer}  >footer      content<//     >  `

html`  <div />  `

html`
  <div />
`

html`<span>one</span><span>two</span><span>three</span>`;

function HelloWorld() {
  return html`
    <h3>Bar List</h3>
    ${bars.map(bar => html`
       <p>${bar}</p>
    `)}
  `;
}

const trickyParens = html`<script> f((${expr}) / 2); </script>`;
const nestedFun = /* HTML */ `${outerExpr( 1 )} <script>const tpl = html\`<div>\${innerExpr( 1 )} ${outerExpr( 2 )}</div>\`</script>`;

const closingScriptTagShouldBeEscapedProperly = /* HTML */ `
  <script>
    const html = /* HTML */ \`<script><\\/script>\`;
  </script>
`;

const closingScriptTag2 = /* HTML */ `<script>const  scriptTag='<\\/script>'; <\/script>`;

html`
 <div style="
 ${ foo}
"></div>
`
html`
 <div style=${
  foo
 }></div>
`

html`<div style="   color : red;
            display    :inline ">
  </div>`

html`<div style="   color : red;
${ foo}
            display    :inline ">
  </div>`
html`<div style="   color : red;
${ foo}:${bar};
            display    :inline ">
  </div>`

"#;
    let syntax = SourceType::tsx();
    let tree = parse(src, syntax);
    let options = JsFormatOptions::new(syntax).with_semicolons(Semicolons::AsNeeded);
    let result = format_node(options.clone(), &tree.syntax())
        .unwrap()
        .print()
        .unwrap();

    let root = &tree.syntax();
    let language = language::JsTestFormatLanguage::new(SourceType::tsx());
    let check_reformat =
        CheckReformat::new(root, result.as_code(), "quick_test", &language, options);
    check_reformat.check_reformat();

    assert_eq!(
        result.as_code(),
        r#"[
	5,
	7234932436,
    // comment 3
];
"#
    );
}
