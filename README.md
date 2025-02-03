# Task Management Library

This library provides a set of Rust structs and enums to manage tasks within projects, along with error handling and data serialization/deserialization capabilities. Below is a brief overview of the main features:

## Core Concepts

1. **Task Structure**: Represents a task with attributes like title, notes, project, status, etc.
2. **Projects**: Manages collections of related tasks.
3. **Relations**: Links tasks within a project for dependencies or related work.
4. **Data Handling**: Implements JSON serialization/deserialization to persist task data.

## Key Components

### Error Types

- `PJError`: An enum that defines possible error states such as:
  - `SomeError`: General errors during operation. **Often never return.**
  - `NotFoundKey/Value`: Indicates missing fields.
  - `FailedOperation`: Errors in task manipulation (archive, delete).

### Task Operations

- **Add Task**: Creates and adds a new task to the system with optional project linking.
- **Archive Task**: Temporarily marks a task as archived without deletion.
- **Delete Task**: Permanently removes a task from storage.

### Status Management

- **Task Status**: Manages the lifecycle of tasks using states like InProgress, NotStarted, etc.

## Example Usage

write a Example Usage and test about sometime.

```rust

```
