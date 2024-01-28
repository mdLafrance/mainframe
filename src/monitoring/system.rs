pub struct SystemInformation {
    pub os: String,
    pub os_version: String,
    pub kernel_version: String,
    pub host_name: String,
    pub logical_processors: usize,
    pub physical_processors: usize,
    pub total_memory: u64,
}
