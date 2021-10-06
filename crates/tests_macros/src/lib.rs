use case::CaseExt;
use globwalk::{GlobWalker, GlobWalkerBuilder};
use proc_macro::TokenStream;
use proc_macro_error::*;
use quote::*;
use std::path::{Path, PathBuf};
use syn::parse::ParseStream;

struct Arguments {
	pattern: syn::ExprLit,
	f: syn::Path,
}

struct Variables {
	test_name: String,
	test_full_path: String,
	test_expected_fullpath: String,
}

struct AllFiles(GlobWalker);

impl Iterator for AllFiles {
	type Item = Result<PathBuf, &'static str>;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.0.next() {
				Some(Ok(entry)) => {
					let file_name = match entry.file_name().to_str().ok_or("File name not UTF8") {
						Ok(v) => v,
						Err(e) => return Some(Err(e)),
					};

					if file_name.contains("expected") {
						continue;
					}
					let meta = match entry.metadata().map_err(|_| "Cannot open file") {
						Ok(v) => v,
						Err(e) => return Some(Err(e)),
					};
					if meta.is_file() {
						let path = entry.path().to_path_buf();
						break Some(Ok(path));
					}
				}
				_ => break None,
			}
		}
	}
}

impl Arguments {
	fn get_all_files(&self) -> Result<AllFiles, &str> {
		let base = std::env::var("CARGO_MANIFEST_DIR")
			.map_err(|_| "Cannot find CARGO_MANIFEST_DIR. Are you using cargo?")?;
		let glob = match &self.pattern.lit {
			syn::Lit::Str(v) => v.value(),
			_ => return Err("Only string literals supported"),
		};
		let walker = GlobWalkerBuilder::new(base, &glob)
			.build()
			.map_err(|_| "Cannot walk the requested glob")?;

		Ok(AllFiles(walker))

		// for entry in walker.flatten() {
		// 	let file_name = entry.file_name().to_str().ok_or("File name not UTF8")?;

		// 	if file_name.contains("expected") {
		// 		continue;
		// 	}
		// 	let meta = entry.metadata().map_err(|_| "Cannot open file")?;
		// 	if meta.is_file() {
		// 		files.push(entry.path().to_path_buf());
		// 	}
		// }

		// Ok(files)
	}

	fn get_variables<P: AsRef<Path>>(path: P) -> Option<Variables> {
		let path = path.as_ref();
		let file_stem = path.file_stem()?;
		let file_stem = file_stem.to_str()?;
		let test_name = file_stem.to_snake();

		let test_full_path = path.display().to_string();

		let extension = match path.extension() {
			Some(ext) => format!(".{}", ext.to_str().unwrap_or("")),
			None => "".into(),
		};

		let mut test_expected_file = path.to_path_buf();
		test_expected_file.pop();
		test_expected_file.push(format!("{}.expected{}", file_stem, extension));
		let test_expected_fullpath = test_expected_file.display().to_string();

		Some(Variables {
			test_name,
			test_full_path,
			test_expected_fullpath,
		})
	}

	pub fn gen(&self) -> Result<TokenStream, &str> {
		let files = self.get_all_files()?;

		let mut q = quote! {};
		for file in files.flatten() {
			let Variables {
				test_name,
				test_full_path,
				test_expected_fullpath,
			} = Arguments::get_variables(&file).ok_or("Cannot generate variables for this file")?;

			let span = self.pattern.lit.span();
			let test_name = syn::Ident::new(&test_name, span);
			let f = &self.f;
			q.extend(quote! {
				#[test]
				pub fn #test_name () {
					let test_file = #test_full_path;
					let test_expected_file = #test_expected_fullpath;
					#f(test_file, test_expected_file);
				}
			});
		}

		// println!("{}", q);

		Ok(q.into())
	}
}

impl syn::parse::Parse for Arguments {
	fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
		let path: syn::ExprLit = input.parse()?;
		let _: syn::Token!(,) = input.parse()?;
		let call: syn::Path = input.parse()?;
		Ok(Arguments {
			pattern: path,
			f: call,
		})
	}
}

#[proc_macro]
#[proc_macro_error]
pub fn gen_tests(input: TokenStream) -> TokenStream {
	let args = syn::parse_macro_input!(input as Arguments);
	match args.gen() {
		Ok(tokens) => tokens,
		Err(e) => abort!(e, "{}", e),
	}
}
