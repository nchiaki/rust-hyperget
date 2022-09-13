use std::env;

//use hyper_native_tls::NativeTlsClient;
use hyper_tls::{HttpsConnector, TlsStream};
use hyper::{Client, Body, Method, Request, Uri};
//use hyper::net::HttpsConnector;
use hyper::body::HttpBody as _;

use tokio::io::{stdout, AsyncWriteExt as _};
//use to_vec::ToVec;

mod help;

static GETURL_CHIAKI: &str = "http://chiaki.sakura.ne.jp";
static GETURL_120: &str = "http://192.168.113.120";
static GETURL_HTTPBIN: &str = "http://httpbin.org/ip";

static POSTURL_HTTPBIN_POST: &str = "http://httpbin.org/post";

static HTTPSURL_CHIAKI: &str = "https://chiaki.sakura.ne.jp";
static HTTPSURL_MIHARU: &str = "https://www.miharu.co.jp";

static GETURLS: [&'static str; 3] = [
    GETURL_CHIAKI,
    GETURL_120,
    GETURL_HTTPBIN,
];

static POSTURLS: [&'static str; 1] = [
    POSTURL_HTTPBIN_POST,
];

static HTTPSURLS: [&'static str; 2] = [
    HTTPSURL_CHIAKI,
    HTTPSURL_MIHARU,
];


#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error + Send + Sync>>
{
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    if help::parse_argv(argc, argv) == false
    {return Ok(());}

    loop
    {
        let funk = help::pop_funk();
        if funk.len() == 0
        {break;}
        println!("{}", funk);

        if funk == "get"
        {sample_get().await?;}  // GETサンプル
        else if funk == "post"
        {sample_post().await?;}  // POSTサンプル
        else if funk == "para"
        {sample_para().await?;}  // FUTUREサンプル
        else if funk == "https"
        {sample_https().await?;}  // TLS サンプル
    }
    Ok(())
}

async fn sample_get() -> Result <(), Box<dyn std::error::Error + Send + Sync>>
{
    for urls in GETURLS
    {
        let url = urls.parse()?;
        let client = Client::new();
        let mut resp = client.get(url).await?;

        println!("\nGET =============================================================");
        println!("{}:\n{:?}\n", urls, resp);
        while let Some(chunk) = resp.body_mut().data().await
        {
            stdout().write_all(&chunk?).await?;
        }
    }
    Ok(())
}

async fn sample_post() -> Result <(), Box<dyn std::error::Error + Send + Sync>>
{
    for urls in POSTURLS
    {
        let req = Request::builder()
            .method(Method::POST)
            .uri(urls)
            .header("content-type", "application/json")
            .body(Body::from(r#"{"library":"hyper"}"#))?;

        let client = Client::new();
        let mut resp = client.request(req).await?;

        println!("\nPOST =============================================================");
        println!("{}:\n{:?}\n", urls, resp);
        while let Some(chunk) = resp.body_mut().data().await
        {
            stdout().write_all(&chunk?).await?;
        }
    }
    Ok(())
}

async fn sample_para() -> Result <(), Box<dyn std::error::Error + Send + Sync>>
{
    let client = Client::new();
    let trgt0_fut = async
    {
        println!("\nFUTURES = {} >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>", GETURLS[0]);
        let resp = client.get(Uri::from_static(GETURLS[0])).await?;
        println!("\nFUTURES = {} <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<\n{:?}\n", GETURLS[0], resp);
        hyper::body::to_bytes(resp.into_body()).await
    };

    let trgt1_fut = async
    {
        println!("\nFUTURES = {} >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>", GETURLS[1]);
        let resp = client.get(Uri::from_static(GETURLS[1])).await?;
        println!("\nFUTURES = {} <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<\n{:?}\n", GETURLS[1], resp);
        hyper::body::to_bytes(resp.into_body()).await
    };

    let trgt2_fut = async
    {
        println!("\nFUTURES = {} >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>", GETURLS[2]);
        let resp = client.get(Uri::from_static(GETURLS[2])).await?;
        println!("\nFUTURES = {} <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<\n{:?}\n", GETURLS[2], resp);
        hyper::body::to_bytes(resp.into_body()).await
    };

    let (trgt0, trgt1, trgt2)  = futures::try_join!(trgt0_fut, trgt1_fut, trgt2_fut)?;
    let mut trgt_body = String::from_utf8((trgt0).to_vec()).unwrap();
    println!("\nFUTURES BODY = {}\n{}\n", GETURLS[0], trgt_body);
    trgt_body = String::from_utf8((trgt1).to_vec()).unwrap();
    println!("\nFUTURES BODY = {}\n{}\n", GETURLS[1], trgt_body);
    trgt_body = String::from_utf8((trgt2).to_vec()).unwrap();
    println!("\nFUTURES BODY = {}\n{}\n", GETURLS[2], trgt_body);

    Ok(())
}

async fn sample_https() -> Result <(), Box<dyn std::error::Error + Send + Sync>>
{
    for url in HTTPSURLS
    {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let mut resp = client.get(url.parse()?).await?;
        println!("\nGET HTTPS =======================================================");
        println!("{}:\n{:?}\n", url, resp);
        let strm = resp.body_mut();
        println!("{:?}", strm);
        let okbody = match strm.data().await
            {
                Some(v) => v,
                None => todo!(),
            };
        //println!("{:?}", okbody);
        let body = String::from_utf8(okbody.expect("Error").to_vec()).unwrap();
        println!("{}", body);
    }
    Ok(())
}
