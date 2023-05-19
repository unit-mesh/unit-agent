# Unit Agent

[![Build](https://github.com/unit-mesh/unit-agent/actions/workflows/ci.yml/badge.svg)](https://github.com/unit-mesh/unit-agent/actions/workflows/ci.yml)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/unit-mesh/unit-lsp-server)

Todos:

- [ ] TreeSitter parser for unit
    - [ ] based on: [https://github.com/BloopAI/bloop/tree/main](https://github.com/BloopAI/bloop/tree/main)
- [ ] Json RPC server with [xi-rpc](https://crates.io/crates/xi-rpc)
    - [ ] CLI
- [ ] Support completion type
    - [ ] Inline
    - [ ] AfterLineEnd
    - [ ] Block

Support for IDE features:

- [ ] Chat with IDE
  - [ ] Generate test
  - [ ] 实现：`重现 xx 功能`, `devti:/chat/feature`
  - [ ] 重构：`重构 xx 方法`
  - [ ] 替换：`替换 xx 方法`，`devti:/refactor/method`
- [x] Custom LLM Server
- [ ] Telemetry Server
  - [ ] Accept
  - [ ] Reject
  - [ ] Feedback
- [ ] Context Engineering
  - [ ] Open Tabs
  - [ ] Related Files
  - [ ] with Dependency Context
    - [ ] parse Gradle for dependencies
  
## JSON RPC Protocol

- Protocol Specification (compatible with Copilot ?)
  - initialize
  - workspace/dependencies

## LICENSE

Apache 2.0
