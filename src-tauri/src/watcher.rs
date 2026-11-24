use crate::{priority, process, rules::Rule};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub fn start(rules: Arc<Mutex<Vec<Rule>>>) {
    thread::spawn(move || loop {
        let active: Vec<Rule> = rules
            .lock()
            .unwrap()
            .iter()
            .filter(|r| r.enabled)
            .cloned()
            .collect();

        if !active.is_empty() {
            if let Ok(procs) = process::list_processes() {
                for rule in &active {
                    let Some(class) = priority::class_from_str(&rule.priority) else {
                        continue;
                    };
                    let mut target = rule.process_name.to_lowercase();
                    if !target.ends_with(".exe") {
                        target.push_str(".exe");
                    }
                    // Matched processes plus all descendants
                    let mut set: HashSet<u32> = procs
                        .iter()
                        .filter(|p| p.name.to_lowercase() == target)
                        .map(|p| p.pid)
                        .collect();
                    loop {
                        let before = set.len();
                        for p in &procs {
                            if set.contains(&p.parent_pid) {
                                set.insert(p.pid);
                            }
                        }
                        if set.len() == before {
                            break;
                        }
                    }
                    for pid in set {
                        let _ = priority::set_priority(pid, class);
                    }
                }
            }
        }
        thread::sleep(Duration::from_secs(2));
    });
}