// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorInfo {
    ///
    #[prost(string, tag = "1")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub mid: i64,
    ///
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub relation_info: ::core::option::Option<RelationInfo>,
    ///
    #[prost(message, optional, tag = "5")]
    pub live_info: ::core::option::Option<LiveInfo>,
    ///
    #[prost(string, tag = "6")]
    pub link: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub right_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub right_icon_dark: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub nickname_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub icon: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bottom {
    ///
    #[prost(message, optional, tag = "1")]
    pub module_buttom: ::core::option::Option<
        super::super::super::super::app::dynamic::v2::ModuleButtom,
    >,
    ///
    #[prost(
        enumeration = "super::super::super::super::app::dynamic::v2::DynModuleType",
        tag = "2"
    )]
    pub module_type: i32,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IconInfo {
    ///
    #[prost(string, tag = "1")]
    pub icon_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub icon_night_url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub icon_width: i32,
    ///
    #[prost(int32, tag = "4")]
    pub icon_height: i32,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpCity {
    ///
    #[prost(message, optional, tag = "1")]
    pub module_copyright: ::core::option::Option<
        super::super::super::super::app::dynamic::v2::ModuleCopyright,
    >,
    ///
    #[prost(
        enumeration = "super::super::super::super::app::dynamic::v2::DynModuleType",
        tag = "2"
    )]
    pub module_type: i32,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemInfoDescVo {
    ///
    #[prost(int64, tag = "1")]
    pub items_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub price: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub price_symbol: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub cache: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub same_kind_desc: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub count: i32,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveInfo {
    ///
    #[prost(int32, tag = "1")]
    pub status: i32,
    ///
    #[prost(string, tag = "2")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub dark_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub live_link: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MallBenefit {
    ///
    #[prost(message, repeated, tag = "1")]
    pub mall_tags: ::prost::alloc::vec::Vec<MallTagInfo>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MallTagInfo {
    ///
    #[prost(string, tag = "1")]
    pub tag_type: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub tag_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub icon: ::core::option::Option<IconInfo>,
    ///
    #[prost(message, optional, tag = "5")]
    pub name_suffix_icon: ::core::option::Option<IconInfo>,
    ///
    #[prost(string, tag = "6")]
    pub url: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpusDetailReq {
    ///
    #[prost(int64, tag = "1")]
    pub dyn_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub ad_extra: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub local_time: i32,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpusDetailResp {
    ///
    #[prost(message, optional, tag = "1")]
    pub opus_item: ::core::option::Option<
        super::super::super::super::app::dynamic::v2::OpusItem,
    >,
    ///
    #[prost(message, optional, tag = "2")]
    pub title: ::core::option::Option<TitleInfo>,
    ///
    #[prost(message, optional, tag = "3")]
    pub author_info: ::core::option::Option<AuthorInfo>,
    ///
    #[prost(message, optional, tag = "4")]
    pub ip_city: ::core::option::Option<IpCity>,
    ///
    #[prost(message, optional, tag = "5")]
    pub bottom: ::core::option::Option<Bottom>,
    ///
    #[prost(message, optional, tag = "6")]
    pub item_info_desc_v_o: ::core::option::Option<ItemInfoDescVo>,
    ///
    #[prost(message, repeated, tag = "7")]
    pub items_info_v_o_s: ::prost::alloc::vec::Vec<ItemInfoDescVo>,
    ///
    #[prost(message, optional, tag = "8")]
    pub pic: ::core::option::Option<Pic>,
    ///
    #[prost(int64, tag = "9")]
    pub pub_time: i64,
    ///
    #[prost(message, optional, tag = "10")]
    pub topic_info: ::core::option::Option<TopicInfo>,
    ///
    #[prost(string, tag = "11")]
    pub item_scene: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "12")]
    pub reserve_info: ::core::option::Option<ReserveInfo>,
    ///
    #[prost(message, optional, tag = "13")]
    pub share_info: ::core::option::Option<ShareInfo>,
    ///
    #[prost(message, optional, tag = "14")]
    pub mall_benefit: ::core::option::Option<MallBenefit>,
    ///
    #[prost(map = "string, string", tag = "15")]
    pub track_map: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pic {
    ///
    #[prost(message, repeated, tag = "1")]
    pub pics: ::prost::alloc::vec::Vec<PicInfo>,
    ///
    #[prost(string, tag = "2")]
    pub style: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PicInfo {
    ///
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
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
pub struct RelationInfo {
    ///
    #[prost(int32, tag = "1")]
    pub status: i32,
    ///
    #[prost(string, tag = "2")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub dark_icon: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveInfo {
    ///
    #[prost(int64, tag = "1")]
    pub sid: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub total: i64,
    ///
    #[prost(string, tag = "4")]
    pub total_str: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub live_time_capsule: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub live_time_float: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "UpActReserveRelationLotteryType", tag = "7")]
    pub lottery_type: i32,
    ///
    #[prost(message, optional, tag = "8")]
    pub prize_info: ::core::option::Option<UpActReserveRelationPrizeInfo>,
    ///
    #[prost(string, tag = "9")]
    pub up_name_str: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub avatar: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "11")]
    pub is_follow: i64,
    ///
    #[prost(string, tag = "12")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "13")]
    pub live_plan_start_time: i64,
    ///
    #[prost(enumeration = "ReserveState", tag = "14")]
    pub state: i32,
    ///
    #[prost(map = "int32, string", tag = "15")]
    pub state_map: ::std::collections::HashMap<i32, ::prost::alloc::string::String>,
    ///
    #[prost(string, tag = "16")]
    pub live_room_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "17")]
    pub prize_info_text: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "18")]
    pub room_id: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareInfo {
    ///
    #[prost(string, tag = "1")]
    pub share_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub share_origin: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub oid: i64,
    ///
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub sub_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub img_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub link_url: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "8")]
    pub share: bool,
    ///
    #[prost(string, tag = "9")]
    pub out_link_url: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TitleInfo {
    ///
    #[prost(message, repeated, tag = "1")]
    pub modules: ::prost::alloc::vec::Vec<
        super::super::super::super::app::dynamic::v2::Module,
    >,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicInfo {
    ///
    #[prost(int64, tag = "1")]
    pub topic_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub topic_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub jump_url: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpActReserveRelationPrizeInfo {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub jump_url: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReserveState {
    ///
    Invalid = 0,
    ///
    Reserve = 1,
    ///
    PrizeReserve = 2,
    ///
    CancelReserve = 3,
    ///
    Cancel = 4,
    ///
    PrizeCancel = 5,
    ///
    Going = 6,
    ///
    End = 7,
    ///
    Expired = -2,
    ///
    Canceled = -1,
}
impl ReserveState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Invalid => "INVALID",
            Self::Reserve => "Reserve",
            Self::PrizeReserve => "PrizeReserve",
            Self::CancelReserve => "CancelReserve",
            Self::Cancel => "Cancel",
            Self::PrizeCancel => "PrizeCancel",
            Self::Going => "Going",
            Self::End => "End",
            Self::Expired => "Expired",
            Self::Canceled => "Canceled",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID" => Some(Self::Invalid),
            "Reserve" => Some(Self::Reserve),
            "PrizeReserve" => Some(Self::PrizeReserve),
            "CancelReserve" => Some(Self::CancelReserve),
            "Cancel" => Some(Self::Cancel),
            "PrizeCancel" => Some(Self::PrizeCancel),
            "Going" => Some(Self::Going),
            "End" => Some(Self::End),
            "Expired" => Some(Self::Expired),
            "Canceled" => Some(Self::Canceled),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UpActReserveRelationLotteryType {
    ///
    Default = 0,
    ///
    Cron = 1,
}
impl UpActReserveRelationLotteryType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Default => "UpActReserveRelationLotteryTypeDefault",
            Self::Cron => "UpActReserveRelationLotteryTypeCron",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UpActReserveRelationLotteryTypeDefault" => Some(Self::Default),
            "UpActReserveRelationLotteryTypeCron" => Some(Self::Cron),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod opus_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct OpusClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> OpusClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OpusClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            OpusClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        ///
        pub async fn opus_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::OpusDetailReq>,
        ) -> std::result::Result<tonic::Response<super::OpusDetailResp>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.mall.tab3.dynamic.v1.Opus/OpusDetail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.mall.tab3.dynamic.v1.Opus", "OpusDetail"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod opus_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with OpusServer.
    #[async_trait]
    pub trait Opus: std::marker::Send + std::marker::Sync + 'static {
        ///
        async fn opus_detail(
            &self,
            request: tonic::Request<super::OpusDetailReq>,
        ) -> std::result::Result<tonic::Response<super::OpusDetailResp>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct OpusServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> OpusServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for OpusServer<T>
    where
        T: Opus,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/bilibili.mall.tab3.dynamic.v1.Opus/OpusDetail" => {
                    #[allow(non_camel_case_types)]
                    struct OpusDetailSvc<T: Opus>(pub Arc<T>);
                    impl<T: Opus> tonic::server::UnaryService<super::OpusDetailReq>
                    for OpusDetailSvc<T> {
                        type Response = super::OpusDetailResp;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpusDetailReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Opus>::opus_detail(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = OpusDetailSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(empty_body());
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for OpusServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "bilibili.mall.tab3.dynamic.v1.Opus";
    impl<T> tonic::server::NamedService for OpusServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
