use serde::{Deserialize, Serialize};

pub type StreamFusion = ICN;
pub type PanoramicMetaData = ICN;
pub type TemperatureMetaData = ICN;
pub type ThermometryDetection = ICN;
pub type ThermometryDiffDetection = ICN;
pub type FishEyeStream = ICN;
pub type FireDetection = ICN;
pub type SmokeDetection = ICN;
pub type ShipsDetection = ICN;
pub type IntelligentRuleDisplay = ICN;
pub type IntelligentBehaviorRule = ICN;
pub type DPC = ICN;
pub type Metadata = ICN;
pub type ThermometryShieldMask = ICN;
pub type LensCorrection = ICN;
pub type BurningPrevention = ICN;
pub type PTZCtrl = ICN;
pub type GlobalCamera = ICN;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
pub struct ICN {
    #[serde(rename = "InsertChanNo")]
    pub insert_chan_no: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
pub struct ChannelInfoList {
    #[serde(rename = "ChannelInfo")]
    pub channel_info_list: Vec<ChannelInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChannelInfo {
    pub id: u32,
    #[serde(rename = "EagleEye")]
    pub eagle_eye: Option<EagleEye>,
    #[serde(rename = "FishEye")]
    pub fish_eye: Option<FishEye>,
    #[serde(rename = "Thermal")]
    pub thermal: Option<Thermal>,
    pub is_support_changed_upload: Option<bool>,
    #[serde(rename = "PanoramaCamera")]
    pub panorama_camera: Option<PanoramaCamera>,
    #[serde(rename = "GlobalCamera")]
    pub global_camera: Option<GlobalCamera>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct EagleEye {
    #[serde(rename = "StreamFusion")]
    pub stream_fusion: Option<StreamFusion>,
    #[serde(rename = "PanoramicMetaData")]
    pub panoramic_meta_data: Option<PanoramicMetaData>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FishEye {
    #[serde(rename = "FishEyeStream")]
    pub fish_eye_stream: Option<FishEyeStream>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PanoramaCamera {
    #[serde(rename = "PTZCtrl")]
    pub ptz_ctrl: Option<PTZCtrl>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Thermal {
    #[serde(rename = "TemperatureMetaData")]
    pub temperature_meta_data: Option<TemperatureMetaData>,
    #[serde(rename = "ThermometryDetection")]
    pub thermometry_detection: Option<ThermometryDetection>,
    #[serde(rename = "ThermometryDiffDetection")]
    pub thermometry_diff_detection: Option<ThermometryDiffDetection>,
    #[serde(rename = "FireDetection")]
    pub fire_detection: Option<FireDetection>,
    #[serde(rename = "SmokeDetection")]
    pub smoke_detection: Option<SmokeDetection>,
    #[serde(rename = "ShipsDetection")]
    pub ships_detection: Option<ShipsDetection>,
    #[serde(rename = "IntelligentRuleDisplay")]
    pub intelligent_rule_display: Option<IntelligentRuleDisplay>,
    #[serde(rename = "IntelligentBehaviorRule")]
    pub intelligent_behavior_rule: Option<IntelligentBehaviorRule>,
    #[serde(rename = "Metadata")]
    pub metadata: Option<Metadata>,
    #[serde(rename = "ThermometryShieldMask")]
    pub thermometry_shield_mask: Option<ThermometryShieldMask>,
    #[serde(rename = "LensCorrection")]
    pub lens_correction: Option<LensCorrection>,
    #[serde(rename = "BurningPrevention")]
    pub burning_prevention: Option<BurningPrevention>,
}
