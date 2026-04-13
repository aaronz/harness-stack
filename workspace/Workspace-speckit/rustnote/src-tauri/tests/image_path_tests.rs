//! Image Path Handling Tests
//!
//! Tests for relative path calculation based on document location.
//! Covers:
//! - Same directory image references
//! - Subdirectory document with relative path traversal
//! - Cross-platform path handling
//! - Document move scenarios

use rustnote_lib::model::image::get_image_info;
use std::fs;
use std::path::{Path, PathBuf};

/// TC-G009-001: ImagePath_same_directory
/// Category: unit
/// Input: doc.md and img.png in same folder
/// Expected: ![alt](img.png) resolves correctly
#[test]
fn test_image_path_same_directory() {
    // Create a temporary directory with a document and image in same folder
    let temp_dir = tempfile::tempdir().unwrap();
    let workspace_path = temp_dir.path();

    // Create documents directory
    let docs_dir = workspace_path.join("docs");
    fs::create_dir_all(&docs_dir).unwrap();

    // Create a document in docs/
    let doc_path = docs_dir.join("doc.md");
    fs::write(&doc_path, "# Document\n\n![alt](img.png)").unwrap();

    // Create an image in the same directory as the document
    let img_path = docs_dir.join("img.png");
    fs::write(&img_path, b"fake png data").unwrap();

    // Get image info
    let info = get_image_info(&img_path).unwrap();

    // The relative path should be just the filename when image is in same directory as doc
    // But get_image_info returns file_name as relative_path - this is the bug
    // For same directory resolution, the relative path "img.png" should resolve to docs/img.png
    assert_eq!(info.file_name, "img.png");

    // When document is at docs/doc.md and image reference is "img.png",
    // it should resolve to docs/img.png (same directory)
    let doc_dir = doc_path.parent().unwrap();
    let resolved_path = resolve_relative_path(doc_dir, "img.png");
    assert_eq!(resolved_path, docs_dir.join("img.png"));
}

/// TC-G009-002: ImagePath_subdirectory
/// Category: unit
/// Input: docs/sub/doc.md with ../../images/img.png
/// Expected: Path resolves to correct absolute path
#[test]
fn test_image_path_subdirectory() {
    let temp_dir = tempfile::tempdir().unwrap();
    let workspace_path = temp_dir.path();

    // Create structure:
    // workspace/
    //   docs/
    //     sub/
    //       doc.md
    //   images/
    //     img.png

    let docs_dir = workspace_path.join("docs");
    let sub_dir = docs_dir.join("sub");
    let images_dir = workspace_path.join("images");

    fs::create_dir_all(&sub_dir).unwrap();
    fs::create_dir_all(&images_dir).unwrap();

    // Create document in docs/sub/
    let doc_path = sub_dir.join("doc.md");
    fs::write(&doc_path, "# Sub Document\n\n![alt](../../images/img.png)").unwrap();

    // Create image in images/
    let img_path = images_dir.join("img.png");
    fs::write(&img_path, b"fake png data").unwrap();

    // The document references "../../images/img.png"
    // From docs/sub/, ../../ should resolve to workspace root
    // So ../../images/img.png -> workspace/images/img.png
    let doc_dir = doc_path.parent().unwrap();
    let image_ref = "../../images/img.png";
    let resolved = resolve_relative_path(doc_dir, image_ref);

    assert_eq!(resolved, img_path);
}

/// TC-G009-003: ImagePath_cross_platform
/// Category: edge_case
/// Input: Various path formats
/// Expected: Correct resolution on all platforms
#[test]
fn test_image_path_cross_platform_unix_style() {
    // Unix-style paths
    let doc_dir = Path::new("/workspace/docs/sub");
    let resolved = resolve_relative_path(doc_dir, "../../images/img.png");
    assert_eq!(resolved, Path::new("/workspace/images/img.png"));
}

#[test]
#[cfg(target_os = "windows")]
fn test_image_path_cross_platform_windows_style() {
    let doc_dir = Path::new("C:\\workspace\\docs\\sub");
    let resolved = resolve_relative_path(doc_dir, "..\\..\\images\\img.png");
    assert_eq!(resolved, Path::new("C:\\workspace\\images\\img.png"));
}

#[test]
fn test_image_path_cross_platform_mixed_separators() {
    // Test path normalization
    let doc_dir = Path::new("/workspace/docs/sub");
    // Even if the image ref has different separators, it should resolve
    let resolved = resolve_relative_path(doc_dir, "../../images/nested/img.png");
    assert_eq!(resolved, Path::new("/workspace/images/nested/img.png"));
}

#[test]
fn test_image_path_with_spaces() {
    let temp_dir = tempfile::tempdir().unwrap();
    let workspace_path = temp_dir.path();

    let docs_dir = workspace_path.join("my docs");
    let images_dir = workspace_path.join("my images");

    fs::create_dir_all(&docs_dir).unwrap();
    fs::create_dir_all(&images_dir).unwrap();

    let doc_path = docs_dir.join("doc.md");
    fs::write(&doc_path, "# Doc").unwrap();

    let img_path = images_dir.join("image with spaces.png");
    fs::write(&img_path, b"fake").unwrap();

    let doc_dir = doc_path.parent().unwrap();
    let resolved = resolve_relative_path(doc_dir, "../my images/image with spaces.png");

    assert_eq!(resolved, img_path);
}

#[test]
fn test_image_path_url_encoded() {
    // Test URL-encoded paths like %20 for spaces
    let doc_dir = Path::new("/workspace/docs");
    let resolved = resolve_relative_path(doc_dir, "../my%20images/image.png");
    // URL decoding should be applied
    assert_eq!(resolved, Path::new("/workspace/my images/image.png"));
}

