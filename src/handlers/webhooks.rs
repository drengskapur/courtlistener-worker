//! Webhook receiver handler

use validator::Validate;
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
    let payload: crate::WebhookEvent = serde_json::from_str(body)
        .map_err(|e| worker::Error::RustError(format!("Failed to parse webhook payload: {}", e)))?;
    
    // Validate specific payload types based on event_type
    let event_type = payload.webhook.event_type.as_deref().unwrap_or("unknown");
    match event_type {
        "pray_and_pay" => {
            // Deserialize and validate PrayAndPayWebhookPayload
            let pray_pay: crate::PrayAndPayWebhookPayload = serde_json::from_value(payload.payload)
                .map_err(|e| worker::Error::RustError(format!("Failed to parse pray_and_pay payload: {}", e)))?;
            
            // Validate using validator crate
            pray_pay.validate()
                .map_err(|e| {
                    let error_msg = e.field_errors()
                        .iter()
                        .map(|(field, errors)| {
                            let details: Vec<String> = errors
                                .iter()
                                .map(|err| format!("{:?}", err.code))
                                .collect();
                            format!("{}: {}", field, details.join(", "))
                        })
                        .collect::<Vec<String>>()
                        .join("; ");
                    worker::Error::RustError(format!("Validation failed for pray_and_pay: {}", error_msg))
                })?;
            
            worker::console_log!("Validated pray_and_pay webhook: id={}, status={}", pray_pay.id, pray_pay.status);
        }
        _ => {
            // For other event types, just log them
            worker::console_log!("Received webhook event type: {} (no specific validation)", event_type);
        }
    }

    // Log webhook details (without sensitive data)
    if let Some(key) = &idempotency_key {
        worker::console_log!("Idempotency-Key: {}", key);
    }
    worker::console_log!(
        "Webhook payload type: {}",
        payload
            .webhook
            .event_type
            .as_deref()
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
