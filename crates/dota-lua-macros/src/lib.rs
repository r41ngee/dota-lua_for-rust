use proc_macro::TokenStream;

#[proc_macro_derive(ProjectileData)]
pub fn projectile_data(_: TokenStream) -> TokenStream {
    TokenStream::new()
}