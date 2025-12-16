//! Type validation tests
//! 
//! Tests that verify our Rust types correctly match the TypeScript definitions
//! by testing serialization/deserialization with known good data.

use courtlistener_worker::*;
use serde_json;

#[test]
fn test_court_type() {
    let court_json = r#"
    {
        "id": "scotus",
        "name": "Supreme Court",
        "full_name": "Supreme Court of the United States",
        "abbreviation": "SCOTUS"
    }
    "#;

    let court: ApiCourt = serde_json::from_str(court_json).unwrap();
    assert_eq!(court.id, "scotus");
    assert_eq!(court.name, Some("Supreme Court".to_string()));
    assert_eq!(court.abbreviation, Some("SCOTUS".to_string()));
}

#[test]
fn test_court_with_optional_fields() {
    let court_json = r#"
    {
        "id": "ca9"
    }
    "#;

    let court: ApiCourt = serde_json::from_str(court_json).unwrap();
    assert_eq!(court.id, "ca9");
    assert_eq!(court.name, None);
}

#[test]
fn test_paginated_response() {
    let response_json = r#"
    {
        "count": 2,
        "next": "http://example.com/next",
        "previous": null,
        "results": [
            {
                "id": "scotus",
                "name": "Supreme Court"
            },
            {
                "id": "ca9",
                "name": "Ninth Circuit"
            }
        ]
    }
    "#;

    let response: CourtsResponse = serde_json::from_str(response_json).unwrap();
    assert_eq!(response.count, 2);
    assert_eq!(response.results.len(), 2);
    assert_eq!(response.results[0].id, "scotus");
}

#[test]
fn test_opinion_cluster() {
    let cluster_json = r#"
    {
        "id": 12345,
        "case_name": "Roe v. Wade",
        "case_name_short": "Roe",
        "date_filed": "1973-01-22",
        "citation_count": 1000,
        "precedential_status": "Published"
    }
    "#;

    let cluster: ApiOpinionCluster = serde_json::from_str(cluster_json).unwrap();
    assert_eq!(cluster.id, 12345);
    assert_eq!(cluster.case_name, Some("Roe v. Wade".to_string()));
    assert_eq!(cluster.citation_count, Some(1000));
    assert_eq!(cluster.precedential_status, Some(PrecedentialStatus::Published));
}

#[test]
fn test_opinion() {
    let opinion_json = r#"
    {
        "id": 67890,
        "cluster_id": 12345,
        "date_filed": "1973-01-22",
        "plain_text": "This is the opinion text...",
        "extracted_by_ocr": false,
        "author_id": 42
    }
    "#;

    let opinion: ApiOpinion = serde_json::from_str(opinion_json).unwrap();
    assert_eq!(opinion.id, 67890);
    assert_eq!(opinion.cluster_id, Some(12345));
    assert_eq!(opinion.extracted_by_ocr, Some(false));
}

#[test]
fn test_person() {
    let person_json = r#"
    {
        "id": 1,
        "name": "John Roberts",
        "slug": "john-roberts"
    }
    "#;

    let person: ApiPerson = serde_json::from_str(person_json).unwrap();
    assert_eq!(person.id, 1);
    assert_eq!(person.name, Some("John Roberts".to_string()));
    assert_eq!(person.slug, Some("john-roberts".to_string()));
}

#[test]
fn test_search_result_with_aliases() {
    // Test that camelCase aliases work
    let search_json = r#"
    {
        "id": 1,
        "caseName": "Roe v. Wade",
        "caseNameShort": "Roe",
        "dateFiled": "1973-01-22",
        "docketNumber": "70-18",
        "suitNature": "Civil"
    }
    "#;

    let result: SearchResult = serde_json::from_str(search_json).unwrap();
    assert_eq!(result.case_name, Some("Roe v. Wade".to_string()));
    assert_eq!(result.case_name_short, Some("Roe".to_string()));
    assert_eq!(result.docket_number, Some("70-18".to_string()));
}

#[test]
fn test_enum_serialization() {
    // Test PrecedentialStatus enum
    let status = PrecedentialStatus::Published;
    let json = serde_json::to_string(&status).unwrap();
    assert_eq!(json, "\"Published\"");

    let deserialized: PrecedentialStatus = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, PrecedentialStatus::Published);
}

#[test]
fn test_enum_with_rename() {
    // Test PrecedentialStatus with hyphen
    let status_json = r#""In-chambers""#;
    let status: PrecedentialStatus = serde_json::from_str(status_json).unwrap();
    assert_eq!(status, PrecedentialStatus::InChambers);
}

#[test]
fn test_opinion_type_enum() {
    let opinion_type = OpinionType::PerCuriam;
    let json = serde_json::to_string(&opinion_type).unwrap();
    assert!(json.contains("140percuriam"));

    let deserialized: OpinionType = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, OpinionType::PerCuriam);
}

#[test]
fn test_citation() {
    let citation_json = r#"
    {
        "id": 1,
        "citing_opinion_id": 100,
        "cited_opinion_id": 200,
        "depth": "1"
    }
    "#;

    let citation: ApiCitation = serde_json::from_str(citation_json).unwrap();
    assert_eq!(citation.id, 1);
    assert_eq!(citation.citing_opinion_id, Some(100));
    assert_eq!(citation.depth, Some("1".to_string()));
}

#[test]
fn test_docket() {
    let docket_json = r#"
    {
        "id": 1,
        "case_name": "Roe v. Wade",
        "docket_number": "70-18",
        "court_id": "scotus",
        "date_filed": "1970-03-03"
    }
    "#;

    let docket: Docket = serde_json::from_str(docket_json).unwrap();
    assert_eq!(docket.id, 1);
    assert_eq!(docket.case_name, Some("Roe v. Wade".to_string()));
    assert_eq!(docket.docket_number, Some("70-18".to_string()));
}

