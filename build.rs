use regex::Regex;
use std::fs;
use std::path::Path;

/// Parse semantic version string (e.g., "4.4" or "4.4.0") into (major, minor, patch)
/// Patch defaults to 0 if not provided
fn parse_version(version_str: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<&str> = version_str.split('.').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return None;
    }

    let major = parts[0].parse().ok()?;
    let minor = parts[1].parse().ok()?;
    let patch = if parts.len() == 3 {
        parts[2].parse().ok()?
    } else {
        0
    };

    Some((major, minor, patch))
}

/// Fetch the current API version from CourtListener's GitHub repository changelog
/// The changelog lists versions in order, with the latest version first
fn fetch_latest_version_from_github() -> Option<String> {
    let url = "https://raw.githubusercontent.com/freelawproject/courtlistener/refs/heads/main/cl/api/templates/rest-change-log.html";

    // Try to fetch, but don't fail the build if it fails (network issues, etc.)
    // Create a client with a user agent and longer timeout
    let client = match reqwest::blocking::Client::builder()
        .user_agent("courtlistener-worker-build-script/1.0")
        .timeout(std::time::Duration::from_secs(30))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "cargo:warning=Failed to create HTTP client: {} (using local versions)",
                e
            );
            return None;
        }
    };

    let response = match client.get(url).send() {
        Ok(r) => r,
        Err(e) => {
            eprintln!(
                "cargo:warning=Failed to fetch version from GitHub: {} (using local versions)",
                e
            );
            return None;
        }
    };

    let html = match response.text() {
        Ok(h) => h,
        Err(e) => {
            eprintln!(
                "cargo:warning=Failed to parse GitHub response: {} (using local versions)",
                e
            );
            return None;
        }
    };

    // Look for the first version entry in the changelog: <strong>v4.4</strong> or similar
    // The changelog lists versions with the latest first, so we want the first match
    let re = match Regex::new(r"<strong>v(\d+)\.(\d+)") {
        Ok(r) => r,
        Err(e) => {
            eprintln!(
                "cargo:warning=Failed to create regex: {} (using local versions)",
                e
            );
            return None;
        }
    };

    if let Some(caps) = re.captures(&html) {
        if let (Some(major), Some(minor)) = (caps.get(1), caps.get(2)) {
            // Return as "4.4.0" format (add .0 patch number, no "v" prefix)
            return Some(format!("{}.{}.0", major.as_str(), minor.as_str()));
        }
    }

    eprintln!(
        "cargo:warning=Could not find version pattern in GitHub changelog (using local versions)"
    );
    None
}

