use core::panic;

struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    // TODO: implement the `new` function.
    //  The following requirements should be met:
    //   - Only `To-Do`, `In Progress`, and `Done` statuses are allowed.
    //   - The `title` and `description` fields should not be empty.
    //   - the `title` should be at most 50 bytes long.
    //   - the `description` should be at most 500 bytes long.
    //  The method should panic if any of the requirements are not met.
    //  You can find the needed panic messages in the tests.
    //
    // You'll have to use what you learned in the previous exercises,
    // as well as some `String` methods. Use the documentation of Rust's standard library
    // to find the most appropriate options -> https://doc.rust-lang.org/std/string/struct.String.html
    fn new(title: String, description: String, status: String) -> Self {
        match Self::validate_title(&title) {
            Ok(_) => (),
            Err(msg) => panic!("{}", msg.to_string()),
        }
        match Self::validate_status(&status) {
            Ok(_) => (),
            Err(msg) => panic!("{}", msg.to_string()),
        }
        match Self::validate_description(&description) {
            Ok(_) => (),
            Err(msg) => panic!("{}", msg.to_string()),
        }
        Self {
            title,
            description,
            status,
        }
    }

    fn validate_title(title: &String) -> Result<(), String> {
        if title.is_empty() {
            return Err(String::from("Title cannot be empty"));
        }
        if title.len() > 50 {
            return Err(String::from("Title cannot be longer than 50 bytes"));
        }
        Ok(())
    }

    fn validate_status(status: &String) -> Result<(), String> {
        if status != "To-Do" && status != "In Progress" && status != "Done" {
            return Err(String::from(
                "Only `To-Do`, `In Progress`, and `Done` statuses are allowed",
            ));
        }
        Ok(())
    }

    fn validate_description(description: &String) -> Result<(), String> {
        if description.is_empty() {
            return Err(String::from("Description cannot be empty"));
        }
        if description.len() > 500 {
            return Err(String::from("Description cannot be longer than 500 bytes"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }
}
