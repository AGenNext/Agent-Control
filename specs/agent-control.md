# Agent Control

**Agent Control** is the control-plane layer that manages desired state, policy enforcement, runtime coordination, drift detection, and reconciliation across governed agent systems.

It is not just orchestration. It is a **governed control loop for agent operating systems**.

```txt
Agent Control =
  Desired State
+ Policy Gate
+ Runtime Coordination
+ Drift Detection
+ Decision Control
+ Event Observation
+ Evidence Verification
+ Reconciliation
+ Audit
```

## Definition

An **Agent Control** plane coordinates one or more agent runtimes and kernels.

It tells the fabric:

1. what desired state should exist
2. which runtimes and agents are responsible
3. which policies apply before change
4. what actual state is observed
5. where drift exists
6. what decision must be made
7. what reconciliation action is allowed
8. what evidence proves convergence
9. what audit trail must be preserved

## Relationship To The Stack

```txt
Agent Blueprint = Build Plan
Agent Protocol  = Communication
Agent Runtime   = Execution Environment
Agent Kernel    = Trusted Core
Agent Control   = Control Plane
```

## Control Loop

```txt
Declare Desired State
        ↓
Observe Actual State
        ↓
Detect Drift
        ↓
Evaluate Policy
        ↓
Record Decision
        ↓
Apply Change / Hold / Deny / Escalate
        ↓
Emit Event
        ↓
Capture Evidence
        ↓
Update State
        ↓
Reconcile Until Trusted
```

## Canonical Shape

```json
{
  "kind": "AgentControl",
  "version": "0.1.0",
  "id": "control_001",
  "name": "Governed Agent Control Plane",
  "scope": {
    "runtimes": ["runtime:default"],
    "kernels": ["kernel:default"],
    "agents": ["agent:operator"],
    "targets": ["github:AGenNext/Agent-Control"]
  },
  "desired_state": {
    "status": "published",
    "artifacts": ["specs/agent-control.md"],
    "policy": "policy_gated",
    "evidence_required": true
  },
  "observed_state": {
    "status": "pending",
    "artifacts": [],
    "evidence": []
  },
  "drift": {
    "detected": true,
    "type": "missing_state",
    "severity": "medium"
  },
  "policy": {
    "required": true,
    "default_effect": "hold_or_deny",
    "engines": ["opa", "openfga", "authzen"]
  },
  "decision": {
    "required": true,
    "record_basis": true
  },
  "reconciliation": {
    "mode": "event_driven_or_continuous",
    "allowed_actions": ["create", "update", "rollback", "hold", "escalate"],
    "forbidden_actions": ["bypass_policy", "trust_without_evidence"]
  },
  "audit": {
    "required": true,
    "records": ["decision", "event", "evidence", "state", "reconciliation"]
  }
}
```

## Control Responsibilities

### Desired State Management

Defines what should exist across agents, runtimes, policies, contracts, tools, and state stores.

### Runtime Coordination

Coordinates execution across multiple runtimes without allowing runtime-local shortcuts to bypass policy.

### Drift Detection

Compares actual state against desired state and classifies missing, stale, unauthorized, conflicting, or external drift.

### Policy-Gated Change

Ensures all changes are evaluated before execution and again before trust elevation.

### Reconciliation Control

Applies repair, rollback, hold, denial, or escalation until the system reaches trusted state or a declared failure state.

### Audit Control

Ensures every control action produces decision, event, evidence, state, and reconciliation records.

## Control vs Runtime

| Runtime | Control |
|---|---|
| Executes commands | Coordinates desired state |
| Runs tools | Decides when change is needed |
| Emits events | Observes event streams |
| Captures evidence locally | Verifies evidence globally |
| Maintains local state | Governs system state |
| Responds to commands | Continuously reconciles drift |

## Conformance Rules

A conformant Agent Control plane must satisfy these rules:

1. Desired state must be explicit.
2. Actual state must be observable.
3. Drift must be classified.
4. Policy must be evaluated before change.
5. Decisions must record their basis.
6. Events must be emitted for control transitions.
7. Evidence must be captured before trusted state.
8. Reconciliation must never bypass policy.
9. Runtime-local execution must not override control-plane authority.
10. Control failure must result in hold, deny, escalate, rollback, or failed state.

## Final Definition

**Agent Control is the governed control plane that continuously drives agent systems from observed state toward policy-compliant trusted state.**
