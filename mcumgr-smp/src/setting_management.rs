// Author: Sascha Zenglein <zenglein@gessler.de>
// Copyright (c) 2023 Gessler GmbH.
use crate::{Group, SmpFrame};

use crate::OpCode::{ReadRequest, WriteRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadSettingRequest {
    pub name: String,
}

pub fn read_setting(sequence: u8, name: String) -> SmpFrame<ReadSettingRequest> {
    let payload = ReadSettingRequest { name };

    SmpFrame::new(ReadRequest, sequence, Group::SettingManagement, 0, payload)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ReadSettingResult {
    Ok {
        #[serde(with = "serde_bytes")]
        val: Vec<u8>,
    },
    Err {
        rc: i32,
    },
}

impl ReadSettingResult {
    pub fn into_result(self) -> Result<Vec<u8>, i32> {
        match self {
            ReadSettingResult::Ok { val } => Ok(val),
            ReadSettingResult::Err { rc } => Err(rc),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WriteSettingRequest {
    pub name: String,
    #[serde(with = "serde_bytes")]
    pub val: Vec<u8>,
}

pub fn write_setting(sequence: u8, name: String, val: Vec<u8>) -> SmpFrame<WriteSettingRequest> {
    let payload = WriteSettingRequest { name, val };

    SmpFrame::new(WriteRequest, sequence, Group::SettingManagement, 0, payload)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WriteSettingResult {
    Ok {},
    Err { rc: i32 },
}

impl WriteSettingResult {
    pub fn into_result(self) -> Result<(), i32> {
        match self {
            WriteSettingResult::Ok {} => Ok(()),
            WriteSettingResult::Err { rc } => Err(rc),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveSettingRequest {}

pub fn save_setting(sequence: u8) -> SmpFrame<SaveSettingRequest> {
    let payload = SaveSettingRequest {};

    SmpFrame::new(WriteRequest, sequence, Group::SettingManagement, 3, payload)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SaveSettingResult {
    Ok {},
    Err { rc: i32 },
}

impl SaveSettingResult {
    pub fn into_result(self) -> Result<(), i32> {
        match self {
            SaveSettingResult::Ok {} => Ok(()),
            SaveSettingResult::Err { rc } => Err(rc),
        }
    }
}
