DEFINE TABLE agent_drift SCHEMAFULL;

DEFINE FIELD agent_id ON agent_drift TYPE string;
DEFINE FIELD category ON agent_drift TYPE string;
DEFINE FIELD severity ON agent_drift TYPE string;
DEFINE FIELD score ON agent_drift TYPE number;
DEFINE FIELD desired ON agent_drift FLEXIBLE TYPE object;
DEFINE FIELD actual ON agent_drift FLEXIBLE TYPE object;
DEFINE FIELD explanation ON agent_drift TYPE string;
DEFINE FIELD created_at ON agent_drift TYPE datetime;
