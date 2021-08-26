use std::cell::RefCell;
use std::rc::Rc;

use egui::Ui;

pub fn path_widget(path: &mut String, ui: &mut Ui) {
    ui.text_edit_singleline(path);
}

/// Displays buttons for moving items within a list
pub fn list_edit_buttons<T>(ui: &mut Ui, item_list: &mut Vec<T>, item_id: usize) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.spacing_mut().button_padding.x = 0.0;
        if ui.add(egui::Button::new('❌').small()).clicked() {
            item_list.remove(item_id);
        };
        if ui
            .add(
                egui::Button::new('⬇')
                    .small()
                    .enabled(item_id + 1 < item_list.len()),
            )
            .clicked()
        {
            item_list.swap(item_id, item_id + 1);
        };
        if ui
            .add(egui::Button::new('⬆').small().enabled(item_id > 0))
            .clicked()
        {
            item_list.swap(item_id, item_id - 1);
        };
    });
}

/// Allows editing of items within a list
pub fn list_edit<T: Clone, F>(ui: &mut Ui, item_list: &mut Vec<T>, mut draw_item_function: F, list_edit_id: &str)
where
    F: FnMut(&mut Ui, usize, &mut T),
{
    let reflist: Vec<Rc<RefCell<T>>> = item_list
        .iter_mut()
        .map(|x| Rc::new(RefCell::new(x.clone())))
        .collect();
    let mut reflistout = reflist.clone();

    egui::Grid::new(list_edit_id)
        .num_columns(2)
        .show(ui, |ui| {
            for (node_id, node) in reflist.iter().enumerate() {
                list_edit_buttons(ui, &mut reflistout, node_id);

                draw_item_function(ui, node_id, &mut (node.borrow_mut()));

                ui.end_row();
            }
        });

    *item_list = reflistout.iter().map(|x| x.borrow().clone()).collect();
}
