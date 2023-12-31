# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.2]
### Added 
- This `CHANGELOG.md` file.

### Fixed 
- The subset `extern "C"` function now works as expected.
- The `free_double_matrix` function now uses `catch_unwind` to better prevent UB.

## [0.0.1]
### Fixed
- A segfault in the transpose `extern "C"` function.

## [0.0.0]
Initial release

<!-- [0.0.2]: https://github.com/ForceOverArea/gmatlib/compare/v0.0.1...v0.0.2 -->
<!-- [0.0.1]: https://github.com/ForceOverArea/gmatlib/compare/v0.0.0...v0.0.1 -->
[0.0.0]: https://github.com/ForceOverArea/gmatlib/releases/tag/v0.0.0