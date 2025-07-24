use actix_web::body::{BoxBody, EitherBody, MessageBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use actix_web::http::header::HeaderValue;
use actix_web::{Error, HttpResponse};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::future::ready;
use std::time::{Duration, SystemTime};

pub struct Identity {
    token: Claim,
}

impl Default for Identity {
    fn default() -> Self {
        Identity {
            token: Claim::default(),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for Identity
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody,
{
    type Response = ServiceResponse<EitherBody<B,BoxBody>>;
    type Error = Error;
    type Transform = IdentityMiddleWare<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(IdentityMiddleWare { service }))
    }
}

pub struct IdentityMiddleWare<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for IdentityMiddleWare<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody,
{
    type Response = ServiceResponse<EitherBody<B,BoxBody>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req.headers().get("Authorization");
        if let Some(value) = token
            && check_token(value)
        {
            let next = self.service.call(req);
            return Box::pin(async move {
                let res = next.await?.map_into_right_body();
                Ok(res)
            });
        }
        let http_res = HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish()
            .map_into_right_body();
        let mut service_res = ServiceResponse::new(req.into_parts().0, http_res);
        Box::pin(async move { Ok(service_res) })
    }
}

fn check_token(value: &HeaderValue) -> bool {
    if let Ok(token) = value.to_str() {
        let claim = Claim::try_from(token.to_string());
        if claim.is_err() {
            false
        } else {
            claim.unwrap().is_exp()
        }
    } else {
        false
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    //aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
                //sub: String, // Optional. Subject (whom token refers to)
}

impl Default for Claim {
    fn default() -> Self {
        let iat = calculate_exp(Duration::from_secs(0));
        Claim {
            //aud: "app".to_string(),
            exp: 0,
            iat,
            iss: "coco".to_string(),
            //sub: "simple-file-server".to_string(),
        }
    }
}
impl Claim {
    pub fn generate_token(&mut self, exp: Duration) -> String {
        let exp = calculate_exp(exp);
        self.exp = exp;
        let token = encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .unwrap();
        token
    }

    #[inline]
    pub fn is_exp(&self) -> bool {
        self.exp < calculate_exp(Duration::from_secs(0))
    }
}

impl TryFrom<String> for Claim {
    type Error = jsonwebtoken::errors::Error;

    fn try_from(token: String) -> Result<Self, Self::Error> {
        let claim = decode::<Claim>(
            &token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        )?;
        Ok(claim.claims)
    }
}

fn calculate_exp(duration: Duration) -> usize {
    let now = SystemTime::now();
    let since_the_epoch = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Calculate exp failed");
    let exp = since_the_epoch + duration;
    exp.as_secs() as usize
}

#[cfg(test)]
mod tests {
    use crate::middleware::identity::Claim;
    use claim::{assert_err, assert_ok};
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_generate_token() {
        let mut claim = Claim::default();
        let token = claim.generate_token(Duration::from_secs(60));
        println!("{}", token);
    }

    #[test]
    fn token_is_valid() {
        let mut claim = Claim::default();
        let token = claim.generate_token(Duration::from_secs(60));
        let parsed_claim = Claim::try_from(token);
        assert_ok!(parsed_claim);
    }

    #[test]
    fn token_is_invalid() {
        let mut claim = Claim::default();
        let mut token = claim.generate_token(Duration::from_secs(60));
        token.push_str("a");
        let parsed_claim = Claim::try_from(token);
        assert_err!(parsed_claim);
    }

    #[test]
    fn token_is_exp() {
        let mut claim = Claim::default();
        let token = claim.generate_token(Duration::from_secs(0));
        sleep(Duration::from_secs(1));
        let parsed_claim = Claim::try_from(token).unwrap();
        assert!(parsed_claim.is_exp());
    }

    #[test]
    fn token_is_not_exp() {
        let mut claim = Claim::default();
        let token = claim.generate_token(Duration::from_secs(60));
        let parsed_claim = Claim::try_from(token).unwrap();
        assert!(!parsed_claim.is_exp());
    }
}
