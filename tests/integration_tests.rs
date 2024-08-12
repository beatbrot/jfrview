use std::fs;
use assert_cmd::Command;
use std::fs::remove_file;

#[test]
fn jfrview_opens_without_file() {
    TestRun::new(None)
        .assert_success();
}

#[test]
fn jfrview_opens_with_file() -> anyhow::Result<()> {
    let files = fs::read_dir("test-data")?;
    for path in files {
        let dir_entry = path?;
        let file_name = dir_entry.file_name().to_string_lossy().to_string();
        if file_name.ends_with(".jfr") {
            TestRun::new(Some(format!("test-data/{file_name}")))
                .assert_success();
        }
    }
    Ok(())
}

struct TestRun {
    file: Option<String>,
}

impl Drop for TestRun {
    fn drop(&mut self) {
        let _ = remove_file("out.png");
    }
}

impl TestRun {
    fn new(file: Option<String>) -> Self {
        Self { file }
    }

    fn assert_success(&self) {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap();
        cmd.env("EFRAME_SCREENSHOT_TO", "out.png");
        if let Some(f) = &self.file {
            cmd.arg(f);
        }

        cmd.assert().success();
    }
}
