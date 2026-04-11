use crate::editor::commands::Command;
use std::collections::VecDeque;

const MAX_UNDO_STACK_SIZE: usize = 100;

#[derive(Debug, Clone)]
pub struct UndoState {
    undo_stack: VecDeque<Command>,
    redo_stack: VecDeque<Command>,
}

impl UndoState {
    pub fn new() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn push(&mut self, command: Command) {
        if self.undo_stack.len() >= MAX_UNDO_STACK_SIZE {
            self.undo_stack.pop_front();
        }
        self.undo_stack.push_back(command);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> Option<Command> {
        self.undo_stack.pop_back().inspect(|cmd| {
            self.redo_stack.push_back(cmd.clone());
        })
    }

    pub fn redo(&mut self) -> Option<Command> {
        self.redo_stack.pop_back().inspect(|cmd| {
            self.undo_stack.push_back(cmd.clone());
        })
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    pub fn undo_stack_size(&self) -> usize {
        self.undo_stack.len()
    }

    pub fn redo_stack_size(&self) -> usize {
        self.redo_stack.len()
    }
}

impl Default for UndoState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct UndoManager {
    state: UndoState,
    batch_level: usize,
}

impl UndoManager {
    pub fn new() -> Self {
        Self {
            state: UndoState::new(),
            batch_level: 0,
        }
    }

    pub fn execute(&mut self, command: Command) {
        if self.batch_level == 0 {
            self.state.push(command);
        }
    }

    pub fn execute_batch<F>(&mut self, f: F) -> Command
    where
        F: FnOnce() -> Command,
    {
        self.batch_level += 1;
        let result = f();
        self.batch_level -= 1;
        self.state.push(result.clone());
        result
    }

    pub fn undo(&mut self) -> Option<Command> {
        self.state.undo()
    }

    pub fn redo(&mut self) -> Option<Command> {
        self.state.redo()
    }

    pub fn can_undo(&self) -> bool {
        self.state.can_undo()
    }

    pub fn can_redo(&self) -> bool {
        self.state.can_redo()
    }
}

impl Default for UndoManager {
    fn default() -> Self {
        Self::new()
    }
}
