// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionButton {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub icon: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Button {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Category {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub topic: ::prost::alloc::vec::Vec<Topic>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTopicReply {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "TopicStatus", tag = "3")]
    pub status: i32,
    ///
    #[prost(int64, tag = "4")]
    pub category_id: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTopicReq {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub category_id: i64,
    ///
    #[prost(int64, tag = "3")]
    pub up_mid: i64,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DelMessageReply {}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DelMessageReq {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub topic_id: i64,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DelTopicReply {}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DelTopicReq {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DialogButton {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub confirm: ::core::option::Option<ActionButton>,
    ///
    #[prost(string, tag = "4")]
    pub cancel: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub text: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditTopicReply {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "TopicStatus", tag = "3")]
    pub status: i32,
    ///
    #[prost(int64, tag = "4")]
    pub category_id: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditTopicReq {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub category_id: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManagePageReply {
    ///
    #[prost(int64, tag = "1")]
    pub up_mid: i64,
    ///
    #[prost(enumeration = "ServerStatus", tag = "2")]
    pub status: i32,
    ///
    #[prost(message, optional, tag = "3")]
    pub online: ::core::option::Option<DialogButton>,
    ///
    #[prost(message, optional, tag = "4")]
    pub offline: ::core::option::Option<DialogButton>,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GetManagePageReq {
    ///
    #[prost(int64, tag = "1")]
    pub up_mid: i64,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HasServerReply {
    ///
    #[prost(bool, tag = "1")]
    pub hit_server: bool,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HasServerReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Input {
    ///
    #[prost(string, tag = "1")]
    pub placeholder: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemContentEmoji {
    ///
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemContentHighlight {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "IconPosition", tag = "3")]
    pub icon_position: i32,
    ///
    #[prost(string, tag = "4")]
    pub jump_url: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemContentItem {
    ///
    #[prost(enumeration = "ItemContentType", tag = "1")]
    pub r#type: i32,
    ///
    #[prost(message, optional, tag = "2")]
    pub text: ::core::option::Option<ItemContentText>,
    ///
    #[prost(message, optional, tag = "3")]
    pub highlight: ::core::option::Option<ItemContentHighlight>,
    ///
    #[prost(message, optional, tag = "4")]
    pub emoji: ::core::option::Option<ItemContentEmoji>,
    ///
    #[prost(string, tag = "5")]
    pub raw_text: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemContentText {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct LikeMessageReply {}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct LikeMessageReq {
    ///
    #[prost(int64, tag = "1")]
    pub topic_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub message_id: i64,
    ///
    #[prost(enumeration = "MessageActionType", tag = "3")]
    pub action: i32,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub timestamp: i64,
    ///
    #[prost(enumeration = "MessageStatus", tag = "3")]
    pub status: i32,
    ///
    #[prost(message, repeated, tag = "4")]
    pub items: ::prost::alloc::vec::Vec<MessageItem>,
    ///
    #[prost(enumeration = "MessageType", tag = "5")]
    pub r#type: i32,
    ///
    #[prost(int64, tag = "6")]
    pub order: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageItem {
    ///
    #[prost(enumeration = "MessageItemType", tag = "1")]
    pub r#type: i32,
    ///
    #[prost(message, optional, tag = "2")]
    pub author: ::core::option::Option<MessageItemAuthor>,
    ///
    #[prost(message, optional, tag = "3")]
    pub notice: ::core::option::Option<MessageItemNotice>,
    ///
    #[prost(message, optional, tag = "4")]
    pub content: ::core::option::Option<MessageItemContent>,
    ///
    #[prost(message, optional, tag = "5")]
    pub picture: ::core::option::Option<MessageItemPicture>,
    ///
    #[prost(message, optional, tag = "6")]
    pub addition: ::core::option::Option<MessageItemAddition>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageItemAddition {
    ///
    #[prost(int64, tag = "1")]
    pub likes: i64,
    ///
    #[prost(bool, tag = "2")]
    pub liked: bool,
    ///
    #[prost(bool, tag = "3")]
    pub show_menu: bool,
    ///
    #[prost(message, optional, tag = "4")]
    pub report: ::core::option::Option<Button>,
    ///
    #[prost(message, optional, tag = "5")]
    pub delete: ::core::option::Option<DialogButton>,
    ///
    #[prost(message, optional, tag = "6")]
    pub blacklist: ::core::option::Option<DialogButton>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageItemAuthor {
    ///
    #[prost(message, optional, tag = "1")]
    pub author: ::core::option::Option<UserInfo>,
    ///
    #[prost(string, tag = "2")]
    pub send_time: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub location: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageItemContent {
    ///
    #[prost(message, repeated, tag = "1")]
    pub desc: ::prost::alloc::vec::Vec<ItemContentItem>,
    ///
    #[prost(string, tag = "2")]
    pub jump_uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub origin_text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub threshold_lines: i32,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageItemNotice {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageItemPicture {
    ///
    #[prost(message, repeated, tag = "1")]
    pub pictures: ::prost::alloc::vec::Vec<Picture>,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MessagePositionReply {}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagePositionReq {
    ///
    #[prost(int64, tag = "1")]
    pub up_mid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub topic_id: i64,
    ///
    #[prost(string, tag = "3")]
    pub next: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagesReply {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub up_mid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub permission: i64,
    ///
    #[prost(message, repeated, tag = "4")]
    pub message: ::prost::alloc::vec::Vec<Message>,
    ///
    #[prost(message, optional, tag = "5")]
    pub page: ::core::option::Option<
        super::super::super::super::pagination::FeedPaginationReply,
    >,
    ///
    #[prost(message, optional, tag = "6")]
    pub setting: ::core::option::Option<Button>,
    ///
    #[prost(message, optional, tag = "7")]
    pub input: ::core::option::Option<Input>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagesReq {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(message, optional, tag = "2")]
    pub page: ::core::option::Option<
        super::super::super::super::pagination::FeedPagination,
    >,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Picture {
    ///
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(double, tag = "2")]
    pub width: f64,
    ///
    #[prost(double, tag = "3")]
    pub height: f64,
    ///
    #[prost(double, tag = "4")]
    pub size: f64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMessageReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<Message>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMessageReq {
    ///
    #[prost(int64, tag = "1")]
    pub topic_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    ///
    #[prost(map = "string, int64", tag = "3")]
    pub at_name_to_mid: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    ///
    #[prost(message, repeated, tag = "4")]
    pub picture: ::prost::alloc::vec::Vec<Picture>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerReply {
    ///
    #[prost(int64, tag = "1")]
    pub up_mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub top_photo: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub user_info: ::core::option::Option<UserInfo>,
    ///
    #[prost(enumeration = "ServerStatus", tag = "4")]
    pub status: i32,
    ///
    #[prost(string, tag = "5")]
    pub head_word: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "6")]
    pub category: ::prost::alloc::vec::Vec<Category>,
    ///
    #[prost(message, optional, tag = "7")]
    pub create: ::core::option::Option<Button>,
    ///
    #[prost(message, optional, tag = "8")]
    pub online: ::core::option::Option<DialogButton>,
    ///
    #[prost(message, optional, tag = "9")]
    pub setting: ::core::option::Option<Button>,
    ///
    #[prost(message, optional, tag = "10")]
    pub tip: ::core::option::Option<Button>,
    ///
    #[prost(int64, tag = "11")]
    pub permission: i64,
    ///
    #[prost(string, tag = "12")]
    pub night_top_photo: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ServerReq {
    ///
    #[prost(int64, tag = "1")]
    pub up_mid: i64,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ServerStatusReply {}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ServerStatusReq {
    ///
    #[prost(enumeration = "ServerStatus", tag = "1")]
    pub status: i32,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Topic {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "TopicStatus", tag = "3")]
    pub status: i32,
    ///
    #[prost(string, tag = "4")]
    pub unread_count: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub head_icon: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "TopicTailType", tag = "6")]
    pub tail_type: i32,
    ///
    #[prost(string, tag = "7")]
    pub tail_string: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "8")]
    pub tail: ::core::option::Option<Button>,
    ///
    #[prost(bool, tag = "9")]
    pub is_template: bool,
    ///
    #[prost(string, tag = "10")]
    pub route: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicReply {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "TopicStatus", tag = "3")]
    pub status: i32,
    ///
    #[prost(bool, tag = "4")]
    pub name_editable: bool,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TopicReq {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpServer {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub mid: i64,
    ///
    #[prost(enumeration = "UpServerType", tag = "3")]
    pub r#type: i32,
    ///
    #[prost(string, tag = "4")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub unread_count: i64,
    ///
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpServerListReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub server: ::prost::alloc::vec::Vec<UpServer>,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct UpServerListReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub route: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IconPosition {
    ///
    Prefix = 0,
    ///
    Suffix = 1,
}
impl IconPosition {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Prefix => "IconPositionPrefix",
            Self::Suffix => "IconPositionSuffix",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IconPositionPrefix" => Some(Self::Prefix),
            "IconPositionSuffix" => Some(Self::Suffix),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ItemContentType {
    ///
    Unknown = 0,
    ///
    Text = 1,
    ///
    Highlight = 2,
    ///
    Emoji = 3,
}
impl ItemContentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "ItemContentTypeUnknown",
            Self::Text => "ItemContentTypeText",
            Self::Highlight => "ItemContentTypeHighlight",
            Self::Emoji => "ItemContentTypeEmoji",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ItemContentTypeUnknown" => Some(Self::Unknown),
            "ItemContentTypeText" => Some(Self::Text),
            "ItemContentTypeHighlight" => Some(Self::Highlight),
            "ItemContentTypeEmoji" => Some(Self::Emoji),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageActionType {
    ///
    Unknown = 0,
    ///
    Like = 1,
    ///
    Unlike = 2,
}
impl MessageActionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "MessageActionTypeUnknown",
            Self::Like => "MessageActionTypeLike",
            Self::Unlike => "MessageActionTypeUnlike",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MessageActionTypeUnknown" => Some(Self::Unknown),
            "MessageActionTypeLike" => Some(Self::Like),
            "MessageActionTypeUnlike" => Some(Self::Unlike),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageItemType {
    ///
    Unknown = 0,
    ///
    Author = 1,
    ///
    Notice = 2,
    ///
    Content = 3,
    ///
    Picture = 4,
    ///
    Addition = 5,
}
impl MessageItemType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "MessageItemTypeUnknown",
            Self::Author => "MessageItemTypeAuthor",
            Self::Notice => "MessageItemTypeNotice",
            Self::Content => "MessageItemTypeContent",
            Self::Picture => "MessageItemTypePicture",
            Self::Addition => "MessageItemTypeAddition",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MessageItemTypeUnknown" => Some(Self::Unknown),
            "MessageItemTypeAuthor" => Some(Self::Author),
            "MessageItemTypeNotice" => Some(Self::Notice),
            "MessageItemTypeContent" => Some(Self::Content),
            "MessageItemTypePicture" => Some(Self::Picture),
            "MessageItemTypeAddition" => Some(Self::Addition),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageStatus {
    ///
    Normal = 0,
    ///
    Audit = 1,
    ///
    SelfSeen = 2,
    ///
    Delete = 3,
}
impl MessageStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Normal => "MessageStatusNormal",
            Self::Audit => "MessageStatusAudit",
            Self::SelfSeen => "MessageStatusSelfSeen",
            Self::Delete => "MessageStatusDelete",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MessageStatusNormal" => Some(Self::Normal),
            "MessageStatusAudit" => Some(Self::Audit),
            "MessageStatusSelfSeen" => Some(Self::SelfSeen),
            "MessageStatusDelete" => Some(Self::Delete),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageType {
    ///
    None = 0,
    ///
    Text = 1,
    ///
    Image = 2,
}
impl MessageType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "MessageTypeNone",
            Self::Text => "MessageTypeText",
            Self::Image => "MessageTypeImage",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MessageTypeNone" => Some(Self::None),
            "MessageTypeText" => Some(Self::Text),
            "MessageTypeImage" => Some(Self::Image),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServerStatus {
    ///
    Unknown = 0,
    ///
    Online = 1,
    ///
    Offline = 2,
}
impl ServerStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "ServerStatusUnknown",
            Self::Online => "ServerStatusOnline",
            Self::Offline => "ServerStatusOffline",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ServerStatusUnknown" => Some(Self::Unknown),
            "ServerStatusOnline" => Some(Self::Online),
            "ServerStatusOffline" => Some(Self::Offline),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TopicStatus {
    ///
    Unknown = 0,
    ///
    Normal = 1,
    ///
    Audit = 2,
    ///
    Deleted = 3,
    ///
    AuditDeleted = 4,
}
impl TopicStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "TopicStatusUnknown",
            Self::Normal => "TopicStatusNormal",
            Self::Audit => "TopicStatusAudit",
            Self::Deleted => "TopicStatusDeleted",
            Self::AuditDeleted => "TopicStatusAuditDeleted",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TopicStatusUnknown" => Some(Self::Unknown),
            "TopicStatusNormal" => Some(Self::Normal),
            "TopicStatusAudit" => Some(Self::Audit),
            "TopicStatusDeleted" => Some(Self::Deleted),
            "TopicStatusAuditDeleted" => Some(Self::AuditDeleted),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TopicTailType {
    ///
    Unknown = 0,
    ///
    UnreadGray = 1,
    ///
    UnreadRed = 2,
    ///
    Edit = 3,
}
impl TopicTailType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "TopicTailTypeUnknown",
            Self::UnreadGray => "TopicTailTypeUnreadGray",
            Self::UnreadRed => "TopicTailTypeUnreadRed",
            Self::Edit => "TopicTailTypeEdit",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TopicTailTypeUnknown" => Some(Self::Unknown),
            "TopicTailTypeUnreadGray" => Some(Self::UnreadGray),
            "TopicTailTypeUnreadRed" => Some(Self::UnreadRed),
            "TopicTailTypeEdit" => Some(Self::Edit),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UpServerType {
    ///
    Unknown = 0,
    ///
    Create = 1,
    ///
    Up = 2,
    ///
    Follow = 3,
}
impl UpServerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "UpServerTypeUnknown",
            Self::Create => "UpServerTypeCreate",
            Self::Up => "UpServerTypeUp",
            Self::Follow => "UpServerTypeFollow",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UpServerTypeUnknown" => Some(Self::Unknown),
            "UpServerTypeCreate" => Some(Self::Create),
            "UpServerTypeUp" => Some(Self::Up),
            "UpServerTypeFollow" => Some(Self::Follow),
            _ => None,
        }
    }
}