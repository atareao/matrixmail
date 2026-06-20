# Changelog
## [0.1.5] - 2026-06-20

### Bug Fixes

- Use alpine:latest and amd64-only Docker build

### Miscellaneous Tasks

- Drop arm/v7 from Docker platforms (unsupported by rust:alpine)
## [0.1.4] - 2026-06-20

### Miscellaneous Tasks

- Prepare release v0.1.3
- Release v0.1.3
- Add QEMU setup for multi-platform Docker build
- Release v0.1.4

### Other

- Bump rustix to 0.37.25
- Bump self_cell to 1.0.2
## [0.1.2] - 2026-06-20

### Miscellaneous Tasks

- Release v0.1.2 (#5)
- Bump version to 0.1.2 and add metadata
## [0.1.1] - 2026-06-20

### Miscellaneous Tasks

- Merge Cargo.lock and .codegraph config
- Configure gitflow with CI/CD workflows
- Release v0.1.1

### Refactor

- Simplify nested conditional logic in `process_response` function
- Change pull_time from u16 to u64 for higher precision
