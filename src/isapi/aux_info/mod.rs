pub mod attributes {
    use crate::{
        isapi::{ChannelInfo, ChannelInfoList},
        utils::{error::HikvisionError, request::RequestHandler},
    };

    pub async fn channels(rh: &RequestHandler) -> Result<ChannelInfoList, HikvisionError> {
        let url = rh.prepare_url_to_request("/ISAPI/AUXInfo/attributes/Channels");
        Ok(rh.recieve(&url).await?)
    }
    pub async fn channels_id(
        rh: &RequestHandler,
        id: usize,
    ) -> Result<ChannelInfo, HikvisionError> {
        let url =
            rh.prepare_url_to_request(format!("/ISAPI/AUXInfo/attributes/Channels/{id}").as_str());
        Ok(rh.recieve(&url).await?)
    }
}

mod tests {
    #[tokio::test]
    async fn test_auxinfo_response() {
        use crate::{
            isapi::{ChannelInfo, ChannelInfoList, ICN},
            utils::{error::HikvisionError, request::RequestHandler},
        };

        let rh = RequestHandler::new("admin", "Admin777", "172.26.226.95", None);
        let channel_info = ChannelInfo {
            id: 1,
            eagle_eye: None,
            fish_eye: None,
            thermal: None,
            is_support_changed_upload: None,
            panorama_camera: None,
            global_camera: Some(ICN {
                insert_chan_no: Some(1),
            }),
        };
        let channel_info_list = ChannelInfoList {
            channel_info_list: vec![channel_info.clone()],
        };

        let c_id = crate::isapi::aux_info::attributes::channels_id(&rh, 1 as usize)
            .await
            .unwrap();
        assert_eq!(channel_info, c_id);

        let c = crate::isapi::aux_info::attributes::channels(&rh)
            .await
            .unwrap();
        assert_eq!(channel_info_list, c);
    }
}
