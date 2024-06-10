use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use tokio_task_pool::Pool;

const SEMESTER: &'static str = "1122";
const LANGUAGE: q::Language = q::Language::Zh;

async fn worker(client: &q::Q, course_no: &str) {
    let details = client.query(SEMESTER, course_no, LANGUAGE).await.unwrap();
    println!(
        "{: <10} | {:　<10} | {}/{}",
        details.course_no, details.course_name, details.choose_student, details.restrict2
    );
}

#[tokio::main]
async fn main() {
    const THREADS: usize = 64;

    let duration = Duration::from_secs(10);

    let times = Arc::new(AtomicUsize::new(0));
    let times_clone = times.clone();

    let run = Arc::new(AtomicBool::new(true));
    let run_clone = run.clone();

    let pool = Pool::bounded(THREADS).with_spawn_timeout(Duration::from_micros(10));

    let mut clients = Vec::<q::Q>::new();

    for _ in 0..(THREADS + 1) {
        clients.push(q::Q::new());
    }

    let clients = Arc::new(clients);

    let courses = vec![
        "AT2005701",
        "CS2006302",
        "CS2008302",
        "CS3001302",
        "GE3729302",
        "PE111B022",
        "TCG041301",
        "TCG047301",
    ];
    let courses_length = courses.len();
    let courses = Arc::new(courses);

    tokio::spawn(async move {
        let mut i = 0;
        while run_clone.load(Ordering::Relaxed) {
            let clients_clone = clients.clone();
            let courses_clone = courses.clone();
            let times_clone = Arc::clone(&times_clone);

            match pool
                .spawn(async move {
                    let c = clients_clone.get(i % THREADS).unwrap();
                    let no = courses_clone.get(i % courses_length).unwrap();

                    worker(c, no).await;

                    times_clone.fetch_add(1, Ordering::Relaxed);
                })
                .await
            {
                Ok(_) => i = i + 1,
                Err(_) => {}
            }
        }
    });

    tokio::time::sleep(duration.clone()).await;

    run.store(false, Ordering::Relaxed);

    println!(
        "queried {} times in {:?}",
        times.load(Ordering::Relaxed),
        duration
    );
}
