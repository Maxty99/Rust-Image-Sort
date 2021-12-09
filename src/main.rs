#![windows_subsystem = "windows"] // https://gabdube.github.io/native-windows-gui/native-windows-docs/distribute.html

use std::cell::RefCell;
use std::env;

use std::fs;

use native_windows_derive as nwd;
use native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::Button;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct App {
    filenames_buffer: RefCell<Vec<String>>,

    loaded_image: RefCell<Option<nwg::Bitmap>>,

    #[nwg_control(flags: "MAIN_WINDOW|VISIBLE", title: "Image Sort", size: (1000,700), center: true)]
    //VERY IMPORTANT OTHERWISE IT DOESNT END PROCESS
    #[nwg_events(OnMinMaxInfo: [App::set_min(SELF, EVT_DATA)], OnResize: [App::upate_img], OnWindowClose: [App::exit], OnKeyPress: [App::process_keypress(SELF, EVT_DATA)], OnInit: [App::set_buttons_disabled])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 2, min_size: [500, 500])]
    grid: nwg::GridLayout,

    #[nwg_resource]
    decoder: nwg::ImageDecoder,

    #[nwg_resource(title: "Select a folder", action: nwg::FileDialogAction::OpenDirectory)]
    folder_select: nwg::FileDialog,

    #[nwg_control]
    #[nwg_layout_item(layout: grid, col: 0, row: 0, col_span: 3, row_span: 10)]
    img_frame_ui: nwg::ImageFrame,

    #[nwg_control(text: "One", focus: false)]
    #[nwg_layout_item(layout: grid, col: 0, row: 10)]
    #[nwg_events( OnButtonClick: [App::process_moving_file(SELF, CTRL)])]
    cat_one_btn: nwg::Button,

    #[nwg_control(text: "Two", focus: false)]
    #[nwg_layout_item(layout: grid, col: 1, row: 10)]
    #[nwg_events( OnButtonClick: [App::process_moving_file(SELF, CTRL)])]
    cat_two_btn: nwg::Button,

    #[nwg_control(text: "Three", focus: false)]
    #[nwg_layout_item(layout: grid, col: 2, row: 10)]
    #[nwg_events( OnButtonClick: [App::process_moving_file(SELF, CTRL)])]
    cat_three_btn: nwg::Button,

    #[nwg_control(text: "Pictures", focus: false)]
    #[nwg_layout_item(layout: grid, col: 2, row: 11)]
    #[nwg_events( OnButtonClick: [App::open_folder(SELF, CTRL)])]
    open_btn: nwg::Button,

    #[nwg_control(text: "", focus: false, readonly: true)]
    #[nwg_layout_item(layout: grid, col: 0, row: 11, col_span: 2)]
    open_dir_text: nwg::TextInput,

    #[nwg_control(text: "Category 1", focus: false)]
    #[nwg_layout_item(layout: grid, col: 2, row: 12)]
    #[nwg_events( OnButtonClick: [App::open_folder(SELF, CTRL)])]
    cat_one_choose_btn: nwg::Button,

    #[nwg_control(text: "", focus: false, readonly: true)]
    #[nwg_layout_item(layout: grid, col: 0, row: 12, col_span: 2)]
    cat_one_dir_text: nwg::TextInput,

    #[nwg_control(text: "Category 2", focus: false)]
    #[nwg_layout_item(layout: grid, col: 2, row: 13)]
    #[nwg_events( OnButtonClick: [App::open_folder(SELF, CTRL)])]
    cat_two_choose_btn: nwg::Button,

    #[nwg_control(text: "", focus: false, readonly: true)]
    #[nwg_layout_item(layout: grid, col: 0, row: 13, col_span: 2)]
    cat_two_dir_text: nwg::TextInput,

    #[nwg_control(text: "Category 3", focus: false)]
    #[nwg_layout_item(layout: grid, col: 2, row: 14)]
    #[nwg_events( OnButtonClick: [App::open_folder(SELF, CTRL)])]
    cat_three_choose_btn: nwg::Button,

    #[nwg_control(text: "", focus: false, readonly: true)]
    #[nwg_layout_item(layout: grid, col: 0, row: 14, col_span: 2)]
    cat_three_dir_text: nwg::TextInput,

    #[nwg_control(text: "Open folder to load images")]
    #[nwg_layout_item(layout: grid, col: 0, row: 15, col_span: 3)]
    // Even though its not part of the grid I need to do this so it
    // isnt drawn over by the category three button and textbox
    status_bar: nwg::StatusBar,
}

impl App {
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn set_min(&self, data: &nwg::EventData) {
        let data = data.on_min_max();
        data.set_min_size(600, 700);
    }

    fn set_buttons_disabled(&self) {
        self.cat_one_btn.set_enabled(false);
        self.cat_two_btn.set_enabled(false);
        self.cat_three_btn.set_enabled(false);
        self.cat_one_choose_btn.set_enabled(false);
        self.cat_two_choose_btn.set_enabled(false);
        self.cat_three_choose_btn.set_enabled(false);
    }

    fn update_img_count(&self) {
        let paths = self.filenames_buffer.borrow();
        self.status_bar
            .set_text(0, format!("Images found: {}", paths.len()).as_str())
    }