/// Fetch OpenAPI spec from CourtListener API
/// This generates a basic spec by querying the API root endpoint
/// `version`: The full version string (e.g., "v4.4")
fn fetch_openapi_spec_from_api(version: &str) -> Option<String> {
    // Extract major version from version string (e.g., "v4.4" -> "4")
    let major_version = version
        .strip_prefix('v')
        .and_then(|v| v.split('.').next())
        .unwrap_or("4");

    let api_version_path = format!("v{}", major_version);
    let api_base = format!(
        "https://www.courtlistener.com/api/rest/{}/",
        api_version_path
    );

    // Create a client with a user agent and longer timeout
    let client = match reqwest::blocking::Client::builder()
        .user_agent("courtlistener-worker-build-script/1.0")
        .timeout(std::time::Duration::from_secs(30))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "cargo:warning=Failed to create HTTP client: {} (skipping auto-fetch)",
                e
            );
            return None;
        }
    };

    // Fetch the API root to get available endpoints
    let response = match client.get(&api_base).send() {
        Ok(r) => r,
        Err(e) => {
            eprintln!(
                "cargo:warning=Failed to fetch API root: {} (skipping auto-fetch)",
                e
            );
            return None;
        }
    };

    // Check status code
    if !response.status().is_success() {
        eprintln!(
            "cargo:warning=API returned status {} (skipping auto-fetch)",
            response.status()
        );
        return None;
    }

    let root: serde_json::Value = match response.json() {
        Ok(r) => r,
        Err(e) => {
            eprintln!(
                "cargo:warning=Failed to parse API root response: {} (skipping auto-fetch)",
                e
            );
            return None;
        }
    };

    // Build a basic OpenAPI spec
    let mut paths = serde_json::Map::new();

    // Extract endpoints from root response
    if let Some(obj) = root.as_object() {
        for (key, value) in obj {
            if let Some(url) = value.as_str() {
                if url.starts_with("http") {
                    // Extract path from URL (dynamic API version path)
                    let api_path_pattern = format!("/api/rest/{}/", api_version_path);
                    if let Some(path_start) = url.find(&api_path_pattern) {
                        let path = &url[path_start + api_path_pattern.len()..];
                        if let Some(query_start) = path.find('?') {
                            let clean_path = format!("/{}", &path[..query_start]);
                            paths.insert(
                                clean_path.clone(),
                                serde_json::json!({
                                    "get": {
                                        "summary": format!("Get {}", key),
                                        "operationId": key.replace("-", "_"),
                                        "responses": {
                                            "200": {
                                                "description": "Successful response",
                                                "content": {
                                                    "application/json": {
                                                        "schema": { "type": "object" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }),
                            );
                        }
                    }
                }
            }
        }
    }

    let spec = serde_json::json!({
        "openapi": "3.0.3",
        "info": {
            "title": "CourtListener API",
            "version": version,
            "description": "Auto-generated OpenAPI spec from CourtListener API"
        },
        "servers": [{
            "url": format!("https://www.courtlistener.com/api/rest/{}", api_version_path),
            "description": "CourtListener API"
        }],
        "paths": paths
    });

    serde_json::to_string_pretty(&spec).ok()
}

fn main() {
    // Tell Cargo to rerun this build script if the openapi directory changes
    println!("cargo:rerun-if-changed=openapi");

    // Try to fetch the latest version from GitHub
    let latest_version_from_github = fetch_latest_version_from_github();

    let openapi_dir = Path::new("openapi");

    if !openapi_dir.exists() {
        fs::create_dir_all(openapi_dir).unwrap_or_else(|e| {
            panic!("Failed to create openapi directory: {}", e);
        });
    }

    // Find all version directories
    let mut versions: Vec<(String, (u32, u32, u32))> = Vec::new();

    if let Ok(entries) = fs::read_dir(openapi_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                        // Try to parse as semantic version (v4.4 or v4.4.0 format)
                        if let Some(version_str) = dir_name.strip_prefix('v') {
                            if let Some(version) = parse_version(version_str) {
                                versions.push((dir_name.to_string(), version));
                            }
                        }
                    }
                }
            }
        }
    }

    // If no local versions and GitHub fetch failed, try to fetch from API with a fallback version
    if versions.is_empty() {
        // Use GitHub version if available, otherwise fallback to "4.4.0"
        let version_to_fetch = latest_version_from_github.clone().unwrap_or_else(|| {
            println!(
                "cargo:warning=GitHub fetch failed and no local versions, using fallback 4.4.0"
            );
            "4.4.0".to_string()
        });

        println!(
            "cargo:warning=No local versions found, attempting to fetch {} from API...",
            version_to_fetch
        );
        // Use version with "v" prefix for API calls
        let api_version = format!("v{}", version_to_fetch);
        if let Some(spec_content) = fetch_openapi_spec_from_api(&api_version) {
            // Create the directory without "v" prefix
            let version_dir = openapi_dir.join(&version_to_fetch);
            fs::create_dir_all(&version_dir).unwrap_or_else(|e| {
                panic!(
                    "Failed to create directory {}: {}",
                    version_dir.display(),
                    e
                );
            });
            let spec_path = version_dir.join("openapi.json");
            fs::write(&spec_path, spec_content).unwrap_or_else(|e| {
                panic!(
                    "Failed to write openapi.json to {}: {}",
                    spec_path.display(),
                    e
                );
            });
            println!(
                "cargo:warning=Successfully fetched and saved OpenAPI spec for {}",
                version_to_fetch
            );

            // Parse the version for the output
            let version_str = version_to_fetch.strip_prefix('v').unwrap();
            let (maj, min, pat) = parse_version(version_str).unwrap();
            versions.push((version_to_fetch, (maj, min, pat)));
        } else {
            panic!("No local versions found and failed to fetch {} from API. Please add the OpenAPI spec manually.", version_to_fetch);
        }
    }

    // Sort by version (highest first): compare major, then minor, then patch
    versions.sort_by(|a, b| {
        let (ma1, mi1, p1) = a.1;
        let (ma2, mi2, p2) = b.1;
        ma2.cmp(&ma1)
            .then_with(|| mi2.cmp(&mi1))
            .then_with(|| p2.cmp(&p1))
    });

    // Determine which version to use
    let (target_version_dir, (major, minor, patch)) =
        if let Some(ref github_version) = latest_version_from_github {
            // GitHub version is without "v" prefix (e.g., "4.4.0")
            // Check if it exists locally (with or without "v" prefix)
            let version_with_v = format!("v{}", github_version);
            if let Some((dir, ver)) = versions
                .iter()
                .find(|(dir, _)| dir == github_version || dir == &version_with_v)
            {
                // Use the GitHub version that exists locally
                (dir.clone(), *ver)
            } else {
                // GitHub version not found locally - try to fetch it
                println!(
                    "cargo:warning=GitHub version {} not found locally, attempting to fetch...",
                    github_version
                );
                // Use version with "v" prefix for API calls
                let api_version = format!("v{}", github_version);
                if let Some(spec_content) = fetch_openapi_spec_from_api(&api_version) {
                    // Create the directory without "v" prefix
                    let version_dir = openapi_dir.join(github_version);
                    fs::create_dir_all(&version_dir).unwrap_or_else(|e| {
                        panic!(
                            "Failed to create directory {}: {}",
                            version_dir.display(),
                            e
                        );
                    });
                    let spec_path = version_dir.join("openapi.json");
                    fs::write(&spec_path, spec_content).unwrap_or_else(|e| {
                        panic!(
                            "Failed to write openapi.json to {}: {}",
                            spec_path.display(),
                            e
                        );
                    });
                    println!(
                        "cargo:warning=Successfully fetched and saved OpenAPI spec for {}",
                        github_version
                    );

                    // Parse the version for the output
                    let (maj, min, pat) = parse_version(github_version).unwrap();
                    (github_version.clone(), (maj, min, pat))
                } else {
                    // Failed to fetch, use highest available local version
                    if versions.is_empty() {
                        panic!("No local versions found and failed to fetch from API");
                    }
                    println!(
                        "cargo:warning=Failed to fetch {}, using highest available: {}",
                        github_version, versions[0].0
                    );
                    versions[0].clone()
                }
            }
        } else {
            // No GitHub version, use highest local version
            if versions.is_empty() {
                panic!("No valid version directories found in openapi/");
            }
            versions[0].clone()
        };

    let spec_path = openapi_dir.join(&target_version_dir).join("openapi.json");

    if !spec_path.exists() {
        panic!("openapi.json not found in {}", spec_path.display());
    }

    // Generate code with the highest version
    // Read and embed the spec content
    let spec_content = fs::read_to_string(&spec_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", spec_path.display(), e));

    // Escape the spec content for embedding in a raw string literal
    // Use a unique delimiter (OPENAPI_SPEC_END) to avoid conflicts with JSON content
    // Format version as "4.4.0" (with patch) for API_VERSION constant
    let api_version_str = format!("{}.{}.{}", major, minor, patch);

    let out_dir = std::env::var("OUT_DIR").unwrap();

    // Generate two separate files to avoid duplicate definitions:
    // 1. api_version.rs - for config.rs (contains API_VERSION)
    let api_version_output = format!(
        "// Auto-generated by build.rs\n\
         // CourtListener API version (semantic version)\n\
         pub const API_VERSION: &str = \"{}\";\n",
        api_version_str
    );
    let api_version_path = Path::new(&out_dir).join("api_version.rs");
    fs::write(&api_version_path, api_version_output).unwrap();

    // 2. openapi_version.rs - for docs.rs (contains COURTLISTENER_API_VERSION_DIR and OPENAPI_SPEC)
    let openapi_output = format!(
        "// Auto-generated by build.rs\n\
         // CourtListener API version directory: v{}.{}.{}\n\
         pub const COURTLISTENER_API_VERSION_DIR: &str = \"{}\";\n\n\
         // OpenAPI spec content embedded at compile time\n\
         pub const OPENAPI_SPEC: &str = r##\"{}\"##;\n",
        major, minor, patch, target_version_dir, spec_content
    );
    let openapi_path = Path::new(&out_dir).join("openapi_version.rs");
    fs::write(&openapi_path, openapi_output).unwrap();
}
