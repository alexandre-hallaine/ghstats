# GHStats

Generate comprehensive statistics from GitHub Archive data.

## Usage

### Quick Start - Process entire month
```bash
# Download and analyze data for an entire month (e.g., January 2025)
./scripts/run.sh 2025 1
```

### Step by step - Process specific date
```bash
# Download data for a specific date (YYYY-MM-DD format)
./scripts/download.sh 2025-01-01

# Analyze the downloaded data
./scripts/analyze.sh 2025-01-01
```

## Scripts

- **`run.sh`**: Processes an entire month by downloading and analyzing data for each day
- **`download.sh`**: Downloads 24 hours of GitHub Archive data for a specific date
- **`analyze.sh`**: Builds the Rust binary and analyzes downloaded data

## Output

Results saved to `output/YYYY-MM-DD.json`:

```json
{
  "event_types": [
    {"count": 456789, "event_type": "PushEvent"},
    {"count": 123456, "event_type": "CreateEvent"},
    {"count": 98765, "event_type": "PullRequestEvent"}
  ],
  "languages": [
    {"count": 1234, "language": "JavaScript"},
    {"count": 987, "language": "Python"}
  ],
  "top_repositories": [
    {"count": 1234, "repository": "owner/repo1"},
    {"count": 987, "repository": "owner/repo2"}
  ],
  "top_actors": [
    {"count": 567, "actor": "username1"},
    {"count": 432, "actor": "username2"}
  ],
  "hourly_activity": [
    {"count": 12345, "hour": 0},
    {"count": 23456, "hour": 1},
    {"count": 34567, "hour": 2}
  ]
}
```

## Statistics Generated

- **Event Types**: Count of different GitHub event types (PushEvent, PullRequestEvent, etc.)
- **Languages**: Programming language statistics from pull request events
- **Top Repositories**: Most active repositories by event count (top 100)
- **Top Actors**: Most active users by event count (top 100)
- **Hourly Activity**: Activity patterns throughout the day (0-23 hours)

## Requirements

- Rust
- curl

## Data Source

Data is downloaded from [GitHub Archive](https://www.gharchive.org/) which provides hourly snapshots of GitHub activity including events like pushes, pull requests, issues, and more.
