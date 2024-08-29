# Project Conventions for Rust Desktop App using iced

## 1. General Guidelines

- Follow Rust's official style guide and idioms.
- Use `rustfmt` to automatically format code.
- Use `clippy` for additional linting and to catch common mistakes.
- Aim for clear, readable, and maintainable code.

## 2. Naming Conventions

- Use snake_case for variables, functions, and file names.
- Use PascalCase for types, traits, and enums.
- Use SCREAMING_SNAKE_CASE for constants.
- Prefer descriptive names over abbreviations.

Example:
```rust
const MAX_WINDOW_SIZE: (u32, u32) = (1920, 1080);

struct UserProfile {
    username: String,
    email: String,
}

fn calculate_total_score(scores: &[i32]) -> i32 {
    // Implementation
}
```

## 3. Code Structure

- Organize code into modules logically.
- Keep functions small and focused on a single task.
- Use Rust's visibility rules (pub, pub(crate), etc.) appropriately.
- Separate GUI logic from business logic where possible.

Example folder structure:
```
src/
  main.rs
  app.rs
  ui/
    mod.rs
    components/
      mod.rs
      button.rs
      input.rs
  logic/
    mod.rs
    calculations.rs
    data_processing.rs
  models/
    mod.rs
    user.rs
    settings.rs
```

## 4. Error Handling

- Use `Result` and `Option` types for error handling.
- Create custom error types for complex error scenarios.
- Propagate errors up the call stack when appropriate using the `?` operator.

Example:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
}

fn read_and_parse(filename: &str) -> Result<i32, AppError> {
    let content = std::fs::read_to_string(filename)?;
    let number = content.trim().parse()?;
    Ok(number)
}
```

## 5. Rust-specific Best Practices

- Prefer borrowing over ownership when possible.
- Use `impl Trait` for function arguments and return types where appropriate.
- Utilize Rust's powerful type system and pattern matching.
- Avoid unnecessary use of `unsafe` code.

## 6. iced-specific Conventions

- Use the `iced` prelude to import common types and traits.
- Follow the Model-View-Update (MVU) architecture pattern.
- Create reusable UI components as separate modules.
- Use `iced`'s built-in widgets when possible, create custom widgets only when necessary.

Example:
```rust
use iced::widget::{button, column, text, Container};
use iced::{Element, Sandbox, Settings};

struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter App")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement)
        ]
        .padding(20)
        .into()
    }
}
```

## 7. Documentation Standards

- Use Rust doc comments (`///` for public items, `//!` for module-level documentation).
- Document all public items (functions, structs, enums, traits).
- Include examples in documentation where appropriate.
- Use markdown formatting in doc comments.

Example:
```rust
/// Represents a user in the application.
///
/// # Fields
///
/// * `username` - The user's unique identifier
/// * `email` - The user's email address
///
/// # Examples
///
/// ```
/// let user = User::new("johndoe", "john@example.com");
/// assert_eq!(user.username, "johndoe");
/// ```
pub struct User {
    pub username: String,
    pub email: String,
}
```

## 8. Version Control Practices

- Use Git for version control.
- Write clear, concise commit messages.
- Use feature branches for new features or significant changes.
- Regularly merge or rebase with the main branch to stay up-to-date.

## 9. Testing Guidelines

- Write unit tests for all public functions and methods.
- Use Rust's built-in testing framework.
- Aim for high test coverage, especially for critical parts of the application.
- Write integration tests for important user flows.
- Use mock objects when testing components with external dependencies.

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(2, 3), 5);
        assert_eq!(add_numbers(-1, 1), 0);
    }
}
```

## 10. Usability Considerations

- Design intuitive and responsive user interfaces.
- Implement keyboard shortcuts for common actions.
- Ensure proper error messages and user feedback.
- Consider accessibility features (e.g., support for screen readers).
- Implement proper window management (resizing, minimizing, maximizing).
- Use appropriate UI scaling for different screen resolutions.