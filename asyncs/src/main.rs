use tokio::time::{sleep, Duration};
use reqwest::Error;

async fn async_task(id: u32, delay: u64) {
    println!("start: {}", id);
    sleep(Duration::from_secs(delay)).await;
    println!("end: {}", id);
}

async fn async_request(id: u32, url: &str) -> Result<(), Error> {
    println!("start req: {}", id);
    sleep(Duration::from_secs(2)).await;
    let response = reqwest::get(url).await?;
    println!("end req: {} : {:?}", id, response.status());
    Ok(())
}

#[tokio::main]
async fn main() {
    
    //simple async task
    let task_one = tokio::spawn(async_task(1, 3));
    let task_two = tokio::spawn(async_task(2, 2));
    
    let result_one = task_one.await.unwrap();
    let result_two = task_two.await.unwrap();

    println!("result_one: {:?}", result_one);
    println!("result_two: {:?}", result_two);

    //request async task

    let request_one = tokio::spawn(async_request(1, "https://www.exemplo.com"));
    let request_two = tokio::spawn(async_request(2, "https://www.rust-lang.org"));

    let result_req_one = request_one.await.unwrap();
    let result_req_two = request_two.await.unwrap();

    if result_req_one.is_ok() {
        println!("request_req_one is ok");
    } else {
        println!("request_req_one is not ok");
    }
    
    if result_req_two.is_ok() {
        println!("request_req_two is ok");
    } else {
        println!("request_req_two is not ok");
    }

}
