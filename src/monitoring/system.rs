/// Holds general information about the name, make, and model of the system.
///
/// Should be instantiated via the appropriate system information monitoring
/// backend.
pub struct SystemInformation {
    pub os: String,
    pub os_version: String,
    pub kernel_version: String,
    pub host_name: String,
    pub logical_processors: usize,
    pub physical_processors: usize,
    pub total_memory: u64,
}

/// Contains information about, and usage of, a single disk known to the
/// operating system.
///
/// Should be instantiated via the appropriate system information monitoring
/// backend.
pub struct DiskInformation {
    pub name: String,
    pub kind: String,
    pub available_space: u64,
    pub total_space: u64,
}