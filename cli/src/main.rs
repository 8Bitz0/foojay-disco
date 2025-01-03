use clap::{Parser, Subcommand};
use foojay_disco::{PackageQueryOptions, MajorVersionsQueryOptions};

const FOOJAY_URL_VAR: &str = "FOOJAY_DISCO_API_URL";

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    PullPackages {
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
    PullMajorVersions {
        #[arg(short = 'p', long)]
        print: bool,
        #[arg(long)]
        early_access: Option<bool>,
        #[arg(long)]
        general_availability: Option<bool>,
        #[arg(long)]
        maintained: Option<bool>,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::PullPackages {
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
            
            let packages = foojay_disco::pull_packages(std::env::var_os(FOOJAY_URL_VAR).and_then(|u| u.to_str().map(String::from)), Some(PackageQueryOptions {
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
        Command::PullMajorVersions {
            print,
            early_access,
            general_availability,
            maintained,
        } => {
            println!("Pulling major versions...");

            let major_versions = foojay_disco::pull_major_versions(std::env::var_os(FOOJAY_URL_VAR).and_then(|u| u.to_str().map(String::from)), Some(MajorVersionsQueryOptions {
                early_access,
                general_availability,
                maintained,
            })).unwrap();

            let mut major_version_names = vec![];

            for v in &major_versions.result {
                if !major_version_names.contains(&v.major_version) {
                    major_version_names.push(v.major_version)
                }
            }

            println!("Major Versions: {:?}", major_version_names);

            if print {
                println!("{:#?}", major_versions);
            }
        }
    }
}
