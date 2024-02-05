#[cfg(test)]
mod tests {
    use forms::form;
    use futures_signals::signal::Mutable;

    pub struct Field<T> {
        pub label: String,
        pub value: Mutable<T>,
        pub error: Mutable<Option<String>>,
        pub regex: Option<String>,
        pub required: bool,
    }

    #[test]
    fn it_works() {
        let data = form!(
            .field(id: "first_name", label: "First Name", pattern: ".{3,}", required: true, type: String)
        );
    }
}
