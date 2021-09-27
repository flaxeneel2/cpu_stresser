use sha2::{Digest, Sha512};
use sysinfo::SystemExt;
use clap::Arg;

fn main() {
    let start = std::time::SystemTime::now();
    let mut sysinfo = sysinfo::System::new_all();
    sysinfo.refresh_all();
    let (sender, receiver) = std::sync::mpsc::channel();
    let args = clap::App::new("CPU stress tester")
        .version("v0.1.0")
        .author("Made by: flaxeneel2")
        .about("Description: This program is a multithreaded hasher designed to stress the CPU. Can be used to check temperatures and max boost clocks.")
        .arg(Arg::new("threads")
            .short('t')
            .long("threads")
            .about("Specify the number of threads to use (Defaults to all)")
            .takes_value(true)
            .default_value(&(sysinfo.processors().len().to_string())))
        .arg(Arg::new("hashes")
            .short('s')
            .long("hashes")
            .about("Specify the number of hashes to do total, if hashes cannot be evenly divided with number of cores, it will be floored")
            .takes_value(true)
            .default_value("100000000"))
        .get_matches();
    let mut threads: i32 = args.value_of("threads").unwrap().parse().expect("Threads is not a number!");
    let hashes_total: i32 = args.value_of("hashes").unwrap().parse().expect("Hashes is not a number!");
    let hashes_per_thread: i32 = hashes_total/threads;
    println!("Amount of threads detected: {}", threads);
    println!("This program will do {} hashes", threads*hashes_total);
    println!("Each program will hash {} hashes!", hashes_per_thread);
    while threads != 0 {
        let sen_clone = sender.clone();
        let runs = hashes_per_thread;

        let _ = std::thread::spawn(move || thread_function(sen_clone, runs, threads.clone()));
        threads -= 1;
    }
    threads = args.value_of("threads").unwrap().parse().expect("Threads is not a number!");

    let receiver_thread = std::thread::spawn(move|| {
        let mut threads_clone = threads.clone();
        for i in receiver {
            let isplit = i.split("|");
            if isplit.clone().next().unwrap()=="finished" {
                println!("[Thread {}]: Status: {}", isplit.clone().last().unwrap(), isplit.clone().next().unwrap());
                threads_clone -= 1;
                if threads_clone == 0 {
                    println!("{}", "Task is finished!");
                    break
                }
            }
        }
    });
    receiver_thread.join().unwrap();
    let end = std::time::SystemTime::now();
    let dur = end.duration_since(std::time::UNIX_EPOCH).expect("aa").as_millis() - start.duration_since(std::time::UNIX_EPOCH).expect("bbbb").as_millis();
    println!("time taken: {}", dur);
}

fn thread_function(senderr: std::sync::mpsc::Sender<String>, mut runs: i32, id: i32) {
    while runs != 0 {
        let mut sha_hash = Sha512::new();
        sha_hash.update(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("bruh").as_nanos().to_string());
        let _hash = sha_hash.finalize();
        runs -= 1;
    }
    senderr.send(String::from("finished|") + &*id.to_string()).unwrap();
}