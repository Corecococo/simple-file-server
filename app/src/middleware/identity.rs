use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use actix_web::{Error, HttpRequest, HttpResponse};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::future::ready;
use std::task::{Context, Poll};
use std::time::{Duration, SystemTime};
use tokio::io::Ready;

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

impl<S> Transform<S, ServiceRequest> for Identity
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = IdentityMiddleWare<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        todo!()
    }
}

pub struct IdentityMiddleWare<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for IdentityMiddleWare<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static
{
    type Response = S::Response;
    type Error = Error;
    type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req.headers().get("Authorization");
        match token {
            Some(token) => {
                todo!()
            }
            None => ready(Ok(ServiceResponse::new(
                req.request().clone(),
                HttpResponse::Found()
                    .append_header(("Location", "/login"))
                    .finish(),
            ))),
        }
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
            iat: 0,
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
