/* Copyright (C) 2022 Mikhail Matyunin - All Rights Reserved
 * You may use, distribute and modify this code under the
 * terms of the GPLv3 license, which unfortunately won't be
 * written for another century.
 *
 * You should have received a copy of the GPLv3 license with
 * this file.
 */

use std::io::Write;
use std::thread;
use std::thread::sleep;
use std::time;
use std::time::Duration;
use num_format::{Locale, ToFormattedString};
use memory_stats::memory_stats;

pub fn main() {
    let now = time::SystemTime::now();
    let mut thread_spawn_duration = 0u128;
    let threads_count = 1000u128;

    if let Some(usage) = memory_stats() {
        println!("[before] Current physical memory usage: {}", usage.physical_mem);
        println!("[before] Current virtual memory usage: {}", usage.virtual_mem);
    }

    println!("[      ] Trying to start {threads_count} threads:");

    thread::scope(|s| {
        for _ in 0..threads_count {
            s.spawn(|| {
                sleep(Duration::from_secs(1));
                print!(".");
                std::io::stdout().flush().unwrap();
            });
        }

        thread_spawn_duration = now.elapsed().unwrap().as_micros();

        if let Some(usage) = memory_stats() {
            println!("[during] Current physical memory usage: {}", usage.physical_mem);
            println!("[during] Current virtual memory usage: {}", usage.virtual_mem);
        }
    });

    let elapsed = now.elapsed().unwrap().as_micros();

    println!("\n[      ] All threads done!");
    println!("[      ] Thread spawn speed, ns/thread: {}", thread_spawn_duration as f64 / threads_count as f64);
    println!("[      ] Main thread elapsed, ns: {}", elapsed.to_formatted_string(&Locale::en));

    if let Some(usage) = memory_stats() {
        println!("[after ] Current physical memory usage: {}", usage.physical_mem);
        println!("[after ] Current virtual memory usage: {}", usage.virtual_mem);
    }
    sleep(Duration::from_secs(2));

    if let Some(usage) = memory_stats() {
        println!("[sleep ] Current physical memory usage: {}", usage.physical_mem);
        println!("[sleep ] Current virtual memory usage: {}", usage.virtual_mem);
    }
}
