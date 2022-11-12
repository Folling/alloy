use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_str, BinOp, DataEnum, Expr, ExprBinary, ExprLit, Ident, Lit, Path, Type};

fn check_path<S: AsRef<str>>(path: &Path, expected: &[S]) -> bool {
    expected
        .iter()
        .zip(path.segments.iter())
        .all(|(l, r)| r.ident.to_string().eq(l.as_ref()))
}

fn validate_bitflag(ident: Ident, data: &DataEnum) {
    for variant in &data.variants {
        if let Some((_, discrim)) = variant.discriminant.as_ref() {
            match discrim {
                Expr::Lit(ExprLit { lit: Lit::Int(int), .. }) => {
                    let num = int.base10_parse::<u64>().unwrap();

                    if !num.is_power_of_two() {
                        panic!(
                            "all discriminants should be powers of two or combinations of other flags. {}::{} was {}",
                            ident, variant.ident, num
                        );
                    }
                }
                // it'd be nice to support expressions here to allow for Flag::AB = Flag::A | Flag::B, but that's just not worth a lot of effort
                // and can be done by the user with a constant value
                v => panic!(
                    "all discriminants must be integer literals, {}::{} was {:?}",
                    ident, variant.ident, v
                ),
            }
        } else {
            panic!("all bitflag members ought to have a discriminant");
        }
    }
}

static mut IDENTS: Vec<String> = vec![];

#[proc_macro_attribute]
pub fn bitflag(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input: syn::DeriveInput = parse_macro_input!(input as syn::DeriveInput);

    match input.data {
        syn::Data::Enum(ref mut data) => {
            validate_bitflag(input.ident.clone(), data);

            let struct_ident = input.ident.clone();
            let len = data.variants.len();

            let str = format!("__{}_enum", struct_ident);

            input.ident = Ident::new(str.as_str(), struct_ident.span());
            let enum_ident = &input.ident;

            unsafe {
                // keep the ident alive
                IDENTS.push(str);
            }

            let actual_variants = data
                .variants
                .iter()
                .filter(|v| {
                    if let Some((_, Expr::Lit(ExprLit { lit: Lit::Int(int), .. }))) = &v.discriminant.as_ref() {
                        return int.base10_parse::<u64>().unwrap().is_power_of_two();
                    }

                    return false;
                })
                .collect::<Vec<_>>();

            let consts = actual_variants
                .iter()
                .map(|v| {
                    let v_ident = &v.ident;

                    quote! { const #v_ident : #struct_ident = #struct_ident(#enum_ident::#v_ident as u64); }
                })
                .collect::<Vec<_>>();

            let const_idents = actual_variants
                .iter()
                .map(|v| {
                    let ident = &v.ident;
                    quote! { (#struct_ident::#ident, stringify!(#ident)) }
                })
                .collect::<Vec<_>>();

            let og = quote! {
                #[allow(non_camel_case_types)]
                #input
            };

            let bin_traits = [("BitAnd", "bitand", "&"), ("BitOr", "bitor", "|"), ("BitXor", "bitxor", "^")]
                .iter()
                .map(|(name, func, op)| {
                    let name = parse_str::<Ident>(name).unwrap();
                    let func = parse_str::<Ident>(func).unwrap();
                    let op = parse_str::<BinOp>(op).unwrap();

                    quote! {
                        impl ::std::ops::#name for #struct_ident {
                            type Output = #struct_ident;

                            fn #func(self, rhs: Self) -> Self::Output {
                                Self(self.0 #op rhs.0)
                            }
                        }
                    }
                });

            let assign_traits = [
                ("BitAndAssign", "bitand_assign", "&="),
                ("BitOrAssign", "bitor_assign", "|="),
                ("BitXorAssign", "bitxor_assign", "^="),
            ]
            .iter()
            .map(|(name, func, op)| {
                let name = parse_str::<Ident>(name).unwrap();
                let func = parse_str::<Ident>(func).unwrap();
                let op = parse_str::<BinOp>(op).unwrap();

                quote! {
                    impl ::std::ops::#name for #struct_ident {
                        fn #func(&mut self, rhs: Self) {
                            self.0 #op rhs.0;
                        }
                    }
                }
            });

            let attrs = input.attrs;

            let new = quote! {
                #(#attrs)*
                struct #struct_ident(u64);

                impl #struct_ident {
                    #(#consts)*

                    const ITEMS: [(#struct_ident, &'static str); #len] = [
                        #(#const_idents),*
                    ];

                    pub fn empty() -> Self {
                        Self(0)
                    }

                    pub fn all() -> Self {
                        Self(::std::u64::MAX)
                    }

                    pub fn none_set(&self) -> bool {
                        self.0 == 0
                    }

                    pub fn any_set(&self) -> bool {
                        self.0 != 0
                    }

                    pub fn all_set(&self) -> bool {
                        self.0 == ::std::u64::MAX
                    }

                    pub fn clear(&mut self) {
                        self.0 = 0;
                    }

                    pub fn set(&mut self, value: #enum_ident) {
                        self.0 |= value as u64;
                    }

                    pub fn remove(&mut self, value: #enum_ident) {
                        self.0 &= !(value as u64);
                    }

                    pub fn toggle(&mut self, value: #enum_ident) {
                        self.0 ^= value as u64;
                    }

                    pub fn test(&self, item: #struct_ident) -> bool {
                        self.0 & item.0 != 0
                    }
                }

                impl std::fmt::Display for #struct_ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        let mut i = 0;

                        write!(f, "(")?;

                        for (val, name) in #struct_ident::ITEMS {
                            if self.test(val) {
                                if i != 0 {
                                    write!(f, " | ")?;
                                }

                                write!(f, "{}", name)?;

                                i += 1;
                            }
                        }

                        write!(f, ")")?;

                        Ok(())
                    }
                }

                #(#bin_traits)*
                #(#assign_traits)*

                impl core::ops::Sub for #struct_ident {
                    type Output = #struct_ident;

                    fn sub(self, other: Self) -> Self::Output {
                        Self(self.0 & !(self.0 & other.0))
                    }
                }

                impl core::ops::Not for #struct_ident {
                    type Output = #struct_ident;

                    fn not(self) -> Self::Output {
                        #struct_ident(!self.0)
                    }
                }
            };

            println!("{}", new);

            return quote! {#og #new}.into();
        }
        _ => panic!("Bitflags only works on enums"),
    }
}

