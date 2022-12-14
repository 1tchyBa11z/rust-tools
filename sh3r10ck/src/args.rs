use clap:: { Parser };

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ArgList{
    /// Target IP or (F)ile containing multiple IP addresses
    #[arg(short,long, value_name = "IP|FILE", default_value="default_target")]
    pub target: String,
    /// Target ports to scan (Defaults to top 1000 most common ports )
    #[arg(short,long, default_value="default_port")]
    pub port: String,
    /// (T)CP or (U)DP or (Q)uic Scan
    #[arg(short,long, default_value="default_scantype")]
    pub scantype: String,
}
