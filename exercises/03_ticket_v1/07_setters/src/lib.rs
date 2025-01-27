// TODO: Add &mut-setters to the `Ticket` struct for each of its fields.
//   Make sure to enforce the same validation rules you have in `Ticket::new`!
//   Even better, extract that logic and reuse it in both places. You can use
//   private functions or private static methods for that.

use core::panic;

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
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

        Ticket {
            title,
            description,
            status,
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    pub fn set_title(&mut self, title: String) {
        match Self::validate_title(&title) {
            Ok(_) => (),
            Err(msg) => panic!("{}", msg.to_string()),
        }
        self.title = title
    }

    pub fn set_description(&mut self, description: String) {
        match Self::validate_description(&description) {
            Ok(_) => (),
            Err(msg) => panic!("{}", msg.to_string()),
        }
        self.description = description
    }

    pub fn set_status(&mut self, status: String) {
        match Self::validate_status(&status) {
            Ok(_) => (),
            Err(msg) => panic!("{}", msg.to_string()),
        }
        self.status = status
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
    use super::Ticket;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    fn works() {
        let mut ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        ticket.set_title("A new title".into());
        ticket.set_description("A new description".into());
        ticket.set_status("Done".into());

        assert_eq!(ticket.title(), "A new title");
        assert_eq!(ticket.description(), "A new description");
        assert_eq!(ticket.status(), "Done");
    }

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_title("".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_description("".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_title(overly_long_title())
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_description(overly_long_description())
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_status("Funny".into());
    }
}