#[proc_macro_derive(EnumCount)]
pub fn derive_enum_count(derive: TokenStream) -> TokenStream {
    let input = parse_macro_input!(derive as syn::DeriveInput);

    match input.data {
        syn::Data::Enum(ref data) => {
            let ident = input.ident;
            let len = data.variants.len();

            return TokenStream::from(quote! {
                impl traits::enums::EnumCount for #ident {
                    fn count() -> usize { #len }
                }
            });
        }
        _ => panic!("Bitflags only works on enums"),
    }
}

#[proc_macro_derive(EnumIter)]
pub fn derive_enum_iter(derive: TokenStream) -> TokenStream {
    let input = parse_macro_input!(derive as syn::DeriveInput);

    match input.data {
        syn::Data::Enum(ref data) => {
            let ident = input.ident;
            let len = data.variants.len();

            let iter_struct_name = format!("{}Iter", ident);

            let const_values = data.variants.iter().map(|v| {
                let ident = &v.ident;
                let named_fields = v.fields.iter().next().and_then(|v| v.ident.as_ref()).is_some();

                let values: Vec<_> = if named_fields {
                    v.fields
                        .iter()
                        .map(|v| {
                            let ident = v.ident.as_ref().unwrap();
                            let ty = &v.ty;
                            quote! {
                                #ident: <#ty as ::std::default::Default>::default()
                            }
                        })
                        .collect()
                } else {
                    v.fields
                        .iter()
                        .map(|v| {
                            let ty = &v.ty;
                            quote! {
                                <#ty as ::std::default::Default>::default()
                            }
                        })
                        .collect()
                };

                if named_fields {
                    quote! { #ident{ #(#values),*} }
                } else {
                    quote! { #ident(#(#values),*) }
                }
            });

            return TokenStream::from(quote! {
                struct #iter_struct_name {

                }

                impl #iter_struct_name {
                    const VALUES = [#ident; #len] = [
                        #(#const_values),*
                    ];
                }

                impl traits::enums::EnumIter<#iter_struct_name> for #ident {

                }
            });
        }
        _ => panic!("EnumIter only works on enums"),
    }
}

#[proc_macro_derive(EnumDisplay)]
pub fn derive_enum_display(derive: TokenStream) -> TokenStream {
    let input = parse_macro_input!(derive as syn::DeriveInput);

    match input.data {
        syn::Data::Enum(ref data) => {
            let ident = input.ident;
            let len = data.variants.len();

            return TokenStream::from(quote! {
                impl traits::enums::EnumCount for #ident {
                    fn count() -> usize { #len }
                }
            });
        }
        _ => panic!("EnumDisplay only works on enums"),
    }
}
