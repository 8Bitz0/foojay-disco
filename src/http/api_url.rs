use url::Url;

use crate::Error;

#[derive(Debug, Clone)]
pub struct PackageQueryOptions {
    pub version: Option<String>,
    pub distribution: Option<String>,
    pub architecture: Option<String>,
    pub archive_type: Option<String>,
    pub package_type: Option<String>,
    pub operating_system: Option<String>,
    pub libc_type: Option<String>,
    pub release_status: Option<String>,
    pub term_of_support: Option<String>,
    pub bitness: Option<String>,
    pub javafx_bundled: Option<bool>,
    pub directly_downloadable: Option<bool>,
    pub latest: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MajorVersionsQueryOptions {
    pub early_access: Option<bool>,
    pub general_availability: Option<bool>,
    pub maintained: Option<bool>,
}

impl std::fmt::Display for PackageQueryOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut query_opts = vec![];

        // Prepare your eyes.

        if let Some(v) = &self.version {
            query_opts.push(format!("version={}", v));
        }

        if let Some(v) = &self.distribution {
            query_opts.push(format!("distribution={}", v));
        }

        if let Some(v) = &self.architecture {
            query_opts.push(format!("architecture={}", v));
        }

        if let Some(v) = &self.archive_type {
            query_opts.push(format!("archive_type={}", v));
        }

        if let Some(v) = &self.package_type {
            query_opts.push(format!("package_type={}", v));
        }

        if let Some(v) = &self.operating_system {
            query_opts.push(format!("operating_system={}", v));
        }

        if let Some(v) = &self.libc_type {
            query_opts.push(format!("libc_type={}", v));
        }

        if let Some(v) = &self.release_status {
            query_opts.push(format!("release_status={}", v));
        }

        if let Some(v) = &self.term_of_support {
            query_opts.push(format!("term_of_support={}", v));
        }

        if let Some(v) = &self.bitness {
            query_opts.push(format!("bitness={}", v));
        }

        if let Some(v) = &self.javafx_bundled {
            query_opts.push(format!("javafx_bundled={}", v));
        }

        if let Some(v) = &self.directly_downloadable {
            query_opts.push(format!("directly_downloadable={}", v));
        }

        if let Some(v) = &self.latest {
            query_opts.push(format!("latest={}", v));
        }

        let mut used_init_char = false;

        for o in query_opts {
            // The query should start with "?", following options will use "&"
            f.write_str(&format!("{}{}", if used_init_char { "&" } else { "" }, o)).unwrap();

            used_init_char = true;
        }

        Ok(())
    }
}

impl std::fmt::Display for MajorVersionsQueryOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut query_opts = vec![];

        if let Some(v) = &self.early_access {
            query_opts.push(format!("ea={}", v));
        }

        if let Some(v) = &self.general_availability {
            query_opts.push(format!("ga={}", v));
        }

        if let Some(v) = &self.maintained {
            query_opts.push(format!("maintained={}", v));
        }

        let mut used_init_char = false;

        for o in query_opts {
            // The query should start with "?", following options will use "&"
            f.write_str(&format!("{}{}", if used_init_char { "&" } else { "" }, o)).unwrap();

            used_init_char = true;
        }

        Ok(())
    }
}

pub fn create_package_query_url(
    base: impl std::fmt::Display,
    options: Option<PackageQueryOptions>,
) -> Result<String, Error> {
    let mut query_url = Url::parse(base.to_string().as_str())
        .map_err(Error::UrlParse)?;

    query_url = query_url.join("v3.0/").map_err(Error::UrlParse)?;
    query_url = query_url.join("packages").map_err(Error::UrlParse)?;

    let query_string = options.map(|o| o.to_string());
    query_url.set_query(query_string.as_deref());

    Ok(query_url.to_string())
}

pub fn create_major_versions_query_url(
    base: impl std::fmt::Display,
    options: Option<MajorVersionsQueryOptions>,
) -> Result<String, Error> {
    let mut query_url = Url::parse(base.to_string().as_str())
        .map_err(Error::UrlParse)?;

    query_url = query_url.join("v3.0/").map_err(Error::UrlParse)?;
    query_url = query_url.join("major_versions").map_err(Error::UrlParse)?;

    let query_string = options.map(|o| o.to_string());
    query_url.set_query(query_string.as_deref());

    Ok(query_url.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_package_query_url_test() {
        let query_url = create_package_query_url(
            crate::http::API_DEFAULT_URL,
            Some(PackageQueryOptions {
                version: Some("17".to_string()),
                distribution: Some("corretto".to_string()),
                architecture: Some("x86".to_string()),
                archive_type: Some("tar.gz".to_string()),
                bitness: Some("32".to_string()),
                directly_downloadable: Some(true),
                javafx_bundled: Some(false),
                latest: Some("per-distro".to_string()),
                libc_type: Some("glibc".to_string()),
                operating_system: Some("linux".to_string()),
                package_type: Some("jdk".to_string()),
                release_status: Some("ga".to_string()),
                term_of_support: Some("lts".to_string())
            })
        ).unwrap();

        assert_eq!(
            query_url,
            format!("{}v3.0/packages?version=17&distribution=corretto&architecture=x86&archive_type=tar.gz&package_type=jdk&operating_system=linux&libc_type=glibc&release_status=ga&term_of_support=lts&bitness=32&javafx_bundled=false&directly_downloadable=true&latest=per-distro", crate::http::API_DEFAULT_URL),
        )
    }

    #[test]
    fn create_major_versions_query_url_test() {
        let query_url = create_major_versions_query_url(
            crate::http::API_DEFAULT_URL,
            Some(MajorVersionsQueryOptions {
                early_access: Some(true),
                general_availability: Some(true),
                maintained: Some(true),
            })
        ).unwrap();

        assert_eq!(
            query_url,
            format!("{}v3.0/major_versions?ea=true&ga=true&maintained=true", crate::http::API_DEFAULT_URL),
        )
    }
}
 