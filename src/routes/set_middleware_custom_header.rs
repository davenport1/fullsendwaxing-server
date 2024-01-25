use axum::{
    extract::Request, 
    //http::HeaderMap, 
    middleware::Next, 
    response::Response,
};

use hyper::StatusCode;
use super::read_middleware_custom_header::HeaderMessage;



pub async fn set_middleware_custom_header(
    // headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let message = headers
        .get("message")
        .ok_or_else(||  StatusCode::BAD_REQUEST)?; // ? if there is an error then return bad request side of result

    let message = message
        .to_str()
        .map_err(|_error| { StatusCode::BAD_REQUEST })?
        .to_owned();
    
    request.extensions_mut().insert(HeaderMessage(message.to_owned())); 

    Ok(next.run(request).await)
}


// Result<Happy Path, Sad Path>