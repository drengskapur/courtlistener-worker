//! Tests for library functionality (types, config, etc.)
//! These tests should work without the worker feature

use courtlistener_worker::{
    API_BASE_URL, API_VERSION, API_VERSION_PATH, get_api_base_url,
    Court, ApiCourt, CourtsResponse, Opinion, OpinionCluster, OpinionsResponse, 
    Person, ApiPerson, PeopleResponse, Docket, DocketsResponse, ApiCitation, CitationsResponse,
};

#[test]
fn test_config_constants() {
    // Test that config constants are available
    assert!(!API_VERSION.is_empty());
    assert!(!API_BASE_URL.is_empty());
    assert_eq!(API_VERSION_PATH, "v4");
}

#[test]
fn test_get_api_base_url() {
    let url = get_api_base_url();
    assert!(url.starts_with("https://"));
    assert!(url.contains("courtlistener.com"));
    assert!(url.contains("/api/rest/"));
}

#[test]
fn test_court_type_creation() {
    let court = Court {
        id: "us".to_string(),
        name: Some("Supreme Court of the United States".to_string()),
        full_name: Some("Supreme Court of the United States".to_string()),
        abbreviation: Some("SCOTUS".to_string()),
        url: None,
        slug: None,
        start_date: None,
        end_date: None,
        jurisdiction: None,
        court_type: None,
        parent_court: None,
        citation_string: None,
        citation_count: None,
        docket_count: None,
        precedential_status: None,
    };

    assert_eq!(court.id, "us");
    assert_eq!(court.name, Some("Supreme Court of the United States".to_string()));
}

#[test]
fn test_courts_response_deserialization() {
    let json = r#"{
        "count": 1,
        "next": null,
        "previous": null,
        "results": [
            {
                "id": "us",
                "name": "Supreme Court of the United States",
                "abbreviation": "SCOTUS"
            }
        ]
    }"#;

    let response: Result<CourtsResponse, _> = serde_json::from_str(json);
    assert!(response.is_ok());
    
    let response = response.unwrap();
    assert_eq!(response.count, 1);
    assert_eq!(response.results.len(), 1);
    assert_eq!(response.results[0].id, "us");
}

#[test]
fn test_opinion_type_creation() {
    let opinion = Opinion {
        id: 123,
        author_id: None,
        author: None,
        author_str: None,
        per_curiam: None,
        joined_by: None,
        joined_by_str: None,
        r#type: None,
        sha1: None,
        page_count: None,
        download_url: None,
        local_path: None,
        plain_text: None,
        html: None,
        html_lawbox: None,
        html_columbia: None,
        html_anon_2020: None,
        xml_harvard: None,
        html_with_citations: None,
        extracted_by_ocr: None,
        ocr_confidence: None,
        resource_uri: None,
        cluster_id: None,
        cluster: None,
        absolute_url: None,
        opinions_cited: None,
    };

    assert_eq!(opinion.id, 123);
}

#[test]
fn test_opinions_response_deserialization() {
    let json = r#"{
        "count": 1,
        "next": null,
        "previous": null,
        "results": [
            {
                "id": 123,
                "case_name": "Test Case"
            }
        ]
    }"#;

    let response: Result<OpinionsResponse, _> = serde_json::from_str(json);
    assert!(response.is_ok());
    
    let response = response.unwrap();
    assert_eq!(response.count, 1);
    assert_eq!(response.results.len(), 1);
    assert_eq!(response.results[0].id, 123);
}

#[test]
fn test_person_type_creation() {
    let person = Person {
        id: 456,
        name: Some("John Doe".to_string()),
        slug: Some("john-doe".to_string()),
        positions: None,
    };

    assert_eq!(person.id, 456);
    assert_eq!(person.name, Some("John Doe".to_string()));
}

#[test]
fn test_people_response_deserialization() {
    let json = r#"{
        "count": 1,
        "next": null,
        "previous": null,
        "results": [
            {
                "id": 456,
                "name_first": "John",
                "name_last": "Doe"
            }
        ]
    }"#;

    let response: Result<PeopleResponse, _> = serde_json::from_str(json);
    assert!(response.is_ok());
    
    let response = response.unwrap();
    assert_eq!(response.count, 1);
    assert_eq!(response.results.len(), 1);
    assert_eq!(response.results[0].id, 456);
}

#[test]
fn test_docket_type_creation() {
    let docket = Docket {
        id: 789,
        resource_uri: None,
        court: None,
        court_id: None,
        original_court_info: None,
        idb_data: None,
        bankruptcy_information: None,
        clusters: None,
        audio_files: None,
        assigned_to: None,
        referred_to: None,
        absolute_url: None,
        date_created: None,
        date_modified: None,
        source: None,
        appeal_from_str: None,
        assigned_to_str: None,
        referred_to_str: None,
        panel_str: None,
        date_last_index: None,
        date_last_filing: None,
        date_cert_granted: None,
        date_cert_denied: None,
        date_argued: None,
        date_reargued: None,
        date_reargument_denied: None,
        date_filed: None,
        date_terminated: None,
        case_name_short: None,
        case_name: None,
        case_name_full: None,
        slug: None,
        docket_number: None,
        docket_number_core: None,
        pacer_case_id: None,
        cause: None,
        nature_of_suit: None,
        jury_demand: None,
        jurisdiction_type: None,
        appellate_fee_status: None,
        appellate_case_type_information: None,
        mdl_status: None,
        filepath_ia: None,
        filepath_ia_json: None,
        ia_upload_failure_count: None,
        ia_needs_upload: None,
        ia_date_first_change: None,
        date_blocked: None,
        blocked: None,
        appeal_from: None,
        tags: None,
        panel: None,
    };

    assert_eq!(docket.id, 789);
}

#[test]
fn test_dockets_response_deserialization() {
    let json = r#"{
        "count": 1,
        "next": null,
        "previous": null,
        "results": [
            {
                "id": 789,
                "docket_number": "1:23-cv-456"
            }
        ]
    }"#;

    let response: Result<DocketsResponse, _> = serde_json::from_str(json);
    assert!(response.is_ok());
    
    let response = response.unwrap();
    assert_eq!(response.count, 1);
    assert_eq!(response.results.len(), 1);
    assert_eq!(response.results[0].id, 789);
}

#[test]
fn test_citation_type_creation() {
    let citation = ApiCitation {
        id: 111,
        citing_opinion_id: None,
        cited_opinion_id: None,
        citing_opinion: None,
        cited_opinion: None,
        depth: None,
    };

    assert_eq!(citation.id, 111);
}

#[test]
fn test_citations_response_deserialization() {
    let json = r#"{
        "count": 1,
        "next": null,
        "previous": null,
        "results": [
            {
                "id": 111,
                "depth": 1
            }
        ]
    }"#;

    let response: Result<CitationsResponse, _> = serde_json::from_str(json);
    assert!(response.is_ok());
    
    let response = response.unwrap();
    assert_eq!(response.count, 1);
    assert_eq!(response.results.len(), 1);
    assert_eq!(response.results[0].id, 111);
}

