use proc_macro::TokenStream;

#[proc_macro_derive(ShaderRepr)]
pub fn derive_shader_repr(item: TokenStream) -> TokenStream {
    item
}
