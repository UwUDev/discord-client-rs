#[derive(Debug, Clone)]
pub struct ClientSession {
    pub client_launch_id: uuid::Uuid,
    pub client_heartbeat_session_id: uuid::Uuid,
    pub launch_signature: uuid::Uuid,
    pub heartbeat_expiry: chrono::DateTime<chrono::Utc>,
}

impl ClientSession {
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        Self {
            client_launch_id: uuid::Uuid::new_v4(),
            client_heartbeat_session_id: uuid::Uuid::new_v4(),
            launch_signature: uuid::Uuid::new_v4(),
            heartbeat_expiry: now + chrono::Duration::minutes(30),
        }
    }

    pub fn refresh_heartbeat(&mut self) {
        self.client_heartbeat_session_id = uuid::Uuid::new_v4();
        self.heartbeat_expiry = chrono::Utc::now() + chrono::Duration::minutes(30);
    }
}

#[derive(Debug, Clone)]
pub struct BuildNumbers {
    pub client_build_number: u32,
    pub native_build_number: Option<u32>,
}

impl BuildNumbers {
    pub fn new(client_build_number: u32, native_build_number: Option<u32>) -> Self {
        Self {
            client_build_number,
            native_build_number,
        }
    }
}
