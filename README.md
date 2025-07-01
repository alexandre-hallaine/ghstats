# GHStats

Stats from GitHub Archive data.

## Features
- Parallel processing
- JSON statistics

## Statistics Collected
- **Event Types**: Count of each GitHub event type
- **Languages**: Programming languages in pull requests
- **Top Repositories**: Most active repositories (top 100)
- **Top Actors**: Most active users (top 100)
- **Top Organizations**: Most active organizations (top 100)
- **Hourly Activity**: Event counts by hour (0-23)

## Quick Start
```sh
# Process a full month (e.g., January 2025)
./scripts/run.sh 2025 1

# Or process a specific day
./scripts/download.sh 2025-01-01
./scripts/analyze.sh 2025-01-01
```

## Output Example
```json
{
  "event_types": [ {"event_type": "PushEvent", "count": 456789} ],
  "languages": [ {"language": "JavaScript", "count": 1234} ],
  "top_repositories": [ {"repository": "owner/repo1", "count": 1234} ],
  "top_actors": [ {"actor": "username1", "count": 567} ],
  "top_organizations": [ {"organization": "orgname", "count": 321} ],
  "hourly_activity": [ {"hour": 0, "count": 12345} ]
}
```
