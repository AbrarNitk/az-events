use crate::Config;

#[derive(thiserror::Error, Debug)]
pub enum BodyError {
    #[error("HyperBodyReadError: {}", _0)]
    HyperBodyRead(#[from] hyper::Error),
    #[error("SerdeDeserialize: {}", _0)]
    SerdeDeserialize(#[from] serde_json::Error),
}

pub async fn from_body<T: serde::de::DeserializeOwned>(b: hyper::Body) -> Result<T, BodyError> {
    let b = hyper::body::to_bytes(b).await?;
    Ok(serde_json::from_slice(b.as_ref())?)
}

pub async fn handler(
    config: Config,
    req: hyper::Request<hyper::Body>,
) -> Result<hyper::Response<hyper::Body>, service::errors::RouteError> {
    tracing::info!(
        target = "request",
        method = req.method().as_str(),
        path = req.uri().path()
    );
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/api/event/health/") => {
            let mut response = hyper::Response::new(hyper::Body::from(serde_json::to_string(
                &serde_json::json!({"success": true, "msg": "health is okay"}),
            )?));
            *response.status_mut() = hyper::StatusCode::OK;
            response.headers_mut().append(
                hyper::header::CONTENT_TYPE,
                hyper::http::HeaderValue::from_str("application/json").unwrap(),
            );
            Ok(response)
        }
        (&hyper::Method::POST, "/api/event/") => {
            let (p, body) = req.into_parts();
            let event: service::controller::RequestEvent = from_body(body).await?;
            let mut response = hyper::Response::new(hyper::Body::empty());
            if let Err(err) = service::controller::handle_event(&config, event).await {
                tracing::error!(
                    method = p.method.as_str(),
                    path = p.uri.path(),
                    "error" = err.to_string()
                );
                *response.body_mut() = hyper::Body::from(serde_json::to_string(
                    &serde_json::json!({"success": false, "msg": "something went wrong"}),
                )?);
                *response.status_mut() = hyper::StatusCode::INTERNAL_SERVER_ERROR;
            } else {
                tracing::info!(
                    method = p.method.as_str(),
                    path = p.uri.path(),
                    "success" = "success"
                );
                *response.body_mut() = hyper::Body::from(serde_json::to_string(
                    &serde_json::json!({"success": true, "msg": "event put successfully"}),
                )?);
                *response.status_mut() = hyper::StatusCode::OK;
            }
            response.headers_mut().append(
                hyper::header::CONTENT_TYPE,
                hyper::http::HeaderValue::from_str("application/json").unwrap(),
            );
            Ok(response)
        }
        (&hyper::Method::GET, "/api/events/") => {
            match service::controller::get_events(&config).await {
                Ok(r) => {
                    let success_response = serde_json::json!({
                        "success": true,
                        "data": serde_json::to_value(&r)?
                    });

                    Ok(response(
                        serde_json::to_string(&success_response)?,
                        hyper::StatusCode::OK,
                    ))
                }
                Err(e) => {
                    tracing::error!(message = "get-event-error", err = format!("{:?}", e));
                    let error_response = serde_json::json!({
                        "success": false,
                        "error": {
                            "status": "INTERNAL_SERVER_ERROR",
                            "message": format!("{:?}", e)
                        }
                    });
                    Ok(response(
                        serde_json::to_string(&error_response)?,
                        hyper::StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                }
            }
        }

        _ => Ok(response(
            serde_json::to_string(&serde_json::json!({
                "success": false,
                "error": {
                    "status": "NOT_FOUND",
                    "path": req.uri().path()
                }
            }))
            .unwrap(),
            hyper::StatusCode::NOT_FOUND,
        )),
    }
}

pub fn response(body: String, status: hyper::StatusCode) -> hyper::Response<hyper::Body> {
    let mut response = hyper::Response::new(hyper::Body::from(body));
    *response.status_mut() = status;
    response.headers_mut().append(
        hyper::header::CONTENT_TYPE,
        hyper::http::HeaderValue::from_static("application/json"),
    );
    response
}
