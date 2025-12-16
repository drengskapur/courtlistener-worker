//! Webhook receiver handler

use worker::*;

/// Receive webhook events FROM CourtListener
/// This endpoint can receive POST requests when you point your domain to this worker
/// Configure webhook URL in CourtListener to: https://your-domain.com/webhook or /webhook/{secret}
///
/// Webhook events come from IPs: 34.210.230.218 or 54.189.59.91
/// Each event includes an Idempotency-Key header for deduplication
pub async fn receive_webhook(_req: &Request, _env: &Env, body: &str) -> Result<Response> {
    // Log webhook receipt
    worker::console_log!("Webhook received: {} {}", _req.method(), _req.path());

    // Get headers
    let idempotency_key = _req.headers().get("Idempotency-Key").ok().flatten();
    let _content_type = match _req.headers().get("Content-Type") {
        Ok(Some(val)) => val,
        _ => "application/json".to_string(),
    };

    // Parse webhook payload
    let payload: serde_json::Value = serde_json::from_str(body)
        .map_err(|e| worker::Error::RustError(format!("Failed to parse webhook payload: {}", e)))?;

    // Log webhook details (without sensitive data)
    if let Some(key) = &idempotency_key {
        worker::console_log!("Idempotency-Key: {}", key);
    }
    worker::console_log!(
        "Webhook payload type: {}",
        payload
            .get("webhook")
            .and_then(|w| w.get("event_type"))
            .and_then(|t| t.as_str())
            .unwrap_or("unknown")
    );

    // TODO: Process webhook event here
    // - Check idempotency key to avoid duplicate processing
    // - Validate webhook signature (if implemented)
    // - Process the event payload
    // - Store/forward as needed

    // Return 200 OK to acknowledge receipt
    // CourtListener will retry if we return non-2xx status
    let mut response = Response::ok("Webhook received")?;
    let headers = response.headers_mut();
    headers.set("Content-Type", "application/json")?;
    Ok(response)
}
