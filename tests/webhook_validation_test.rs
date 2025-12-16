//! Tests for webhook validation functionality

use courtlistener_worker::{PrayAndPayWebhookPayload, WebhookEvent, WebhookMetadata};
use validator::Validate;

#[test]
fn test_pray_and_pay_webhook_payload_valid() {
    let payload = PrayAndPayWebhookPayload {
        id: 123,
        date_created: "2024-01-01T00:00:00Z".to_string(),
        status: 1, // Waiting
        recap_document: 456,
    };

    assert!(payload.validate().is_ok());
}

#[test]
fn test_pray_and_pay_webhook_payload_invalid_id() {
    let payload = PrayAndPayWebhookPayload {
        id: 0, // Invalid: must be >= 1
        date_created: "2024-01-01T00:00:00Z".to_string(),
        status: 1,
        recap_document: 456,
    };

    let result = payload.validate();
    assert!(result.is_err());
    
    if let Err(errors) = result {
        assert!(errors.field_errors().contains_key("id"));
    }
}

#[test]
fn test_pray_and_pay_webhook_payload_invalid_status() {
    let payload = PrayAndPayWebhookPayload {
        id: 123,
        date_created: "2024-01-01T00:00:00Z".to_string(),
        status: 3, // Invalid: must be 1 or 2
        recap_document: 456,
    };

    let result = payload.validate();
    assert!(result.is_err());
    
    if let Err(errors) = result {
        assert!(errors.field_errors().contains_key("status"));
    }
}

#[test]
fn test_pray_and_pay_webhook_payload_invalid_date_created() {
    let payload = PrayAndPayWebhookPayload {
        id: 123,
        date_created: String::new(), // Invalid: must have length >= 1
        status: 1,
        recap_document: 456,
    };

    let result = payload.validate();
    assert!(result.is_err());
    
    if let Err(errors) = result {
        assert!(errors.field_errors().contains_key("date_created"));
    }
}

#[test]
fn test_pray_and_pay_webhook_payload_invalid_recap_document() {
    let payload = PrayAndPayWebhookPayload {
        id: 123,
        date_created: "2024-01-01T00:00:00Z".to_string(),
        status: 1,
        recap_document: 0, // Invalid: must be >= 1
    };

    let result = payload.validate();
    assert!(result.is_err());
    
    if let Err(errors) = result {
        assert!(errors.field_errors().contains_key("recap_document"));
    }
}

#[test]
fn test_pray_and_pay_webhook_payload_status_granted() {
    let payload = PrayAndPayWebhookPayload {
        id: 123,
        date_created: "2024-01-01T00:00:00Z".to_string(),
        status: 2, // Granted - valid
        recap_document: 456,
    };

    assert!(payload.validate().is_ok());
}

#[test]
fn test_webhook_event_deserialization() {
    let json = r#"{
        "payload": {
            "id": 123,
            "date_created": "2024-01-01T00:00:00Z",
            "status": 1,
            "recap_document": 456
        },
        "webhook": {
            "version": "1.0",
            "event_type": "pray_and_pay",
            "date_created": "2024-01-01T00:00:00Z"
        }
    }"#;

    let event: Result<WebhookEvent, _> = serde_json::from_str(json);
    assert!(event.is_ok());
    
    let event = event.unwrap();
    assert_eq!(event.webhook.event_type, Some("pray_and_pay".to_string()));
    assert_eq!(event.webhook.version, Some("1.0".to_string()));
}

#[test]
fn test_webhook_event_deserialization_missing_fields() {
    let json = r#"{
        "payload": {},
        "webhook": {}
    }"#;

    let event: Result<WebhookEvent, _> = serde_json::from_str(json);
    assert!(event.is_ok());
    
    let event = event.unwrap();
    assert_eq!(event.webhook.event_type, None);
    assert_eq!(event.webhook.version, None);
}

#[test]
fn test_pray_and_pay_from_webhook_event() {
    let json = r#"{
        "payload": {
            "id": 789,
            "date_created": "2024-01-02T00:00:00Z",
            "status": 2,
            "recap_document": 101
        },
        "webhook": {
            "event_type": "pray_and_pay"
        }
    }"#;

    let event: WebhookEvent = serde_json::from_str(json).unwrap();
    let payload: Result<PrayAndPayWebhookPayload, _> = serde_json::from_value(event.payload);
    
    assert!(payload.is_ok());
    let payload = payload.unwrap();
    assert_eq!(payload.id, 789);
    assert_eq!(payload.status, 2);
    assert_eq!(payload.recap_document, 101);
    assert!(payload.validate().is_ok());
}

