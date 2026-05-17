use std::collections::VecDeque;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use anyhow::{Context, Result};
use chrono::Local;
use log::{LevelFilter, Log, Metadata, Record};

const MAX_LOG_ENTRIES: usize = 500;

/// 全局日志缓冲区
static LOG_BUFFER: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());

/// 取当前缓存的日志行（供 UI 使用）
pub fn recent_entries() -> Vec<String> {
    LOG_BUFFER.lock().unwrap().iter().cloned().collect()
}

// ──────────────────────────────────────
// 自定义 Logger
// ──────────────────────────────────────

struct AppLogger {
    file: Option<Mutex<fs::File>>,
    level: LevelFilter,
}

impl Log for AppLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let ts = Local::now().format("%H:%M:%S%.3f");
        let level = record.level();
        let target = record.target();
        let msg = record.args();

        let line = format!("[{ts}] {level:<5} [{target}] {msg}");

        // 写入内存缓冲区
        if let Ok(mut buf) = LOG_BUFFER.lock() {
            if buf.len() >= MAX_LOG_ENTRIES {
                buf.pop_front();
            }
            buf.push_back(line.clone());
        }

        // 控制台
        eprintln!("{line}");

        // 文件
        if let Some(ref file) = self.file {
            if let Ok(mut f) = file.lock() {
                let _ = writeln!(f, "{line}");
            }
        }
    }

    fn flush(&self) {
        if let Some(ref file) = self.file {
            if let Ok(mut f) = file.lock() {
                let _ = f.flush();
            }
        }
    }
}

// ──────────────────────────────────────
// 公开 API
// ──────────────────────────────────────

pub fn init(level: LevelFilter, log_file: Option<&Path>) -> Result<()> {
    let file = match log_file {
        Some(path) => {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("failed to create log dir: {:?}", parent))?;
            }
            let f = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .with_context(|| format!("failed to open log file: {:?}", path))?;
            Some(Mutex::new(f))
        }
        None => None,
    };

    let logger = AppLogger { file, level };

    log::set_boxed_logger(Box::new(logger))
        .map(|()| log::set_max_level(level))
        .context("logger already initialized")?;

    log_info!("logger initialized | level: {level}, file: {log_file:?}");
    Ok(())
}

pub fn init_from_env() -> Result<()> {
    let level = match std::env::var("RUST_LOG")
        .unwrap_or_default()
        .to_lowercase()
        .as_str()
    {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    let log_file = std::env::var("LOG_FILE").ok().map(PathBuf::from);
    init(level, log_file.as_deref())
}

/// 把 anyhow::Error 展开打印全链路上下文。
#[allow(dead_code)]
pub fn describe_error(err: &anyhow::Error) {
    log_error!("{err}");
    for cause in err.chain().skip(1) {
        log_error!("  ├─ caused by: {cause}");
    }
}

#[allow(unused_imports)]
pub use log::{
    debug as log_debug, error as log_error, info as log_info, trace as log_trace, warn as log_warn,
};
