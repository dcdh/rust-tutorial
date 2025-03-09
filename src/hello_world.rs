use std::string::ToString;

pub trait SayHelloWorld {
    fn say(&self) -> String;
}

pub struct FrenchSayHelloWorld;

impl SayHelloWorld for FrenchSayHelloWorld {
    fn say(&self) -> String {
        "Bonjour le monde!".to_string()
    }
}

pub struct HelloWorldService<'a> {
    say_hello_world: &'a dyn SayHelloWorld,
}

impl<'a> HelloWorldService<'a> {
    pub fn new(say_hello_world: &'a dyn SayHelloWorld) -> Self {
        Self {say_hello_world}
    }

    pub fn say_hello_world(&self) -> String {
        self.say_hello_world.say()
    }
}

#[cfg(test)]
mod tests {
    use mockall::mock;
    use super::*;

    struct StubSayHelloWorld;

    impl SayHelloWorld for StubSayHelloWorld {
        fn say(&self) -> String {
            "stubbed".to_string()
        }
    }

    #[test]
    fn should_say_hello_world_in_french() {
        let french_say_hello_world = FrenchSayHelloWorld;
        assert_eq!(french_say_hello_world.say(), "Bonjour le monde!");
    }

    #[test]
    fn should_stub_say_hello_world_call_say_stubbed() {
        let stub_say_hello_world = StubSayHelloWorld;
        assert_eq!(stub_say_hello_world.say(), "stubbed");
    }

    mock! {
        pub SayHelloWorldMock {}

        impl SayHelloWorld for SayHelloWorldMock {
            fn say(&self) -> String;
        }
    }

    #[test]
    fn should_say_hello_world_call_mocked_say() {
        // Given
        let mut mock_say_hello_world = MockSayHelloWorldMock::new();

        mock_say_hello_world.expect_say()
            .times(1)
            .return_const("hello from mock !".to_string());

        let hello_world_service = HelloWorldService::new(&mock_say_hello_world);

        // when
        let said = hello_world_service.say_hello_world();

        // then
        assert_eq!(said, "hello from mock !");
        mock_say_hello_world.checkpoint(); // Vérifie que toutes les attentes ont été respectées
    }
}
