#![recursion_limit="128"]
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    parse, parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Visibility, Type, LitInt, token,
    Token, bracketed, braced
};

#[derive(Clone)]
struct BfField {
    vis: Visibility,
    name: Ident,
    start_pos: LitInt,
    end_pos: LitInt,
}

impl Parse for BfField {
    fn parse(stream: ParseStream) -> Result<Self, parse::Error> {
        let (vis, name) = if stream.fork().parse::<Visibility>().is_ok() {
            (stream.parse()?, stream.parse()?)
        } else {
            (Visibility::Inherited, stream.parse()?)
        };
        token::Colon::parse(stream)?;

        let start_pos = stream.parse()?;
        token::Colon::parse(stream)?;
        let end_pos = stream.parse()?;

        Ok(Self {
            vis: vis,
            name: name,
            start_pos: start_pos,
            end_pos: end_pos
        })
    }
}

#[derive(Clone)]
struct BfInfo {
    vis: Visibility,
    name: Ident,
    ty: Type,
    fields: Vec<BfField>
}

impl Parse for BfInfo {
    fn parse(stream: ParseStream) -> Result<Self, parse::Error> {
        let (vis, name) = if stream.fork().parse::<Visibility>().is_ok() {
            (stream.parse()?, stream.parse()?)
        } else {
            (Visibility::Inherited, stream.parse()?)
        };

        let ty_inner;
        bracketed!(ty_inner in stream);
        let ty = ty_inner.parse()?;

        let fields_inner;
        braced!(fields_inner in stream);

        let field_parser = Punctuated::<BfField, Token![,]>::parse_terminated;
        let fields = field_parser(&fields_inner)?;
        let fields = fields.iter().cloned().collect();

        Ok(Self {
            vis: vis,
            name: name,
            ty: ty,
            fields: fields
        })
    }
}

fn make_accessor(ty: &Type, field: &BfField) -> impl ToTokens {
    let BfField { vis, name, start_pos, end_pos } = field;
    let set_name = Ident::new(&format!("set_{}", name), Span::call_site());
    let upd_name = Ident::new(&format!("upd_{}", name), Span::call_site());

    let base_mask_const = quote!(
        const BASE_MASK: #ty = (1 << (#end_pos - #start_pos + 1)) - 1;
    );

    quote!(
        #[inline(always)]
        #[allow(dead_code)]
        #vis fn #name(&self) -> #ty {
            #base_mask_const
            (self.val >> #start_pos) & BASE_MASK
        }

        #[inline(always)]
        #[allow(dead_code)]
        #vis fn #set_name(&mut self, val: #ty) -> &mut Self {
            #base_mask_const
            self.val &= !(BASE_MASK << #start_pos);
            self.val |= (val & BASE_MASK) << #start_pos;
            self
        }

        #[inline(always)]
        #[allow(dead_code)]
        #vis fn #upd_name<F>(&mut self, func: F) -> &mut Self
            where F: FnOnce(#ty) -> #ty {
            let old = self.#name();
            self.#set_name(func(old))
        }
    )
}

#[cfg(feature="use_std")]
fn make_debug<'a>(name: &Ident, fields: impl Iterator<Item=&'a Ident>) -> impl ToTokens {
    let (field_names, fields): (Vec<_>, Vec<_>) = fields.map(|f| (f.to_string(), f)).unzip();
    quote!(
        impl ::std::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.debug_struct(stringify!(#name))
                    #(.field(#field_names, &self.#fields()))*
                    .finish()
            }
        }
    )
}


#[cfg(not(feature="use_std"))]
fn make_debug<'a>(_: &Ident, _: impl Iterator<Item=&'a Ident>) -> impl ToTokens {
    quote!()
}

#[proc_macro]
pub fn bf(tok: TokenStream) -> TokenStream {
    let bfinfo: BfInfo = parse(tok).unwrap();
    let BfInfo{vis, name, ty, fields} = bfinfo;
    let accessors = fields.iter()
        .map(|f| make_accessor(&ty, f));

    let fmt = make_debug(&name, fields.iter().map(|f| &f.name));
    
    quote!(
        #[derive(Copy, Clone)]
        #[repr(transparent)]
        #vis struct #name {
            pub val: #ty
        }

        impl #name {
            #[inline(always)]
            pub fn new(val: #ty) -> Self {
                Self { val: val }
            }

            #[allow(dead_code)]
            #[inline(always)]
            pub fn alias<'a>(val: &'a #ty) -> &'a Self {
                unsafe { &*(val as *const #ty as *const Self) }
            }

            #[allow(dead_code)]
            #[inline(always)]
            pub fn alias_mut<'a>(val: &'a mut #ty) -> &'a mut Self {
                unsafe { &mut *(val as *mut #ty as *mut Self) }
            }

            #(#accessors)*
        }

        #fmt
    ).into()
}
