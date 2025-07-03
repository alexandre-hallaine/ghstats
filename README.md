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
# Process a full year, month, or day:
./scripts/process.sh 2025        # Year
./scripts/process.sh 2025 1      # Month
./scripts/process.sh 2025 1 1    # Day

# Or run specific gharchives dates directly:
parallel --eta ./scripts/core.sh ::: 2025-01-01 2025-02-15 2025-03-10 2025-04-25
```

## Output Example

```json
{
  "events": { "ForkEvent": 7144, "PushEvent": 119242 },
  "hours": { "0": 7702, "1": 7427 },
  "languages": { "C": 365, "Rust": 129 }
}
```
