use std::cell::RefCell;
use std::rc::Rc;

use egui::Ui;

pub fn path_widget(path: &mut String, ui: &mut Ui) {
    ui.text_edit_singleline(path);
}

#[derive(PartialEq)]
pub enum ListEditResponse {
    None,
    Remove(usize),
    Swap(usize, usize),
}



fn list_edit_buttons(ui: &mut Ui, item_id: usize, num_items: usize) -> ListEditResponse {
    let mut response = ListEditResponse::None;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.spacing_mut().button_padding.x = 0.0;
        if ui.add(egui::Button::new('❌').small()).clicked() {
            response = ListEditResponse::Remove(item_id);
        };
        if ui
            .add(
                egui::Button::new('⬇')
                    .small()
                    .enabled(item_id + 1 < num_items),
            )
            .clicked()
        {
            response = ListEditResponse::Swap(item_id, item_id + 1);
        };
        if ui
            .add(egui::Button::new('⬆').small().enabled(item_id > 0))
            .clicked()
        {
            response = ListEditResponse::Swap(item_id, item_id - 1);
        };
    });
    response
}


/// Allows editing of items within a list
pub fn list_edit<T: Clone, F>(ui: &mut Ui, item_list: &Vec<T>, mut draw_item_function: F, list_edit_id: &str) -> ListEditResponse
where
    F: FnMut(&mut Ui, usize, &T)
{
    let mut response = ListEditResponse::None;
    egui::Grid::new(list_edit_id)
        .num_columns(2)
        .show(ui, |ui| {
            for (item_id, node) in item_list.iter().enumerate() {
                
                let edit_button_response = list_edit_buttons(ui, item_id, item_list.len());
                if edit_button_response != ListEditResponse::None {
                    response = edit_button_response;
                }

                draw_item_function(ui, item_id, node);

                ui.end_row();
            }
        });

    //*item_list = reflistout.iter().map(|x| x.borrow().clone()).collect();
    response
}


/// Allows editing of items within a list
pub fn list_edit_mut<T: Clone, F>(ui: &mut Ui, item_list: &mut Vec<T>, mut draw_item_function: F, list_edit_id: &str)
where
    F: FnMut(&mut Ui, usize, &mut T)
{
    let reflist: Vec<Rc<RefCell<T>>> = item_list
        .iter_mut()
        .map(|x| Rc::new(RefCell::new(x.clone())))
        .collect();
    let mut reflistout = reflist.clone();

    egui::Grid::new(list_edit_id)
        .num_columns(2)
        .show(ui, |ui| {
            for (item_id, node) in reflist.iter().enumerate() {
                match list_edit_buttons(ui, item_id, item_list.len()) {
                    ListEditResponse::None => {},
                    ListEditResponse::Remove(item_id) => {reflistout.remove(item_id);},
                    ListEditResponse::Swap(item_id_1, item_id_2) => {reflistout.swap(item_id_1, item_id_2)},
                }

                draw_item_function(ui, item_id, &mut (node.borrow_mut()));

                ui.end_row();
            }
        });

    *item_list = reflistout.iter().map(|x| x.borrow().clone()).collect();
}
