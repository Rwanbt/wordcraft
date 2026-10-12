//! An MCP client doing realistic tasks through the protocol only, checking results via MCP.

use serde_json::{Value, json};

use crate::{Headless, Server};

fn call(s: &mut Server, id: u64, method: &str, params: Value) -> Value {
    let line = json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}).to_string();
    let r = s.handle_line(&line).expect("reply");
    serde_json::from_str(&r).expect("json")
}

fn tool(s: &mut Server, name: &str, args: Value) -> Value {
    let r = call(s, 9, "tools/call", json!({"name": name, "arguments": args}));
    let res = &r["result"];
    assert_eq!(res["isError"], false, "{name}: {res}");
    let text = res["content"][0]["text"].as_str().unwrap_or("null");
    serde_json::from_str(text).unwrap_or(Value::String(text.to_string()))
}

#[test]
fn lifecycle_and_tools() {
    let mut s = Server::new(Box::new(Headless::default()));
    let init = call(&mut s, 1, "initialize", json!({"protocolVersion": "2025-06-18"}));
    assert_eq!(init["result"]["serverInfo"]["name"], "wordcraft");
    assert!(s.handle_line(r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#).is_none());
    let tools = call(&mut s, 2, "tools/list", json!({}));
    assert!(tools["result"]["tools"].as_array().unwrap().len() >= 12);
    let bad = call(&mut s, 3, "nope", json!({}));
    assert!(bad["error"].is_object());
    assert!(s.handle_line("{not json").unwrap().contains("parse error"));
}

#[test]
fn agent_writes_a_formatted_document() {
    let mut s = Server::new(Box::new(Headless::default()));
    tool(&mut s, "new_document", json!({}));
    tool(&mut s, "type_text", json!({"text": "Quarterly Report"}));
    tool(&mut s, "execute", json!({"command": "para.style", "params": {"style": "Heading 1"}}));
    tool(&mut s, "execute", json!({"command": "text.newParagraph"}));
    tool(&mut s, "type_text", json!({"text": "Sales grew strongly this quarter.\nCosts fell.", "paragraphs": true}));
    tool(&mut s, "select_text", json!({"text": "strongly"}));
    tool(&mut s, "execute", json!({"command": "format.bold"}));
    tool(&mut s, "batch", json!({"commands": [{"command": "caret.docEnd"}, {"command": "insert.table", "params": {"rows": 2, "cols": 2}}]}));
    let doc = tool(&mut s, "inspect_document", json!({}));
    let blocks = doc["blocks"].as_array().unwrap();
    assert_eq!(blocks[0]["style"], "Heading1");
    assert_eq!(blocks[0]["text"], "Quarterly Report");
    let runs = blocks[1]["runs"].as_array().unwrap();
    assert!(runs.iter().any(|r| r["props"]["bold"] == true));
    assert!(blocks.iter().any(|b| b["type"] == "table"));
    let text = tool(&mut s, "get_text", json!({}));
    assert!(text["text"].as_str().unwrap().contains("Costs fell."));
    let r = call(&mut s, 10, "tools/call", json!({"name": "render_page", "arguments": {"page": 1, "scale": 0.5}}));
    assert_eq!(r["result"]["content"][0]["type"], "image");
    let r = call(&mut s, 11, "tools/call", json!({"name": "screenshot", "arguments": {}}));
    assert_eq!(r["result"]["isError"], true);
    let p = tool(&mut s, "parity", json!({}));
    assert!(p["percent"].as_f64().unwrap() > 50.0);
}

#[test]
fn resources() {
    let mut s = Server::new(Box::new(Headless::default()));
    let l = call(&mut s, 1, "resources/list", json!({}));
    assert_eq!(l["result"]["resources"].as_array().unwrap().len(), 2);
    let r = call(&mut s, 2, "resources/read", json!({"uri": "wordcraft://document"}));
    assert!(r["result"]["contents"][0]["text"].as_str().unwrap().contains("blocks"));
}

/// A backend standing in for a running app: it answers `ui.render` and `ui.screenshot` with a
/// PNG in the reply and records what it was asked.
struct FakeApp {
    calls: std::sync::Arc<std::sync::Mutex<Vec<(String, Value)>>>,
}

impl crate::backend::Backend for FakeApp {
    fn call(&mut self, method: &str, params: Value) -> Result<Value, String> {
        self.calls.lock().unwrap().push((method.to_string(), params.clone()));
        match method {
            "ui.render" | "ui.screenshot" => Ok(json!({"png": wordcraft_engine::cmd::insert::base64_encode(b"\x89PNG"), "width": 1, "height": 1})),
            other => Err(format!("unexpected {other}")),
        }
    }
    fn has_ui(&self) -> bool {
        true
    }
    fn describe(&self) -> String {
        "fake app".into()
    }
}

/// With an app attached, pictures travel inside the reply: the bridge never writes a file with a
/// predictable name into the shared temporary folder (another user could plant a link there).
#[test]
fn render_and_screenshot_through_the_app_use_no_temp_file() {
    let calls = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let mut s = Server::new(Box::new(FakeApp { calls: calls.clone() }));
    for (tool, method) in [("render_page", "ui.render"), ("screenshot", "ui.screenshot")] {
        let r = call(&mut s, 10, "tools/call", json!({"name": tool, "arguments": {"page": 1}}));
        let content = &r["result"]["content"];
        assert_eq!(r["result"]["isError"], false, "{tool}: {r}");
        assert_eq!(content[0]["type"], "image", "{tool}");
        assert_eq!(content[0]["data"], wordcraft_engine::cmd::insert::base64_encode(b"\x89PNG"), "{tool}");
        let (m, params) = calls.lock().unwrap().pop().expect("one call");
        assert_eq!(m, method);
        assert!(params.get("path").is_none(), "{tool} asked for a file: {params}");
    }
}

/// #40: a headless session never reads or writes the person's custom dictionary file: an agent's
/// Add to Dictionary stays in memory (only the desktop app saves the dictionary).
#[test]
fn headless_sessions_keep_the_dictionary_in_memory() {
    let mut h = Headless::default();
    assert!(h.session.user_dict_path.is_none());
    let mut s = Server::new(Box::new(Headless::default()));
    tool(&mut s, "execute", json!({"command": "review.addToDictionary", "params": {"word": "zxqmcpword"}}));
    h.session.run("review.addToDictionary", &json!({"word": "zxqmcpword2"})).expect("added");
    assert!(h.session.user_dict_path.is_none());
}

/// Through real MCP tools, a read root grants no write authority and vice versa.
/// Unlike a path-only unit test, this checks routing through Server -> Headless -> file I/O.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn mcp_file_roots_confine_open_save_and_keep_outside_documents_unchanged() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU64, Ordering};
    use wordcraft_engine::file_access::AutomationRoots;

    struct TempDir(std::path::PathBuf);
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let dir = TempDir(std::env::temp_dir().join(format!("wordcraft-mcp-roots-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed))));
    for part in ["read", "write", "outside"] {
        std::fs::create_dir_all(dir.0.join(part)).unwrap();
    }
    let source = dir.0.join("read/input.txt");
    let forbidden = dir.0.join("outside/private.txt");
    let saved = dir.0.join("write/result.docx");
    std::fs::write(&source, b"Allowed source marker.").unwrap();
    std::fs::write(&forbidden, b"Untouched private marker.").unwrap();

    let roots = AutomationRoots::new(Some(&dir.0.join("read")), Some(&dir.0.join("write"))).unwrap().unwrap();
    let mut server = Server::new(Box::new(Headless::default().with_automation_roots(Some(Arc::clone(&roots)))));
    let request = |server: &mut Server, tool: &str, path: &std::path::Path| {
        call(server, 17, "tools/call", json!({"name": tool, "arguments": {"path": path.to_string_lossy()}}))
    };

    let denied_read = request(&mut server, "open_document", &forbidden);
    assert_eq!(denied_read["result"]["isError"], true, "{denied_read}");
    assert!(denied_read["result"]["content"][0]["text"].as_str().unwrap().contains("outside the read root"));

    let allowed_read = request(&mut server, "open_document", &source);
    assert_eq!(allowed_read["result"]["isError"], false, "{allowed_read}");
    let text = tool(&mut server, "get_text", json!({}));
    assert!(text["text"].as_str().unwrap().contains("Allowed source marker."));

    let denied_write = request(&mut server, "save_document", &forbidden);
    assert_eq!(denied_write["result"]["isError"], true, "{denied_write}");
    assert!(denied_write["result"]["content"][0]["text"].as_str().unwrap().contains("outside the write root"));
    assert_eq!(std::fs::read(&forbidden).unwrap(), b"Untouched private marker.");

    let denied_read_root_write = request(&mut server, "save_document", &source);
    assert_eq!(denied_read_root_write["result"]["isError"], true, "{denied_read_root_write}");
    assert_eq!(std::fs::read(&source).unwrap(), b"Allowed source marker.");

    let allowed_write = request(&mut server, "save_document", &saved);
    assert_eq!(allowed_write["result"]["isError"], false, "{allowed_write}");
    assert!(saved.is_file());
    let denied_reopen = request(&mut server, "open_document", &saved);
    assert_eq!(denied_reopen["result"]["isError"], true, "{denied_reopen}");

    let document = wordcraft_engine::io::open_path(&saved).unwrap();
    assert!(document.plain_text(wordcraft_doc::StoryRef::Body).contains("Allowed source marker."));
    assert_eq!(std::fs::read(&forbidden).unwrap(), b"Untouched private marker.");
}
