// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientStatsPayload {
    ///
    #[prost(message, optional, tag = "1")]
    pub transport: ::core::option::Option<RtcTransport>,
    ///
    #[prost(message, optional, tag = "2")]
    pub selected_candidate_pair: ::core::option::Option<RtcCandidatePair>,
    ///
    #[prost(message, repeated, tag = "3")]
    pub candidate_pairs: ::prost::alloc::vec::Vec<RtcCandidatePair>,
    ///
    #[prost(message, repeated, tag = "4")]
    pub av_senders: ::prost::alloc::vec::Vec<RtcAvSenderStats>,
    ///
    #[prost(message, repeated, tag = "5")]
    pub av_receivers: ::prost::alloc::vec::Vec<RtcAvReceiverStats>,
    ///
    #[prost(message, repeated, tag = "6")]
    pub data_channels: ::prost::alloc::vec::Vec<RtcDataChannel>,
    ///
    #[prost(int64, tag = "7")]
    pub generate_ts: i64,
    ///
    #[prost(string, tag = "8")]
    pub business_name: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcAvReceiverStats {
    ///
    #[prost(int64, tag = "1")]
    pub uid: i64,
    ///
    #[prost(message, repeated, tag = "2")]
    pub video: ::prost::alloc::vec::Vec<RtcVideoReceiverInfo>,
    ///
    #[prost(message, repeated, tag = "3")]
    pub audio: ::prost::alloc::vec::Vec<RtcAudioReceiverInfo>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcAvSenderStats {
    ///
    #[prost(int64, tag = "1")]
    pub uid: i64,
    ///
    #[prost(message, repeated, tag = "2")]
    pub video: ::prost::alloc::vec::Vec<RtcVideoSenderInfo>,
    ///
    #[prost(message, repeated, tag = "3")]
    pub audio: ::prost::alloc::vec::Vec<RtcAudioSenderInfo>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcAudioReceiverInfo {
    ///
    #[prost(int32, tag = "1")]
    pub ssrc: i32,
    ///
    #[prost(double, tag = "2")]
    pub audio_level: f64,
    ///
    #[prost(int64, tag = "3")]
    pub bytes_received: i64,
    ///
    #[prost(int64, tag = "4")]
    pub concealed_samples: i64,
    ///
    #[prost(int64, tag = "5")]
    pub concealment_events: i64,
    ///
    #[prost(int64, tag = "6")]
    pub silent_concealed_samples: i64,
    ///
    #[prost(double, tag = "7")]
    pub estimated_playout_timestamp: f64,
    ///
    #[prost(int64, tag = "8")]
    pub fec_packets_discarded: i64,
    ///
    #[prost(int64, tag = "9")]
    pub fec_packets_received: i64,
    ///
    #[prost(int64, tag = "10")]
    pub header_bytes_received: i64,
    ///
    #[prost(int64, tag = "11")]
    pub inserted_samples_for_deceleration: i64,
    ///
    #[prost(double, tag = "12")]
    pub jitter: f64,
    ///
    #[prost(double, tag = "13")]
    pub jitter_buffer_delay: f64,
    ///
    #[prost(int64, tag = "14")]
    pub jitter_buffer_emitted_count: i64,
    ///
    #[prost(double, tag = "15")]
    pub last_packet_received_timestamp: f64,
    ///
    #[prost(int64, tag = "16")]
    pub packets_discarded: i64,
    ///
    #[prost(int64, tag = "17")]
    pub packets_lost: i64,
    ///
    #[prost(int64, tag = "18")]
    pub packets_received: i64,
    ///
    #[prost(int64, tag = "19")]
    pub removed_samples_for_acceleration: i64,
    ///
    #[prost(double, tag = "20")]
    pub total_audio_energy: f64,
    ///
    #[prost(double, tag = "21")]
    pub total_samples_duration: f64,
    ///
    #[prost(int64, tag = "22")]
    pub total_samples_received: i64,
    ///
    #[prost(int64, tag = "23")]
    pub delayed_packet_outage_samples: i64,
    ///
    #[prost(int32, tag = "24")]
    pub interruption_count: i32,
    ///
    #[prost(double, tag = "25")]
    pub total_interruption_duration: f64,
    ///
    #[prost(int64, tag = "26")]
    pub jitter_buffer_flushes: i64,
    ///
    #[prost(double, tag = "27")]
    pub jitter_buffer_target_delay: f64,
    ///
    #[prost(double, tag = "28")]
    pub relative_packet_arrival_delay: f64,
    ///
    #[prost(int32, tag = "29")]
    pub stream_id: i32,
    ///
    #[prost(bool, tag = "30")]
    pub mute: bool,
    ///
    #[prost(string, tag = "31")]
    pub track_id: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "32")]
    pub uid: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcAudioSenderInfo {
    ///
    #[prost(int32, tag = "1")]
    pub ssrc: i32,
    ///
    #[prost(int64, tag = "2")]
    pub bytes_sent: i64,
    ///
    #[prost(int64, tag = "3")]
    pub header_bytes_sent: i64,
    ///
    #[prost(int32, tag = "4")]
    pub nack_count: i32,
    ///
    #[prost(int64, tag = "5")]
    pub packets_sent: i64,
    ///
    #[prost(int64, tag = "6")]
    pub retransmitted_bytes_sent: i64,
    ///
    #[prost(int64, tag = "7")]
    pub retransmitted_packets_sent: i64,
    ///
    #[prost(double, tag = "8")]
    pub fraction_lost: f64,
    ///
    #[prost(double, tag = "9")]
    pub jitter: f64,
    ///
    #[prost(int64, tag = "10")]
    pub packets_lost: i64,
    ///
    #[prost(double, tag = "11")]
    pub round_trip_time: f64,
    ///
    #[prost(int64, tag = "12")]
    pub round_trip_time_measurements: i64,
    ///
    #[prost(double, tag = "13")]
    pub total_round_trip_time: f64,
    ///
    #[prost(double, tag = "14")]
    pub audio_level: f64,
    ///
    #[prost(double, tag = "15")]
    pub total_audio_energy: f64,
    ///
    #[prost(double, tag = "16")]
    pub total_samples_duration: f64,
    ///
    #[prost(int32, tag = "17")]
    pub stream_id: i32,
    ///
    #[prost(bool, tag = "18")]
    pub mute: bool,
    ///
    #[prost(string, tag = "19")]
    pub remote_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "20")]
    pub media_source_id: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcCandidatePair {
    ///
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub state: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub priority: i64,
    ///
    #[prost(message, optional, tag = "4")]
    pub local_candidate: ::core::option::Option<RtcLocalCandidate>,
    ///
    #[prost(message, optional, tag = "5")]
    pub remote_candidate: ::core::option::Option<RtcRemoteCandidate>,
    ///
    #[prost(float, tag = "6")]
    pub available_outgoing_bitrate: f32,
    ///
    #[prost(float, tag = "7")]
    pub available_incoming_bitrate: f32,
    ///
    #[prost(int64, tag = "8")]
    pub bytes_discarded_on_send: i64,
    ///
    #[prost(int64, tag = "9")]
    pub bytes_received: i64,
    ///
    #[prost(int64, tag = "10")]
    pub bytes_sent: i64,
    ///
    #[prost(int64, tag = "11")]
    pub consent_requests_sent: i64,
    ///
    #[prost(float, tag = "12")]
    pub current_round_trip_time: f32,
    ///
    #[prost(bool, tag = "13")]
    pub nominated: bool,
    ///
    #[prost(int64, tag = "14")]
    pub packets_discarded_on_send: i64,
    ///
    #[prost(int64, tag = "15")]
    pub packets_received: i64,
    ///
    #[prost(int64, tag = "16")]
    pub packets_sent: i64,
    ///
    #[prost(int64, tag = "17")]
    pub requests_received: i64,
    ///
    #[prost(int64, tag = "18")]
    pub requests_sent: i64,
    ///
    #[prost(int64, tag = "19")]
    pub responses_received: i64,
    ///
    #[prost(int64, tag = "20")]
    pub responses_sent: i64,
    ///
    #[prost(float, tag = "21")]
    pub total_round_trip_time: f32,
    ///
    #[prost(bool, tag = "22")]
    pub writable: bool,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcDataChannel {
    ///
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub label: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub state: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub protocol: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub bytes_sent: i64,
    ///
    #[prost(int64, tag = "6")]
    pub bytes_received: i64,
    ///
    #[prost(int32, tag = "7")]
    pub messages_sent: i32,
    ///
    #[prost(int32, tag = "8")]
    pub messages_received: i32,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcLocalCandidate {
    ///
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub network_type: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub ip: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub address: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub port: i32,
    ///
    #[prost(string, tag = "6")]
    pub protocol: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub candidate_type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "8")]
    pub priority: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcRemoteCandidate {
    ///
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub ip: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub port: i32,
    ///
    #[prost(string, tag = "5")]
    pub protocol: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub candidate_type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub priority: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcTransport {
    ///
    #[prost(string, tag = "1")]
    pub selected_candidate_pair_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub dtls_cipher: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub dtls_state: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub bytes_received: i64,
    ///
    #[prost(int64, tag = "5")]
    pub bytes_sent: i64,
    ///
    #[prost(int64, tag = "6")]
    pub packets_received: i64,
    ///
    #[prost(int64, tag = "7")]
    pub packets_sent: i64,
    ///
    #[prost(int32, tag = "8")]
    pub selected_candidate_pair_changes: i32,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcVideoReceiverInfo {
    ///
    #[prost(int32, tag = "1")]
    pub ssrc: i32,
    ///
    #[prost(string, tag = "2")]
    pub decoder_implementation: ::prost::alloc::string::String,
    ///
    #[prost(double, tag = "3")]
    pub estimated_playout_timestamp: f64,
    ///
    #[prost(int32, tag = "4")]
    pub frame_width: i32,
    ///
    #[prost(int32, tag = "5")]
    pub frame_height: i32,
    ///
    #[prost(int32, tag = "6")]
    pub frames_decoded: i32,
    ///
    #[prost(int32, tag = "7")]
    pub frames_dropped: i32,
    ///
    #[prost(int32, tag = "8")]
    pub frames_received: i32,
    ///
    #[prost(int64, tag = "9")]
    pub bytes_received: i64,
    ///
    #[prost(int64, tag = "10")]
    pub header_bytes_received: i64,
    ///
    #[prost(int64, tag = "11")]
    pub packets_lost: i64,
    ///
    #[prost(int64, tag = "12")]
    pub packets_received: i64,
    ///
    #[prost(double, tag = "13")]
    pub jitter: f64,
    ///
    #[prost(double, tag = "14")]
    pub jitter_buffer_delay: f64,
    ///
    #[prost(int64, tag = "15")]
    pub jitter_buffer_emitted_count: i64,
    ///
    #[prost(int32, tag = "16")]
    pub key_frames_decoded: i32,
    ///
    #[prost(double, tag = "17")]
    pub last_packet_received_timestamp: f64,
    ///
    #[prost(int32, tag = "18")]
    pub fir_count: i32,
    ///
    #[prost(int32, tag = "19")]
    pub nack_count: i32,
    ///
    #[prost(int32, tag = "20")]
    pub pli_count: i32,
    ///
    #[prost(int64, tag = "21")]
    pub qp_sum: i64,
    ///
    #[prost(double, tag = "22")]
    pub total_decode_time: f64,
    ///
    #[prost(double, tag = "23")]
    pub total_inter_frame_delay: f64,
    ///
    #[prost(double, tag = "24")]
    pub total_squared_inter_frame_delay: f64,
    ///
    #[prost(int32, tag = "25")]
    pub freeze_count: i32,
    ///
    #[prost(int32, tag = "26")]
    pub pause_count: i32,
    ///
    #[prost(double, tag = "27")]
    pub sum_of_squared_frames_duration: f64,
    ///
    #[prost(double, tag = "28")]
    pub total_freezes_duration: f64,
    ///
    #[prost(double, tag = "29")]
    pub total_pauses_duration: f64,
    ///
    #[prost(int64, tag = "30")]
    pub first_frame_cost: i64,
    ///
    #[prost(int32, tag = "31")]
    pub stream_id: i32,
    ///
    #[prost(bool, tag = "32")]
    pub mute: bool,
    ///
    #[prost(int64, tag = "33")]
    pub freeze_samples: i64,
    ///
    #[prost(int64, tag = "34")]
    pub freeze_duration: i64,
    ///
    #[prost(string, tag = "35")]
    pub track_id: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "36")]
    pub uid: i64,
    ///
    #[prost(double, tag = "37")]
    pub frames_per_second: f64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtcVideoSenderInfo {
    ///
    #[prost(int32, tag = "1")]
    pub ssrc: i32,
    ///
    #[prost(string, tag = "2")]
    pub encoder_implementation: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub bytes_sent: i64,
    ///
    #[prost(int32, tag = "4")]
    pub fir_count: i32,
    ///
    #[prost(int32, tag = "5")]
    pub frame_height: i32,
    ///
    #[prost(int32, tag = "6")]
    pub frame_width: i32,
    ///
    #[prost(int32, tag = "7")]
    pub frames_encoded: i32,
    ///
    #[prost(int32, tag = "8")]
    pub frames_sent: i32,
    ///
    #[prost(int32, tag = "9")]
    pub header_bytes_sent: i32,
    ///
    #[prost(int32, tag = "10")]
    pub huge_frames_sent: i32,
    ///
    #[prost(int32, tag = "11")]
    pub key_frames_encoded: i32,
    ///
    #[prost(int32, tag = "12")]
    pub nack_count: i32,
    ///
    #[prost(int64, tag = "13")]
    pub packets_sent: i64,
    ///
    #[prost(int32, tag = "14")]
    pub pli_count: i32,
    ///
    #[prost(int64, tag = "15")]
    pub retransmitted_bytes_sent: i64,
    ///
    #[prost(int64, tag = "16")]
    pub retransmitted_packets_sent: i64,
    ///
    #[prost(double, tag = "17")]
    pub total_encode_time: f64,
    ///
    #[prost(int64, tag = "18")]
    pub total_encoded_bytes_target: i64,
    ///
    #[prost(double, tag = "19")]
    pub total_packet_send_delay: f64,
    ///
    #[prost(int64, tag = "20")]
    pub qp_sum: i64,
    ///
    #[prost(string, tag = "21")]
    pub quality_limitation_reason: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "22")]
    pub quality_limitation_resolution_changes: i32,
    ///
    #[prost(double, tag = "23")]
    pub bandwidth_q_l_durations: f64,
    ///
    #[prost(double, tag = "24")]
    pub cpu_q_l_durations: f64,
    ///
    #[prost(double, tag = "25")]
    pub none_q_l_durations: f64,
    ///
    #[prost(double, tag = "26")]
    pub other_q_l_durations: f64,
    ///
    #[prost(double, tag = "27")]
    pub fraction_lost: f64,
    ///
    #[prost(double, tag = "28")]
    pub jitter: f64,
    ///
    #[prost(int64, tag = "29")]
    pub packets_lost: i64,
    ///
    #[prost(double, tag = "30")]
    pub round_trip_time: f64,
    ///
    #[prost(int64, tag = "31")]
    pub round_trip_time_measurements: i64,
    ///
    #[prost(double, tag = "32")]
    pub total_round_trip_time: f64,
    ///
    #[prost(int32, tag = "33")]
    pub stream_id: i32,
    ///
    #[prost(bool, tag = "34")]
    pub mute: bool,
    ///
    #[prost(string, tag = "35")]
    pub content_type: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "36")]
    pub remote_id: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "37")]
    pub frames: i32,
    ///
    #[prost(double, tag = "38")]
    pub framerate_input: f64,
    ///
    #[prost(int32, tag = "39")]
    pub framerate_sent: i32,
    ///
    #[prost(int32, tag = "40")]
    pub avg_encode_ms: i32,
    ///
    #[prost(int32, tag = "41")]
    pub frames_dropped_by_capturer: i32,
    ///
    #[prost(int32, tag = "42")]
    pub frames_dropped_by_congestion_window: i32,
    ///
    #[prost(int32, tag = "43")]
    pub frames_dropped_by_encoder: i32,
    ///
    #[prost(int32, tag = "44")]
    pub frames_dropped_by_encoder_queue: i32,
    ///
    #[prost(int32, tag = "45")]
    pub frames_dropped_by_rate_limiter: i32,
    ///
    #[prost(double, tag = "46")]
    pub target_bitrate: f64,
}