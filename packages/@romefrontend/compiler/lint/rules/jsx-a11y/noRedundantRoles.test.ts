import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"jsx-a11y no redundant roles",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<article role='article'></article>",
					"<button role='button'></button>",
					"<h1 role='heading' aria-level='1'></h1>",
					"<h1 role='heading'></h1>",
					"<dialog role='dialog'></dialog>",
					"<input  type='checkbox' role='checkbox' />",
					"<figure  role='figure'></figure>",
					"<form  role='form'></form>",
					"<table  role='grid'></table>",
					"<td  role='gridcell'></td>",
					"<fieldset  role='group'></fieldset>",
					"<img src='foo' alt='bar'  role='img' />",
					"<a role='link'></a>",
					"<link role='link' />",
					"<ol role='list' ></ol>",
					"<ul role='list' ></ul>",
					"<select role='listbox' ></select>",
					"<li role='listitem' ></li>",
					"<nav role='navigation' ></nav>",
					"<option role='option' ></option>",
					"<tr role='row' ></tr>",
					"<tbody role='rowgroup' ></tbody>",
					"<tfoot role='rowgroup' ></tfoot>",
					"<thead role='rowgroup' ></thead>",
					"<th scope='row' role='rowheader' ></th>",
					"<input type='search' role='searchbox' />",
					"<table role='table' ></table>",
					"<dt role='term' ></dt>",
					"<textarea role='textbox' ></textarea>",
					"<input type='text' role='textbox' />",
				],
				valid: [
					"<article role='presentation' ></article>",
					"<Button role='button' ></Button>",
					"<span></span>",
				],
			},
			{category: "lint/jsx-a11y/noRedundantRoles"},
		);
	},
);
