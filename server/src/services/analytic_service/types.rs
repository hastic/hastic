use std::{fmt, sync::Arc};

use crate::services::segments_service::Segment;

use super::analytic_unit::{
    pattern_analytic_unit::{self},
    types::AnalyticUnitConfig,
};

use super::analytic_unit::types::PatchConfig;

use anyhow::Result;
use serde::Serialize;
use tokio::sync::{RwLock, oneshot};

use crate::services::analytic_service::analytic_unit::types::AnalyticUnit;

pub type AnalyticUnitRF = Arc<RwLock<Box<dyn AnalyticUnit + Send + Sync>>>;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum LearningStatus {
    Initialization,
    Starting,
    Learning,
    Error,
    Ready,
}

// TODO: move to analytic_unit config of pattern detector
#[derive(Clone, Serialize, Debug)]
pub struct LearningTrain {
    pub features: Vec<pattern_analytic_unit::Features>,
    pub target: Vec<bool>,
}

impl Default for LearningTrain {
    fn default() -> Self {
        return LearningTrain {
            features: Vec::new(),
            target: Vec::new(),
        };
    }
}

pub enum ResponseType {
    LearningStarted,
    LearningFinished(Box<dyn AnalyticUnit + Send + Sync>),
    LearningFinishedEmpty,
    LearningDatasourceError,
}

impl fmt::Debug for ResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: implement
        f.debug_tuple("foo").finish()
    }
}

#[derive(Debug)]
pub struct DetectionTask {
    pub sender: oneshot::Sender<Result<Vec<Segment>>>,
    pub from: u64,
    pub to: u64,
}

// HSR Stands for Hastic Signal Representation,
// varies for different analytic units
#[derive(Debug, Serialize)]
pub enum HSR {
    TimeSerie(Vec<(u64, f64)>),
    ConfidenceTimeSerie(Vec<(u64, f64, (f64, f64))>),
}

#[derive(Debug)]
pub struct HSRTask {
    // TODO: make enum for HSR which is different for different Analytic Types
    pub sender: oneshot::Sender<Result<HSR>>,
    pub from: u64,
    pub to: u64,
}

#[derive(Debug)]
pub enum LearningWaiter {
    Detection(DetectionTask),
    HSR(HSRTask),
}

// TODO: review if it's needed
#[derive(Debug, Clone)]
pub struct DetectionRunnerConfig {
    // pub sender: mpsc::Sender<Result<Vec<Segment>>>,
    pub endpoint: String,
    pub from: u64,
}

#[derive(Debug)]
pub enum RequestType {
    // TODO: convert to result RunLearning(anyhow::Result<()>)
    RunLearning,
    GetHSR(HSRTask),
    RunDetection(DetectionTask),
    GetStatus(oneshot::Sender<LearningStatus>),
    // TODO: make type of Value
    PatchConfig(PatchConfig, oneshot::Sender<()>),
    GetConfig(oneshot::Sender<AnalyticUnitConfig>),
    // GetLearningTrain(oneshot::Sender<LearningTrain>),
}

#[derive(Debug)]
pub enum AnalyticServiceMessage {
    // Status,
    Request(RequestType),
    Response(ResponseType), // Detect { from: u64, to: u64 },
}
