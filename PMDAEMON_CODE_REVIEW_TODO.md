# PMDaemon Code Review - Todo List

This document outlines suggested changes, improvements, and areas for future development for the PMDaemon project, based on a comprehensive code review.

## I. Features to Implement / Complete

These are features noted as planned, partially implemented, or desirable for enhanced functionality.

-   **[ ] `config.rs` - Implement Process Watching:**
    -   Add functionality for the `watch: Option<bool>` field in `ProcessConfig`.
    -   Implement logic to monitor specified paths/files for changes and automatically restart processes.
    -   Consider using a crate like `notify` for efficient file system event monitoring.
-   **[ ] `config.rs` - Implement `ignore_watch`:**
    -   Add support for the `ignore_watch: Option<Vec<String>>` field to exclude certain files/patterns from watch-triggered restarts.
-   **[ ] `config.rs` - Implement User/Group Switching:**
    -   Implement functionality for `user: Option<String>` and `group: Option<String>` fields in `ProcessConfig`.
    -   Ensure PMDaemon has necessary permissions to switch users/groups (typically requires running as root initially or specific capabilities).
    -   Carefully handle security implications.
-   **[ ] `web.rs` - Implement True Log Streaming for HTTP API:**
    -   The `follow: Option<bool>` parameter in `LogsQuery` for `/api/processes/:id/logs` currently doesn't implement real-time streaming over HTTP.
    -   Consider using Server-Sent Events (SSE) or a chunked transfer encoding response to stream logs when `follow` is true.
-   **[ ] `web.rs` - Implement Bidirectional WebSocket Communication (Optional):**
    -   Currently, the WebSocket connection in `handle_socket` is primarily server-to-client for broadcasts. Client messages are logged but not acted upon.
    -   If desired, implement handlers for client-sent WebSocket messages (e.g., requesting specific process logs, sending commands).

## II. Enhancements & Potential Refinements

Suggestions for improving existing code, clarity, or robustness.

### `manager.rs`
-   **[ ] Review/Confirm Implied Methods:**
    -   The CLI (`src/bin/pmdaemon.rs`) and Web API (`web.rs`) handlers imply the existence of several `ProcessManager` methods that were not fully detailed in the reviewed snippets (e.g., `start_existing`, `reload`, specific `delete` variants for status, `*_cli` formatting methods, `get_logs`, `stream_logs`, `stream_all_logs`).
    -   **Action**: Ensure these methods are fully implemented, robust, well-tested, and documented within `manager.rs`.
-   **[ ] Cluster Mode Rollback Logic (Already Good, Consider Edge Cases):**
    -   The rollback logic in `start_cluster` (stopping already started instances if one fails) is good.
    -   **Action**: Briefly review for any subtle edge cases, e.g., what happens if a cleanup `stop` operation itself fails during rollback.

### `process.rs`
-   **[ ] `Process::check_status` Guard Clause Review:**
    -   The guard clause `if self.child.is_none() { return Ok(true); }` (line 531-533) might be slightly counter-intuitive if the process state implies it *should* have a child.
    -   **Action**: Review the contexts where `check_status` is called. If `self.child` is `None`, it usually means the process is already considered stopped or errored. Ensure this logic is sound in all calling paths (likely fine as is, but worth a quick double-check).

### `web.rs`
-   **[ ] CORS Policy Review for Production:**
    -   The current CORS policy (`CorsLayer::new().allow_origin(Any)...`) is very permissive.
    -   **Action**: If the web API is intended for wider access than just a local UI, consider restricting origins, methods, and headers for enhanced security. For a local-only tool, it might be acceptable.
-   **[ ] API Error Standardization (Already Good, Maintain):**
    -   The `api_error_response` helper is good for mapping internal errors to HTTP responses.
    -   **Action**: Ensure all new API endpoints consistently use this or a similar structured error reporting mechanism.

### General
-   **[ ] Logging Review (Already Good, Consider Granularity):**
    -   The project uses `tracing` effectively.
    -   **Action**: As features are added (like file watching), ensure logging levels and messages provide clear diagnostic information without being excessively verbose in default configurations.

## III. Testing Suggestions

Recommendations for expanding test coverage. The current integration tests are a good foundation.

-   **[ ] `integration_tests.rs` - Log Content Verification:**
    -   Extend tests for `pmdaemon logs` (or API log endpoints) to verify the *content* of the logs, not just command success.
-   **[ ] `integration_tests.rs` - PID Change on Restart:**
    -   For `test_restart_process`, assert that the Process ID (PID) changes after a restart, providing stronger evidence of a true restart. This would require capturing the PID from the `list` command output.
-   **[ ] `integration_tests.rs` - Web API Tests:**
    -   Create a new test suite (or add to existing) specifically for the Web API (`/api/*` endpoints and `/ws`).
    -   Use a crate like `reqwest` for HTTP calls and a WebSocket client crate to test API responses, status codes, and WebSocket broadcast behavior.
-   **[ ] `integration_tests.rs` - Health Check Scenarios:**
    -   Add dedicated integration tests for various health check configurations:
        -   HTTP health checks (success, failure, timeout).
        -   Script-based health checks (success, failure, timeout, script arguments/env vars).
        -   Processes transitioning between healthy/unhealthy states.
-   **[ ] `integration_tests.rs` - Process Failure & Auto-Restart:**
    -   Test scenarios where managed processes crash or exit unexpectedly.
    -   Verify PMDaemon's auto-restart logic, including `max_restarts` and `min_uptime` behavior.
    -   Check that processes are correctly marked as `Errored` or transition through restart attempts.
-   **[ ] `integration_tests.rs` - Resource Limit Tests (Max Memory):**
    -   Test the `max_memory_restart` feature by creating a process that intentionally exceeds its memory limit and verify it gets restarted.
-   **[ ] Unit Tests for Complex Logic:**
    -   While integration tests are broad, ensure complex logic within specific modules (e.g., port parsing in `src/bin/pmdaemon.rs`, intricate state transitions in `manager.rs`) also has focused unit tests.

## IV. Documentation

The inline Rustdoc comments are excellent.

-   **[ ] Maintain High Standard of Inline Documentation:**
    -   Continue providing thorough Rustdoc comments for all public functions, structs, enums, and important internal logic.
-   **[ ] README.md / User Guide Enhancement (Ongoing):**
    *   Ensure `README.md` is comprehensive, covering installation, all CLI commands with examples, configuration options, and API usage if applicable.
    *   Consider a separate `GUIDE.md` or `docs/` directory for more detailed documentation as the project grows.
-   **[ ] `CHANGELOG.md` (Already Good, Maintain):**
    *   Continue to diligently update the changelog for each release.
-   **[ ] Contribution Guidelines (`CONTRIBUTING.md`):**
    *   If open to external contributions, create a `CONTRIBUTING.md` outlining how to contribute, coding standards, and the development process.

This todo list should provide a good roadmap for further development and refinement of PMDaemon.
