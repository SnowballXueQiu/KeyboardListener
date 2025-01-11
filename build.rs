const APP_NAME: &str = "Runtime Broker";

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set("FileDescription", APP_NAME);
        res.set("InternalName", APP_NAME);
        res.set("ProductName", APP_NAME);
        res.set("OriginalFilename", format!("{}.exe", APP_NAME).as_str());
        res.compile().expect("Failed to compile resources");
    }
}
