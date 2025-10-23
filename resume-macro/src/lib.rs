use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(CleanupDocs)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_cleanup_docs_macro(&ast)
}

fn impl_cleanup_docs_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let gen = quote! {
        #[doc(hidden)]
        impl #impl_generics !Send for #name #ty_generics #where_clause {}
        #[doc(hidden)]
        impl #impl_generics !Sync for #name #ty_generics #where_clause {}
        #[doc(hidden)]
        impl #impl_generics !Unpin for #name #ty_generics #where_clause {}
        #[doc(hidden)]
        impl #impl_generics !core::panic::RefUnwindSafe for #name #ty_generics #where_clause {}
        #[doc(hidden)]
        impl #impl_generics !core::panic::UnwindSafe for #name #ty_generics #where_clause {}
        #[doc(hidden)]
        impl #impl_generics !std::marker::Freeze for #name #ty_generics #where_clause {}
    };
    gen.into()
}
