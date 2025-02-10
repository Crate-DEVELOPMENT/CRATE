use anchor_lang::prelude::*;
use std::collections::HashMap;

#[account]
#[derive(Default)]
pub struct Workspace {
    pub owner: Pubkey,
    pub name: String,
    pub description: Option<String>,
    pub apps: Vec<ConnectedApp>,
    pub automations: Vec<Pubkey>,
    pub stats: WorkspaceStats,
    pub settings: WorkspaceSettings,
    pub created_at: i64,
    pub updated_at: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ConnectedApp {
    pub id: String,
    pub app_type: AppType,
    pub config: HashMap<String, Vec<u8>>,
    pub connected_at: i64,
    pub last_used: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct WorkspaceStats {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub total_value_locked: u64,
    pub last_execution_time: Option<i64>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct WorkspaceSettings {
    pub max_automations: u8,
    pub auto_retry: bool,
    pub notification_settings: NotificationSettings,
    pub risk_level: RiskLevel,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct NotificationSettings {
    pub email_enabled: bool,
    pub discord_enabled: bool,
    pub telegram_enabled: bool,
    pub notification_types: Vec<NotificationType>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum AppType {
    Dex,
    PriceFeed,
    Lending,
    Yield,
    Custom,
}

impl Default for AppType {
    fn default() -> Self {
        AppType::Custom
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum NotificationType {
    ExecutionSuccess,
    ExecutionFailure,
    ConditionMet,
    LowBalance,
    PriceAlert,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

impl Default for RiskLevel {
    fn default() -> Self {
        RiskLevel::Medium
    }
}

impl Workspace {
    pub fn space() -> usize {
        // Calculate space required for the workspace account
        8 + // discriminator
        32 + // owner
        4 + 200 + // name (string with max 200 chars)
        1 + 4 + 200 + // optional description
        4 + 10 * (32 + 8 + 8) + // apps vector
        4 + 50 * 32 + // automations vector (pubkeys)
        8 * 5 + // stats
        1 + 1 + 1 + 1 + // settings
        8 + // created_at
        8 + // updated_at
        1 // bump
    }

    pub fn initialize(
        &mut self,
        owner: Pubkey,
        name: String,
        description: Option<String>,
        bump: u8,
    ) -> Result<()> {
        require!(name.len() <= 200, ErrorCode::NameTooLong);
        if let Some(desc) = &description {
            require!(desc.len() <= 200, ErrorCode::DescriptionTooLong);
        }

        self.owner = owner;
        self.name = name;
        self.description = description;
        self.apps = Vec::new();
        self.automations = Vec::new();
        self.stats = WorkspaceStats::default();
        self.settings = WorkspaceSettings {
            max_automations: 10,
            auto_retry: true,
            notification_settings: NotificationSettings::default(),
            risk_level: RiskLevel::Medium,
        };
        self.created_at = Clock::get()?.unix_timestamp;
        self.updated_at = self.created_at;
        self.bump = bump;

        Ok(())
    }

    pub fn add_app(&mut self, app: ConnectedApp) -> Result<()> {
        require!(self.apps.len() < 10, ErrorCode::TooManyApps);
        self.apps.push(app);
        self.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn add_automation(&mut self, automation_pubkey: Pubkey) -> Result<()> {
        require!(
            self.automations.len() < self.settings.max_automations as usize,
            ErrorCode::TooManyAutomations
        );
        self.automations.push(automation_pubkey);
        self.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn update_stats(&mut self, execution_success: bool) -> Result<()> {
        self.stats.total_executions += 1;
        if execution_success {
            self.stats.successful_executions += 1;
        } else {
            self.stats.failed_executions += 1;
        }
        self.stats.last_execution_time = Some(Clock::get()?.unix_timestamp);
        self.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Name must be less than 200 characters")]
    NameTooLong,
    #[msg("Description must be less than 200 characters")]
    DescriptionTooLong,
    #[msg("Maximum number of apps reached")]
    TooManyApps,
    #[msg("Maximum number of automations reached")]
    TooManyAutomations,
}
