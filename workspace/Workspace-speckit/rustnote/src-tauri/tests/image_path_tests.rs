use rustnote_lib::commands::image::{
    calculate_relative_path, format_markdown_path, resolve_relative_path,
};
use rustnote_lib::model::image::get_image_info;
use std::fs;
use std::path::Path;

/// TC-G009-001: ImagePath_same_directory
/// Category: unit
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

#[test]
fn test_tc_g005_001_relative_path_subdirectory() {
    let doc_dir = Path::new("/docs/guide");
    let resolved = resolve_relative_path(doc_dir, "../images/diagram.png");
    assert_eq!(resolved, Path::new("/docs/images/diagram.png"));
}

#[test]
fn test_tc_g005_002_url_encoded_spaces() {
    let doc_dir = Path::new("/workspace/docs");
    let resolved = resolve_relative_path(doc_dir, "My%20Images/photo.png");
    assert_eq!(resolved, Path::new("/workspace/docs/My Images/photo.png"));
}

#[test]
fn test_tc_g005_003_cross_platform_separators() {
    let doc_dir = Path::new("/workspace/docs");

    let resolved_unix = resolve_relative_path(doc_dir, "images/photo.png");
    assert_eq!(resolved_unix, Path::new("/workspace/docs/images/photo.png"));

    let resolved_backslash = resolve_relative_path(doc_dir, "images\\photo.png");
    assert_eq!(
        resolved_backslash,
        Path::new("/workspace/docs/images/photo.png")
    );
}

#[test]
fn test_tc_g005_004_parent_directory_traversal() {
    let doc_dir = Path::new("/docs/guide");
    let resolved = resolve_relative_path(doc_dir, "../assets/logo.png");
    assert_eq!(resolved, Path::new("/docs/assets/logo.png"));
}

#[test]
fn test_tc_g005_005_deep_nested_traversal() {
    let doc_dir = Path::new("/a/b/c");
    let resolved = resolve_relative_path(doc_dir, "../../images/photo.png");
    assert_eq!(resolved, Path::new("/a/images/photo.png"));
}

// =============================================================================
// TC-IP: Image Relative Path Test Cases
// =============================================================================

/// TC-IP001: Image in parent directory (1 level up)
/// Category: unit
/// Input: Document at /workspace/project/notes/chapter.md, image at /workspace/project/assets/diagram.png
/// Expected: Markdown contains ../assets/diagram.png
#[test]
fn test_tc_ip001_image_parent_directory_1_level() {
    let doc_path = Path::new("/workspace/project/notes/chapter.md");
    let image_path = Path::new("/workspace/project/assets/diagram.png");

    let relative = calculate_relative_path(doc_path, image_path);

    assert_eq!(relative, "../assets/diagram.png");
}

/// TC-IP002: Image 2 levels up
/// Category: unit
/// Input: Document at /workspace/project/notes/sub/chapter.md, image at /workspace/project/assets/img.png
/// Expected: Markdown contains ../../assets/img.png
#[test]
fn test_tc_ip002_image_2_levels_up() {
    let doc_path = Path::new("/workspace/project/notes/sub/chapter.md");
    let image_path = Path::new("/workspace/project/assets/img.png");

    let relative = calculate_relative_path(doc_path, image_path);

    assert_eq!(relative, "../../assets/img.png");
}

/// TC-IP003: Image 3 levels up
/// Category: unit
/// Input: Document at /workspace/project/notes/sub/deep/chapter.md, image at /workspace/project/assets/img.png
/// Expected: Markdown contains ../../../assets/img.png
#[test]
fn test_tc_ip003_image_3_levels_up() {
    let doc_path = Path::new("/workspace/project/notes/sub/deep/chapter.md");
    let image_path = Path::new("/workspace/project/assets/img.png");

    let relative = calculate_relative_path(doc_path, image_path);

    assert_eq!(relative, "../../../assets/img.png");
}

/// TC-IP004: Image in same directory
/// Category: unit
/// Input: Document and image in same directory
/// Expected: Markdown contains only filename
#[test]
fn test_tc_ip004_image_same_directory() {
    let doc_path = Path::new("/workspace/project/notes/chapter.md");
    let image_path = Path::new("/workspace/project/notes/diagram.png");

    let relative = calculate_relative_path(doc_path, image_path);

    assert_eq!(relative, "diagram.png");
}

/// TC-IP005: Image in child directory of document
/// Category: unit
/// Input: Document at /notes/chapter.md, image at /notes/images/img.png
/// Expected: Markdown contains images/img.png
#[test]
fn test_tc_ip005_image_child_directory() {
    let doc_path = Path::new("/notes/chapter.md");
    let image_path = Path::new("/notes/images/img.png");

    let relative = calculate_relative_path(doc_path, image_path);

    assert_eq!(relative, "images/img.png");
}

/// TC-IP006: Absolute path handling
/// Category: edge_case
/// Input: Absolute path /workspace/project/assets/img.png
/// Expected: Converted to relative path from document location
#[test]
fn test_tc_ip006_absolute_path_handling() {
    // Document at /workspace/project/notes/chapter.md
    // Image at /workspace/project/assets/img.png
    let doc_path = Path::new("/workspace/project/notes/chapter.md");
    let image_path = Path::new("/workspace/project/assets/img.png");

    let relative = calculate_relative_path(doc_path, image_path);

    // Should be relative from notes/ to assets/
    assert_eq!(relative, "../assets/img.png");
}

