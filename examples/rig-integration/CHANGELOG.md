# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/modelcontextprotocol/rust-sdk/compare/rig-integration-v0.1.5...rig-integration-v0.2.0) - 2025-07-02

### Added

- *(model)* add json schema generation support for all model types ([#176](https://github.com/modelcontextprotocol/rust-sdk/pull/176))
- *(transport)* support streamable http server ([#152](https://github.com/modelcontextprotocol/rust-sdk/pull/152))

### Fixed

- *(examples)* add clients in examples's readme ([#225](https://github.com/modelcontextprotocol/rust-sdk/pull/225))
- cleanup zombie processes for child process client ([#156](https://github.com/modelcontextprotocol/rust-sdk/pull/156))
- fix 2 cargo warnings ([#112](https://github.com/modelcontextprotocol/rust-sdk/pull/112))
- *(rig-integration)* implement ToolEmbeddingDyn trait for McpToolAdaptor ([#99](https://github.com/modelcontextprotocol/rust-sdk/pull/99))
- fix rig chat agent output format ([#86](https://github.com/modelcontextprotocol/rust-sdk/pull/86))
- *(typo)* s/marcos/macros/ ([#85](https://github.com/modelcontextprotocol/rust-sdk/pull/85))
- *(test)* fix tool deserialization error ([#68](https://github.com/modelcontextprotocol/rust-sdk/pull/68))

### Other

- refactor tool macros and router implementation ([#261](https://github.com/modelcontextprotocol/rust-sdk/pull/261))
- *(deps)* update rig-core requirement from 0.12.0 to 0.13.0 ([#259](https://github.com/modelcontextprotocol/rust-sdk/pull/259))
- *(deps)* update rig-core requirement from 0.10.0 to 0.12.0 ([#231](https://github.com/modelcontextprotocol/rust-sdk/pull/231))
- add example-test workflow for testing and building examples ([#211](https://github.com/modelcontextprotocol/rust-sdk/pull/211))
- revert badge ([#202](https://github.com/modelcontextprotocol/rust-sdk/pull/202))
- use hierarchical readme for publishing ([#198](https://github.com/modelcontextprotocol/rust-sdk/pull/198))
- Ci/coverage badge ([#191](https://github.com/modelcontextprotocol/rust-sdk/pull/191))
- fix error introduced by merge, and reorganize feature ([#185](https://github.com/modelcontextprotocol/rust-sdk/pull/185))
- Transport trait and worker transport, and streamable http client with those new features. ([#167](https://github.com/modelcontextprotocol/rust-sdk/pull/167))
- add oauth2 support ([#130](https://github.com/modelcontextprotocol/rust-sdk/pull/130))
- update calculator example description ([#115](https://github.com/modelcontextprotocol/rust-sdk/pull/115))
- fix the url ([#120](https://github.com/modelcontextprotocol/rust-sdk/pull/120))
- add a simple chat client for example ([#119](https://github.com/modelcontextprotocol/rust-sdk/pull/119))
- add spell check ([#82](https://github.com/modelcontextprotocol/rust-sdk/pull/82))
- Adopt Devcontainer for Development Environment ([#81](https://github.com/modelcontextprotocol/rust-sdk/pull/81))
- fix typos ([#79](https://github.com/modelcontextprotocol/rust-sdk/pull/79))
- format and fix typo ([#72](https://github.com/modelcontextprotocol/rust-sdk/pull/72))
- add documentation generation job ([#59](https://github.com/modelcontextprotocol/rust-sdk/pull/59))
- fmt the project ([#54](https://github.com/modelcontextprotocol/rust-sdk/pull/54))
- fix broken link ([#53](https://github.com/modelcontextprotocol/rust-sdk/pull/53))
- fix the branch name for git dependency ([#46](https://github.com/modelcontextprotocol/rust-sdk/pull/46))
- Move whole rmcp crate to offcial rust sdk ([#44](https://github.com/modelcontextprotocol/rust-sdk/pull/44))
- Initial commit
