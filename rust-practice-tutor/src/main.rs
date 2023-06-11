use futures::executor::block_on;
use futures::Future;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parts::one::variables();
    // parts::two::ownership();
    // parts::three::functions();
    // parts::four::structures();
    // parts::five::enumerations();
    // parts::six::collections();
    // parts::seven::optionals();
    // parts::eight::errors()?;
    // parts::nine::lifetimes();
    // parts::ten::traits();
    // parts::eleven::pointers();
    // parts::twelve::generics();

    // Asynchronous
    // With block_on
    let name = block_on(get_name());
    println!("Hello, {}", name);

    // With async-await
    let result_one = call_api_one().await;
    println!("{}", result_one);

    let result_two = call_api_two().await;
    println!("{}", result_two);

    // Return Future directly

    //Move variables
    let name2 = get_async_name().await;
    println!("{}", name2);

    Ok(())
}

async fn get_name() -> String {
    "John".to_string()
}

fn call_api_one() -> impl Future<Output = String> {
    async {
        sleep(Duration::from_secs(1)).await;
        "Api one".to_string()
    }
}

fn get_async_name() -> impl Future<Output = String> {
    let name = "John".to_string();

    async move {
        sleep(Duration::from_secs(1)).await;
        format!("Hello, {} Doe", name)
    }
}

async fn call_api_two() -> String {
    sleep(Duration::from_secs(1)).await;
    "Api two".to_string()
}

mod parts {
    // pub mod five;
    // pub mod four;
    // pub mod one;
    // pub mod three;
    // pub mod two;
    // pub mod six;
    // pub mod seven;
    // pub mod eight;
    // pub mod nine;
    // pub mod ten;
    // pub mod eleven;
    // pub mod twelve;
}
