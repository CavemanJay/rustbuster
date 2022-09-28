use clap::{Parser, Subcommand};
use reqwest::Url;

#[derive(Subcommand, Debug)]
enum Command {
    Dir {
        #[clap(short = 'f', long)]
        add_slash: bool,
        #[clap(short, long)]
        cookies: Option<String>,
        #[clap(short, long)]
        discover_backup: bool,
        #[clap(long, multiple_values = true)]
        exclude_length: Vec<String>,
        #[clap(short = 'r', long)]
        follow_redirect: bool,
        #[clap(short, long)]
        expanded: bool,
        #[clap(short = 'x', long, multiple_values = true, value_delimiter = ',')]
        extensions: Vec<String>,
        #[clap(
            short = 'H',
            long,
            multiple_values = true,
            help = "Specify HTTP headers, -H 'Header1:val1' -H 'Header2: val2'"
        )]
        headers: Vec<String>,
        #[clap(long)]
        hide_length: bool,
        #[clap(short, long, default_value = "GET")]
        method: String,
        #[clap(short, long)]
        no_status: bool,
        #[clap(short, long, help = "Password for Basic Auth")]
        password: Option<String>,
        #[clap(
            short = 's',
            long,
            multiple_values = true,
            value_delimiter = ',',
            help = "Positive status codes (will be overwritten with status-codes-blacklist if set)"
        )]
        status_codes: Vec<String>,
        #[clap(
            short = 'b',
            long,
            multiple_values = true,
            value_delimiter = ',',
            default_values_t = [404],
            help = "Negative status codes (will override status-codes if set)"
        )]
        status_codes_blacklist: Vec<u32>,
        #[clap(long, default_value = "10")]
        timeout: u32,
        #[clap(short, long,value_parser=UrlParser)]
        url: Url,
        #[clap(short = 'a', long, default_value = "rustbuster")]
        useragent: String,
        #[clap(short = 'U', long, help = "Username for Basic Auth")]
        username: Option<String>,
        #[clap(short, long)]
        wordlist: String,
    },
    Vhost {},
}

#[derive(Parser, Debug)]
pub(crate) struct Args {
    #[clap(subcommand)]
    mode: Command,
    #[clap(long, global = true, default_value = "1500")]
    delay: u64,
    #[clap(short, long, global = true)]
    quiet: bool,
    #[clap(short, long, global = true)]
    verbose: bool,
    #[clap(short, long, global = true, default_value_t = 10)]
    threads: u8,
}

#[derive(Clone, Debug)]
struct UrlParser;

impl clap::builder::TypedValueParser for UrlParser {
    type Value = reqwest::Url;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        match Url::parse(value.to_str().unwrap()) {
            Ok(url) => Ok(url),
            Err(e) => Err(clap::Error::raw(
                clap::ErrorKind::ValueValidation,
                format!("Invalid url: {}", e),
            )),
        }
    }
}
