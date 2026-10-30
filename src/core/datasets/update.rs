use iced::Task;

use crate::core::datasets::models::{CellCoord, Dataset, Message, State};

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::DatasetSelected(dataset) => {
            save_current_edit(state);
            state.dataset = Some(dataset);
            state.selected_row = match dataset {
                Dataset::LocaisDeEstoque => Some(1),
                Dataset::Usuarios => Some(0),
            };
            state.editing_cell = None;
            state.editing_value.clear();
            state.search_query.clear();
            Task::none()
        }
        Message::SelectRow(row_idx) => {
            save_current_edit(state);
            state.selected_row = Some(row_idx);
            Task::none()
        }
        Message::StartEditingCell(coord) => {
            save_current_edit(state);
            let current_val = match coord {
                CellCoord::Header(col) => state
                    .current_table()
                    .and_then(|t| t.headers.get(col))
                    .cloned()
                    .unwrap_or_default(),
                CellCoord::Cell { row, col } => state
                    .current_table()
                    .and_then(|t| t.rows.get(row))
                    .and_then(|r| r.get(col))
                    .cloned()
                    .unwrap_or_default(),
            };
            state.editing_cell = Some(coord);
            state.editing_value = current_val;
            Task::none()
        }
        Message::EditingValueChanged(val) => {
            state.editing_value = val;
            Task::none()
        }
        Message::SaveCell => {
            save_current_edit(state);
            Task::none()
        }
        Message::CancelEditing => {
            state.editing_cell = None;
            state.editing_value.clear();
            Task::none()
        }
        Message::AddColumn => {
            save_current_edit(state);
            state.column_counter += 1;
            let col_name = format!("COLUNA {}", state.column_counter);
            if let Some(table) = state.current_table_mut() {
                table.add_column(col_name);
            }
            iced::widget::operation::scrollable::snap_to(
                crate::core::datasets::models::TABLE_SCROLLABLE_ID,
                iced::widget::scrollable::RelativeOffset {
                    x: Some(1.0),
                    y: None,
                },
                iced::widget::operation::Animation::Smooth,
            )
        }
        Message::AddRow => {
            save_current_edit(state);
            if let Some(table) = state.current_table_mut() {
                table.add_row();
                state.selected_row = Some(table.rows.len() - 1);
            }
            iced::widget::operation::scrollable::snap_to(
                crate::core::datasets::models::TABLE_SCROLLABLE_ID,
                iced::widget::scrollable::RelativeOffset {
                    x: None,
                    y: Some(1.0),
                },
                iced::widget::operation::Animation::Smooth,
            )
        }
        Message::SearchChanged(query) => {
            state.search_query = query;
            Task::none()
        }
        Message::ImportModal(msg) => {
            crate::core::widgets::import_modal::update(&mut state.import_modal, msg)
                .map(Message::ImportModal)
        }
        Message::Nothing => Task::none(),
    }
}

fn save_current_edit(state: &mut State) {
    if let Some(coord) = state.editing_cell.take() {
        let val = state.editing_value.clone();
        if let Some(table) = state.current_table_mut() {
            match coord {
                CellCoord::Header(col) => table.set_header(col, val),
                CellCoord::Cell { row, col } => table.set_cell(row, col, val),
            }
        }
    }
    state.editing_value.clear();
}
