# GHStats

Analyze GitHub language trends from GitHub Archive data.

## Usage

```bash
# Download and analyze data for a specific date
./scripts/run.sh 2025 1 1

# Or step by step
./scripts/download.sh 2025 1 1
./scripts/analyze.sh 2025 1 1
```

## Output

Results saved to `output/analysis_YYYY-MM-DD.json`:

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
