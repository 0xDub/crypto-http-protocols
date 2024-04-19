use log::{warn, error};
use std::sync::Arc;
use std::future;


static ALPN: &[u8] = b"h3";

pub fn get_roots() -> rustls::RootCertStore {
    let mut roots = rustls::RootCertStore::empty();
    match rustls_native_certs::load_native_certs() {
        Ok(certs) => {
            for cert in certs {
                if let Err(e) = roots.add(&rustls::Certificate(cert.0)) {
                    warn!("[!] get_roots |:| Failed to parse trust anchor: {}", e);
                }
            }
        }
        Err(e) => {
            warn!("[!] get_roots |:| Couldn't load any default trust roots: {}", e);
        }
    };

    roots
}

pub async fn parse_uri(uri: &str) -> Option<(http::Uri, http::uri::Authority, std::net::SocketAddr)> {
    let uri = uri.parse::<http::Uri>().expect("[!] parse_url |:| Failed to parse URI");
    if uri.scheme() != Some(&http::uri::Scheme::HTTPS) {
        return None;
    }

    let auth = uri.authority().ok_or("[!] parse_url |:| URI must have a host").expect("[!] parse_url |:| URI must have a host").clone();
    let port = auth.port_u16().unwrap_or(443);
    let addr = tokio::net::lookup_host((auth.host(), port))
        .await.expect("[!] parse_url |:| DNS lookup failed")
        .next()
        .ok_or("[!] parse_url |:| DNS found no addresses").expect("[!] parse_url |:| DNS found no addresses");

    return Some((uri, auth, addr));
}

pub async fn get_config(roots: rustls::RootCertStore) -> rustls::ClientConfig {
    let mut tls_config = rustls::ClientConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[&rustls::version::TLS13]).expect("[!] get_config |:| Failed to set protocol versions")
        .with_root_certificates(roots)
        .with_no_client_auth();
    tls_config.enable_early_data = true;
    tls_config.alpn_protocols = vec![ALPN.into()];

    tls_config
}

pub fn get_quinn_endpoint(tls_config: rustls::ClientConfig) -> quinn::Endpoint {
    let mut client_endpoint = h3_quinn::quinn::Endpoint::client("[::]:0".parse().unwrap()).expect("[!] get_config |:| Failed to create client endpoint");
    let client_config = quinn::ClientConfig::new(Arc::new(tls_config));
    client_endpoint.set_default_client_config(client_config);

    client_endpoint
}



pub async fn test_h3(uri: &str, tls_config: rustls::ClientConfig) -> Option<(http::Version, http::StatusCode)> {
    if let Some((uri, auth, addr)) = parse_uri(uri).await {
        let client_endpoint = get_quinn_endpoint(tls_config);
        let conn = client_endpoint.connect(addr, auth.host()).expect("[!] Failed to connect to client endpoint").await;
        if let Ok(conn) = conn {

            let quinn_conn = h3_quinn::Connection::new(conn);
            let (mut driver, mut send_request) = h3::client::new(quinn_conn).await.expect("[!] Failed to create h3 client");
            let drive = async move {
                let x = future::poll_fn(|cx| driver.poll_close(cx)).await;
                match x {
                    Ok(()) => {}
                    Err(e) => {
                        error!("[!] Drive failed: {}", e);
                    }
                }
            };

            let request = async move {
                let req = http::Request::builder().uri(uri).body(());
                match req {
                    Ok(req) => {
                        let stream = send_request.send_request(req).await;
                        match stream {
                            Ok(mut stream) => {
                                let stream_finish_result = stream.finish().await;
                                match stream_finish_result {
                                    Ok(()) => {
                                        let resp = stream.recv_response().await;
                                        match resp {
                                            Ok(resp) => {
                                                Some((resp.version(), resp.status()))
                                            }
                                            Err(_) => {
                                                None
                                            }
                                        }
                                    }
                                    Err(_) => {
                                        None
                                    }
                                }
                            }
                            Err(_) => {
                                return None;
                            }
                        }
                    }
                    Err(_) => {
                        return None;
                    }
                }
            };

            let (req_res, _) = tokio::join!(request, drive);
            client_endpoint.wait_idle().await;

            if let Some((version, status_code)) = req_res {
                Some((version, status_code))
            } else {
                None
            }
        } else {
            return None;
        }
    } else {
        None
    }

}