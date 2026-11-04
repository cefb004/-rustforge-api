# RustForge API

**A resilient, high-throughput REST API built with Rust, Tokio, Axum, and PostgreSQL.**

RustForge API is a backend engineering project focused on designing and implementing reliable, high-performance systems capable of handling concurrent workloads, protecting downstream resources, maintaining data consistency, and recovering gracefully from failures.

The project starts as a modular monolith, with an architecture designed to evolve as new scalability and reliability requirements emerge.

Rather than focusing only on API functionality, RustForge explores the engineering challenges behind production-grade backend systems, including concurrency management, database optimization, fault tolerance, asynchronous processing, observability, and performance testing.

> **Project Status:** Planning / Initial Development.
>
> This project is under active development. Performance targets and architectural decisions will be validated through implementation, automated testing, and reproducible load experiments.

---

# Table of Contents

- [Project Overview](#project-overview)
- [Project Goals](#project-goals)
- [System Architecture](#system-architecture)
- [Technology Stack](#technology-stack)
- [Functional Requirements](#functional-requirements)
- [Non-Functional Requirements](#non-functional-requirements)
- [API Design](#api-design)
- [Database Architecture](#database-architecture)
- [Concurrency Management](#concurrency-management)
- [Rate Limiting and Backpressure](#rate-limiting-and-backpressure)
- [Fault Tolerance and Resilience](#fault-tolerance-and-resilience)
- [Idempotency](#idempotency)
- [Transactional Outbox Pattern](#transactional-outbox-pattern)
- [Background Workers](#background-workers)
- [Database Performance Optimization](#database-performance-optimization)
- [Observability](#observability)
- [Testing Strategy](#testing-strategy)
- [Performance and Load Testing](#performance-and-load-testing)
- [Security Considerations](#security-considerations)
- [Deployment Strategy](#deployment-strategy)
- [Project Structure](#project-structure)
- [Development Roadmap](#development-roadmap)
- [Engineering Principles](#engineering-principles)
- [Learning Objectives](#learning-objectives)
- [License](#license)

---

# Project Overview

RustForge API is designed to explore how to build a backend system that remains responsive, predictable, and reliable under significant workloads and unexpected failures.

The application will expose REST endpoints for order processing while implementing mechanisms to control resource consumption and protect critical infrastructure components.

The primary challenge is not simply processing as many requests as possible, but maintaining system stability and data integrity when resources become constrained.

The architecture will explore:

- Asynchronous request processing.
- Controlled concurrency.
- Database connection pooling.
- Request rate limiting.
- Backpressure mechanisms.
- Fault isolation.
- Timeout and retry strategies.
- Circuit breaker patterns.
- Transactional consistency.
- Idempotent operations.
- Asynchronous background processing.
- Structured logging and distributed tracing.
- Performance monitoring and load testing.

The project will be developed incrementally, with each architectural component introduced, tested, and documented.

# Project Goals

The main objectives are:

1. Build a high-performance REST API using Rust and Tokio.
2. Handle concurrent requests without overwhelming application resources or the database.
3. Implement resource protection through concurrency limits, connection pools, and backpressure.
4. Maintain data consistency during concurrent operations and failure scenarios.
5. Prevent duplicate operations through idempotency mechanisms.
6. Implement reliable asynchronous event processing.
7. Protect the system against cascading failures.
8. Monitor application performance and identify bottlenecks.
9. Validate system behavior through automated and load testing.
10. Document architectural decisions and engineering trade-offs.

# System Architecture

The initial implementation will follow a **Modular Monolith Architecture**.

This approach keeps the system relatively simple while allowing clear separation of responsibilities.

The application will be internally organized into independent modules, making future service extraction possible if justified by scalability or operational requirements.

## High-Level Architecture

```text
                    API CLIENTS
                         |
                         |
                         v
                +------------------+
                |   Rust API       |
                |                  |
                |   Axum + Tokio   |
                +------------------+
                         |
          +--------------+--------------+
          |              |              |
          v              v              v
     Rate Limiting   Concurrency    Idempotency
                     Control
          |              |              |
          +--------------+--------------+
                         |
                         v
                +------------------+
                |   Service Layer  |
                |                  |
                |  Business Logic  |
                +------------------+
                         |
                         v
                +------------------+
                |   PostgreSQL     |
                |                  |
                | Orders           |
                | Idempotency Keys |
                | Outbox Events    |
                +------------------+
                         |
                         |
                         v
                +------------------+
                | Background Worker|
                |                  |
                | Event Processing |
                | Retry Mechanism  |
                +------------------+
                         |
                         v
                +------------------+
                | External Services|
                |                  |
                | Notifications    |
                +------------------+


                OBSERVABILITY
          Logs | Metrics | Tracing
```

## Architecture Components

### API Layer

Responsible for:

- Receiving HTTP requests.
- Request validation.
- Authentication and authorization.
- Rate limiting.
- Request timeout management.
- HTTP response generation.

### Service Layer

Responsible for:

- Business rules.
- Order creation.
- Transaction orchestration.
- Idempotency validation.
- Coordination between application components.

### Repository Layer

Responsible for:

- PostgreSQL queries.
- Database transactions.
- Connection pool usage.
- Data persistence.

### Background Processing

Responsible for:

- Processing Outbox events.
- Executing asynchronous tasks.
- Retry management.
- Handling temporary failures.
- Recovering abandoned processing tasks.

### Observability Layer

Responsible for:

- Structured logging.
- Application metrics.
- Request tracing.
- Performance monitoring.
- Error tracking.

# Technology Stack

| Component | Technology | Purpose |
|---|---|---|
| Programming Language | Rust | Memory safety and performance |
| Async Runtime | Tokio | Asynchronous execution |
| HTTP Framework | Axum | REST API |
| Database | PostgreSQL | Persistent data storage |
| Database Library | SQLx | Async SQL queries and transactions |
| Database Migrations | SQLx Migrate | Schema versioning |
| Serialization | Serde | JSON serialization |
| Local Infrastructure | Docker Compose | Development environment |
| Load Testing | k6 | Performance testing |
| Logging and Tracing | tracing | Structured application telemetry |
| CI/CD | GitHub Actions | Automated validation and testing |

Additional infrastructure components may be introduced as the architecture evolves.

# Functional Requirements

The system will initially provide the following capabilities.

## 1. Order Management

- Create orders.
- Retrieve orders by ID.
- List orders with pagination.
- Retrieve order processing status.
- Validate incoming requests.

## 2. Idempotent Requests

- Accept idempotency keys.
- Prevent duplicate order creation.
- Return consistent results for repeated requests.
- Handle concurrent requests using the same idempotency key.

## 3. Background Processing

- Generate events when orders are created.
- Persist events transactionally.
- Process pending events asynchronously.
- Retry failed processing attempts.
- Recover events abandoned by interrupted workers.

## 4. Health Checks

Expose endpoints to monitor application health and dependency availability.

# Non-Functional Requirements

The system will be designed around the following quality attributes.

| Requirement | Description |
|---|---|
| Performance | Efficient request processing and database access |
| Scalability | Support multiple application instances |
| Reliability | Graceful handling of temporary failures |
| Consistency | Preserve transactional integrity |
| Availability | Avoid unnecessary service interruptions |
| Maintainability | Modular and testable codebase |
| Observability | Measure and investigate system behavior |
| Resource Management | Prevent uncontrolled resource consumption |

Specific performance targets will be established and validated through benchmarking.

# API Design

The API will follow REST principles and use JSON for request and response payloads.

## Initial Endpoints

| Method | Endpoint | Description |
|---|---|---|
| POST | `/api/v1/orders` | Create an order |
| GET | `/api/v1/orders/{id}` | Retrieve an order |
| GET | `/api/v1/orders` | List orders |
| GET | `/health` | Application health |
| GET | `/ready` | Readiness check |

## Example: Create Order

Request:

```http
POST /api/v1/orders
Content-Type: application/json
Idempotency-Key: 7f4d8b2a-1234-4567-8901-abcdef123456
```

```json
{
  "customer_id": "customer-123",
  "items": [
    {
      "product_id": "product-001",
      "quantity": 2,
      "unit_price": 49.90
    }
  ]
}
```

Example response:

```json
{
  "id": "order-123",
  "customer_id": "customer-123",
  "status": "PENDING",
  "total_amount": 99.80,
  "created_at": "2026-11-04T12:00:00Z"
}
```

The actual API contract, validation rules, and error responses will be documented as the implementation progresses.

# Database Architecture

PostgreSQL will be the primary persistence layer.

The database will be responsible for maintaining transactional consistency and enforcing critical business constraints.

## Initial Database Tables

### Orders

Stores order information.

| Column | Type | Description |
|---|---|---|
| id | UUID | Primary key |
| customer_id | UUID | Customer identifier |
| status | VARCHAR | Order status |
| total_amount | NUMERIC | Order total |
| created_at | TIMESTAMPTZ | Creation timestamp |
| updated_at | TIMESTAMPTZ | Last update timestamp |

### Order Items

Stores individual order items.

| Column | Type | Description |
|---|---|---|
| id | UUID | Primary key |
| order_id | UUID | Associated order |
| product_id | UUID | Product identifier |
| quantity | INTEGER | Item quantity |
| unit_price | NUMERIC | Unit price |

### Idempotency Keys

Stores information about previously processed requests.

| Column | Type | Description |
|---|---|---|
| id | UUID | Primary key |
| idempotency_key | VARCHAR | Unique request key |
| request_hash | VARCHAR | Request fingerprint |
| response_status | INTEGER | Original HTTP status |
| response_body | JSONB | Stored response |
| created_at | TIMESTAMPTZ | Creation timestamp |

A unique constraint will protect idempotency keys against concurrent duplicate requests.

### Outbox Events

Stores events that must be processed asynchronously.

| Column | Type | Description |
|---|---|---|
| id | UUID | Primary key |
| aggregate_id | UUID | Related entity |
| event_type | VARCHAR | Event type |
| payload | JSONB | Event data |
| status | VARCHAR | Processing status |
| attempts | INTEGER | Processing attempts |
| available_at | TIMESTAMPTZ | Next eligible processing time |
| locked_until | TIMESTAMPTZ | Processing lease expiration |
| created_at | TIMESTAMPTZ | Creation timestamp |

Indexes will be introduced according to actual query patterns and validated through execution plans.

# Concurrency Management

Rust and Tokio provide asynchronous execution capabilities, but asynchronous execution alone does not guarantee that downstream resources will remain healthy.

The application must explicitly control concurrency.

## Tokio Semaphore

A semaphore will limit the number of operations allowed to execute simultaneously.

Example conceptual configuration:

```rust
const MAX_CONCURRENT_TASKS: usize = 50;
```

The actual configuration will be environment-driven.

Potential applications include:

- Limiting expensive business operations.
- Protecting database-intensive workloads.
- Restricting concurrent external service calls.
- Preventing uncontrolled task creation.

## Database Connection Pool

SQLx will manage PostgreSQL connections through a bounded connection pool.

Example initial configuration:

```text
MAX_CONNECTIONS=50
MIN_CONNECTIONS=5
ACQUIRE_TIMEOUT_SECONDS=3
```

These values are experimental starting points, not production recommendations.

The connection pool must be sized according to PostgreSQL capacity and the total number of application instances.

For example, three application instances configured with 50 connections each may create demand for up to 150 database connections.

The application must avoid creating more concurrent database work than the database can efficiently process.

# Rate Limiting and Backpressure

Rate limiting and concurrency control solve related but different problems.

## Rate Limiting

Controls how many requests a client or group of clients can submit within a defined time interval.

Possible algorithms:

- Token Bucket.
- Sliding Window.
- Fixed Window.

Example experimental configuration:

```text
MAX_REQUESTS_PER_SECOND=200
```

This value represents a possible initial per-instance limit, not a validated system capacity.

For multiple application instances, a distributed rate limiter may be necessary if a global limit must be enforced.

## Backpressure

Backpressure prevents the application from accepting unlimited work when internal resources become saturated.

The system will implement:

- Bounded concurrency.
- Database acquisition timeouts.
- Request deadlines.
- Bounded background processing.
- Controlled rejection of excessive workloads.

When the system cannot safely process additional work, it should reject requests predictably rather than accumulate unlimited tasks.

Possible HTTP responses include:

- `429 Too Many Requests`
- `503 Service Unavailable`

# Fault Tolerance and Resilience

The system will implement several resilience patterns.

## Timeout

Every operation involving external dependencies should have an appropriate timeout.

Timeouts prevent requests from waiting indefinitely for unavailable or overloaded services.

## Retry

Temporary failures may be retried when the operation is safe to repeat.

The retry strategy will include:

- Exponential backoff.
- Randomized jitter.
- Maximum retry attempts.
- Maximum total retry duration.
- Error classification.

Retries must not amplify an existing outage.

## Circuit Breaker

The circuit breaker will protect the application when a downstream dependency becomes unavailable.

It will use three conceptual states:

| State | Behavior |
|---|---|
| Closed | Requests are allowed |
| Open | Requests are rejected temporarily |
| Half-Open | Limited requests test recovery |

The circuit breaker will prevent repeated calls to a dependency that is consistently failing.

## Failure Isolation

Failures in background processing should not unnecessarily interrupt synchronous API operations.

The application will distinguish between:

- Request processing failures.
- Database availability failures.
- External service failures.
- Background worker failures.

# Idempotency

Idempotency is essential for APIs that may receive duplicate requests due to network failures, timeouts, or client retries.

The system will support idempotent order creation through an `Idempotency-Key` request header.

## Expected Behavior

1. The client sends a request with an idempotency key.
2. The API checks whether the key has already been processed.
3. If the key is new, the operation is executed.
4. The result and key are stored transactionally.
5. If the same request is repeated, the original result is returned.
6. If the same key is reused with a different payload, the request is rejected.

A database unique constraint will protect against race conditions involving concurrent requests.

The implementation must also define how requests that are still being processed are handled.

# Transactional Outbox Pattern

The Transactional Outbox Pattern will be used to maintain consistency between business transactions and asynchronous event processing.

A common distributed systems problem occurs when a database transaction succeeds but publishing the corresponding event fails.

The Outbox Pattern addresses this problem by storing the business operation and its event in the same database transaction.

## Processing Flow

```text
             API REQUEST
                  |
                  v
           BEGIN TRANSACTION
                  |
          +-------+-------+
          |               |
          v               v
      Create Order    Create Outbox
          |               |
          +-------+-------+
                  |
                  v
             COMMIT
                  |
                  v
          Background Worker
                  |
                  v
           Claim Outbox Event
                  |
                  v
           Process Event
                  |
                  v
        Mark Event as Processed
```

If the transaction fails, both the order and its Outbox event are rolled back.

If the worker fails after claiming an event, the event can become available again after its processing lease expires.

The system will use at-least-once event delivery and idempotent consumers rather than claiming exactly-once delivery.

# Background Workers

Background workers will process pending Outbox events independently of HTTP request handling.

The worker architecture will explore:

- Concurrent event processing.
- Bounded worker concurrency.
- PostgreSQL row locking.
- `SELECT FOR UPDATE SKIP LOCKED`.
- Lease expiration.
- Retry scheduling.
- Failed event handling.
- Idempotent event processing.

## Worker Recovery

If a worker crashes while processing an event, the event must not remain permanently locked.

A lease expiration mechanism will allow another worker to claim the event after the configured processing lease expires.

This enables recovery from interrupted processing without requiring manual database intervention for every failure.

# Database Performance Optimization

PostgreSQL performance is a central part of this project.

The goal is to understand how application concurrency and database resource management interact.

The optimization process will include:

## Query Analysis

Use:

```sql
EXPLAIN ANALYZE
```

to investigate query execution plans and identify expensive operations.

## Indexing

Evaluate indexes according to real query patterns.

Potential examples:

- Primary key indexes.
- Foreign key access patterns.
- Customer order lookup.
- Order status filtering.
- Pending Outbox event retrieval.
- Unique idempotency key lookup.

Indexes will not be added indiscriminately because they also increase storage requirements and write overhead.

## Connection Pool Optimization

Experiment with different pool sizes and observe:

- Query latency.
- Connection acquisition time.
- Database CPU usage.
- Memory consumption.
- Throughput.
- Lock contention.

Increasing the connection pool size does not necessarily improve performance.

When PostgreSQL is saturated, additional connections can increase contention and make the system slower.

# Observability

A resilient backend requires visibility into its internal behavior.

The project will implement structured logging, metrics, and distributed tracing.

## Structured Logging

Use the Rust `tracing` ecosystem to record relevant application events.

Examples include:

- Incoming requests.
- Request completion.
- Database errors.
- Retry attempts.
- Circuit breaker state changes.
- Worker failures.
- Outbox processing.
- Idempotency conflicts.

Sensitive information must not be included in logs.

## Metrics

The system will monitor:

| Metric | Purpose |
|---|---|
| Requests per second | API throughput |
| HTTP error rate | Application reliability |
| p50 latency | Typical response time |
| p95 latency | High-percentile response time |
| p99 latency | Tail latency |
| Active requests | Application concurrency |
| Database connections | Pool utilization |
| Connection acquisition time | Database resource pressure |
| Retry count | Dependency instability |
| Circuit breaker state | Failure protection |
| Outbox backlog | Background processing health |
| Worker processing time | Asynchronous performance |

## Distributed Tracing

Tracing will help identify where requests spend time across application components and external dependencies.

The objective is to distinguish between application processing time, database latency, connection acquisition delays, and external service delays.

# Testing Strategy

Testing will be introduced throughout development.

## Unit Tests

Validate individual components and business rules.

Examples:

- Request validation.
- Order calculations.
- Idempotency behavior.
- Retry policies.
- Circuit breaker transitions.

## Integration Tests

Validate interactions between application components and PostgreSQL.

Examples:

- Order creation.
- Database transactions.
- Unique constraints.
- Concurrent idempotency requests.
- Outbox event persistence.
- Worker recovery.

## Failure Scenario Tests

Simulate unexpected conditions.

Examples:

- PostgreSQL unavailable.
- Database connection timeout.
- External service unavailable.
- Worker interruption.
- Duplicate requests.
- Concurrent event processing.

## Concurrency Tests

Validate system behavior under simultaneous operations.

The objective is to detect race conditions, resource exhaustion, and consistency problems.

# Performance and Load Testing

Performance will be measured rather than assumed.

The project will use k6 to simulate concurrent API workloads.

## Initial Experiments

| Experiment | Objective |
|---|---|
| Gradually increase virtual users | Understand throughput and latency |
| Increase request arrival rate | Identify saturation points |
| Reduce PostgreSQL pool size | Measure connection contention |
| Increase application concurrency | Observe resource consumption |
| Introduce slow database queries | Evaluate timeout behavior |
| Interrupt PostgreSQL | Validate failure handling |
| Simulate external service failures | Validate retries and circuit breakers |
| Execute duplicate requests | Validate idempotency |
| Run multiple API instances | Evaluate horizontal scaling |

## Performance Metrics

The experiments will record:

- Requests per second.
- Average response time.
- p50, p95, and p99 latency.
- HTTP error rate.
- Database connection utilization.
- CPU and memory consumption.
- Worker processing throughput.

Performance reports will include the test environment, configuration, workload profile, and observed limitations.

No throughput or latency guarantee will be claimed without reproducible measurements.

# Security Considerations

Security will be addressed as part of the API design.

Initial considerations include:

- Input validation.
- Parameterized SQL queries.
- Secure configuration management.
- No hardcoded credentials.
- Request size limits.
- Authentication and authorization.
- Rate limiting.
- Sensitive data protection.
- Safe error responses.
- Dependency auditing.

Authentication and authorization mechanisms will be introduced according to the project's evolving requirements.

# Deployment Strategy

The initial development environment will use Docker Compose.

Expected local services:

```text
RustForge API
PostgreSQL
Background Worker
```

The application will be configurable through environment variables.

The deployment architecture will initially prioritize simplicity and low operational overhead.

Future possibilities include:

- Multiple API instances.
- Reverse proxy or load balancer.
- Centralized metrics collection.
- Centralized logging.
- Distributed rate limiting.
- Cloud deployment.

Kubernetes and more complex orchestration will only be considered if there is a clear technical justification.

# Project Structure

The intended project structure is:

```text
rustforge-api/
|
+-- src/
|   |
|   +-- main.rs
|   +-- lib.rs
|   |
|   +-- config/
|   |   +-- mod.rs
|   |
|   +-- api/
|   |   +-- mod.rs
|   |   +-- routes.rs
|   |   +-- middleware/
|   |
|   +-- domain/
|   |   +-- mod.rs
|   |   +-- orders/
|   |
|   +-- services/
|   |   +-- mod.rs
|   |
|   +-- repositories/
|   |   +-- mod.rs
|   |
|   +-- database/
|   |   +-- mod.rs
|   |
|   +-- workers/
|   |   +-- mod.rs
|   |
|   +-- resilience/
|   |   +-- mod.rs
|   |
|   +-- observability/
|       +-- mod.rs
|
+-- migrations/
|
+-- tests/
|
+-- load-tests/
|
+-- docs/
|   +-- architecture/
|   +-- adr/
|
+-- docker-compose.yml
+-- Dockerfile
+-- .env.example
+-- .gitignore
+-- Cargo.toml
+-- README.md
```

The structure may evolve as implementation requirements become clearer.

# Development Roadmap

The project will be developed incrementally.

## Phase 1 — Foundation

- Initialize Rust project.
- Configure Axum and Tokio.
- Implement application configuration.
- Establish modular architecture.
- Implement health endpoints.
- Define standardized error handling.
- Introduce unit tests.

## Phase 2 — PostgreSQL Integration

- Configure PostgreSQL.
- Integrate SQLx.
- Create database migrations.
- Implement order persistence.
- Implement database transactions.
- Configure connection pooling.
- Add integration tests.

## Phase 3 — Concurrency and Resource Protection

Introduce Tokio Semaphore.

Configure bounded concurrency.

Implement rate limiting.

Add request timeouts.

Implement backpressure.

Test database pool saturation.

## Phase 4 — Fault Tolerance

Implement retry policies.

Introduce exponential backoff and jitter.

Implement circuit breaker.

Define dependency timeout policies.

Test failure and recovery scenarios.

## Phase 5 — Idempotency and Event Processing

Implement idempotency keys.

Add database uniqueness constraints.

Implement Transactional Outbox.

Develop background workers.

Implement SELECT FOR UPDATE SKIP LOCKED.

Introduce lease expiration.

Test worker recovery and duplicate event processing.

## Phase 6 — Observability and Performance

Implement structured logging.

Add application metrics.

Introduce request tracing.

Build load testing scenarios with k6.

Analyze PostgreSQL query performance.

Measure throughput and tail latency.

Document bottlenecks and optimization results.

## Phase 7 — Production Readiness

Finalize Docker configuration.

Implement GitHub Actions workflows.

Expand automated tests.

Review security considerations.

Document deployment procedures.

Publish architecture diagrams.

Document performance benchmarks.

# Engineering Principles

RustForge follows several engineering principles:

### Correctness before optimization

Data consistency and predictable behavior are more important than premature performance optimization.

### Measure before optimizing

Performance decisions must be supported by measurements and reproducible experiments.

### Bounded resource consumption

Concurrency, queues, connection pools, and retries must have explicit limits.

### Failure is expected

External dependencies can become unavailable. The system must handle these conditions gracefully.

### Explicit architectural decisions

Technical trade-offs and design alternatives should be documented.

### Incremental complexity

Start with a simple architecture and introduce additional infrastructure only when justified.

# Learning Objectives

RustForge is also an opportunity to apply and consolidate advanced Rust backend engineering concepts.

The project will explore:

- Rust ownership and borrowing.
- Error handling with Result and Option.
- Async programming with Tokio.
- Task scheduling and concurrency control.
- PostgreSQL transactions and isolation.
- SQL query optimization.
- Connection pool management.
- Rate limiting and backpressure.
- Distributed systems reliability patterns.
- Idempotency and data consistency.
- Asynchronous event processing.
- Observability and performance engineering.
- Automated testing and CI/CD.

The ultimate objective is to demonstrate practical knowledge of backend architecture, system design, and reliable API development using Rust.

# License

The project license will be defined before the first public release.