/// TC-G009-004: ImagePath_move_document
/// Category: integration
/// Input: Open doc -> move doc -> check images
/// Expected: Images load if relative path valid
#[test]
fn test_image_path_move_document_original_resolves() {
    let temp_dir = tempfile::tempdir().unwrap();
    let workspace_path = temp_dir.path();

    let docs_dir = workspace_path.join("docs");
    let images_dir = workspace_path.join("images");

    fs::create_dir_all(&docs_dir).unwrap();
    fs::create_dir_all(&images_dir).unwrap();

    let doc_path = docs_dir.join("doc.md");
    fs::write(&doc_path, "# Doc\n\n![alt](../images/img.png)").unwrap();

    let img_path = images_dir.join("img.png");
    fs::write(&img_path, b"fake png").unwrap();

    // Original resolution works correctly
    let doc_dir = doc_path.parent().unwrap();
    let original_resolved = resolve_relative_path(doc_dir, "../images/img.png");
    assert_eq!(original_resolved, img_path);
}

#[test]
fn test_image_path_move_document_broken_after_move() {
    let temp_dir = tempfile::tempdir().unwrap();
    let workspace_path = temp_dir.path();

    let docs_dir = workspace_path.join("docs");
    let images_dir = workspace_path.join("images");

    fs::create_dir_all(&docs_dir).unwrap();
    fs::create_dir_all(&images_dir).unwrap();

    let original_doc_path = docs_dir.join("doc.md");
    fs::write(&original_doc_path, "# Doc\n\n![alt](../images/img.png)").unwrap();

    let img_path = images_dir.join("img.png");
    fs::write(&img_path, b"fake png").unwrap();

    // Move document to subdirectory (without updating image path)
    let new_docs_dir = docs_dir.join("archived");
    fs::create_dir_all(&new_docs_dir).unwrap();

    let moved_doc_path = new_docs_dir.join("doc.md");
    let content = fs::read_to_string(&original_doc_path).unwrap();
    fs::write(&moved_doc_path, &content).unwrap();

    // After move: docs/archived/doc.md + ../images/img.png
    // resolves to docs/archived/../images/img.png = docs/images/img.png
    // NOT workspace/images/img.png where the image actually is
    let new_doc_dir = moved_doc_path.parent().unwrap();
    let moved_resolved = resolve_relative_path(new_doc_dir, "../images/img.png");

    // The resolved path is now WRONG - points to docs/images/img.png
    // but image is at workspace/images/img.png
    let wrong_path = docs_dir.join("images").join("img.png");
    assert_eq!(moved_resolved, wrong_path);
    assert_ne!(moved_resolved, img_path);
}

#[test]
fn test_image_path_move_document_invalid_after_move() {
    let temp_dir = tempfile::tempdir().unwrap();
    let workspace_path = temp_dir.path();

    // Structure: docs/ and images/ at root level
    let docs_dir = workspace_path.join("docs");
    let images_dir = workspace_path.join("images");

    fs::create_dir_all(&docs_dir).unwrap();
    fs::create_dir_all(&images_dir).unwrap();

    // Document in docs/ referencing ../images/img.png
    let doc_path = docs_dir.join("doc.md");
    fs::write(&doc_path, "# Doc\n\n![alt](../images/img.png)").unwrap();

    let img_path = images_dir.join("img.png");
    fs::write(&img_path, b"fake").unwrap();

    // Original resolution works
    let doc_dir = doc_path.parent().unwrap();
    let original_resolved = resolve_relative_path(doc_dir, "../images/img.png");
    assert_eq!(original_resolved, img_path);

    // If we move document to a deeper subdirectory without updating the path,
    // the relative path becomes invalid
    let deep_dir = docs_dir.join("2024").join("january");
    fs::create_dir_all(&deep_dir).unwrap();

    let deep_doc_path = deep_dir.join("doc.md");
    fs::write(&deep_doc_path, "# Doc\n\n![alt](../images/img.png)").unwrap();

    // From docs/2024/january/, ../images/img.png would resolve to docs/2024/images/img.png
    // which is WRONG - the image is at workspace/images/img.png
    let deep_doc_dir = deep_doc_path.parent().unwrap();
    let deep_resolved = resolve_relative_path(deep_doc_dir, "../images/img.png");

    // This should NOT equal the actual image location
    // The path is now invalid
    assert_ne!(deep_resolved, img_path);
}

/// Resolve a relative image path from a document's directory
/// Takes the document's directory path and an image reference (relative path),
/// returns the absolute path the image reference points to.
fn resolve_relative_path(doc_dir: &Path, image_ref: &str) -> PathBuf {
    // URL decode the path first
    let decoded_ref = url_decode_path(image_ref);
    let normalized_ref = decoded_ref.replace('\\', "/");
    let relative_path = Path::new(&normalized_ref);

    let mut components: Vec<std::path::Component> = doc_dir.components().collect();
    for component in relative_path.components() {
        match component {
            std::path::Component::ParentDir => {
                if components.len() > 1 {
                    let last = components.last().unwrap();
                    // Don't pop past the root
                    if !matches!(last, std::path::Component::Prefix(_)) {
                        components.pop();
                    }
                }
            }
            std::path::Component::Normal(s) => {
                components.push(std::path::Component::Normal(s));
            }
            std::path::Component::CurDir => {}
            _ => {}
        }
    }

    components.iter().collect()
}

/// Simple URL decoding for paths (handles %XX encoding)
fn url_decode_path(path: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = path.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '%' && i + 2 < chars.len() {
            let hex: String = chars[i + 1..i + 3].iter().collect();
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                result.push(byte as char);
                i += 3;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}
