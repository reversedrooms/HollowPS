use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod oct_data;

/// Generate `OctData` implementation for structs and enums that have all fields
/// implement `OctData`.
///
/// For structs, fields are written one by one in order.
/// If the struct must pad a byte due to it being a `CPropertyObject`, the `#[property_object]`
/// attribute must be present, and must contain the value to pad with.
///
/// e.g. `#[property_object(u16, 0x01)]` will pad with 0x01 as a u16.
///
/// In the presence of these property objects, all fields must be Optional, e.g. `Option<T>`,
/// and must also have a tag attribute attached to them for marshalling and unmarshalling, of the
/// form `#[tag = <number>]`.
///
/// For enums, the structure starts with a discriminant with the type specified in the `#[repr]` of
/// the enum, followed by the fields of the enum one by one.
#[proc_macro_derive(
    OctData,
    attributes(
        property_object,
        property_blob,
        skip_property,
        tag,
        root,
        base,
        polymorphic_none
    )
)]
pub fn derive_message(item: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(item as DeriveInput);
    match oct_data::imp(&parsed) {
        Ok(item) => item.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
