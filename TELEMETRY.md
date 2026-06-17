# Telemetry Policy

## Zero Telemetry

MongoDesk collects **zero telemetry data** by default.

### What We Don't Collect

- No analytics or tracking
- No usage statistics
- No error reports (unless explicitly enabled)
- No network requests to external servers
- No user identification data

### What We Do Store Locally

- Connection configurations (encrypted in SQLite)
- Query history (stored in SQLite)
- User settings (stored in SQLite)
- Window state (stored in SQLite)

### Future Opt-In (v1.1+)

If we ever add optional analytics, it will be:

1. **Opt-in only** — Never enabled by default
2. **Transparent** — Clear disclosure of what is collected
3. **Controllable** — Easy to disable in Settings
4. **Documented** — Listed in this file and Settings page

### Network Requests

MongoDesk only makes network requests to:

- MongoDB servers you explicitly connect to
- DNS resolution (system default)

No other network traffic is generated.

### Verification

You can verify this by:

1. Checking the source code (open source)
2. Monitoring network traffic with tools like Wireshark
3. Reviewing the Tauri security configuration

## Contact

If you have questions about our telemetry policy, please open an issue on GitHub.
