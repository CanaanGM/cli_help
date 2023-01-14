use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct HelpMeArgs{
    /// the Qustion you would ask
    #[arg(short, long)]
    pub Qustion: String
}