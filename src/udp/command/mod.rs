use super::{
    ApiError, Result, UdpClient, client,
    response::UdpResponse,
    status::{ErrorCode, StatusCode},
    util::check_status,
};
use ecow::EcoString;
use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    net::SocketAddr,
    ops::{Deref, DerefMut},
};
use tracing::{instrument, warn};

pub(crate) struct CommandBuilder<'a> {
    cmd: String,
    map: HashMap<Cow<'a, str>, String>,
}
impl<'a> CommandBuilder<'a> {
    pub fn build(&self) -> String {
        let map = self.iter().map(|(k, v)| format!("{k}={v}"));
        let params = itertools::intersperse(map, "&".to_string()).collect::<String>();
        format!("{} {}", self.cmd, params)
    }

    pub fn new<K, V, const N: usize>(cmd: String, arr: [(K, V); N]) -> Self
    where
        K: Into<Cow<'a, str>> + Eq + Hash,
        V: Into<String>,
    {
        let map = arr
            .into_iter()
            .map(|(a, b)| (a.into(), b.into()))
            .collect::<HashMap<Cow<'a, str>, String>>();
        Self { cmd, map }
    }
}
impl<'a> Deref for CommandBuilder<'a> {
    type Target = HashMap<Cow<'a, str>, String>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}
impl DerefMut for CommandBuilder<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

pub(crate) trait Command {
    fn build_command(&self, client: &UdpClient) -> CommandBuilder;

