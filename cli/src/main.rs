use clap::{Parser, Subcommand};
use foojay_disco::QueryOptions;

const FOOJAY_URL_VAR: &str = "FOOJAY_DISCO_API_URL";

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Pull {
        #[arg(short = 'p', long)]
        print: bool,
        #[arg(long)]
        version: Option<String>,
        #[arg(long)]
        distribution: Option<String>,
        #[arg(long)]
        architecture: Option<String>,
        #[arg(long)]
        archive_type: Option<String>,
        #[arg(long)]
        package_type: Option<String>,
        #[arg(long)]
        operating_system: Option<String>,
        #[arg(long)]
        libc_type: Option<String>,
        #[arg(long)]
        release_status: Option<String>,
        #[arg(long)]
        term_of_support: Option<String>,
        #[arg(long)]
        bitness: Option<String>,
        #[arg(long)]
        javafx_bundled: Option<bool>,
        #[arg(long)]
        directly_downloadable: Option<bool>,
        #[arg(long)]
        latest: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Pull {
            print,
            version,
            distribution,
            architecture,
            archive_type,
            package_type,
            operating_system,
            libc_type,
            release_status,
            term_of_support,
            bitness,
            javafx_bundled,
            directly_downloadable,
            latest,
        } => {
            println!("Pulling packages...");
            
            let packages = foojay_disco::pull(std::env::var_os(FOOJAY_URL_VAR).and_then(|u| u.to_str().map(String::from)), Some(QueryOptions {
                version,
                distribution,
                architecture,
                archive_type,
                package_type,
                operating_system,
                libc_type,
                release_status,
                term_of_support,
                bitness,
                javafx_bundled,
                directly_downloadable,
                latest,
            })).unwrap();

            println!("Total Packages: {}", packages.result.len());
            
            let mut distros = vec![];
            let mut architectures = vec![];
            let mut operating_systems = vec![];
            let mut versions = vec![];

            for p in &packages.result {
                if !distros.contains(&p.distribution) {
                    distros.push(p.distribution.clone());
                }

                if !architectures.contains(&p.architecture) {
                    architectures.push(p.architecture.clone());
                }

                if !operating_systems.contains(&p.operating_system) {
                    operating_systems.push(p.operating_system.clone());
                }

                if !versions.contains(&p.major_version) {
                    versions.push(p.major_version);
                }
            }

            println!("Distributions: {:?}", distros);
            println!("Architectures: {:?}", architectures);
            println!("Operating Systems: {:?}", operating_systems);
            println!("Versions: {:?}", versions);

            if print {
                println!("{:#?}", packages);
            }
        }
    }
}
