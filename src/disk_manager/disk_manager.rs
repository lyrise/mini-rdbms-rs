use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;

pub struct DiskManager {
    heap_file: File,
    next_page_id: u64,
}
const PAGE_SIZE: u64 = 1024 * 4096;

impl DiskManager {
    pub fn new(heap_file: File) -> io::Result<Self> {
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / PAGE_SIZE as u64;
        Ok(Self {
            heap_file,
            next_page_id,
        })
    }
    pub fn open(heap_file_path: impl AsRef<Path>) -> io::Result<Self> {
        let heap_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(heap_file_path)?;
        Self::new(heap_file)
    }
    pub fn allocate_page(&mut self) -> PageId {}
    pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {}
    pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {}
}
