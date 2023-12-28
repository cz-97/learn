use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
    static R: AtomicU64 = AtomicU64::new(0);
    // 创建一个子线程
    thread::spawn(move || {
        // 循环自增
        loop {
            R.fetch_add(1, Ordering::Relaxed);
        }
        // 通过通道发送这个整数
    });
    // 主线程等待3秒
    thread::sleep(Duration::from_secs(1));
    // 尝试从通道中获取子线程的结果
    println!("{}",R.load(Ordering::Relaxed));
}
