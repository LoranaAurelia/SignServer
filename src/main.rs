// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2025 Moew72 <Moew72@proton.me>

mod config;
mod error;
mod service;
mod sign;

use ntex::web;

use crate::{
    config::CONFIG,
    sign::{load_module, set_module_path, set_offset, unload_module},
};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    set_module_path(CONFIG.module_path.clone());
    set_offset(CONFIG.offset);
    load_module();

    web::HttpServer::new(|| {
        web::App::new()
            .service(service::sign_get)
            .service(service::sign_post)
            .service(service::appinfo)
            .service(service::ping)
    })
    .bind(CONFIG.listen.clone())?
    .run()
    .await?;

    unload_module();

    Ok(())
}
