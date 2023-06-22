use serde::Deserialize;
use thiserror::Error;

use std::fmt::Display;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum StatusCode {
    OK,
    DeviceBusy,
    DeviceError,
    InvalidOperation,
    InvalidXMLFormat,
    InvalidXMLContent,
    RebootRequired,
    AdditionalError,
    Unknown,
}

pub type SubStatusCode = StatusCode;

impl From<u8> for StatusCode {
    fn from(sc: u8) -> Self {
        use StatusCode::*;

        match sc {
            0 | 1 => OK,
            2 => DeviceBusy,
            3 => DeviceError,
            4 => InvalidOperation,
            5 => InvalidXMLFormat,
            6 => InvalidXMLContent,
            7 => RebootRequired,
            8 => AdditionalError,
            9 => AdditionalError,
            _ => Unknown,
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Error, Debug)]
pub enum ErrorCode {
    //StatusCode = 1
    #[error("Operation completed")]
    OK,
    #[error("Risky password")]
    RiskPassword,
    #[error("Arming process")]
    ArmProcess,

    //StatusCode = 2
    #[error("Insufficient memory")]
    NoMemory,
    #[error("The service is not availiable")]
    ServiceUnavailiable,
    #[error("Upgrading")]
    Upgrading,
    #[error("The device is busy or no response")]
    DeviceBusy,
    #[error("The video server is reconnected")]
    ReConnectIpc,
    #[error("Transmitting device upgrade data failed")]
    TransferUpgradePackageFailed,
    #[error("Starting upgrading device failed")]
    StartUpgradeFailed,
    #[error("Getting upgrade status failed")]
    GetUpgradeProcessfailed,
    #[error("The Authentication certificate already exists")]
    CertificateExist,

    //StatusCode = 3
    #[error("Hardware error")]
    DeviceError,
    #[error("Flash operation error")]
    BadFlash,
    #[error("The 28181 configuration is not initialized")]
    _28181Uninitialized,
    #[error("Connecting to socket failed")]
    SocketConnectError,
    #[error("Receive response message failed")]
    RecieveError,
    #[error("Deleting picture failed")]
    DeletePictureError,
    #[error("Too large picture size")]
    PictureSizeExceedLimit,
    #[error("Clearing cache failed")]
    ClearCacheError,
    #[error("Updating database failed")]
    UpdateDatabaseError,
    #[error("Searching in the database failed")]
    SearchDatabaseError,
    #[error("Writing to database failed")]
    WriteDatabaseError,
    #[error("Deleting database element failed")]
    DeleteDatabaseError,
    #[error("Getting number of database elements failed")]
    SearchDatabaseElementError,
    #[error("Downloading upgrade packet from cloud and upgrading failed")]
    CloudAutoUpgradeException,
    #[error("HBP exception")]
    HBPException,
    #[error("UDEP exception")]
    UDEPException,
    #[error("Elastic exception")]
    ElasticSearchException,
    #[error("Kafka exception")]
    KafkaException,
    #[error("HBase exception")]
    HBaseException,
    #[error("Spark exception")]
    SparkException,
    #[error("Yarn exception")]
    YarnException,
    #[error("Cache exception")]
    CacheException,
    #[error("Monitoring point big data server exception")]
    TrafficException,
    #[error("Human face big data server exception")]
    FaceException,
    #[error("SSD file system error (Error occurs when it is non-Ext4 file system)")]
    SSDFileSystemIsError,
    #[error("Insufficient SSD space for person frequency detection")]
    InsufficientSSDCapacityForFPD,
    #[error("Wi-Fi big data server exception")]
    WifiException,
    #[error("Video parameters structure server exception")]
    StructException,
    #[error("Data collection timed out")]
    CaptureTimeout,
    #[error("Low quality of collected data")]
    LowScore,

    //StatusCode = 4
    //TODO

    #[error("Unknown error")]
    Unknown,
}

impl From<u64> for ErrorCode {
    fn from(ec: u64) -> Self {
        use ErrorCode::*;

        match ec {
            0x1 => OK,
            0x10000002 => RiskPassword,
            0x10000005 => ArmProcess,

            0x20000001 => NoMemory,
            0x20000002 => ServiceUnavailiable,
            0x20000003 => Upgrading,
            0x20000004 => DeviceBusy,
            0x20000005 => ReConnectIpc,
            0x20000006 => TransferUpgradePackageFailed,
            0x20000007 => StartUpgradeFailed,
            0x20000008 => GetUpgradeProcessfailed,
            0x2000000B => CertificateExist,

            0x30000001 => DeviceError,
            0x30000002 => BadFlash,
            0x30000003 => _28181Uninitialized,
            0x30000005 => SocketConnectError,
            0x30000007 => RecieveError,
            0x3000000A => DeletePictureError,
            0x3000000C => PictureSizeExceedLimit,
            0x3000000D => ClearCacheError,
            0x3000000F => UpdateDatabaseError,
            0x30000010 => SearchDatabaseError,
            0x30000011 => WriteDatabaseError,
            0x30000012 => DeleteDatabaseError,
            0x30000013 => SearchDatabaseElementError,
            0x30000016 => CloudAutoUpgradeException,
            0x30001000 => HBPException,
            0x30001001 => UDEPException,
            0x30001002 => ElasticSearchException,
            0x30001003 => KafkaException,
            0x30001004 => HBaseException,
            0x30001005 => SparkException,
            0x30001006 => YarnException,
            0x30001007 => CacheException,
            0x30001008 => TrafficException,
            0x30001009 => FaceException,
            0x30001013 => SSDFileSystemIsError,
            0x30001014 => InsufficientSSDCapacityForFPD,
            0x3000100A => WifiException,
            0x3000100D => StructException,
            0x30006000 => CaptureTimeout,
            0x30006001 => LowScore,
            //TODO

            _ => Unknown,
        }
    }
}
