// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntryAdded {
    /// {block-number}-{tx-hash}-{log-index}
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub index: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub author: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub space: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntriesAdded {
    #[prost(message, repeated, tag="1")]
    pub entries: ::prost::alloc::vec::Vec<EntryAdded>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleGranted {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration="Role", tag="2")]
    pub role: i32,
    #[prost(string, tag="3")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub space: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RolesGranted {
    #[prost(message, repeated, tag="1")]
    pub roles: ::prost::alloc::vec::Vec<RoleGranted>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleRevoked {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration="Role", tag="2")]
    pub role: i32,
    #[prost(string, tag="3")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub space: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RolesRevoked {
    #[prost(message, repeated, tag="1")]
    pub roles: ::prost::alloc::vec::Vec<RoleRevoked>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoOutput {
    #[prost(message, repeated, tag="1")]
    pub entries: ::prost::alloc::vec::Vec<EntryAdded>,
    #[prost(message, repeated, tag="2")]
    pub roles_granted: ::prost::alloc::vec::Vec<RoleGranted>,
    #[prost(message, repeated, tag="3")]
    pub roles_revoked: ::prost::alloc::vec::Vec<RoleRevoked>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Role {
    Null = 0,
    Moderator = 1,
    Member = 2,
    Admin = 3,
}
impl Role {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Role::Null => "NULL",
            Role::Moderator => "MODERATOR",
            Role::Member => "MEMBER",
            Role::Admin => "ADMIN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NULL" => Some(Self::Null),
            "MODERATOR" => Some(Self::Moderator),
            "MEMBER" => Some(Self::Member),
            "ADMIN" => Some(Self::Admin),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
