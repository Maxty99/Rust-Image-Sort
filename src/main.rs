#![windows_subsystem = "windows"] // https://gabdube.github.io/native-windows-gui/native-windows-docs/distribute.html

use std::cell::RefCell;
use std::env;

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use native_windows_derive as nwd;
use native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::Button;
use nwg::NativeUi;

enum ActionType {
    Move,
    Delete,
}

pub struct Action {
    from: String,
    to: Option<String>,
    action_type: ActionType,
}

#[derive(Default, NwgUi)]
pub struct App {
    filenames_buffer: RefCell<Vec<String>>,

    loaded_image: RefCell<Option<nwg::Bitmap>>,

    actions: RefCell<Vec<Action>>,

    open_folder_os_path: RefCell<PathBuf>,

    cat_one_os_path: RefCell<PathBuf>,

    cat_two_os_path: RefCell<PathBuf>,

    cat_three_os_path: RefCell<PathBuf>,

    #[nwg_control(flags: "MAIN_WINDOW|VISIBLE", title: "Image Sort", size: (1000,700), center: true)]
    #[nwg_events(OnMinMaxInfo: [App::set_min(SELF, EVT_DATA)], OnResize: [App::upate_img], OnWindowClose: [App::exit], OnKeyPress: [App::process_keypress(SELF, EVT_DATA)], OnInit: [App::update_button_status])]
    main_window: nwg::Window,

    #[nwg_layout(parent: main_window, spacing: 2, margin: [10,10,30,10])]
    grid: nwg::GridLayout,

    #[nwg_resource]
    decoder: nwg::ImageDecoder,

    #[nwg_resource(title: "Select a folder", action: nwg::FileDialogAction::OpenDirectory)]
    folder_select: nwg::FileDialog,

    #[nwg_control(parent: main_window)]
    #[nwg_layout_item(layout: grid, col: 0, row: 0, col_span: 3, row_span: 10)]
    img_frame_ui: nwg::ImageFrame,

    #[nwg_control(parent: main_window, text: "One", focus: false)]
    #[nwg_layout_item(layout: grid, col: 0, row: 10)]
    #[nwg_events( OnButtonClick: [App::move_file(SELF, CTRL)])]
    cat_one_btn: nwg::Button,

    #[nwg_control(parent: main_window, text: "Two", focus: false)]
    #[nwg_layout_item(layout: grid, col: 1, row: 10)]
    #[nwg_events( OnButtonClick: [App::move_file(SELF, CTRL)])]
    cat_two_btn: nwg::Button,

    #[nwg_control(parent: main_window, text: "Three", focus: false)]
    #[nwg_layout_item(layout: grid, col: 2, row: 10)]
    #[nwg_events( OnButtonClick: [App::move_file(SELF, CTRL)])]
    cat_three_btn: nwg::Button,

    #[nwg_control(parent: main_window, text: "Undo", focus: false)]
    #[nwg_layout_item(layout: grid, col: 0, row: 11)]
    #[nwg_events( OnButtonClick: [App::undo])]
    undo_btn: nwg::Button,

    #[nwg_control(parent: main_window, text: "Config", focus: false)]
    #[nwg_layout_item(layout: grid, col: 2, row: 11)]
    #[nwg_events( OnButtonClick: [App::open_config])]
    config_btn: nwg::Button,

    #[nwg_control(parent: main_window, text: "Delete", focus: false)]
    #[nwg_layout_item(layout: grid, col: 1, row: 11)]
    #[nwg_events( OnButtonClick: [App::delete_file])]
    delete_btn: nwg::Button,

    #[nwg_control(parent: main_window, text: "Open folder to load images")]
    status_bar: nwg::StatusBar,

    #[nwg_control(flags: "WINDOW", title: "Image Sort", center: true)]
    #[nwg_events(OnWindowClose: [App::close_config])]
    config_window: nwg::Window,

    #[nwg_layout(parent: config_window, spacing: 2, min_size: [500, 500])]
    config_grid: nwg::GridLayout,

    #[nwg_control(parent: config_window, text: "Pictures", size: (100,25), focus: false)]
    #[nwg_layout_item(layout: config_grid, col: 2, row: 0)]
    #[nwg_events( OnButtonClick: [App::open_pictures_folder])]
    open_pictures_btn: nwg::Button,

