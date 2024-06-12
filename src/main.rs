mod hifini;

use std::{env, error::Error};
use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenvy::dotenv()?;
  start().await;
  Ok(())
}

async fn start() {
  let schedule = env::var("SCHEDULE_RULE").unwrap_or("10 24 12 * * * *".to_string());
  let sched = JobScheduler::new().await.expect("Canot create JobScheduler");
  let job = Job::new_tz(schedule.as_str(), chrono_tz::Asia::Shanghai, |_, _| {
    println!(">>> HiFiNi");
    tokio::spawn(hifini::sign_in());
  })
  .expect("Canot create a new job");
  sched.add(job).await.expect("Canot add a job to the JobScheduler");
  sched.start().await.expect("Cannot run all jobs");
  tokio::signal::ctrl_c().await.unwrap();
}
