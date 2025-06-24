# GHStats

Analyze GitHub language trends from GitHub Archive data.

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
  "languages": [
    {"language": "JavaScript", "count": 1234},
    {"language": "Python", "count": 987}
  ],
  "files_processed": 24
}
```

## Requirements

- Rust
- curl

## Data Source

Data is downloaded from [GitHub Archive](https://www.gharchive.org/) which provides hourly snapshots of GitHub activity.
