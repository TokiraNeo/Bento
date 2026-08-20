<div style="text-align: center">

# Bento

> A lunchbox for tools — compartments, not a buffet; a switchboard, not a pile of adapters.

**A tool-relay hub for AI agents and tool hosts.**

![Rust](https://img.shields.io/badge/Rust-1.85+-e44c26?style=flat-square&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-GPL--3.0-333850?style=flat-square&logo=gnu&logoColor=white)
![Status](https://img.shields.io/badge/status-early--stage-e8a33d?style=flat-square)
![Arch](https://img.shields.io/badge/platform-Windows%20%2F%20macOS%20%2F%20Linux-6c7293?style=flat-square)

把 **N×M 的不可复用集成**，变成 **N+M 的标准化适配**。

</div>

---

## 这是什么

AI 正在进入游戏开发流程（建模、摆放、材质、动画），但集成形态高度碎片化：

- **宿主侧**：Blender（bpy）、UE（Python/C++）、Maya…… 每家暴露工具的方式完全不同；
- **Agent 侧**：Codex、OpenCode、Cursor…… 各家已有成熟的 Agent 循环（工具调用、上下文压缩、审批）。

每对接一个 Agent × 一个宿主，就要重写一遍连接、协议、工具注册——这就是 **N×M 的组合爆炸**。

**Bento 在中间加一层 Hub**：宿主只写一个薄插件把工具暴露出来，Agent 通过标准 MCP **零适配**即插即用，任何一端改动都不牵连另一端。

对 Agent 来说，Bento 更像一位 **总机接线员**：手里有一本实时更新的黄页（工具检索）。你用大白话描述需求，她帮你查号、给你看说明书、替你转接到真正干活的宿主；危险操作还要先请示。

---

## 核心设计

整个项目围绕三个核心理念展开。

### 1. 一个适配中间层：让一切能暴露工具的进程接入

```
Agent 层（Codex / OpenCode / Cursor ...）
          │  MCP（零适配，即插即用）
          ▼
┌──────────────────────────────┐
│        Bento Hub             │
│  路由 · 命名空间 · 审批 · RAG│
└──────┬───────────────┬───────┘
       │ WS+JSON-RPC   │ MCP Client
       ▼               ▼
原生宿主（Blender/UE/Maya）   已有 MCP Server（fs/git/browser...）
```

- **原生宿主**：写一个薄插件，主动连入 Hub 的 WS 端口（`host.hello` 握手 → `tools.register` 注册 → `host.ready` 就绪）；
- **Agent**：把 Hub 当成一个普通的 MCP Server 配置，主流客户端天然可用；
- **唯一通路**：Agent 永不直连宿主，一切消息经 Hub 中转，审计 / 安全 / 路由单点完成。

### 2. 在中间层对暴露做操作：RAG 渐进式工具加载

一个 UE5 这样的宿主，工具集可能有数百上千个。把全部 schema 灌进 LLM 上下文，既不现实也浪费
token——问题结构和搜索引擎一样：语料大到塞不进消费者的脑子。

**让 LLM 自己「搜」工具，而不是「灌」给它。结果页只给摘要，点进去才有全文。**

```
Layer 1  bento.list_domains          → 域级目录，LLM 知道「存在什么」
Layer 2  bento.search_tools(query)   → 混合检索 Top-K，LLM 知道「该用哪个」
Layer 3  bento.get_tool_schema(name) → 调用前取完整 schema，LLM 知道「怎么调」
```

每个工具注册时携带 `name / description / input_schema / risk / domain / tags / example`， description 本身即为检索文本。Hub
只把少量候选工具暴露给 Agent，上下文自然瘦身。

### 3. 过程式工具：与其封装一百个工具，不如给一个 `execute_code()`

长尾能力不应「一能力一 schema」——那会让工具列表爆炸、维护成本失控、LLM 注意力被稀释。

```python
# 与其封装 create_cube / set_material / rotate_object / ...
# 不如给 agent 一把「钥匙」：
blender.execute_script("""
    bpy.ops.mesh.primitive_cube_add(size=2.0)
    mat = bpy.data.materials.new("Red")
    ...
""")
```

- 常驻工具目录因此极小，RAG 压力骤减；
- RAG 检索的是 **安全的高层封装工具**，长尾能力交给脚本执行；
- `execute_script` 类工具 **默认 `risk = high`**，走 Hub 侧强制审批。

---

## 工程布局

```
bento_core                         ← 组装根：创建各组件、注入端口
  ├── bento_host_server            ← 宿主 WS：握手 / 注册 / 调用分发 / 断连清理
  ├── bento_agent_server           ← Agent MCP：search_tools / get_schema / list_domains
  └── bento_tool_rag               ← 内存工具目录 + 混合检索（词法 BM25 → 语义）
  └── bento_protocol               ← 传输契约（JSON-RPC / ToolDefinition / 检索 DTO）
```

| Crate                 | 包名                 | 职责                                                                                        |
|-----------------------|----------------------|---------------------------------------------------------------------------------------------|
| `crates/protocol`     | `bento_protocol`     | 跨边界传输类型：宿主 JSON-RPC 命令、`ToolDefinition`、`ToolSearchQuery` / `ToolSearchHit`。 |
| `crates/utility`      | `bento_utility`      | 进程内小工具（如 UUID）。                                                                   |
| `crates/tool_rag`     | `bento_tool_rag`     | 纯内存工具索引：会话桶目录、词法倒排、检索快照。不落盘；宿主断连即清、进程重启即空。        |
| `crates/host_server`  | `bento_host_server`  | 被动监听宿主 WS。通过 `ToolIndexSink` 端口登记 / 就绪 / 移除工具。                          |
| `crates/agent_server` | `bento_agent_server` | 面向 Agent 的 MCP 服务端。通过 `ToolQuerySink` 端口检索。                                   |
| `crates/core`         | `bento_core`         | 组装根：持有 `ToolRagEngine`，用薄适配器接到两个 server 的端口。运行时调用直达引擎。        |

## License

[GPL-3.0-or-later](LICENSE) · © 2026 TokiraNeo
