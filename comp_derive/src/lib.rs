// List comprehension grammar:
//
// comp: mapping for_if_clause+
//
// mapping: expression
//
// for_if_clause:
//     | 'for' pattern 'in' expression ('if' expression)*
//
// pattern: name (, name)*

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use vec1::Vec1;

// [[1, 2], [3, 4], [5, 6]] => [1, 2, 3, 4, 5, 6]
// x for row in matrix for x in row

// [[0, 2], [0, 2], [0, 2]]
// x for row in 0..3 for x in 0..3 if x % 2 == 0
struct Comp {
    mapping: Mapping,
    for_if_clauses: Vec1<ForIfClause>,
}

impl Parse for Comp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mapping = input.parse()?;
        let for_if_clauses = Vec1::try_from_vec(parse_zero_or_more(input))
            .map_err(|_| syn::Error::new(input.span(), "at least one for-if clause is required"))?;

        Ok(Self {
            mapping,
            for_if_clauses,
        })
    }
}

impl ToTokens for Comp {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Mapping(mapping) = &self.mapping;

        fn build_iter(mapping: TokenStream2, clauses: &[ForIfClause]) -> TokenStream2 {
            if let Some((first, rest)) = clauses.split_first() {
                let ForIfClause {
                    pattern,
                    sequence,
                    conditions,
                } = first;
                let cond = quote! { (true #(&& (#conditions))* ) };
                let inner = build_iter(mapping, rest);
                quote! {
                    ::core::iter::IntoIterator::into_iter(#sequence).flat_map(move |#pattern| {
                        #cond.then(|| { #inner }).into_iter().flatten()
                    })
                }
            } else {
                quote! { ::core::iter::once(#mapping) }
            }
        }

        let iter = build_iter(quote! { #mapping }, &self.for_if_clauses);
        tokens.extend(iter);
    }
}

struct Mapping(syn::Expr);

impl Parse for Mapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

impl ToTokens for Mapping {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens)
    }
}

struct ForIfClause {
    pattern: Pattern,
    sequence: syn::Expr,
    conditions: Vec<Condition>,
}

impl Parse for ForIfClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<syn::Token![for]>()?;
        let pattern = input.parse()?;
        _ = input.parse::<syn::Token![in]>()?;
        let sequence = input.parse()?;
        let conditions = parse_zero_or_more(input);

        Ok(Self {
            pattern,
            sequence,
            conditions,
        })
    }
}

fn parse_zero_or_more<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut items = vec![];
    while let Ok(item) = input.parse() {
        items.push(item);
    }
    items
}

struct Pattern(syn::Pat);

impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        syn::Pat::parse_single(input).map(Self)
    }
}

impl ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens)
    }
}

struct Condition(syn::Expr);

impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<syn::Token![if]>()?;
        input.parse().map(Self)
    }
}

impl ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens)
    }
}

#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let c = parse_macro_input!(input as Comp);
    quote! { #c }.into()
}
