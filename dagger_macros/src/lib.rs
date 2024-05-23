use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields};

#[proc_macro_derive(Layout, attributes(layout, children))]
pub fn layout_derive_macro(tokens: TokenStream) -> TokenStream {
	let ast = syn::parse(tokens).unwrap();

	impl_calculate_layout(ast)
}

fn impl_calculate_layout(ast: DeriveInput) -> TokenStream {
	let name: proc_macro2::Ident = ast.ident;
	let mut layout: Option<proc_macro2::Ident> = None;
	let mut layout_assigned = false;
	let mut children: Option<proc_macro2::Ident> = None;
	let mut children_assigned = false;

	match ast.data {
		Data::Struct(data) => match data.fields {
			Fields::Named(fields) => {
				for field in fields.named.iter() {
					for attr in field.attrs.iter() {
						match attr.meta.path().get_ident().unwrap().to_string().as_str() {
							"layout" => {
								if !layout_assigned {
									layout = Some(field.ident.clone().unwrap());
									layout_assigned = true;
								} else {
									panic!("Only one field can be annotated with #[layout]!")
								}
							},
							"children" => {
								if !children_assigned {
									children = Some(field.ident.clone().unwrap());
									children_assigned = true;
								} else {
									panic!("Only one field can be annotated with #[children]!")
								}
							},
							_ => {},
						}
					}
				}
			},
			_ => {},
		},
		_ => panic!("`Layout` may only be derived on structs!"),
	}

	match layout {
		None => panic!("One field must be annotated with #[layout]!"),
		_ => {},
	}

	match children {
		None => panic!("One field must be annotated with #[children]!"),
		_ => {},
	}

	quote::quote! {
		impl ::dagger_layout::CalculateLayout for #name {
			fn get_layout(&self) -> &::dagger_layout::Layout {
				&self.#layout
			}

			fn get_children(&self) -> &Vec<Self> {
				&self.#children
			}
		}
	}.into()
}