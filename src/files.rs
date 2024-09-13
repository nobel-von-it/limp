pub struct FileManager;

impl FileManager {
    pub fn create(path: &str) -> std::io::Result<File> {
        std::fs::File::create(path)
    }
    pub fn rwopen(path: &str) -> std::io::Result<File> {
        std::fs::File::options().read(true).write(true).open(path)
    }
}
