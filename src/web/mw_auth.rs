use crate::error::{MyError, MyError::AuthFailNoAuthTokenCookie};
use crate::response;
use crate::web::token::Claims;
use crate::web::{AUTH_TOKEN, SECRET};
use axum::body::{Body, BoxBody, Bytes, HttpBody};
use axum::extract::{Extension, FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::BoxError;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::collections::HashSet;
use std::convert::Infallible;
use tower_cookies::{Cookie, Cookies};

pub async fn mw_require_auth<B>(cookies: Cookies, req: Request<B>, next: Next<B>) -> Response {
  //next.run(req).await
  if let Some(auth_cookie) = cookies.get(AUTH_TOKEN) {
    let mut validation = Validation::default();
    validation.set_audience(
      std::env::var("AUTH_AUDIENCE")
        .unwrap_or(String::default())
        .split(",")
        .map(String::from)
        .collect::<Vec<String>>()
        .as_slice(),
    );

    let token = decode::<Claims>(
      auth_cookie.value(),
      &DecodingKey::from_secret(SECRET.as_ref()),
      &validation,
    );

    if token.is_err() {
      println!("{:?}", token.err().unwrap());
      return Response::builder()
        .status(401)
        .body(BoxBody::default())
        .unwrap();
    }

    next.run(req).await
  } else {
    Response::builder()
      .status(401)
      .body(BoxBody::default())
      .unwrap()
  }
}

// pub async fn mw_ctx_resolver<B>(
//   _mc: State<ModelController>,
//   cookies: Cookies,
//   mut req: Request<B>,
//   next: Next<B>,
// ) -> Result<Response> {
//   println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

//   let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

//   // Compute Result<Ctx>.
//   let result_ctx = match auth_token.and_then(parse_token) {
//     Ok((user_id, _exp, _sign)) => {
//       // TODO: Token components validations.
//       Ok(Ctx::new(user_id))
//     }
//     Err(e) => Err(e),
//   };

//   // Remove the cookie if something went wrong other than NoAuthTokenCookie.
//   if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
//     cookies.remove(Cookie::named(AUTH_TOKEN))
//   }

//   // Store the ctx_result in the request extension.
//   req.extensions_mut().insert(result_ctx);

//   Ok(next.run(req).await)
// }

// // region:    --- Ctx Extractor
// #[async_trait]
// impl<S: Send + Sync> FromRequestParts<S> for Ctx {
//   type Rejection = Error;

//   async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
//     println!("->> {:<12} - Ctx", "EXTRACTOR");

//     parts
//       .extensions
//       .get::<Result<Ctx>>()
//       .ok_or(Error::AuthFailCtxNotInRequestExt)?
//       .clone()
//   }
// }

// // endregion: --- Ctx Extractor

// /// Parse a token of format `user-[user-id].[expiration].[signature]`
// /// Returns (user_id, expiration, signature)
// fn parse_token(token: String) -> Result<(u64, String, String)> {
//   let (_whole, user_id, exp, sign) = regex_captures!(
//     r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
//     &token
//   )
//   .ok_or(Error::AuthFailTokenWrongFormat)?;

//   let user_id: u64 = user_id
//     .parse()
//     .map_err(|_| Error::AuthFailTokenWrongFormat)?;

//   Ok((user_id, exp.to_string(), sign.to_string()))
// }
