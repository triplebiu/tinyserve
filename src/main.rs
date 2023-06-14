use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use axum::{Extension, Router};
use axum::body::StreamBody;
use axum::extract::{ConnectInfo};
use axum::http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use axum::response::{IntoResponse};
use axum::routing::get;
use clap::{Parser};
use tokio_util::io::ReaderStream;
use tracing_subscriber::{prelude::*, filter::LevelFilter};

#[derive(Clone)]
struct ApiContext {
    header: HashMap<String, String>,
    filepath: PathBuf,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// 监听端口，默认8808
    #[clap(short = 'p',
    value_parser = clap::value_parser!(u16).range(1..65535),
    value_name = "port",
    default_value = "8808",
    next_display_order = 1
    )]
    port: u16,

    /// 需要修改的Header项，如： Content-Type: image/png; （因被转为小写，暂禁用）
    #[clap(short = 'H', value_parser,
    value_name = "headers",
    next_display_order = 2
    )]
    headers: Option<Vec<String>>,

    /// 指定待分享的文件，（暂不支持目录）。
    #[clap(short = 'T', value_name = "target")]
    target: Option<PathBuf>,
}

#[axum_macros::debug_handler]
async fn getfile(ctx: Extension<ApiContext>,
                 ConnectInfo(addr): ConnectInfo<SocketAddr>,
                 axum::extract::Path(reqpath): axum::extract::Path<String>,
    ) -> Result<impl IntoResponse , impl IntoResponse >{

    // println!("{:?}",HeaderMap::try_from(&ctx.header).unwrap_or(HeaderMap::new()));

    let mut headers_cap = HeaderMap::new();

    // HeaderName不接受大写，，，，
    headers_cap.insert(HeaderName::from_static("Content-Type"),HeaderValue::from_static("image/png"));
    // 没处理好。。。。
    // for (K,V) in &ctx.header.clone() {
    //     headers_cap.insert(HeaderName::from_str(K.as_str()),HeaderValue::from_str(V.as_str()).unwrap());
    // }


    let targetpath = Path::new(&ctx.filepath);
    // let filename;
    if targetpath.exists(){
        if targetpath.is_dir() {
            println!("暂时不支持文件夹~");    // 文件夹浏览模式，需要避免路径穿越。
            return Err((StatusCode::INTERNAL_SERVER_ERROR, headers_cap))
        }
        else if targetpath.is_file() {
            println!("{:?} - - GET /{}  target to {}", addr,reqpath, &ctx.filepath.to_string_lossy());

            let file = match tokio::fs::File::open(&ctx.filepath).await {
                Ok(file) => file,
                Err(err) => return Err((StatusCode::NOT_FOUND, headers_cap)),
            };
            let stream = ReaderStream::new(file);

            let body = StreamBody::new(stream);

            // let headers = &ctx.header;

            return Ok((StatusCode::OK, headers_cap, body))
            // return StatusCode::OK
        }
    }
    else {
        tracing::debug!("target file/directory is not exists.");
        return Err((StatusCode::NOT_FOUND, headers_cap))
    }

    Err((StatusCode::INTERNAL_SERVER_ERROR, headers_cap))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(LevelFilter::DEBUG))
        .init();
    tracing::debug!("Command args: {:?}",std::env::args_os());
    let cli: Cli = Cli::parse();
    tracing::debug!("Parsed command args: {:#?}",cli);

    let worktarget = match cli.target{
        Some(target) => {
            if target.exists(){
                target
            }
            else { panic!("target file/directory is not exists.") }
        },
        None => std::env::current_dir().expect("Internal Error..."),
    };

    let mut headers = HashMap::new();
    match cli.headers {
        Some(headerlist) => {
            for header in headerlist {
                let header2:Vec<String> = header.split(":").map(str::to_string).collect();
                headers.insert(header2[0].clone(),header2[1].trim().to_string());
            };
            ()
        },
        None => (),
    }

    // println!("{:?}",headers);
    let bindaddr: String = format!("0.0.0.0:{}", cli.port);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/*reqpath", get(getfile))
        .layer(Extension(ApiContext{
            header: headers,
            filepath: worktarget.clone()
        }));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    tracing::debug!("Serving HTTP on {} , target to {}", bindaddr,worktarget.to_string_lossy());

    axum::Server::bind(&bindaddr.parse().expect("wrong port"))
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("error running HTTP server");

    // let listener = tokio::net::TcpListener::bind(&bindaddr)
    //     .await
    //     .unwrap();
    // axum::serve(listener, app.into_make_service_with_connect_info())
    //     .await
    //     .expect("lanuch service failed.");

}
