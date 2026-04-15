use oberon::tools::base::Tool;
use oberon::tools::file_read::FileReadTool;
use serde_json::json;
use std::io::Write;
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_file_read() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Hello, world!").unwrap();
    let path = temp_file.path().to_str().unwrap();

    let tool = FileReadTool::new();
    let input = json!({ "path": path });
    let result = tool.execute(input).await.unwrap();
    assert!(result.success);
    assert_eq!(result.output.trim(), "Hello, world!");
}

#[tokio::test]
async fn test_file_write() {
    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_str().unwrap();

    let tool = oberon::tools::file_write::FileWriteTool::new();
    let input = json!({ "path": path, "content": "test content" });
    let result = tool.execute(input).await.unwrap();
    assert!(result.success);

    let content = std::fs::read_to_string(path).unwrap();
    assert_eq!(content, "test content");
}
