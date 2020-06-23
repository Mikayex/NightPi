use serde::Deserialize;

fn user_from_env() -> Option<String> {
    match std::env::var("SUDO_UID") {
        Ok(uid_string) => {
            let uid = uid_string.parse::<users::uid_t>().unwrap();
            match users::get_user_by_uid(uid) {
                Some(user) => Some(user.name().to_str()?.into()),
                None => None,
            }
        }
        Err(_) => None,
    }
}

fn group_from_env() -> Option<String> {
    match std::env::var("SUDO_GID") {
        Ok(gid_string) => {
            let gid = gid_string.parse::<users::gid_t>().unwrap();
            match users::get_group_by_gid(gid) {
                Some(group) => Some(group.name().to_str()?.into()),
                None => None,
            }
        }
        Err(_) => None,
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    pub server: Server,
    pub user: Option<String>,
    pub group: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        //Try to get user and group from environment variables
        let user = user_from_env();
        let group = group_from_env();

        Self {
            server: Server::default(),
            user,
            group,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Server {
    pub ip: String,
    pub port: u16,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            ip: "0.0.0.0".to_string(),
            port: 8080,
        }
    }
}
