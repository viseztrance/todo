extern crate todo;

#[cfg(test)]
mod entry {
    use todo::todo::entry::Entry;
    use todo::todo::entry::State;

    #[test]
    fn creation_sets_status() {
        let entry = Entry::new(1, "[completed] my entry".to_string());
        assert_eq!(entry.status, State::COMPLETED);
    }

    #[test]
    fn creation_sets_pending_status_when_none_was_provided() {
        let entry = Entry::new(1, "my entry".to_string());
        assert_eq!(entry.status, State::PENDING);
    }

    #[test]
    fn creation_sets_content() {
        let entry = Entry::new(1, "my entry".to_string());
        assert_eq!(entry.content, "my entry");
    }

    #[test]
    fn creation_sets_content_from_context() {
        let entry = Entry::new(1, "[completed] my entry in context #666".to_string());
        assert_eq!(entry.content, "my entry in context");
    }

    #[test]
    fn creation_sets_color() {
        let entry = Entry::new(1, "[completed] my entry in context #666aaa".to_string());
        assert_eq!(entry.color.unwrap(), "666aaa");
    }

    #[test]
    fn creation_sets_no_color_when_none_was_provided() {
        let entry = Entry::new(1, "[completed] my entry in context".to_string());
        assert!(entry.color.is_none());
    }

    #[test]
    fn updating_sets_new_content() {
        let mut entry = Entry::new(1, "my initial content".to_string());
        entry.update("my new content".to_string());
        assert_eq!(entry.content, "my new content".to_string());
    }

    #[test]
    fn serializes_data() {
        let entry = Entry {
            id: 1,
            content: "You have to catch 10 leaves".to_string(),
            status: State::PENDING,
            color: Some("9a9a9a".to_string())
        };
        assert_eq!(entry.to_data(), "[pending] You have to catch 10 leaves #9a9a9a".to_string());
    }
}
