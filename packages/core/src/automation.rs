use anchor_lang::prelude::*;
use std::collections::HashMap;

#[account]
#[derive(Default)]
pub struct Automation {
    pub owner: Pubkey,
    pub workspace: Pubkey,
    pub name: String,
    pub trigger: Trigger,
    pub actions: Vec<Action>,
    pub status: AutomationStatus,
    pub execution_stats: ExecutionStats,
    pub created_at: i64,
    pub last_executed_at: Option<i64>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Trigger {
    pub trigger_type: TriggerType,
    pub conditions: Vec<Condition>,
    pub schedule: Option<Schedule>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TriggerType {
    Price,
    Schedule,
    Balance,
    Custom,
}

impl Default for TriggerType {
    fn default() -> Self {
        TriggerType::Price
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Condition {
    pub condition_type: ConditionType,
    pub parameters: HashMap<String, Vec<u8>>,
    pub last_check: Option<i64>,
    pub last_value: Option<Vec<u8>>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ConditionType {
    PriceAbove,
    PriceBelow,
    BalanceAbove,
    BalanceBelow,
    TimeElapsed,
    Custom,
}

impl Default for ConditionType {
    fn default() -> Self {
        ConditionType::PriceAbove
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Action {
    pub action_type: ActionType,
    pub target: Pubkey,
    pub parameters: HashMap<String, Vec<u8>>,
    pub retry_config: Option<RetryConfig>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ActionType {
    Swap,
    Transfer,
    Stake,
    Unstake,
    Custom,
}

impl Default for ActionType {
    fn default() -> Self {
        ActionType::Swap
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Schedule {
    pub interval: u64,  // in seconds
    pub next_execution: i64,
    pub max_executions: Option<u64>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct RetryConfig {
    pub max_attempts: u8,
    pub delay_between_attempts: u64,  // in seconds
    pub current_attempts: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ExecutionStats {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub last_error: Option<String>,
    pub average_execution_time: Option<u64>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum AutomationStatus {
    Active,
    Paused,
    Failed,
    Completed,
}

impl Default for AutomationStatus {
    fn default() -> Self {
        AutomationStatus::Active
    }
}

impl Automation {
    pub fn space() -> usize {
        8 + // discriminator
        32 + // owner
        32 + // workspace
        4 + 200 + // name
        200 + // trigger
        4 + (10 * 200) + // actions vector
        1 + // status
        100 + // execution stats
        8 + // created_at
        9 + // last_executed_at (Option<i64>)
        1 // bump
    }

    pub fn initialize(
        &mut self,
        owner: Pubkey,
        workspace: Pubkey,
        name: String,
        trigger: Trigger,
        bump: u8,
    ) -> Result<()> {
        require!(name.len() <= 200, AutomationError::NameTooLong);
        
        self.owner = owner;
        self.workspace = workspace;
        self.name = name;
        self.trigger = trigger;
        self.actions = Vec::new();
        self.status = AutomationStatus::Active;
        self.execution_stats = ExecutionStats::default();
        self.created_at = Clock::get()?.unix_timestamp;
        self.bump = bump;

        Ok(())
    }

    pub fn add_action(&mut self, action: Action) -> Result<()> {
        require!(self.actions.len() < 10, AutomationError::TooManyActions);
        self.actions.push(action);
        Ok(())
    }

    pub fn check_conditions(&self) -> Result<bool> {
        for condition in &self.trigger.conditions {
            match condition.condition_type {
                ConditionType::PriceAbove => {
                    // Implement price check logic
                }
                ConditionType::PriceBelow => {
                    // Implement price check logic
                }
                ConditionType::BalanceAbove => {
                    // Implement balance check logic
                }
                ConditionType::BalanceBelow => {
                    // Implement balance check logic
                }
                ConditionType::TimeElapsed => {
                    // Implement time check logic
                }
                ConditionType::Custom => {
                    // Implement custom condition logic
                }
            }
        }
        Ok(true)
    }

    pub fn execute(&mut self) -> Result<()> {
        require!(
            self.status == AutomationStatus::Active,
            AutomationError::AutomationNotActive
        );

        let start_time = Clock::get()?.unix_timestamp;

        for action in &self.actions {
            match action.action_type {
                ActionType::Swap => {
                    // Implement swap logic
                }
                ActionType::Transfer => {
                    // Implement transfer logic
                }
                ActionType::Stake => {
                    // Implement stake logic
                }
                ActionType::Unstake => {
                    // Implement unstake logic
                }
                ActionType::Custom => {
                    // Implement custom action logic
                }
            }
        }

        self.last_executed_at = Some(Clock::get()?.unix_timestamp);
        self.execution_stats.total_executions += 1;
        self.execution_stats.successful_executions += 1;

        // Update average execution time
        let execution_time = Clock::get()?.unix_timestamp - start_time;
        if let Some(avg) = self.execution_stats.average_execution_time {
            self.execution_stats.average_execution_time = Some(
                (avg * (self.execution_stats.total_executions - 1) + execution_time as u64) 
                / self.execution_stats.total_executions
            );
        } else {
            self.execution_stats.average_execution_time = Some(execution_time as u64);
        }

        Ok(())
    }
}

#[error_code]
pub enum AutomationError {
    #[msg("Name must be less than 200 characters")]
    NameTooLong,
    #[msg("Maximum number of actions reached")]
    TooManyActions,
    #[msg("Automation is not active")]
    AutomationNotActive,
}