    #[nwg_control(parent: config_window,text: "", focus: false, size: (100,25), readonly: true)]
    #[nwg_layout_item(layout: config_grid, col: 0, row: 0, col_span: 2)]
    open_dir_text: nwg::TextInput,

    #[nwg_control(parent: config_window, text: "Category 1", size: (100,25), focus: false)]
    #[nwg_layout_item(layout: config_grid, col: 2, row: 1)]
    #[nwg_events( OnButtonClick: [App::open_cat_one_folder])]
    cat_one_choose_btn: nwg::Button,

    #[nwg_control(parent: config_window, text: "", focus: false, size: (100,25), readonly: true)]
    #[nwg_layout_item(layout: config_grid, col: 0, row: 1, col_span: 2)]
    cat_one_dir_text: nwg::TextInput,

    #[nwg_control(parent: config_window, text: "Category 2", size: (100,25), focus: false)]
    #[nwg_layout_item(layout: config_grid, col: 2, row: 2)]
    #[nwg_events( OnButtonClick: [App::open_cat_two_folder])]
    cat_two_choose_btn: nwg::Button,

    #[nwg_control(parent: config_window, text: "", focus: false, size: (100,25), readonly: true)]
    #[nwg_layout_item(layout: config_grid, col: 0, row: 2, col_span: 2)]
    cat_two_dir_text: nwg::TextInput,

    #[nwg_control(parent: config_window, text: "Category 3", size: (100,25), focus: false)]
    #[nwg_layout_item(layout: config_grid, col: 2, row: 3)]
    #[nwg_events( OnButtonClick: [App::open_cat_three_folder])]
    cat_three_choose_btn: nwg::Button,

    #[nwg_control(parent: config_window, text: "", focus: false, size: (100,25), readonly: true)]
    #[nwg_layout_item(layout: config_grid, col: 0, row: 3, col_span: 2)]
    cat_three_dir_text: nwg::TextInput,
}

impl App {
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn set_min(&self, data: &nwg::EventData) {
        let data = data.on_min_max();
        data.set_min_size(600, 700);
    }

    fn open_config(&self) {
        self.config_window.set_visible(true);
        self.config_window.set_focus();
    }

    fn close_config(&self) {
        self.config_window.set_visible(false);
    }

    fn update_button_status(&self) {
        let image_list_empty = self.filenames_buffer.borrow().is_empty();
        let action_list_empty = self.actions.borrow().is_empty();
        self.cat_one_btn
            .set_enabled(Path::new(&self.cat_one_dir_text.text()).exists() && !image_list_empty);
        self.cat_two_btn
            .set_enabled(Path::new(&self.cat_two_dir_text.text()).exists() && !image_list_empty);
        self.cat_three_btn
            .set_enabled(Path::new(&self.cat_three_dir_text.text()).exists() && !image_list_empty);
        self.cat_one_choose_btn.set_enabled(!image_list_empty);
        self.cat_two_choose_btn.set_enabled(!image_list_empty);
        self.cat_three_choose_btn.set_enabled(!image_list_empty);
        self.undo_btn.set_enabled(!action_list_empty);
        self.delete_btn.set_enabled(!image_list_empty);
    }

    fn update_img_count(&self) {
        let paths = self.filenames_buffer.borrow();
        self.status_bar
            .set_text(0, format!("Images found: {}", paths.len()).as_str())
    }
    fn undo(&self) {
        self.main_window.set_focus(); //Always focus window for keydown events
        let mut actions = self.actions.borrow_mut();
        let mut paths = self.filenames_buffer.borrow_mut();
        if let Some(action_to_undo) = actions.pop() {
            match action_to_undo.action_type {
                ActionType::Delete => {
                    let to_restore = match trash::os_limited::list() {
                        Ok(trash_item_vec) => trash_item_vec
                            .into_iter()
                            .filter(|trash_item| action_to_undo.from.ends_with(&trash_item.name)),
                        Err(_) => {
                            nwg::modal_error_message(
                                &self.main_window,
                                "Error",
                                "Can't find item in recycle bin to restore",
                            );
                            return;
                        }
                    };
                    match trash::os_limited::restore_all(to_restore) {
                        Ok(_) => {
                            paths.insert(0, action_to_undo.from);
                        }
                        Err(_) => {
                            nwg::modal_error_message(
                                &self.main_window,
                                "Error",
                                "Can't restore item in recycle bin",
                            );
                            return;
                        }
                    }
                }
                ActionType::Move => {
                    match fs::rename(&action_to_undo.to.unwrap(), &action_to_undo.from) {
                        //I can unwrap here becasue it will not be Null
                        Ok(_) => {
                            paths.insert(0, action_to_undo.from);
                        }
                        Err(err) => {
                            nwg::modal_error_message(
                                &self.main_window,
                                "Error",
                                format!("Could not move image {} !", err).as_str(),
                            );
                        }
                    }
                }
            }
        }
        drop(actions); // So I dont cause a borrow mut with regular borrow
        drop(paths); // So I dont cause a double borrow mut
        self.upate_img();
        self.update_img_count();
        self.update_button_status();
    }

