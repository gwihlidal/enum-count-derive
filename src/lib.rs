extern crate proc_macro;
use proc_macro::TokenStream;

extern crate syn;

#[macro_use]
extern crate quote;

#[proc_macro_derive(EnumCount)]
pub fn count(input: TokenStream) -> TokenStream {
	let source = input.to_string();

	// Parse the string representation into a syntax tree
	let ast = syn::parse_derive_input(&source).unwrap();

	// Build the output
	let expanded = expand_count(&ast);

	// Return the generated impl as a TokenStream
	expanded.parse().unwrap()
}

fn expand_count(ast: &syn::DeriveInput) -> quote::Tokens {
	let n = match ast.body {
		syn::Body::Struct(_) => panic!("#[derive(EnumCount)] can only be used with enums"),
		syn::Body::Enum(ref variants) => variants.len(),
	};

	// Used in the quasi-quotation below as `#name`
	let name = &ast.ident;

	// Helper is provided for handling complex generic types correctly and effortlessly
	let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

	quote! {
		// The generated impl
		impl #impl_generics ::enum_count::EnumCount for #name #ty_generics #where_clause {
			fn count() -> usize {
				#n
			}
		}
	}
}