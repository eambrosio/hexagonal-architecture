# Hexagonal Architecture

The purpose of this project is ti simulate a kind of hexagonal architecture implemented in Rust 🦀.

The goal is to make the core, or domain, completelely isolated from the dependencies. All rules and entities will be implemented with no dependencies to databases, frameworks, libraries, ... The hexagonal architecture is supposed to separate the business logic from the implementation details.

A few advantages about this architecture:

- you can change the domain without changing the dependencies
- you can change the dependencies without changing the domain
- you can easily test the domain
- you can think about which technical dependencies to use when the need comes, and not at the very start of your implementation
