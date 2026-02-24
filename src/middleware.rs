use actix_web::{web, HttpRequest, HttpResponse, Error, dev::{Service, ServiceRequest, ServiceResponse, Transform}};
use futures::future::{ok, Ready, LocalBoxFuture};
use crate::models::Location;
use serde_json;

/// Middleware that extracts geolocation from client IP and stores it in request extensions
pub struct GeoIpMiddleware;

impl<S, B> Transform<S, ServiceRequest> for GeoIpMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = GeoIpMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(GeoIpMiddlewareService { service })
    }
}

pub struct GeoIpMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for GeoIpMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let ip = extract_client_ip(&req);
        let ip_clone = ip.clone();

        let fut = self.service.call(req);

        Box::pin(async move {
            let location = fetch_geolocation(&ip_clone).await;
            let mut resp = fut.await?;
            resp.request_mut().extensions_mut().insert(location);
            Ok(resp)
        })
    }
}

/// Extract client IP from request headers or peer address
fn extract_client_ip(req: &ServiceRequest) -> String {
    req.headers()
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .or_else(|| {
            req.peer_addr()
                .map(|addr| addr.ip().to_string())
        })
        .unwrap_or_else(|| "8.8.8.8".to_string())
}

/// Fetch geolocation data from ipinfo.io API
async fn fetch_geolocation(ip: &str) -> Location {
    let url = format!("https://ipinfo.io/{}/json", ip);

    match reqwest::get(&url).await {
        Ok(resp) => {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                let loc = json.get("loc").and_then(|l| l.as_str()).unwrap_or("0,0");
                let coords: Vec<&str> = loc.split(',').collect();
                let (lat, lon) = if coords.len() == 2 {
                    (
                        coords[0].parse::<f64>().unwrap_or(0.0),
                        coords[1].parse::<f64>().unwrap_or(0.0),
                    )
                } else {
                    (0.0, 0.0)
                };

                Location {
                    latitude: lat,
                    longitude: lon,
                    city: json.get("city").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    region: json.get("region").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    country: json.get("country").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    ip: Some(ip.to_string()),
                }
            } else {
                default_location(ip)
            }
        }
        Err(_) => default_location(ip),
    }
}

/// Default location when geolocation fails
fn default_location(ip: &str) -> Location {
    Location {
        latitude: 0.0,
        longitude: 0.0,
        city: None,
        region: None,
        country: None,
        ip: Some(ip.to_string()),
    }
}

/// Extract geolocation from request extensions (used in handlers)
pub fn get_location_from_request(req: &HttpRequest) -> Option<Location> {
    req.extensions().get::<Location>().cloned()
}
