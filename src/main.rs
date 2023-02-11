use cc_lib::run_container;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    run_container()
        .await
        .expect("failed to create container");
}
