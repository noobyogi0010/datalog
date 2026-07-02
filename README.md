# Datalog 🗣️📊

Talk to your data like it's a human.

```
You: "Show me temperature variations from the farm's IoT device over the last 6 months"

Datalog: *understands your query, finds the right database, fetches the data, 
          plots a gorgeous chart, streams it to your screen*

You: "now filter by device B"

Datalog: *remembers context, reruns the query, updates the chart in real-time*
```

---

## What's this?

Datalog is a natural language interface to any data source — databases, data warehouses, S3 lakes, REST APIs, anything. You ask it a question in plain English. It figures out what you mean, builds the SQL, fetches the results, and renders an interactive chart or table in a chat-like UI.

No more writing SQL. No more jumping between tools. Just conversation.

---

## The tech inside

Built from scratch to learn **hard systems concepts** while shipping something genuinely useful:

- **Rust** for the compute-heavy bits: LLM intent parsing, query planning, RAG vector retrieval, streaming Arrow data
- **Go** for the network layer: WebSocket hub, auth, rate limiting, credential vaults
- **React** for the UI: chat panels, real-time charts, schema explorer
- **Qdrant** vector DB for semantic schema retrieval — because `select * from tables` gets old fast at scale
- **MCP servers** as an extensible connector pattern — add a new data source without touching the core engine

No frameworks, no abstractions doing magic behind the scenes. The whole point is to understand how the pieces actually work.

---

## Current status

🚧 **In development** — 7 epics, 47 stories, ~28 weeks of building.

Sprints 1–2 are scaffold + CI. By sprint 6 you've got a working LLM-powered intent parser. By sprint 12 you've shipped a production-grade beta with auth, observability, and RAG. Sprints 13–14 are the crown jewel: semantic retrieval so good your users stop writing SQL entirely.

---

## Quick start (coming soon)

```bash
# Once the project exists:
git clone <this-repo>
docker-compose up
# Visit http://localhost:3000
# Configure a database
# Ask it a question in English
# Watch it work
```

For now, read the [architecture docs](./docs/) and the [story list](./docs/STORIES.md) to see what's being built.

---

## Learning targets

By the end of this project, you'll understand:

- How WebSockets actually work (and fail) with real connection pooling and keepalive
- Building async Rust that doesn't leak memory or deadlock
- LLM integration beyond just "call an API"
- RAG pipelines from scratch — chunking, embedding, retrieval, augmentation
- Vector databases and what cosine similarity actually means
- Query planning and execution in a real system
- gRPC and protobuf contracts between languages
- Building for multi-tenancy and security at the systems level

---

## Architecture

- **[Layer diagram](./docs/ARCHITECTURE.md)** — what sits where
- **[Query execution flow](./docs/QUERY_FLOW.md)** — what happens when you type something
- **[Onboarding flow](./docs/ONBOARDING_FLOW.md)** — how a new user gets set up

---

## The team

Just you for now. Optionally: a frontend person if this gets momentum, a data engineer to add more connectors.

---

## Why build this?

Because natural language interfaces to data are the future, and every tool out there either:

1. Hides the implementation behind abstractions (LangChain, etc.) so you don't learn anything
2. Only works with one data source
3. Isn't built to actually scale

Datalog is built differently. Production-grade from sprint 1. Extensible by design. Built for learning.

---

## What's next?

- [ ] Sprints 1–2: Mono-repo scaffold + CI
- [ ] Sprints 3–4: WebSocket real-time streaming
- [ ] Sprints 5–6: LLM intent parser + clarification loop
- [ ] Sprints 7–9: Connectors (SQL, S3, BigQuery, HTTP)
- [ ] Sprints 10–11: Chart rendering & result transformation
- [ ] Sprint 12: Auth, audit, observability
- [ ] Sprints 13–14: RAG pipeline + semantic retrieval
- [ ] 🎉 Beta launch to 100 users

---

## Contributing

Not open yet — building in public, but heads-down on the learning curve right now.

If you're interested in what this becomes, watch the repo. 👀

---

**Made with Rust, Go, React, and way too much coffee.**
