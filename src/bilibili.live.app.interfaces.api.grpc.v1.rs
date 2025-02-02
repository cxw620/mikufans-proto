// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMoreLiveRoomsReq {
    ///
    #[prost(int64, tag = "1")]
    pub room_id: i64,
    ///
    #[prost(message, optional, tag = "2")]
    pub idol_pagination: ::core::option::Option<
        super::super::super::super::super::super::pagination::Pagination,
    >,
    ///
    #[prost(message, optional, tag = "3")]
    pub rooms_pagination: ::core::option::Option<
        super::super::super::super::super::super::pagination::Pagination,
    >,
    ///
    #[prost(int64, tag = "4")]
    pub uid: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMoreLiveRoomsResp {
    ///
    #[prost(message, optional, tag = "1")]
    pub idol_pagination_reply: ::core::option::Option<
        super::super::super::super::super::super::pagination::PaginationReply,
    >,
    ///
    #[prost(message, optional, tag = "2")]
    pub rooms_pagination_reply: ::core::option::Option<
        super::super::super::super::super::super::pagination::PaginationReply,
    >,
    ///
    #[prost(bool, tag = "3")]
    pub idol_has_more: bool,
    ///
    #[prost(message, repeated, tag = "4")]
    pub my_idol_info: ::prost::alloc::vec::Vec<MyIdolInfo>,
    ///
    #[prost(message, repeated, tag = "5")]
    pub more_live_info: ::prost::alloc::vec::Vec<MoreLiveRoomInfo>,
    ///
    #[prost(message, repeated, tag = "6")]
    pub view_history: ::prost::alloc::vec::Vec<ViewHistory>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetViewHistoryReq {
    ///
    #[prost(int64, tag = "1")]
    pub uid: i64,
    ///
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::super::super::pagination::Pagination,
    >,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetViewHistoryResp {
    ///
    #[prost(message, repeated, tag = "1")]
    pub history: ::prost::alloc::vec::Vec<ViewHistory>,
    ///
    #[prost(message, optional, tag = "2")]
    pub pagination_reply: ::core::option::Option<
        super::super::super::super::super::super::pagination::PaginationReply,
    >,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoreLiveRoomInfo {
    ///
    #[prost(int64, tag = "1")]
    pub room_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub uname: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub room_cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub room_title: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "RoomType", tag = "5")]
    pub room_type: i32,
    ///
    #[prost(string, tag = "6")]
    pub text_small: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub area_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub link: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub face: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyIdolInfo {
    ///
    #[prost(int64, tag = "1")]
    pub room_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub uid: i64,
    ///
    #[prost(string, tag = "3")]
    pub uname: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub link: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveViewHistoryReq {
    ///
    #[prost(int64, repeated, tag = "1")]
    pub kids: ::prost::alloc::vec::Vec<i64>,
    ///
    #[prost(int64, tag = "2")]
    pub uid: i64,
    ///
    #[prost(string, tag = "3")]
    pub buvid: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct RemoveViewHistoryResp {}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewHistory {
    ///
    #[prost(int64, tag = "1")]
    pub kid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub anchor_uid: i64,
    ///
    #[prost(string, tag = "3")]
    pub uname: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub room_id: i64,
    ///
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "8")]
    pub living_status: i64,
    ///
    #[prost(int64, tag = "9")]
    pub parent_area_id: i64,
    ///
    #[prost(string, tag = "10")]
    pub parent_area_name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "11")]
    pub area_id: i64,
    ///
    #[prost(string, tag = "12")]
    pub area_name: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "13")]
    pub follow_status: bool,
    ///
    #[prost(int64, tag = "14")]
    pub view_time: i64,
    ///
    #[prost(string, tag = "15")]
    pub device: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "16")]
    pub link: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoomType {
    ///
    Watched = 0,
    ///
    Popularity = 1,
}
impl RoomType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Watched => "Watched",
            Self::Popularity => "Popularity",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Watched" => Some(Self::Watched),
            "Popularity" => Some(Self::Popularity),
            _ => None,
        }
    }
}
