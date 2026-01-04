use criterion::{criterion_group, criterion_main, Criterion};
use std::time::{Duration, Instant};
use tokio::runtime::Builder;
use tokio::sync::mpsc;

// Simulates the App's event loop structure
async fn run_event_loop(blocking: bool, duration: Duration) -> Duration {
    let (tx, mut rx) = mpsc::channel(100);
    let mut interval = tokio::time::interval(Duration::from_millis(10));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    let start = Instant::now();
    let mut max_ping_latency = Duration::ZERO;

    // Simulate external events (pings) arriving every 2ms
    let pinger = tokio::spawn(async move {
        while start.elapsed() < duration {
            let _ = tx.send(Instant::now()).await;
            tokio::time::sleep(Duration::from_millis(2)).await;
        }
    });

    // Main loop
    while start.elapsed() < duration {
        tokio::select! {
            // High priority: Process networking events (pings)
            Some(ping_time) = rx.recv() => {
                let latency = ping_time.elapsed();
                if latency > max_ping_latency {
                    max_ping_latency = latency;
                }
            }

            // Low priority: Periodic State Saving (Tick)
            _ = interval.tick() => {
               // Simulate "Save State" happening (e.g., every tick for stress test)

               // Write 50KB to disk (simulated size of Coordinator)
               // Real world: Serializing 50KB to TOML + Writing to disk
               let data = vec![0u8; 50 * 1024];
               let dir = tempfile::tempdir().unwrap();
               let path = dir.path().join("test_state.toml");

               if blocking {
                   // BLOCKING: Serializing + Writing blocks the loop
                   // Simulate TOML serialization cost (CPU) + Disk Write (IO)
                   // std::thread::sleep(Duration::from_millis(5)); // CPU equivalent
                   std::fs::write(&path, &data).unwrap();
               } else {
                   // NON-BLOCKING: Offload
                   tokio::task::spawn_blocking(move || {
                        std::fs::write(&path, &data).unwrap();
                   });
               }
            }
        }
    }

    let _ = pinger.await;
    max_ping_latency
}

fn bench_io_latency(c: &mut Criterion) {
    // Force single-threaded runtime to demonstrate the effect clearly
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    let mut group = c.benchmark_group("event_loop_latency");

    group.bench_function("blocking_save", |b| {
        b.to_async(&rt).iter_custom(|iters| async move {
            let mut total_latency = Duration::ZERO;
            for _ in 0..iters {
                total_latency += run_event_loop(true, Duration::from_millis(100)).await;
            }
            total_latency
        })
    });

    group.bench_function("async_save", |b| {
        b.to_async(&rt).iter_custom(|iters| async move {
            let mut total_latency = Duration::ZERO;
            for _ in 0..iters {
                total_latency += run_event_loop(false, Duration::from_millis(100)).await;
            }
            total_latency
        })
    });

    group.finish();
}

criterion_group!(benches, bench_io_latency);
criterion_main!(benches);
