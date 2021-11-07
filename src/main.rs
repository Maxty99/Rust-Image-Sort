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
    filenames: RefCell<Option<Vec<String>>>,

    // loaded_image: RefCell<Option<nwg::Bitmap>>,
    #[nwg_control(flags: "MAIN_WINDOW|VISIBLE", title: "Image Sort", size: (1000,700), center: true)]
    //VERY IMPORTANT OTHERWISE IT DOESNT END PROCESS
    #[nwg_events( OnWindowClose: [App::exit], OnKeyPress: [App::process_keypress(SELF, EVT_DATA)], OnResize: [App::size])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 2, min_size: [500, 500])]
    grid: nwg::GridLayout,

    #[nwg_resource]
    decoder: nwg::ImageDecoder,

    #[nwg_resource(title: "Select a folder", action: nwg::FileDialogAction::OpenDirectory)]
    folder_select: nwg::FileDialog,

    #[nwg_control]
    #[nwg_layout_item(layout: grid, col: 0, row: 0, col_span: 3, row_span: 10)]
    img: nwg::ImageFrame,

    #[nwg_control(text: "One", focus: false)]
    #[nwg_layout_item(layout: grid, col: 0, row: 10)]
    cat_one_btn: nwg::Button,

    #[nwg_control(text: "Two", focus: false)]
    #[nwg_layout_item(layout: grid, col: 1, row: 10)]
    cat_two_btn: nwg::Button,

    #[nwg_control(text: "Three", focus: false)]
    #[nwg_layout_item(layout: grid, col: 2, row: 10)]
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
}

impl App {
    fn exit(&self) {
        nwg::stop_thread_dispatch();
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
            "Category 1" => text_feild = &self.cat_one_dir_text,
            "Category 2" => text_feild = &self.cat_two_dir_text,
            "Category 3" => text_feild = &self.cat_three_dir_text,
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
                        e.path()
                            .file_name()
                            .and_then(|n| n.to_str().map(|s| String::from(s)))
                    })
                })
                .filter(|x| x.ends_with(".jpg") | x.ends_with(".jpeg") | x.ends_with(".png"))
                .collect();

            let mut paths_ref = self.filenames.borrow_mut();
            paths_ref.replace(names);
        }
    }

    fn process_keypress(&self, data: &nwg::EventData) {
        if data.on_key() == nwg::keys::_A {
            nwg::modal_info_message(&self.window, "haha", "lol");
        }
    }
    fn size(&self) {
        self.grid.fit();
    }
}
fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = App::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
