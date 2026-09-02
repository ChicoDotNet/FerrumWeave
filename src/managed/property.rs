use super::base::{ManagedMetadataError, ManagedMethodRef};
use super::call::{
    ManagedMemberRef, method_calls_member_ref, resolve_public_static_member_ref,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedPropertyRef {
    pub namespace: String,
    pub type_name: String,
    pub property_name: String,
    pub getter: ManagedMemberRef,
    pub setter: ManagedMemberRef,
}

pub fn resolve_public_property_accessors(
    image: &[u8],
    namespace: &str,
    type_name: &str,
    property_name: &str,
) -> Result<ManagedPropertyRef, ManagedMetadataError> {
    let getter_name = format!("get_{property_name}");
    let setter_name = format!("set_{property_name}");
    let getter = resolve_public_static_member_ref(image, namespace, type_name, &getter_name)?;
    let setter = resolve_public_static_member_ref(image, namespace, type_name, &setter_name)?;

    Ok(ManagedPropertyRef {
        namespace: namespace.to_owned(),
        type_name: type_name.to_owned(),
        property_name: property_name.to_owned(),
        getter,
        setter,
    })
}

pub fn method_calls_property_accessors(
    image: &[u8],
    method: &ManagedMethodRef,
    property: &ManagedPropertyRef,
) -> Result<bool, ManagedMetadataError> {
    Ok(
        method_calls_member_ref(image, method, &property.getter)?
            && method_calls_member_ref(image, method, &property.setter)?,
    )
}