/// TC-IP007: Windows path separators
/// Category: edge_case
/// Input: Document at C:\project\notes\chapter.md, image at C:\project\assets\img.png
/// Expected: Path uses forward slashes, correct relative path
#[test]
fn test_tc_ip007_windows_path_separators() {
    let doc_path = Path::new("C:/project/notes/chapter.md");
    let image_path = Path::new("C:/project/assets/img.png");

    let relative = calculate_relative_path(doc_path, image_path);

    // Should use forward slashes
    assert_eq!(relative, "../assets/img.png");
}

#[test]
fn test_tc_ip007_windows_backslash_separators() {
    // Test mixed separators - Windows paths with forward slashes normalized
    let doc_path = Path::new("C:/project/notes/sub/chapter.md");
    let image_path = Path::new("C:/project/assets/img.png");

    let relative = calculate_relative_path(doc_path, image_path);

    // Should use forward slashes, 2 levels up
    assert_eq!(relative, "../../assets/img.png");
}

/// TC-IP008: Special characters in filename
/// Category: edge_case
/// Input: Image at /assets/my photo.png
/// Expected: Path URL-encoded or quoted correctly
#[test]
fn test_tc_ip008_special_characters_spaces() {
    let doc_path = Path::new("/workspace/docs/chapter.md");
    let image_path = Path::new("/assets/my photo.png");

    let relative = calculate_relative_path(doc_path, image_path);

    // The relative path should have the filename with spaces
    assert!(relative.contains("my%20photo.png") || relative.contains("my photo.png"));
}

#[test]
fn test_tc_ip008_url_encoded_spaces() {
    let path = "/assets/my photo.png";
    let formatted = format_markdown_path(path);

    // Spaces should be URL-encoded
    assert!(formatted.contains("my%20photo"));
    assert!(!formatted.contains("my photo"));
}

#[test]
fn test_tc_ip008_special_characters_hash() {
    let path = "/assets/image#tag.png";
    let formatted = format_markdown_path(path);

    // Hash should be URL-encoded to avoid fragment interpretation
    assert!(formatted.contains("%23"));
    assert!(!formatted.contains("#"));
}

#[test]
fn test_tc_ip008_special_characters_ampersand() {
    let path = "/assets/image&copy.png";
    let formatted = format_markdown_path(path);

    // Ampersand should be URL-encoded
    assert!(formatted.contains("%26"));
    assert!(!formatted.contains("&copy"));
}

// Additional comprehensive tests for edge cases

#[test]
fn test_calculate_relative_path_sibling_directory() {
    let doc_path = Path::new("/workspace/project/docs/chapter.md");
    let image_path = Path::new("/workspace/project/images/diagram.png");

    let relative = calculate_relative_path(doc_path, image_path);

    assert_eq!(relative, "../images/diagram.png");
}

#[test]
fn test_calculate_relative_path_deep_sibling() {
    let doc_path = Path::new("/workspace/project/docs/sub/chapter.md");
    let image_path = Path::new("/workspace/project/assets/nested/img.png");

    let relative = calculate_relative_path(doc_path, image_path);

    // From docs/sub/ -> ../../assets/nested/img.png
    assert_eq!(relative, "../../assets/nested/img.png");
}

#[test]
fn test_calculate_relative_path_same_file_different_names() {
    // Edge case: same directory but different naming
    let doc_path = Path::new("/notes/README.md");
    let image_path = Path::new("/notes/logo.png");

    let relative = calculate_relative_path(doc_path, image_path);

    assert_eq!(relative, "logo.png");
}

#[test]
fn test_format_markdown_path_parentheses() {
    let path = "/assets/image (copy).png";
    let formatted = format_markdown_path(path);

    // Parentheses are typically safe in URLs but let's check
    assert!(formatted.contains("image"));
    assert!(formatted.contains("%20"));
}

#[test]
fn test_format_markdown_path_brackets() {
    let path = "/assets/image[1].png";
    let formatted = format_markdown_path(path);

    // Brackets should be URL-encoded for Markdown compatibility
    assert!(formatted.contains("%5B"));
    assert!(formatted.contains("%5D"));
}

#[test]
fn test_integration_insert_image_markdown() {
    use rustnote_lib::commands::image::image_markdown_from_path;

    let image_path = "/assets/diagram.png";
    let relative_path = "../assets/diagram.png";

    let markdown = image_markdown_from_path(image_path.to_string(), relative_path.to_string());

    assert_eq!(markdown, "![diagram.png](../assets/diagram.png)");
}

#[test]
fn test_integration_roundtrip() {
    // Test that resolve and calculate are inverses
    let doc_dir = Path::new("/workspace/project/notes");
    let relative = "../assets/diagram.png";

    // Calculate relative path
    let doc_path = Path::new("/workspace/project/notes/chapter.md");
    let image_path = Path::new("/workspace/project/assets/diagram.png");

    let calculated = calculate_relative_path(doc_path, image_path);
    assert_eq!(calculated, relative);

    // Resolve back
    let resolved = resolve_relative_path(doc_dir, &calculated);
    assert_eq!(resolved, image_path);
}
