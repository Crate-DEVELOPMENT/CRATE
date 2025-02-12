#![feature(test)]

extern crate test;
extern crate crate_core;

use test::Bencher;
use crate_core::{Browser, Profile, Config};

#[bench]
fn bench_browser_creation(b: &mut Bencher) {
    b.iter(|| {
        let browser = Browser::new();
        browser
    });
}

#[bench]
fn bench_browser_with_profile(b: &mut Bencher) {
    let profile = Profile::new("test_profile");
    b.iter(|| {
        let browser = Browser::with_profile(profile.clone());
        browser
    });
}

#[bench]
fn bench_profile_creation(b: &mut Bencher) {
    b.iter(|| {
        let profile = Profile::new("benchmark_profile");
        profile
    });
}

#[bench]
fn bench_profile_preferences(b: &mut Bencher) {
    let profile = Profile::new("test_profile");
    b.iter(|| {
        profile.set_preference("browser.window_size", (1920, 1080));
        profile.set_preference("browser.headless", true);
        profile.get_preferences()
    });
}

#[bench]
fn bench_config_operations(b: &mut Bencher) {
    let config = Config::new();
    b.iter(|| {
        config.set("automation.retry_attempts", 5);
        config.set("automation.timeout", 30.0);
        config.get("automation.retry_attempts")
    });
}

#[bench]
fn bench_browser_navigation(b: &mut Bencher) {
    let browser = Browser::new();
    b.iter(|| {
        browser.navigate("https://example.com");
    });
}

#[bench]
fn bench_element_operations(b: &mut Bencher) {
    let browser = Browser::new();
    browser.navigate("https://example.com");
    b.iter(|| {
        browser.find_element("h1");
        browser.get_element_text("h1");
    });
}

#[bench]
fn bench_screenshot_capture(b: &mut Bencher) {
    let browser = Browser::new();
    browser.navigate("https://example.com");
    b.iter(|| {
        browser.screenshot("benchmark.png");
    });
}

#[bench]
fn bench_concurrent_browsers(b: &mut Bencher) {
    use std::thread;

    b.iter(|| {
        let handles: Vec<_> = (0..4).map(|_| {
            thread::spawn(|| {
                let browser = Browser::new();
                browser.navigate("https://example.com");
                browser.screenshot("concurrent.png");
            })
        }).collect();

        for handle in handles {
            handle.join().unwrap();
        }
    });
}

#[bench]
fn bench_profile_serialization(b: &mut Bencher) {
    let profile = Profile::new("test_profile");
    profile.set_preference("browser.window_size", (1920, 1080));
    profile.set_preference("browser.headless", true);

    b.iter(|| {
        serde_json::to_string(&profile.get_preferences()).unwrap()
    });
}

#[bench]
fn bench_config_validation(b: &mut Bencher) {
    let config = Config::new();
    b.iter(|| {
        config.validate_key("automation.retry_attempts");
        config.validate_value("automation.retry_attempts", 5);
    });
}

#[bench]
fn bench_error_handling(b: &mut Bencher) {
    use crate_core::error::Error;
    
    b.iter(|| {
        let result: Result<(), Error> = Err(Error::BrowserError("test error".into()));
        let _ = result.map_err(|e| e.to_string());
    });
}