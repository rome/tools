---
title: Credits
layout: layouts/page.liquid
---

# Credits

## Team Alumni

<ul class="credits-people-list">
	<li>
		<a href="https://github.com/EduardoLopes">
			<img src="https://github.com/EduardoLopes.png?s=176">
			<span>Eduardo Lopes</span>
		</a>
	</li>
	<li>
		<a href="https://github.com/ooflorent">
			<img src="https://github.com/ooflorent.png?s=176">
			<span>Florent Cailhol</span>
		</a>
	</li>
	<li>
		<a href="https://github.com/jamiebuilds">
			<img src="https://github.com/jamiebuilds.png?s=176">
			<span>Jamie Kyle</span>
		</a>
	</li>
	<li>
		<a href="https://github.com/jer3m01">
			<img src="https://github.com/jer3m01.png?s=176">
			<span>Jeremy Bagnall</span>
		</a>
	</li>
	<li>
		<a href="https://github.com/Kelbie">
			<img src="https://github.com/Kelbie.png?s=176">
			<span>Kevin Kelbie</span>
		</a>
	</li>
	<li>
		<a href="https://github.com/diokey">
			<img src="https://github.com/diokey.png?s=176">
			<span>Olivier Dusabimana</span>
		</a>
	</li>
	<li>
		<a href="https://github.com/bitpshr">
			<img src="https://github.com/bitpshr.png?s=176">
			<span>Paul Bouchon</span>
		</a>
	</li>
	<li>
		<a href="https://github.com/VictorHom">
			<img src="https://github.com/VictorHom.png?s=176">
			<span>Victor Hom</span>
		</a>
	</li>
	<li>
		<a href="https://github.com/yeonjuan">
			<img src="https://github.com/yeonjuan.png?s=176">
			<span>Yeon Juan</span>
		</a>
	</li>
</ul>

{% include ./contributors.md %}

## Acknowledgements

Rome contains code that is heavily inspired from other projects. They have been adapted to Rome's
language/infrastructure.

- [Prettier](https://github.com/prettier/prettier/)
  - [LICENSE](https://github.com/rome/tools/blob/main/crates/rome_js_formatter/LICENSE)

## Forks

Rome contains code forked from other projects. They have been transformed in some way, sometimes
substantially rewritten.

- [`crates/rome_diagnostics`](https://github.com/rome/tools/tree/main/crates/rome_diagnostics)
  - **Original**: [`rslint/rslint_errors`](https://github.com/rslint/rslint/tree/master/crates/rslint_errors)
  - **License**: MIT

- [`crates/rome_console/src/codespan`](https://github.com/rome/tools/tree/main/crates/rome_console/src/codespan)
  - **Original**: [`brendanzab/codespan`](https://github.com/brendanzab/codespan)
  - **License**: Apache License, Version 2.0

- [`crates/rome_js_parser`](https://github.com/rome/tools/tree/main/crates/rome_js_parser)
  - **Original**: [`rslint/rslint_parser`](https://github.com/rslint/rslint/tree/master/crates/rslint_parser)
  - **License**: MIT

- [`crates/rome_js_parser/lexer`](https://github.com/rome/tools/tree/main/crates/rome_js_parser/src/lexer)
  - **Original**:  [`rslint/rslint_lexer`](https://github.com/rslint/rslint/tree/master/crates/rslint_lexer)
  - **License**: MIT

- [`crates/rome_js_syntax`](https://github.com/rome/tools/tree/main/crates/rome_js_syntax)
	- **Original**: [`rslint/rslint_syntax`](https://github.com/rslint/rslint/tree/master/crates/rslint_syntax)
	- **License**: MIT

- [`crates/rome_text_edit`](https://github.com/rome/tools/tree/main/crates/rome_text_edit)
	- **Original**: [`rslint/rslint_text_edit`](https://github.com/rslint/rslint/tree/master/crates/rslint_text_edit)
	- **License**: MIT

- [`crates/rome_rowan`](https://github.com/rome/tools/tree/main/crates/rome_rowan)
	- **Original**: [`rust-analyzer/rowan`](https://github.com/rust-analyzer/rowan)
	- **License**: Apache License, Version 2.0

- [`crates/rome_text_size`](https://github.com/rome/tools/tree/main/crates/rome_text_size)
  - **Original**: [`rust-analyzer/text-size`](https://github.com/rust-analyzer/text-size)
  - **License**: Apache License, Version 2.0 or MIT

- [`crates/rome_service/src/ignore/pattern`](https://github.com/rome/tools/tree/main/crates/rome_service/src/ignore/pattern)
    - **Original**: [`rust-lang/glob`](https://github.com/rust-lang/glob/blob/master/src/lib.rs)
    - **License**: Apache License, Version 2.0 or MIT
