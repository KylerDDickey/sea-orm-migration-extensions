use crate::attributes::*;
use darling::ToTokens;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;
use syn::punctuated::Punctuated;
use syn::token::Comma;

macro_rules! impl_to_tokens {
    ($t:ty) => {
        impl ToTokens for $t {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                let t = Ident::new(stringify!($t), Span::call_site());

                tokens.extend(quote! {
                    ColumnType::#t
                });
            }
        }
    };
    ($t:ty: $($p:ident),* $(?$o:ident),* $(,)?) => {
        impl ToTokens for $t {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                let mut args = Punctuated::<TokenStream, Comma>::new();

                $(
                    args.push(self.$p.into_token_stream());
                )*

                $(
                    args.push(match &self.$o {
                        Some(x) => {
                            let ts = x.into_token_stream();
                            quote! { Some(#ts) }
                        },
                        None => {
                            quote! { None }
                        },
                    });
                )*

                let t = Ident::new(stringify!($t), Span::call_site());

                tokens.extend(quote! {
                    ColumnType::#t(#args)
                });
            }
        }
    };
}

impl_to_tokens!(Char: ?len);
impl_to_tokens!(String: ?len);
impl_to_tokens!(Text);
impl_to_tokens!(TinyInteger: ?len);
impl_to_tokens!(SmallInteger: ?len);
impl_to_tokens!(Integer: ?len);
impl_to_tokens!(BigInteger: ?len);
impl_to_tokens!(TinyUnsigned: ?len);
impl_to_tokens!(SmallUnsigned: ?len);
impl_to_tokens!(Unsigned: ?len);
impl_to_tokens!(BigUnsigned: ?len);
impl_to_tokens!(Float: ?len);
impl_to_tokens!(Double: ?len);

impl_to_tokens!(Timestamp: ?len);
impl_to_tokens!(TimestampWithTimeZone: ?len);
impl_to_tokens!(Time: ?len);
impl_to_tokens!(Date);


impl_to_tokens!(VarBinary: len);
impl_to_tokens!(Boolean);

impl_to_tokens!(Json);
impl_to_tokens!(JsonBinary);

impl_to_tokens!(Uuid);

impl_to_tokens!(Array: ?elem_type);
impl_to_tokens!(Cidr);
impl_to_tokens!(Inet);
impl_to_tokens!(MacAddr);

impl ToTokens for DataType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::Char(inner) => inner.into_token_stream(),
            Self::String(inner) => inner.into_token_stream(),
            Self::Text(inner) => inner.into_token_stream(),
            Self::TinyInteger(inner) => inner.into_token_stream(),
            Self::SmallInteger(inner) => inner.into_token_stream(),
            Self::Integer(inner) => inner.into_token_stream(),
            Self::BigInteger(inner) => inner.into_token_stream(),
            Self::TinyUnsigned(inner) => inner.into_token_stream(),
            Self::SmallUnsigned(inner) => inner.into_token_stream(),
            Self::Unsigned(inner) => inner.into_token_stream(),
            Self::BigUnsigned(inner) => inner.into_token_stream(),
            Self::Float(inner) => inner.into_token_stream(),
            Self::Double(inner) => inner.into_token_stream(),

            Self::Timestamp(inner) => inner.into_token_stream(),
            Self::TimestampWithTimeZone(inner) => inner.into_token_stream(),
            Self::Time(inner) => inner.into_token_stream(),
            Self::Date(inner) => inner.into_token_stream(),


            Self::VarBinary(inner) => inner.into_token_stream(),
            Self::Boolean(inner) => inner.into_token_stream(),

            Self::Json(inner) => inner.into_token_stream(),
            Self::JsonBinary(inner) => inner.into_token_stream(),

            Self::Uuid(inner) => inner.into_token_stream(),

            Self::Array(inner) => inner.into_token_stream(),
            Self::Cidr(inner) => inner.into_token_stream(),
            Self::Inet(inner) => inner.into_token_stream(),
            Self::MacAddr(inner) => inner.into_token_stream(),
        });
    }
}
