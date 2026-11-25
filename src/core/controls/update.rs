use iced::Task;

use crate::core::controls::models::{Message, State};

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::PrinterSelected(printer) => {
            state.printer = Some(printer);
            state.print_status = None;
            Task::none()
        }
        Message::CopiesChanged(copies) => {
            state.copies = copies;
            state.print_status = None;
            Task::none()
        }
        Message::PrintPressed => {
            state.is_printing = true;
            state.print_status = Some("Processando impressão...".to_string());
            Task::none()
        }
        Message::PrintResult(res) => {
            state.is_printing = false;
            state.print_status_seq = state.print_status_seq.wrapping_add(1);
            match res {
                Ok(msg) => state.print_status = Some(format!("✓ {msg}")),
                Err(err) => state.print_status = Some(format!("✗ {err}")),
            }
            Task::none()
        }
        Message::AutoDismissPrintStatus(seq) => {
            if state.print_status_seq == seq {
                state.print_status = None;
            }
            Task::none()
        }
        Message::ClearPrintStatus => {
            state.print_status = None;
            Task::none()
        }
        Message::ToggleCollapse => {
            state.collapsed = !state.collapsed;
            Task::none()
        }
    }
}
