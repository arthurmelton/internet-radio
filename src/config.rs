lazy_static! {
    pub static ref ARGS: Args = Args::parse(); 
}

use clap::Parser;

/// A internet radio that you can talk to anyone else
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// Port to use on the internet radio
   #[arg(short, long, default_value_t = 57314)]
   pub port: u16,

   /// Chat channel you want to be on
   #[arg(short, long)]
   pub chat: String,
   
   /// Timeout when trying to find all the peers in ms
   #[arg(short, long, default_value_t = 2000)]
   pub timeout: u64,
   
   /// This is how many ips we want to send requests to in a batch
   #[arg(short, long, default_value_t = 1000000)]
   pub rate: u64,
}