    fn upate_img(&self) {
        let paths = self.filenames_buffer.borrow();

        if paths.len() > 0 {
            let path = match paths.get(0) {
                Some(path) => path,
                None => {
                    nwg::modal_error_message(
                        &self.main_window,
                        "Error",
                        "Vector empty after check (Shouldn't happen)",
                    );
                    return;
                }
            };

            // nwg::modal_info_message(&self.window, "DebugInfo", format!("{}", path).as_str());
            // Evil pattern matching
            let image = match self.decoder.from_filename(path) {
                Ok(img) => img,
                Err(_) => {
                    nwg::modal_error_message(&self.main_window, "Error", "Could not read image!");
                    return;
                }
            };

            let mut image_frame = match image.frame(0) {
                Ok(bmp) => bmp,
                Err(_) => {
                    nwg::modal_error_message(
                        &self.main_window,
                        "Error",
                        "Could not read image frame!",
                    );
                    return;
                }
            };

            let (frame_width, frame_height) = self.img_frame_ui.size();
            let (image_width, image_height) = image_frame.size();

            let mut factor: f32;
            if frame_height < image_height {
                factor = frame_height as f32 / image_height as f32;
            } else {
                factor = 1.0;
            }

            //Scale down by certain factor
            let image_height = image_height as f32 * factor;
            let image_width = image_width as f32 * factor;

            if frame_width < image_width as u32 {
                factor = frame_width as f32 / image_width as f32;
            } else {
                factor = 1.0;
            }

            //Scale down by certain factor
            let image_height = image_height as f32 * factor;
            let image_width = image_width as f32 * factor;

            image_frame = match self
                .decoder
                .resize_image(&image_frame, [image_width as u32, image_height as u32])
            {
                Ok(frame) => frame,
                Err(_) => {
                    nwg::modal_error_message(
                        &self.main_window,
                        "Error",
                        "Could not resize image frame!",
                    );
                    return;
                }
            };

            match image_frame.as_bitmap() {
                Ok(bitmap) => {
                    let mut img = self.loaded_image.borrow_mut();
                    img.replace(bitmap);
                    self.img_frame_ui.set_bitmap(img.as_ref());
                }
                Err(_) => {
                    nwg::modal_error_message(
                        &self.main_window,
                        "Error",
                        "Could not convert image to bitmap!",
                    );
                }
            }
        } else {
            let mut img = self.loaded_image.borrow_mut();
            img.replace(nwg::Bitmap::default());
            self.img_frame_ui.set_bitmap(img.as_ref());
        }
    }
    fn open_folder(&self) -> Option<PathBuf> {
        self.main_window.set_focus(); //Always focus window for keydown events

        if let Ok(d) = env::current_dir() {
            if let Some(d) = d.to_str() {
                self.folder_select
                    .set_default_folder(d)
                    .expect("Failed to set default folder.");
            }
        }

        // Open the file dialog
        if !self.folder_select.run(Some(&self.main_window)) {
            return None;
        }
        let path = self
            .folder_select
            .get_selected_item()
            .expect("System error occured"); //TODO Modal Error

        Some(PathBuf::from(path))
    }

    fn open_pictures_folder(&self) {
        if let Some(path) = self.open_folder() {
            let text = path.to_string_lossy();

            self.open_dir_text.set_text(&text);

            let paths = fs::read_dir(&path).expect("Not enough permissions");

            self.open_folder_os_path.replace(path);
            let names = paths
                .filter_map(|entry| {
                    // Skips non files
                    entry.ok().and_then(|e| {
                        //Turns into string
                        e.path().as_os_str().to_str().map(String::from)
                    })
                })
                .filter(|x| x.ends_with(".jpg") | x.ends_with(".jpeg") | x.ends_with(".png"))
                .collect::<Vec<String>>();

            self.filenames_buffer.replace(names);
            self.upate_img();
            self.update_img_count();

            self.update_button_status();
        }
    }

