use std::path::PathBuf;

pub type FilterTuple = Option<(&'static str, Vec<&'static str>)>;

pub fn open_file_dialog_single(initial_directory: &str, filter: FilterTuple) -> Option<PathBuf> {
    let mut dialog = rfd::FileDialog::new();
    dialog = dialog.set_directory(initial_directory);
    match filter {
        None => {}
        Some((name, ext)) => {
            dialog = dialog.add_filter(name, &ext);
        }
    }

    dialog.pick_file()
}

pub fn open_file_dialog_multiple(
    initial_directory: &str,
    filter: FilterTuple,
) -> Option<Vec<PathBuf>> {
    let mut dialog = rfd::FileDialog::new();
    dialog = dialog.set_directory(initial_directory);
    match filter {
        None => {}
        Some((name, ext)) => {
            dialog = dialog.add_filter(name, &ext);
        }
    }

    dialog.pick_files()
}

pub fn open_folder_dialog_single(initial_directory: &str, filter: FilterTuple) -> Option<PathBuf> {
    let mut dialog = rfd::FileDialog::new();
    dialog = dialog.set_directory(initial_directory);
    match filter {
        None => {}
        Some((name, ext)) => {
            dialog = dialog.add_filter(name, &ext);
        }
    }

    dialog.pick_folder()
}

pub fn open_folder_dialog_multiple(
    initial_directory: &str,
    filter: FilterTuple,
) -> Option<Vec<PathBuf>> {
    let mut dialog = rfd::FileDialog::new();
    dialog = dialog.set_directory(initial_directory);
    match filter {
        None => {}
        Some((name, ext)) => {
            dialog = dialog.add_filter(name, &ext);
        }
    }

    dialog.pick_folders()
}
