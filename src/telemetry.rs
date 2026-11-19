use std::collections::{HashMap, HashSet};

use crate::error::DrmError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Active,
    Warning,
    Revoked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnomalyEvent {
    WindowBlur,
    ScreenShareAttempt,
    DevToolsDetected,
    ImpossibleTravel { previous_ip: String, new_ip: String },
}

#[derive(Debug, Clone)]
pub struct SessionRecord {
    pub session_id: String,
    pub user_id: u32,
    pub client_ip: String,
    pub last_heartbeat_secs: u64,
    pub anomaly_score: u32,
    pub state: SessionState,
}

pub struct TelemetryGateway {
    sessions: HashMap<String, SessionRecord>,
    revoked_sessions: HashSet<String>,
}

impl Default for TelemetryGateway {
    fn default() -> Self {
        Self::new()
    }
}

impl TelemetryGateway {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            revoked_sessions: HashSet::new(),
        }
    }

    pub fn register_session(
        &mut self,
        session_id: String,
        user_id: u32,
        client_ip: String,
        now_secs: u64,
    ) -> Result<(), DrmError> {
        // Detect concurrent playback from a different IP for the same user
        let concurrent_conflict = self.sessions.values().any(|s| {
            s.user_id == user_id
                && s.state != SessionState::Revoked
                && s.client_ip != client_ip
                && now_secs.saturating_sub(s.last_heartbeat_secs) <= 15
        });

        if concurrent_conflict {
            self.revoked_sessions.insert(session_id.clone());
            return Err(DrmError::SessionRevoked(format!(
                "concurrent session anomaly for user {user_id} from {client_ip}"
            )));
        }

        self.sessions.insert(
            session_id.clone(),
            SessionRecord {
                session_id,
                user_id,
                client_ip,
                last_heartbeat_secs: now_secs,
                anomaly_score: 0,
                state: SessionState::Active,
            },
        );
        Ok(())
    }

    pub fn record_heartbeat(
        &mut self,
        session_id: &str,
        client_ip: &str,
        now_secs: u64,
    ) -> Result<SessionState, DrmError> {
        if self.revoked_sessions.contains(session_id) {
            return Err(DrmError::SessionRevoked(session_id.to_string()));
        }

        let record = self
            .sessions
            .get_mut(session_id)
            .ok_or_else(|| DrmError::Unauthorized(format!("unknown session {session_id}")))?;

        if record.client_ip != client_ip {
            record.state = SessionState::Revoked;
            record.anomaly_score = 100;
            self.revoked_sessions.insert(session_id.to_string());
            return Err(DrmError::SessionRevoked(format!(
                "IP mismatch on session {session_id}"
            )));
        }

        record.last_heartbeat_secs = now_secs;
        Ok(record.state)
    }

    pub fn report_anomaly(
        &mut self,
        session_id: &str,
        event: AnomalyEvent,
    ) -> Result<SessionState, DrmError> {
        let record = self
            .sessions
            .get_mut(session_id)
            .ok_or_else(|| DrmError::Unauthorized(format!("unknown session {session_id}")))?;

        let penalty = match event {
            AnomalyEvent::WindowBlur => 20,
            AnomalyEvent::DevToolsDetected => 100,
            AnomalyEvent::ScreenShareAttempt => 100,
            AnomalyEvent::ImpossibleTravel { .. } => 100,
        };

        record.anomaly_score = record.anomaly_score.saturating_add(penalty);
        if record.anomaly_score >= 100 {
            record.state = SessionState::Revoked;
            self.revoked_sessions.insert(session_id.to_string());
        } else if record.anomaly_score >= 40 {
            record.state = SessionState::Warning;
        }
        Ok(record.state)
    }

    pub fn is_revoked(&self, session_id: &str) -> bool {
        self.revoked_sessions.contains(session_id)
    }
}
