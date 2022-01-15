use std::any::{Any, TypeId};
use std::sync::Arc;
use dddk_core::dddk::command::command::Command;
use dddk_core::dddk::command::command_handler::{CommandHandlerInBus, CommandHandler};
use dddk_core::dddk::event::event::Event;
use crate::domain::foo::FooRepository;

pub struct ACommand {}

impl Command for ACommand {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ACommandHandler {
    foo_repository: Arc<dyn FooRepository>,
}

impl ACommandHandler {
    pub fn new(foo_repository: Arc<dyn FooRepository>) -> ACommandHandler {
        ACommandHandler {
            foo_repository
        }
    }
}

impl CommandHandler<ACommand> for ACommandHandler {
    fn handle(&self, _command: &ACommand) -> Vec<Box<dyn Event>> {
        println!("Has Been Called");
        let foos = self.foo_repository.get_all_foo();
        for foo in foos {
            println!("there is a foo uuid: {}, title: {}", foo.get_id(), foo.get_title());
        }
        return Vec::new();
    }
}

impl CommandHandlerInBus for ACommandHandler {
    fn handle_from_bus(&self, command: &dyn Command) -> Vec<Box<dyn Event>> {
        return self.handle_generic_command(command);
    }

    fn get_associated_command_from_bus(&self) -> TypeId {
        return self.get_associated_command();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
