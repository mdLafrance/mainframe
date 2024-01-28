use mainframe::monitoring::{
    monitor::{SystemMonitor, SystemMonitorTargets},
    sysinfo_monitor::SiSystemMonitor,
};

fn main() {
    let mut s = SiSystemMonitor::new(vec![SystemMonitorTargets::CpuUsage], 0, 0);
    let res = s.poll();

    println!("Poll: {:?}", res);
}
