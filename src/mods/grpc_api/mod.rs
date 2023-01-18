pub mod grpc_headers {
    include!("bilibili.metadata.device.rs");
    include!("bilibili.metadata.fawkes.rs");
    include!("bilibili.metadata.locale.rs");
    include!("bilibili.metadata.network.rs");
    include!("bilibili.metadata.parabox.rs");
    include!("bilibili.metadata.restriction.rs");
}
pub mod grpc_metadata {
    include!("bilibili.metadata.rs");
}

pub mod grpc_playurl_v1 {
    include!("bilibili.pgc.gateway.player.v2.rs");
}
pub mod grpc_playurl_v2 {
    include!("bilibili.pgc.gateway.player.v2.rs");
}

pub mod grpc_search {
    include!("bilibili.polymer.app.search.v1.rs");
}

pub mod bilibili_pagination {
    include!("bilibili.pagination.rs");
}

pub mod bilibili_app_archive_middleware_v1 {
    include!("bilibili.app.archive.middleware.v1.rs");
}

pub mod server_auth_v1 {
    include!("server.auth.v1.rs");
}