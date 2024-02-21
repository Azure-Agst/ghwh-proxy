use std::env;
use std::path::Path;
use std::net::SocketAddr;
use std::process::ExitCode;

use url::Url;
use hyper::header;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper::{Method, StatusCode};
use http_body_util::{BodyExt, Empty, Full};
use hyper_util::{client::legacy::Client, rt::TokioExecutor, rt::TokioIo};
use hyper_tls::HttpsConnector;
use tokio::net::TcpListener;

mod models;
use models::ghost::GhostWebhook;
use models::discord::{DiscordWebhook, DiscordAuthor, 
    DiscordEmbed, DiscordFooter, DiscordImage};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

fn empty() -> BoxBody {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

async fn router(
    req: Request<hyper::body::Incoming>
) -> Result<Response<BoxBody>> {

    let reqpath = req.uri().path();
    let dirs: Vec<_> = Path::new(reqpath).components().collect();
    let basedir = match dirs.len() {
        1 => "",
        _ => dirs[1].as_os_str().to_str().unwrap()
    };

    match (req.method(), basedir) {

        (&Method::GET, "") => {
            let res = Response::builder()
                .status(StatusCode::OK)
                .body(full("Hi! Try POSTing to /discord!\n"))?;
            Ok(res)
        }

        (&Method::POST, "discord") => {

            // ensure its an actual webhook
            if dirs.len() < 4 {
                let res = Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(full("Bad request.\n"))?;
                return Ok(res);
            }

            // formulate dest_url before consumption
            let dest_url = reqpath.replace("/discord", "https://discord.com/api/webhooks");
            println!("INFO: Proxying request to {}", dest_url);

            // parse body
            let body = req.collect().await?.to_bytes();
            let data: GhostWebhook = serde_json::from_slice(&body)?;

            // other useful vars
            let post_url = Url::parse(&data.post.current.url)?;
            let host = post_url.host_str().expect("Failed parsing host");

            // format discord webhook
            let payload = DiscordWebhook {
                content: format!("A new post has just been published! :tada:").to_string(),
                embeds: Vec::from([
                    DiscordEmbed {
                        author: DiscordAuthor{
                            name: host.to_string()
                        },
                        title: data.post.current.title,
                        url: data.post.current.url,
                        description: data.post.current.excerpt.expect("No exerpt"),
                        image: match data.post.current.feature_image {
                            Some(imgurl) => DiscordImage {
                                url: imgurl
                            },
                            None => DiscordImage {
                                url: "".to_string()
                            }
                        },
                        footer: DiscordFooter {
                            text: format!("{}", data.post.current.primary_author.name).to_string(),
                            icon_url: match data.post.current.primary_author.profile_image {
                                Some(pfpurl) => pfpurl,
                                None => "".to_string()
                            }
                        },
                        timestamp: data.post.current.published_at.expect("Not published")
                    }
                ])
            };

            // serialize webhook
            let reqbody = serde_json::to_string(&payload)?;

            // format upstream request
            let req = Request::builder()
                .method(Method::POST)
                .uri(dest_url)
                .header(header::CONTENT_TYPE, "application/json")
                .body(full(reqbody))?;
            
            // send upstream request
            let https = HttpsConnector::new();
            let client = Client::builder(TokioExecutor::new())
                .build::<_, BoxBody>(https);
            let result = client.request(req).await?;

            // respond
            let res = Response::builder()
                .status(result.status())
                .body(full(result.collect().await?.to_bytes()))?;
            Ok(res)
        },
        
        _ => {
            Ok(
                Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(empty())
                .unwrap()
            )
        }
    }
}

#[tokio::main]
async fn main() -> Result<ExitCode> {

    // parse args
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("ERROR: Please specify a port to bind to.\ne.g. '{} 7120')", &args[0]);
        return Ok(ExitCode::from(1));
    }
    
    // Init server
    let port = &args[1].parse::<u16>().unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], *port));
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    // Main loop
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(router))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}