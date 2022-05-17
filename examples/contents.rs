use jupyter_api::*;

#[tokio::main]
async fn main() {
    let client = JupyterClient::default();
    //let resp = client.get_root_contents().await.unwrap();
    //println!("{resp:?}");
    //let resp = client.get_sessions().await.unwrap();

    //println!("{resp:?}");

    let resp = client
        .put_contents("test.ipynb", vec!["12 + 24".to_string()].into())
        .await
        .unwrap();
    println!("{resp:?}");

    let kernels = client.get_kernels().await.unwrap();
    let resp = kernels.iter().find(|each| each.name == "rust").unwrap();
    let kernsl_cli = resp.kernel_client(&client.base_host, client.secure);
    //let resp = kernsl_cli.run_code("a12 * 23".into(), None).await;
    //let resp = kernsl_cli.run_code(":dep tokio".into(), None).await;
    let resp = kernsl_cli.run_code("12 * 32".into(), None).await;
    println!("{resp:?}");
}