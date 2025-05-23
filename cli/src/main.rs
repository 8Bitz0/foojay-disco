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
    Packages {
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
    PackageInfo {
        package: String,
        #[arg(short = 'p', long)]
        print: bool,
    },
    MajorVersions {
        #[arg(short = 'p', long)]
        print: bool,
        #[arg(long)]
        early_access: Option<bool>,
        #[arg(long)]
        general_availability: Option<bool>,
        #[arg(long)]
        maintained: Option<bool>,
    },
    Distributions {
        #[arg(short = 'p', long)]
        print: bool,
    },
    DistributionInfo {
        distribution: String,
        #[arg(short = 'p', long)]
        print: bool,
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Packages {
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
        Command::PackageInfo {
            package,
            print,
        } => {
            println!("Pulling package info...");
            
            let package = foojay_disco::pull_package_info(std::env::var_os(FOOJAY_URL_VAR).and_then(|u| u.to_str().map(String::from)), package).unwrap();

            println!("Filename: {}", package.result[0].filename);
            println!("Direct Download URI: {}", package.result[0].direct_download_uri);
            println!("Download Site URI: {}", package.result[0].download_site_uri);
            println!("Signature URI: {}", package.result[0].signature_uri);
            println!("Checksum URI: {}", package.result[0].checksum_uri);
            println!("Checksum: {}", package.result[0].checksum);
            println!("Checksum Type: {}", package.result[0].checksum_type);

            if print {
                println!("{:#?}", package);
            }
        }
        Command::MajorVersions {
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
                    major_version_names.push(v.major_version);
                }
            }

            println!("Major Versions: {:?}", major_version_names);

            if print {
                println!("{:#?}", major_versions);
            }
        }
        Command::Distributions {
            print,
        } => {
            println!("Pulling distributions...");

            let distributions = foojay_disco::pull_distributions(std::env::var_os(FOOJAY_URL_VAR).and_then(|u| u.to_str().map(String::from))).unwrap();

            let mut distribution_names = vec![];

            for v in &distributions.result {
                if !distribution_names.contains(&v.name) {
                    distribution_names.push(v.name.clone());
                }
            }

            println!("Distribution Names: {:?}", distribution_names);

            if print {
                println!("{:#?}", distributions);
            }
        }
        Command::DistributionInfo {
            distribution,
            print,
        } => {
            println!("Pulling distribution info...");

            let distribution_info = foojay_disco::pull_distribution_info(
                std::env::var_os(FOOJAY_URL_VAR).and_then(|u| u.to_str().map(String::from)),
                distribution,
            ).unwrap();

            println!("Distribution Name: {:?}", distribution_info.result[0].name);
            println!("Maintained: {:?}", distribution_info.result[0].maintained);
            println!("Available: {:?}", distribution_info.result[0].available);
            println!("Build of OpenJDK: {:?}", distribution_info.result[0].build_of_openjdk);
            println!("Build of GraalVM: {:?}", distribution_info.result[0].build_of_graalvm);
            println!("Official URL: {:?}", distribution_info.result[0].official_uri);
            println!("Versions: {:?}", distribution_info.result[0].versions);

            if print {
                println!("{:#?}", distribution_info);
            }
        }
    }
}
