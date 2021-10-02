pub mod data_context;
pub use data_context::DataContext;
pub mod deserializer;
pub use deserializer::Deserializer;
pub mod flexpiler;
pub use self::flexpiler::Flexpiler;
pub mod generics_where;
pub use generics_where::GenericsWhere;
pub mod identity;
pub mod struct_rustc;


pub trait Trait<DefinitionType> {
    fn gen(definition: DefinitionType) -> proc_macro2::TokenStream;
}


pub trait TryTrait<DefinitionType> {
    type Error;

    fn try_gen(definition: DefinitionType) -> std::result::Result<proc_macro2::TokenStream, Self::Error>;
}
