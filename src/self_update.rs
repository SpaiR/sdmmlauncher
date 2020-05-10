const LAUNCHER_LATEST_RELEASE: &str = "https://github.com/SpaiR/sdmmlauncher/releases/latest";
const LAUNCHER_REPO_OWNER: &str = "SpaiR";
const LAUNCHER_REPO_NAME: &str = "sdmmlauncher";

#[cfg(target_os = "linux")]
const LAUNCHER_BIN_NAME: &str = "sdmmlauncher";

#[cfg(target_os = "windows")]
const LAUNCHER_BIN_NAME: &str = "sdmmlauncher.exe";

pub fn auto_update() {
    if env!("CARGO_PKG_VERSION").ne(get_latest_version().as_str()) {
        println!("[ Found the new SDMMLauncher version! ]");
        println!("[ Updating... ]");
        update();
        println!("[ Changes will be applied after restart ]\n");
    }
}

fn get_latest_version() -> String {
    let resp = reqwest::blocking::get(LAUNCHER_LATEST_RELEASE);
    let full_path = resp.unwrap().url().to_string();
    let idx = full_path.rfind('/').unwrap() + 2;
    full_path[idx..].to_string()
}

fn update() {
    self_update::backends::github::Update::configure()
        .repo_owner(LAUNCHER_REPO_OWNER)
        .repo_name(LAUNCHER_REPO_NAME)
        .bin_name(LAUNCHER_BIN_NAME)
        .show_download_progress(true)
        .no_confirm(true)
        .show_output(false)
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()
        .unwrap()
        .update()
        .unwrap();
}
