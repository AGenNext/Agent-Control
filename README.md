# Agent-Control

**Agent-Control** is a Flux-style drift detection and reconciliation service for enterprise agents.

It answers one operational question:

> Is the running agent still the agent that was approved?

Agent-Control compares an approved **desired agent definition** with observed **runtime snapshots**, detects drift, scores risk, explains what changed, and recommends remediation.

## Why this exists

Long-lived enterprise agents change over time through prompt edits, tool changes, memory growth, knowledge updates, model swaps, workflow changes, and policy updates. Those changes can make the running agent diverge from the approved agent.

Agent-Control tracks that divergence as **Agent Drift**.

## Drift categories

- Capability drift
- Tool drift
- Prompt drift
- Memory drift
- Knowledge drift
- Workflow drift
- Policy drift
- Objective drift
- Model drift
- Configuration drift

## Core loop

```text
Observe -> Compare -> Classify -> Score -> Explain -> Recommend -> Verify
```

## v0.1 scope

- Rust API service using Axum
- Drift comparison engine
- Drift category classifier
- Drift risk scoring
- In-memory service for local development
- SurrealDB schema files for production storage
- Docker and Docker Compose
- GitHub Actions CI

## Quick start

```bash
cargo test
cargo run
```

API runs on `0.0.0.0:8080` by default.

```bash
curl http://localhost:8080/health
```

## Example drift comparison

```bash
curl -X POST http://localhost:8080/api/v1/drift/compare \
  -H 'content-type: application/json' \
  -d '{
    "desired": {
      "agent_id": "agent:research",
      "version": "1.0.0",
      "skills": ["research", "summarize"],
      "tools": ["github"],
      "prompts": ["prompt:v1"],
      "workflows": ["review-before-publish"],
      "policies": ["human-approval"],
      "objectives": ["accurate-research"],
      "constraints": ["no-external-email"],
      "model": "gpt-5",
      "knowledge_version": "2026-05-01",
      "memory_policy": "approved"
    },
    "actual": {
      "agent_id": "agent:research",
      "captured_at": "2026-05-30T00:00:00Z",
      "skills": ["research", "summarize", "shell"],
      "tools": ["github", "slack"],
      "prompts": ["prompt:v2"],
      "workflows": ["direct-publish"],
      "policies": [],
      "objectives": ["accurate-research"],
      "constraints": ["no-external-email"],
      "model": "gpt-5",
      "knowledge_version": "2026-06-01",
      "memory_policy": "unbounded"
    }
  }'
```

## Repository layout

```text
src/
  api/          HTTP handlers
  drift/        drift comparison, classification, scoring
  model/        domain models
  storage/      storage interface and in-memory implementation
schemas/        SurrealDB schema definitions
docs/           product and technical specifications
```

## Non-goals

Agent-Control is not an agent runtime, orchestrator, governance product, identity system, or evaluation framework. It is focused on detecting and managing drift between approved and running agent state.