    fn open_cat_one_folder(&self) {
        if let Some(path) = self.open_folder() {
            let text = path.to_string_lossy();
            self.cat_one_dir_text.set_text(&text);
            self.cat_one_os_path.replace(path);
        }
        self.update_button_status();
    }
    fn open_cat_two_folder(&self) {
        if let Some(path) = self.open_folder() {
            let text = path.to_string_lossy();
            self.cat_two_dir_text.set_text(&text);
            self.cat_two_os_path.replace(path);
        }
        self.update_button_status();
    }
    fn open_cat_three_folder(&self) {
        if let Some(path) = self.open_folder() {
            let text = path.to_string_lossy();
            self.cat_three_dir_text.set_text(&text);
            self.cat_three_os_path.replace(path);
        }
        self.update_button_status();
    }

    fn delete_file(&self) {
        self.main_window.set_focus(); //Always focus window for keydown events
        let mut paths = self.filenames_buffer.borrow_mut();
        let path_of_file = paths.swap_remove(0);
        let mut actions = self.actions.borrow_mut();
        match trash::delete(&path_of_file) {
            Ok(_) => {
                let action = Action {
                    from: path_of_file,
                    to: None,
                    action_type: ActionType::Delete,
                };
                actions.push(action);
            }
            Err(err) => {
                nwg::modal_error_message(
                    &self.main_window,
                    "Error",
                    format!("Could not delete image {} !", err).as_str(),
                );
            }
        }
        drop(paths); // So I dont cause a double borrow mut
        drop(actions); // So I dont cause a double borrow mut
        self.update_button_status();
        self.upate_img();
        self.update_img_count();
    }

    fn move_file(&self, ctrl: &Button) {
        self.main_window.set_focus(); //Always focus window for keydown events
        let mut paths = self.filenames_buffer.borrow_mut();
        let btn_text = ctrl.text();
        let path_of_file = paths.swap_remove(0);
        let name_of_file = path_of_file.split('\\').last().unwrap().to_owned();
        let path_to_move_to: String;
        match btn_text.as_str() {
            "One" => {
                let cat_one_folder_path = self.cat_one_dir_text.text() + "\\";
                path_to_move_to = cat_one_folder_path + &name_of_file;
            }
            "Two" => {
                let cat_two_folder_path = self.cat_two_dir_text.text() + "\\";
                path_to_move_to = cat_two_folder_path + &name_of_file;
            }
            "Three" => {
                let cat_three_folder_path = self.cat_three_dir_text.text() + "\\";
                path_to_move_to = cat_three_folder_path + &name_of_file;
            }
            _ => panic!("This should not happen, match statement error"),
        }

        let mut actions = self.actions.borrow_mut();

        match fs::rename(&path_of_file, &path_to_move_to) {
            Ok(_) => {
                let action = Action {
                    from: path_of_file,
                    to: Some(path_to_move_to),
                    action_type: ActionType::Move,
                };
                actions.push(action);
            }
            Err(err) => {
                nwg::modal_error_message(
                    &self.main_window,
                    "Error",
                    format!("Could not move image {} !", err).as_str(),
                );
            }
        }
        drop(paths); // So I dont cause a double borrow mut
        drop(actions); // So I dont cause a double borrow mut
        self.update_button_status();
        self.upate_img();
        self.update_img_count();
    }

    fn process_keypress(&self, data: &nwg::EventData) {
        if data.on_key() == nwg::keys::_A && self.cat_one_btn.enabled() {
            self.cat_one_btn.click()
        }
        if data.on_key() == nwg::keys::_W && self.cat_two_btn.enabled() {
            self.cat_two_btn.click();
        }
        if data.on_key() == nwg::keys::_D && self.cat_three_btn.enabled() {
            self.cat_three_btn.click();
        }
        if data.on_key() == nwg::keys::_Z && self.undo_btn.enabled() {
            self.undo_btn.click();
        }
        if (data.on_key() == nwg::keys::DELETE || data.on_key() == nwg::keys::_S)
            && self.delete_btn.enabled()
        {
            self.delete_btn.click();
        }
    }
}
fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = App::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
