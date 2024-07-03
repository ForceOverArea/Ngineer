# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0]
### Added
- Created a json schema for model files
- Added heat transfer module with new model components
    - This module is supported by the default solver engine as well
### Changed
- CLI now has `--precision` and `--iterations` flags which specify the maximum allowable error and 
maximum number of allowed iterations, respectively 

## [0.2.0]
### Added 
- neapolitan executable for creating `.soln.json` files for a given JSON-formatted model file
- serde-friendly `NodalAnalysisModel` struct for deserializing from JSON (or other formats if support is added manually)
- "factory pattern-esque" and builder pattern structs for modelling natively in Rust

## [0.1.0]
Initial release