// Copyright (c) 2024 by Alibaba.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};

use crate::config::HttpServerConfig;

pub fn tls_config(config: &HttpServerConfig) -> Result<openssl::ssl::SslAcceptorBuilder> {
    use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

    let cert_file = config
        .certificate
        .as_ref()
        .ok_or_else(|| anyhow!("Missing certificate"))?;

    let key_file = config
        .private_key
        .as_ref()
        .ok_or_else(|| anyhow!("Missing private key"))?;

    let mut builder = SslAcceptor::mozilla_modern(SslMethod::tls())?;
    builder.set_private_key_file(key_file, SslFiletype::PEM)?;
    builder.set_certificate_chain_file(cert_file)?;

    Ok(builder)
}

pub fn tls_config_rustls(config: &HttpServerConfig) -> Result<rustls::server::ServerConfig> {
    use rustls::server::ServerConfig;
    use rustls::{Certificate, PrivateKey};
    use rustls_pemfile::{certs, pkcs8_private_keys};
    use std::fs::File;
    use std::io::BufReader;

    let cert_path = config.certificate.as_ref()
        .expect("Missing certificate");
    let key = config.private_key.as_ref()
        .expect("Missing PrvKey");
    let cert_file = &mut BufReader::new(File::open(cert_path.to_str().unwrap())?);
    let pk_file = &mut BufReader::new(File::open(key.to_str().unwrap())?);
    let cert = certs(cert_file)?.into_iter()
        .map(Certificate).collect();
    let mut pkey: Vec<PrivateKey> = pkcs8_private_keys(pk_file)?.into_iter()
        .map(PrivateKey).collect();
    let serv_config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert, pkey.remove(0))
        .expect("TLS Config builder failed");

    Ok(serv_config)
}
