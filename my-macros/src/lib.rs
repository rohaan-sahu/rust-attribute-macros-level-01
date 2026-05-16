use proc_macro::TokenStream;
use quote::quote;
use syn::{
	parse_macro_input,
	ItemFn,
	parse::Parse,
	parse::ParseStream,
	LitInt,
	LitBool,
	Token,
	ItemStruct,
	Fields
};

#[proc_macro_attribute]
pub fn log_call(_attr: TokenStream, item: TokenStream)-> TokenStream {
	let func = parse_macro_input!(item as ItemFn);
	let name = &func.sig.ident;
	let block = &func.block;
	let sig = &func.sig;
	let attrs = &func.attrs;
	
	quote!{
		#(#attrs)*
		#sig{
			println!(">> entering {}",stringify!(#name));
			let result = {#block};
			println!("<< exiting {}",stringify!(#name));
			result
		}
	}
	.into()
}

#[proc_macro_attribute]
pub fn log_call_comp(_attr: TokenStream, item: TokenStream)-> TokenStream {
	let func = parse_macro_input!(item as ItemFn);
	let name = &func.sig.ident;
	let name_str = name.to_string();
	let block = &func.block;
	let sig = &func.sig;
	let attrs= &func.attrs;
	
	quote!{
		#(#attrs)*
		#sig{
			println!(">> entering {}", #name_str);
			let result = {#block};
			println!("<< exiting {}", #name_str);
			result
		}
	}
	.into()
}

#[proc_macro_attribute]
pub fn double_output(_attr: TokenStream, item: TokenStream)-> TokenStream {
	let func = parse_macro_input!(item as ItemFn);
	let block = &func.block;
	let sig = &func.sig;
	let attrs = &func.attrs;
	
	quote!{
		#(#attrs)*
		#sig {
			let result = {#block};
			result * 2
		}
	}
	.into()
}


#[proc_macro_attribute]
pub fn multiply(attr: TokenStream, item: TokenStream)->TokenStream {
	let factor	= parse_macro_input!(attr as LitInt);
	// let factor_val: u64 = factor.base10_parse().unwrap();

	let func = parse_macro_input!(item as ItemFn);
	let attrs = &func.attrs;
	let sig = &func.sig;
	let block = &func.block;

	quote!{
		#(#attrs)*
		#sig {
			let result = #block;
			(result * #factor) as i32
		}
	}.into()
}

// --------------

struct LogMultiplyArgs {
	factor: LitInt,
	log: LitBool
}

impl Parse for LogMultiplyArgs {
	fn parse(input: ParseStream)->syn::Result<Self> {
		// parse: factor = 3
		let _factor_kw: syn::Ident = input.parse()?;
		let _eq: Token![=] = input.parse()?;
		let factor: LitInt = input.parse()?;

		// parse the comma
		let _comma: Token![,] = input.parse()?;

		// parse: log = true
		let _log_kw: syn::Ident = input.parse()?;
		let _eq: Token![=] = input.parse()?;
		let log: LitBool = input.parse()?;

		Ok(LogMultiplyArgs { factor, log })
	}
}


#[proc_macro_attribute]
pub fn log_multiply(attr: TokenStream, item: TokenStream)-> TokenStream {
	let args = parse_macro_input!(attr as LogMultiplyArgs);
	let factor = &args.factor;
	let log = args.log.value();

	let func = parse_macro_input!(item as ItemFn);
	let attrs = &func.attrs;
	let sig = &func.sig;
	let block = &func.block;
	let name = &func.sig.ident;
	let name_str = name.to_string();

	let log_code = if log {
		quote!{
			println!(">> entering {}",#name_str);
		}
	} else {
		quote! {}
	};
	
	quote!{
		#(#attrs)*
		#sig {
			#log_code
			let result = {#block};
			result * #factor
		}
	}
	.into()
}


#[proc_macro_attribute]
pub fn describe(_attr: TokenStream, item: TokenStream)-> TokenStream{
	let struct_item = parse_macro_input!(item as ItemStruct);
	// to prevent recursion in 'quote!()'
	//struct_item.attrs.retain(|a| !a.path().is_ident("describe"));
	let struct_name = &struct_item.ident;
	//let attrs = &struct_item.attrs;

	// extract fields from struct
	let field_names = match &struct_item.fields {
		Fields::Named(fields) => fields.named.iter()
			.map(|f| f.ident.as_ref().unwrap().to_string())
			.collect::<Vec<_>>(),
		_=> vec![],
	};

	let field_names_str = field_names.join(", ");
	let struct_name_str = struct_name.to_string();

	quote!{
		//#(#attrs)*
		#struct_item

		impl #struct_name {
			pub fn describe(&self) {
				println!("Struct: {}",#struct_name_str);
				println!("Fields: {}",#field_names_str);
			}
		}
	}
	.into()

}

#[proc_macro_attribute]
pub fn builder(_attr: TokenStream, item: TokenStream )->TokenStream {

	let struct_item = parse_macro_input!(item as ItemStruct);
	// preventing recursive emitting of struct via 'quote!'
	//
	// method - 1 
	// remove #[builder] from the struct's attribute
	//struct_item.attrs.retain(|a| !a.path().is_ident("builder"));
	let struct_name = &struct_item.ident;
	//let attrs = &struct_item.attrs;

	// method - 2 (Alternative)
	// filter out the [builders] attribute before re-emitting
	//let filtered_attrs = attrs.iter()
	//						.filter(|a| !a.path().is_ident("builder"));
	
	

	// extract field names & types
	let fields = match &struct_item.fields {
		Fields::Named(f) => &f.named,
		_ => panic!("builders only supports named fields"),
	};

	let field_names: Vec<_> = fields.iter()
		.map(|f| f.ident.as_ref().unwrap())
		.collect();
		
	let field_types: Vec<_> = fields.iter()
		.map(|f| &f.ty)
		.collect();

	let field_names_str: Vec<_> = field_names.iter()
		.map(|f| f.to_string())
		.collect();
	
	quote!{
		// #(#filtered_attrs)*
		//#(#attrs)*
		#struct_item

		impl #struct_name {
			// generate new constructor
			pub fn new (#(#field_names: #field_types),*)-> Self {
				Self {
					#(#field_names),*
				}
			}

			// generate a display method
			pub fn display(&self) {
				println!("{}",stringify!(#struct_name));
				#(
					println!(" {}: {:?}",#field_names_str,self.#field_names);
				)*
			}
		}
	}.into()
}
