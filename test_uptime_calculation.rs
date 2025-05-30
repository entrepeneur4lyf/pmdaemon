// Test script to verify uptime calculation is working correctly
use chrono::{Duration, Utc};

fn format_duration(duration: chrono::Duration) -> String {
    let total_seconds = duration.num_seconds();

    if total_seconds < 60 {
        format!("{}s", total_seconds)
    } else if total_seconds < 3600 {
        format!("{}m", total_seconds / 60)
    } else if total_seconds < 86400 {
        format!("{}h", total_seconds / 3600)
    } else {
        format!("{}d", total_seconds / 86400)
    }
}

fn main() {
    println!("🧪 Testing Uptime Calculation");
    println!("==============================");
    
    // Test current time calculation
    let now = Utc::now();
    println!("Current time: {}", now);
    
    // Test various durations
    let test_cases = vec![
        (5, "5s"),
        (30, "30s"),
        (90, "1m"),
        (150, "2m"),
        (3661, "1h"),
        (7200, "2h"),
        (86401, "1d"),
        (172800, "2d"),
    ];
    
    println!("\nTesting duration formatting:");
    for (seconds, expected) in test_cases {
        let duration = Duration::seconds(seconds);
        let formatted = format_duration(duration);
        let status = if formatted == expected { "✓" } else { "✗" };
        println!("  {} {}s -> {} (expected: {})", status, seconds, formatted, expected);
    }
    
    // Test with actual time differences
    println!("\nTesting with actual time differences:");
    
    // Simulate a process that started 30 seconds ago
    let started_30s_ago = now - Duration::seconds(30);
    let uptime_30s = now - started_30s_ago;
    println!("  Process started 30s ago: {}", format_duration(uptime_30s));
    
    // Simulate a process that started 2 minutes ago
    let started_2m_ago = now - Duration::seconds(120);
    let uptime_2m = now - started_2m_ago;
    println!("  Process started 2m ago: {}", format_duration(uptime_2m));
    
    // Simulate a process that started 1 hour ago
    let started_1h_ago = now - Duration::seconds(3600);
    let uptime_1h = now - started_1h_ago;
    println!("  Process started 1h ago: {}", format_duration(uptime_1h));
    
    println!("\n✅ Uptime calculation test complete!");
    println!("If all tests show ✓, the uptime calculation logic is working correctly.");
}
