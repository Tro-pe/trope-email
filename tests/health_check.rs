use std::io;
#[tokio::test]
async fn health_check_works (){
	spawn_app().await.expect("Failed to spawn app");

	//Bring reqwest to perform HTTP requests against our app
	let client = reqwest::Client::new();

	let response = client
		.get("http://127.0.0.1:8000/health_check")
		.send()
		.await
		.expect("Failed to send request");
	
	assert!(response.status().is_success());
	assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> io::Result<()> {
	troperust::run().await
}