    fn upate_img(&self) {
        let paths = self.filenames_buffer.borrow();

        if paths.len() > 0 {
            let path = match paths.get(0) {
                Some(path) => path,
                None => {
                    nwg::modal_error_message(
                        &self.window,
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
                    nwg::modal_error_message(&self.window, "Error", "Could not read image!");
                    return;
                }
            };

            let mut image_frame = match image.frame(0) {
                Ok(bmp) => bmp,
                Err(_) => {
                    nwg::modal_error_message(&self.window, "Error", "Could not read image frame!");
                    return;
                }
            };

            let (frame_width, frame_height) = self.img_frame_ui.size();
            let (image_width, image_height) = image_frame.size();
            //If the frame is bigger then the image
            if frame_width < image_width || frame_height < image_height {
                let factor: f32;
                if frame_width < image_width {
                    factor = frame_width as f32 / image_width as f32;
                } else {
                    factor = frame_height as f32 / image_height as f32;
                }

                //Scale down by certain factor
                let new_image_height = image_height as f32 * factor;
                let new_image_width = image_width as f32 * factor;
                image_frame = match self.decoder.resize_image(
                    &image_frame,
                    [new_image_width as u32, new_image_height as u32],
                ) {
                    Ok(frame) => frame,
                    Err(_) => {
                        nwg::modal_error_message(
                            &self.window,
                            "Error",
                            "Could not resize image frame!",
                        );
                        return;
                    }
                };
            }
            match image_frame.as_bitmap() {
                Ok(bitmap) => {
                    let mut img = self.loaded_image.borrow_mut();
                    img.replace(bitmap);
                    self.img_frame_ui.set_bitmap(img.as_ref());
                }
                Err(_) => {
                    nwg::modal_error_message(
                        &self.window,
                        "Error",
                        "Could not convert image to bitmap!",
                    );
                }
            }
        }
    }
    fn open_folder(&self, ctrl: &Button) {
        // See which text box to update with the new path
        let btn_text = ctrl.text();
        let text_feild: &nwg::TextInput;
        let mut process_pictures: bool = false;
        match btn_text.as_str() {
            "Pictures" => {
                text_feild = &self.open_dir_text;
                process_pictures = true
            }
            "Category 1" => {
                text_feild = &self.cat_one_dir_text;
                self.cat_one_btn.set_enabled(true);
            }
            "Category 2" => {
                text_feild = &self.cat_two_dir_text;
                self.cat_two_btn.set_enabled(true);
            }
            "Category 3" => {
                text_feild = &self.cat_three_dir_text;
                self.cat_three_btn.set_enabled(true);
            }
            _ => panic!("This should not happen, match statement error"),
        }

        if let Ok(d) = env::current_dir() {
            if let Some(d) = d.to_str() {
                self.folder_select
                    .set_default_folder(d)
                    .expect("Failed to set default folder.");
            }
        }

        // Open the file dialog
        if !self.folder_select.run(Some(&self.window)) {
            return;
        }
        let path = self
            .folder_select
            .get_selected_item()
            .expect("System error occured");

        text_feild.set_text(path.to_str().expect("Path isn't valid unicode"));
        if process_pictures {
            let paths = fs::read_dir(path).expect("Not enough permissions"); //Expects not enough permissions

            let names = paths
                .filter_map(|entry| {
                    // Skips non files
                    entry.ok().and_then(|e| {
                        //Turns into string
                        e.path().as_os_str().to_str().map(|s| String::from(s))
                    })
                })
                .filter(|x| x.ends_with(".jpg") | x.ends_with(".jpeg") | x.ends_with(".png"))
                .collect::<Vec<String>>();

            self.filenames_buffer.replace(names);
            self.upate_img();
            self.update_img_count();
            if self.filenames_buffer.borrow().len() > 0 {
                // Only enable when theres images to move
                self.cat_one_choose_btn.set_enabled(true);
                self.cat_two_choose_btn.set_enabled(true);
                self.cat_three_choose_btn.set_enabled(true);
            } else {
                self.cat_one_choose_btn.set_enabled(false);
                self.cat_two_choose_btn.set_enabled(false);
                self.cat_three_choose_btn.set_enabled(false);
            }
        }
    }

    fn process_moving_file(&self, ctrl: &Button) {
        self.move_file(ctrl);
        self.upate_img();
        self.update_img_count();
        // Have to do this through another function to make sure borrow of refcell
        // goes out fo scope and doesnt panic
    }

    fn move_file(&self, ctrl: &Button) {
        let mut paths = self.filenames_buffer.borrow_mut();
        let btn_text = ctrl.text();
        let path_of_file = paths.swap_remove(0);
        let name_of_file = path_of_file.split("\\").last().unwrap().to_owned();
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
        fs::rename(path_of_file, path_to_move_to); //TODO: Actuall error handling
    }

    //TODO: Yeah uh just for debug ok
    fn process_keypress(&self, data: &nwg::EventData) {
        if data.on_key() == nwg::keys::_A {
            nwg::modal_info_message(&self.window, "haha", "lol");
        }
    }
}
fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = App::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