    #[instrument(skip(self, client), level = "debug")]
    async fn request(&self, client: &mut UdpClient, command: &str) -> Result<UdpResponse> {
        let response = client.request(command.as_bytes()).await;
        if let Ok(resp) = &response {
            let error_code = ErrorCode::try_from(resp.status);
            match error_code {
                // Need reauth
                Ok(ErrorCode::LoginFirst | ErrorCode::InvalidSession) => {
                    warn!("Server return {}, trying to reauthenticate", resp.status);
                    client.reset();
                    client.execute::<UdpResponse>(self::Auth()).await?;
                    let command = self.build_command(client).build();
                    return client.request(command.as_bytes()).await;
                }
                _ => return response,
            }
        } else if let Err(error) = &response {
            if error == &ApiError::Decrypt {
                warn!(
                    "Cannot decrypt server message. Assume auth timeout, trying to reauthenticate"
                );
                client.reset();
                client.execute::<UdpResponse>(self::Encrypt()).await?;
                client.execute::<UdpResponse>(self::Auth()).await?;
                let command = self.build_command(client).build();
                return client.request(command.as_bytes()).await;
            }
        }
        response
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn execute<T: From<UdpResponse> + Debug>(&self, client: &mut UdpClient) -> Result<T> {
        if client.session_key.is_none() {
            return Err(ApiError::NotLoggedIn);
        }
        let command = self.build_command(client).build();
        self.request(client, &command).await.map(Into::into)
    }
}

// !TODO: maybe convert this enum to enum CommandImpl<CommandTrait>
// #[derive(Debug)]
// pub enum Command<T>
// where
//     T: CommandTrait,
// {
//     Auth(T),
//     Anime(T),
// }

#[derive(Debug)]
pub struct Encrypt();
impl Command for Encrypt {
    fn build_command(&self, client: &UdpClient) -> CommandBuilder {
        let username = &client.config.username;
        let params: [(&str, &str); 2] = [("user", username), ("type", "1")];
        CommandBuilder::new("ENCRYPT".to_string(), params)
    }
    async fn request(&self, client: &mut UdpClient, command: &str) -> Result<UdpResponse> {
        client.request(command.as_bytes()).await
    }
    async fn execute<T: From<UdpResponse>>(&self, client: &mut UdpClient) -> Result<T> {
        if client.config.udp_api_key.is_none() {
            return Err(ApiError::NoApiKey);
        }
        let command = self.build_command(client).build();
        let resp = self.request(client, &command).await?;
        if resp.status == StatusCode::EncryptionEnabled {
            let mut salt = resp.message.split_whitespace();
            let salt = salt.next().ok_or(ApiError::UnexpectedResponse)?;
            client.calculate_enc_key(salt)?;
            client.enc_enabled = true;
            Ok(resp.into())
        } else {
            Err(check_status(resp.status))
        }
    }
}

#[derive(Debug)]
pub struct Auth();
impl Command for Auth {
    fn build_command(&self, client: &UdpClient) -> CommandBuilder {
        let params: [(&str, &str); 9] = [
            ("user", &client.config.username),
            ("pass", &client.config.password),
            ("protover", client::ANIDB_API_VER),
            ("client", &client.config.client_name),
            ("clientver", &client.config.client_ver),
            ("nat", "1"),
            ("comp", "1"),
            ("imgserver", "1"),
            ("enc", "utf8"),
        ];
        CommandBuilder::new("AUTH".to_string(), params)
    }
    async fn request(&self, client: &mut UdpClient, command: &str) -> Result<UdpResponse> {
        client.request(command.as_bytes()).await
    }
    async fn execute<T: From<UdpResponse>>(&self, client: &mut UdpClient) -> Result<T> {
        let command = self.build_command(client).build();
        let resp = self.request(client, &command).await?;
        // Check if login successful
        if resp.status == StatusCode::LoginFailed {
            client.reset();
            return Err(ApiError::IncorrectUsernameOrPassword);
        }

        if resp.status != StatusCode::LoginAccepted
            && resp.status != StatusCode::LoginAcceptedNewVersion
        {
            return Err(check_status(resp.status));
        }

        let mut header = resp.message.split_whitespace();
        // Session key
        let session_key = header.next().ok_or(ApiError::UnexpectedResponse)?;
        client.session_key = Some(session_key.into());
        client.authenticated = true;
        // Parse ipv4:port
        let ip_str = header.next().ok_or(ApiError::UnexpectedResponse)?;
        let addr = ip_str
            .parse::<SocketAddr>()
            .map_err(|_| ApiError::ParseError("Invalid socket address syntax".to_string()))?;
        if addr.port() != client.config.local_port {
            client.behind_nat = true;
        }

        Ok(resp.into())
    }
}

#[derive(Debug)]
pub struct Logout();
impl Command for Logout {
    fn build_command(&self, client: &UdpClient) -> CommandBuilder {
        let s = client.session_key.clone().unwrap_or_default();
        let params: [(&str, &str); 1] = [("s", &s)];
        CommandBuilder::new("LOGOUT".to_string(), params)
    }
    async fn execute<T: From<UdpResponse>>(&self, client: &mut UdpClient) -> Result<T> {
        if client.session_key.is_none() {
            return Err(ApiError::NotLoggedIn);
        }
        let command = self.build_command(client).build();
        let resp = self.request(client, &command).await?;

        if resp.status == StatusCode::LoggedOut {
            client.reset();
            Ok(resp.into())
        } else {
            Err(check_status(resp.status))
        }
    }
}

#[derive(Debug)]
pub struct Ping();
impl Command for Ping {
    fn build_command(&self, _client: &UdpClient) -> CommandBuilder {
        CommandBuilder::new("PING".to_string(), [("", "")])
    }
    async fn execute<T: From<UdpResponse>>(&self, client: &mut UdpClient) -> Result<T> {
        let command = self.build_command(client).build();
        let resp = self.request(client, &command).await?;
        if resp.status == StatusCode::Pong {
            Ok(resp.into())
        } else {
            Err(check_status(resp.status))
        }
    }
}

#[derive(Debug)]
pub struct Anime(pub String);
impl Command for Anime {
    fn build_command(&self, client: &UdpClient) -> CommandBuilder {
        let s = client.session_key.clone().unwrap_or_default();
        let params: [(&str, &str); 2] = [("aid", &self.0), ("s", &s)];
        CommandBuilder::new("ANIME".to_string(), params)
    }
}

#[derive(Debug)]
pub struct AnimeDescription(pub String);
impl Command for AnimeDescription {
    fn build_command(&self, client: &UdpClient) -> CommandBuilder {
        let s = client.session_key.clone().unwrap_or_default();
        let params: [(&str, &str); 3] = [("aid", &self.0), ("s", &s), ("part", "0")];
        CommandBuilder::new("ANIMEDESC".to_string(), params)
    }
    #[instrument(skip(client), level = "debug")]
    async fn execute<T: From<UdpResponse> + Debug>(&self, client: &mut UdpClient) -> Result<T> {
        fn parse_data(data: &[String]) -> Result<(u32, u32, EcoString)> {
            let data = data.first().ok_or(ApiError::UnexpectedResponse)?;
            let mut data_iter = data.split_terminator('|');
            let current = data_iter
                .next()
                .ok_or(ApiError::UnexpectedResponse)?
                .parse::<u32>()
                .map_err(|err| {
                    ApiError::ParseError(format!("Failed conversion to u32: {:?}", err.kind()))
                })?;
            let max = data_iter
                .next()
                .ok_or(ApiError::UnexpectedResponse)?
                .parse::<u32>()
                .map_err(|err| {
                    ApiError::ParseError(format!("Failed conversion to u32: {:?}", err.kind()))
                })?;
            let description = data_iter.next().ok_or(ApiError::UnexpectedResponse)?;
            Ok((current, max, description.into()))
        }

        if client.session_key.is_none() {
            return Err(ApiError::NotLoggedIn);
        }
        let mut command = self.build_command(client);
        let mut response = self.request(client, &command.build()).await?;
        if response.status != StatusCode::AnimeDescription {
            return Ok(response.into());
        }
        // Parse index and description
        let (mut current, mut max, mut description) = parse_data(&response.data)?;
        while current + 1 < max {
            command.insert("part".into(), (current + 1).to_string());
            let new_command = self.build_command(client).build();
            let new_response = self.request(client, &new_command).await?;
            let (new_current, new_max, new_description) = parse_data(&new_response.data)?;
            current = new_current;
            max = new_max;
            description.push_str(&new_description);
        }
        response.data = vec![description.into()];
        Ok(response.into())
    }
}

#[cfg(test)]
mod test_command {
    use super::*;

    #[test]
    fn test_command_builder() {
        let params = [("s", "token"), ("aid", "1234")];
        let command_builder = CommandBuilder::new("ANIME".to_string(), params);
        let command_str = command_builder.build();
        let success =
            command_str.eq("ANIME s=token&aid=1234") || command_str.eq("ANIME aid=1234&s=token");
        assert!(success);
    }
}
