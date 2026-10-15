//! Asks the Suíte FisiLab release for the latest version (Help ▸ Check for updates, issue #28).
//! PDF reads the `manifest.json` published with the latest suite release at
//! github.com/mrcandrade/larfic, never upstream PdfCraft's releases.

use std::time::Duration;

use pdfcraft_ui_egui::updates::{RELEASES_PAGE, Release};

const MANIFEST: &str = "https://github.com/mrcandrade/larfic/releases/latest/download/manifest.json";

/// This program's entry in the manifest's `apps` list.
const SLUG: &str = "fisilab-pdf";

/// The version this program has in the latest suite release. The answer is untrusted: its
/// size is capped, and the only page ever offered for download is [`RELEASES_PAGE`].
pub fn latest_release() -> Result<Release, String> {
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(10)))
        // The download redirects to GitHub's storage: follow it, but only over HTTPS.
        .https_only(true)
        .max_redirects(5)
        .tls_config(ureq::tls::TlsConfig::builder().root_certs(os_roots()?).build())
        .build()
        .new_agent();
    let mut response = agent
        .get(MANIFEST)
        .header("Accept", "application/json")
        .header("User-Agent", concat!("FisiLab-PDF/", env!("CARGO_PKG_VERSION")))
        .config()
        .http_status_as_error(false)
        .build()
        .call()
        .map_err(|e| format!("couldn't reach GitHub ({e})"))?;
    let status = response.status().as_u16();
    if !(200..300).contains(&status) {
        return Err(format!("couldn't reach GitHub (http status: {status})"));
    }
    let body = response.body_mut().with_config().limit(1 << 20).read_to_string().map_err(|e| format!("unreadable answer ({e})"))?;
    parse(&body)
}

/// The certificate authorities the operating system trusts.
fn os_roots() -> Result<ureq::tls::RootCerts, String> {
    let found = rustls_native_certs::load_native_certs();
    let certs: Vec<ureq::tls::Certificate<'static>> = found.certs.iter().map(|c| ureq::tls::Certificate::from_der(c.as_ref()).to_owned()).collect();
    if certs.is_empty() {
        return Err("no trusted certificates found on this system".into());
    }
    Ok(ureq::tls::RootCerts::new_with_certs(&certs))
}

/// Reads this program's entry from the manifest. The download page is always [`RELEASES_PAGE`]:
/// nothing from the answer is ever opened.
fn parse(body: &str) -> Result<Release, String> {
    let v: serde_json::Value = serde_json::from_str(body).map_err(|e| format!("unreadable answer ({e})"))?;
    let version = v["apps"]
        .as_array()
        .and_then(|apps| apps.iter().find(|a| a["slug"].as_str() == Some(SLUG)))
        .and_then(|a| a["version"].as_str())
        .map(str::trim)
        .filter(|t| !t.is_empty() && t.len() <= 64)
        .ok_or("no release found")?
        .to_string();
    Ok(Release { version, url: RELEASES_PAGE.to_string() })
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST_JSON: &str = r#"{"generated":"2026-10-15T12:00:00Z","suite":{"version":"2026.10.15"},"apps":[
        {"slug":"fisilab-efeitos","name":"Efeitos","version":"3.1.0","file":"FisiLab-Efeitos-Setup-3.1.0.exe"},
        {"slug":"fisilab-pdf","name":"PDF","version":"0.8.0","file":"FisiLab-PDF-Setup-0.8.0.exe"}]}"#;

    #[test]
    fn our_entry_is_read_and_only_the_suite_release_page_is_offered() {
        let r = parse(MANIFEST_JSON).unwrap();
        assert_eq!(r, Release { version: "0.8.0".into(), url: "https://github.com/mrcandrade/larfic/releases/latest".into() });
        assert!(RELEASES_PAGE.starts_with("https://github.com/mrcandrade/larfic/releases"));
        // A hostile manifest cannot redirect the download link.
        let r = parse(r#"{"apps":[{"slug":"fisilab-pdf","version":"9.9.9","url":"https://example.com/x.exe","file":"x.exe"}]}"#).unwrap();
        assert_eq!(r.url, RELEASES_PAGE);
        assert_eq!(r.version, "9.9.9");
    }

    #[test]
    fn manifests_without_our_entry_or_unreadable_are_errors() {
        assert!(parse(r#"{"apps":[{"slug":"fisilab-efeitos","version":"3.1.0"}]}"#).is_err());
        assert!(parse(r#"{"apps":[]}"#).is_err());
        assert!(parse(r#"{"apps":[{"slug":"fisilab-pdf"}]}"#).is_err());
        assert!(parse(r#"{"apps":[{"slug":"fisilab-pdf","version":""}]}"#).is_err());
        assert!(parse(r#"{"message":"Not Found"}"#).is_err());
        assert!(parse("<html>").is_err());
    }

    #[test]
    fn manifest_versions_compare_with_the_running_one() {
        let r = parse(MANIFEST_JSON).unwrap();
        assert!(pdfcraft_ui_egui::updates::is_newer(&r.version, "0.7.9"));
        assert!(!pdfcraft_ui_egui::updates::is_newer(&r.version, "0.8.0"));
        assert!(!pdfcraft_ui_egui::updates::is_newer(&r.version, "0.9.0"));
    }

    /// Live: asks GitHub over TLS with the OS's roots (`cargo test -p pdfcraft -- --ignored`).
    #[test]
    #[ignore = "needs network access"]
    fn github_answers_with_the_latest_manifest() {
        let r = latest_release().unwrap();
        assert!(pdfcraft_ui_egui::updates::is_newer(&r.version, "0.0.0"), "{r:?}");
        assert!(r.url.starts_with(RELEASES_PAGE), "{r:?}");
    }
}
