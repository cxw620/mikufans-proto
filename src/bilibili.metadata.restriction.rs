// This file is @generated by prost-build.
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Restriction {
    ///
    #[prost(bool, tag = "1")]
    pub teenagers_mode: bool,
    ///
    #[prost(bool, tag = "2")]
    pub lessons_mode: bool,
    ///
    #[prost(enumeration = "ModeType", tag = "3")]
    pub mode: i32,
    ///
    #[prost(bool, tag = "4")]
    pub review: bool,
    ///
    #[prost(bool, tag = "5")]
    pub disable_rcmd: bool,
    ///
    #[prost(bool, tag = "6")]
    pub basic_mode: bool,
    ///
    #[prost(int32, tag = "7")]
    pub teenagers_age: i32,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModeType {
    ///
    Normal = 0,
    ///
    Teenagers = 1,
    ///
    Lessons = 2,
    ///
    Basic = 3,
}
impl ModeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Normal => "NORMAL",
            Self::Teenagers => "TEENAGERS",
            Self::Lessons => "LESSONS",
            Self::Basic => "BASIC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NORMAL" => Some(Self::Normal),
            "TEENAGERS" => Some(Self::Teenagers),
            "LESSONS" => Some(Self::Lessons),
            "BASIC" => Some(Self::Basic),
            _ => None,
        }
    }
}