use proc_macro::TokenStream;

#[proc_macro_derive(ProjectileData)]
pub fn projectile_data(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn ability(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}