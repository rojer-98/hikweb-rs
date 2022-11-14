use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RacmCap {
    is_support_zero_chan: Option<bool>,
    input_proxy_nums: Option<u32>,
    #[serde(rename = "eSATANums")]
    esata_nums: Option<u32>,
    #[serde(rename = "miniSASNums")]
    mini_sas_nums: Option<u32>,
    nas_nums: Option<u32>,
    ip_san_nums: Option<u32>,
    is_support_raid: Option<bool>,
    is_support_ext_hd_cfg: Option<bool>,
    is_support_trans_code: Option<bool>,
    //TODO
}
