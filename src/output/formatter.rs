use chrono::DateTime;
use clap::ValueEnum;
use serde::Serialize;
use somfy_sdk::commands::execute_action_group::ExecuteActionGroupResponse;
use somfy_sdk::commands::get_devices::GetDevicesResponse;
use somfy_sdk::commands::get_execution::GetExecutionResponse;
use somfy_sdk::commands::types::{DeviceState, DeviceStateValue};
use std::collections::HashMap;
use tabled::builder::Builder;
use tabled::settings::object::{Columns, Rows};
use tabled::settings::{Alignment, Panel, Style};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputStyle {
    Json,
    Table,
}

pub trait CliOutput: Serialize {
    fn to_styled_cli_output(&self, style: OutputStyle) -> anyhow::Result<String> {
        match style {
            OutputStyle::Json => self.to_json(),
            OutputStyle::Table => self.to_table(),
        }
    }
    fn to_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(&self)?)
    }
    fn to_table(&self) -> anyhow::Result<String>;
}

impl CliOutput for ExecuteActionGroupResponse {
    fn to_table(&self) -> anyhow::Result<String> {
        let mut builder = Builder::new();
        builder.push_record(["Id", self.exec_id.as_str()]);
        let mut table = builder.build();
        table.with(Panel::header("Execution Result"));

        let str = table.with(Style::modern_rounded()).to_string();
        Ok(str)
    }
}

impl CliOutput for GetExecutionResponse {
    fn to_table(&self) -> anyhow::Result<String> {
        let dt = DateTime::from_timestamp_millis(self.start_time);
        let timestamp = dt
            .map(|f| f.to_string())
            .unwrap_or(self.start_time.to_string());

        let mut builder = Builder::new();
        builder.push_record(["Id", self.id.as_str()]);
        builder.push_record(["Description", self.description.as_str()]);
        builder.push_record(["State", self.state.as_str()]);
        builder.push_record(["Execution Type", self.execution_type.as_str()]);
        builder.push_record(["Execution Subtype", self.execution_sub_type.as_str()]);
        builder.push_record(["Owner", self.owner.as_str()]);
        builder.push_record(["Start time", timestamp.as_str()]);

        let mut table = builder.build();
        table.with(Panel::header("Execution Result"));
        table.modify(Columns::first(), Alignment::right());
        table.modify(Rows::first(), Alignment::left());

        let str = table.with(Style::modern_rounded()).to_string();
        Ok(str)
    }
}

pub trait HumanFriendly {
    fn to_human_friendly_string(&self) -> String;
}

pub trait Searchable<T> {
    fn find_by_name(&self, name: &str) -> Option<T>;
    fn value_from_name(&self, name: &str) -> String;
}

impl Searchable<DeviceState> for Vec<DeviceState> {
    fn find_by_name(&self, name: &str) -> Option<DeviceState> {
        self.iter()
            .find(|ds: &&DeviceState| ds.name == name)
            .cloned()
    }

    fn value_from_name(&self, name: &str) -> String {
        let val = self
            .find_by_name(name)
            .map(|v| v.value.to_human_friendly_string())
            .unwrap_or_default();

        format!("{val:>3}")
    }
}

impl HumanFriendly for DeviceStateValue {
    fn to_human_friendly_string(&self) -> String {
        match self {
            DeviceStateValue::String(s) => s.clone(),
            DeviceStateValue::Int(i) => i.to_string(),
            DeviceStateValue::Map(_) => "Map".to_string(),
            DeviceStateValue::Array(_) => "Array".to_string(),
            DeviceStateValue::Boolean(b) => b.to_string(),
        }
    }
}

impl CliOutput for GetDevicesResponse {
    fn to_table(&self) -> anyhow::Result<String> {
        let mut builder = Builder::new();
        builder.push_record([
            "Label",
            "Device URL",
            "Device Type",
            "Open/Close",
            "Status",
            "Closure (%)",
            "Tilt (%)",
            "'My' position (%)",
            "'My' tilt (%)",
            "Is Moving?",
        ]);

        for device in self {
            builder.push_record([
                &device.label,
                &device.device_url,
                &device.controllable_name,
                device
                    .states
                    .value_from_name("core:OpenClosedState")
                    .as_str(),
                device.states.value_from_name("core:StatusState").as_str(),
                device.states.value_from_name("core:ClosureState").as_str(),
                device
                    .states
                    .value_from_name("core:SlateOrientationState")
                    .as_str(),
                device
                    .states
                    .value_from_name("core:Memorized1PositionState")
                    .as_str(),
                device
                    .states
                    .value_from_name("core:Memorized1OrientationState")
                    .as_str(),
                device.states.value_from_name("core:MovingState").as_str(),
            ]);
        }

        let mut table = builder.build();
        // table.modify(Rows::first(), Color::rgb_bg(211,211,211));
        // table.modify(Rows::first(), Justification::colored(' ', Color::rgb_bg(211,211,211)));
        table.modify(Columns::new(5..=8), Alignment::right());

        let str = table.with(Style::sharp()).to_string();
        Ok(str)
    }
}

impl CliOutput for HashMap<String, String> {
    fn to_table(&self) -> anyhow::Result<String> {
        let builder = Builder::from(self.clone());
        let mut table = builder.build();
        let str = table.with(Style::modern_rounded()).to_string();
        Ok(str)
    }
}
