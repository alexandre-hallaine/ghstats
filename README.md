# GHStats

Stats from GitHub Archive data.

## Features
- Parallel processing
- JSON statistics

## Statistics Collected
- **Event Types**: Count of each GitHub event type
- **Languages**: Programming languages in pull requests
- **Hourly Activity**: Event counts by hour (0-23)

## Quick Start
```sh
# Process a specific day
./scripts/download.sh 2025-01-01
./scripts/analyze.sh 2025-01-01

# Or process a full month (e.g., January 2025)
./scripts/run.sh 2025 1

# Or process in a loop (e.g., January 2015 - December 2024)
for y in {2015..2024}; do for m in {1..12}; do bash scripts/run.sh "$y" "$m"; done; done
```

## Output Example
```json
{
  "events": { "ForkEvent": 7144, "PushEvent": 119242 },
  "hours": { "0": 7702, "1": 7427 },
  "languages": { "C": 365, "Rust": 129 }
}
```
