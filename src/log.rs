use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use crate::device::{Device, IndexData, MetaData};
use serde_json;
use serde_xml_rs;
use ron;
use ron::ser::PrettyConfig;
use serde::Serialize;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Log {
    log_folder: String,
    rsx_log_path: String,
    xmd_log_path: String,
    pub(crate) log_data: HashMap<String, String>,
}

impl Log {
    pub fn new(log_folder: String, rsx_log_path: String, xmd_log_path: String) -> Self {
        let log_data = HashMap::new();

        Log {
            log_folder,
            rsx_log_path,
            xmd_log_path,
            log_data
        }

    }

    pub fn write(&mut self) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(&self.log_folder)?;

        let mut rsx_log = File::create(&self.rsx_log_path)?;
        let mut xmd_log = File::create(&self.xmd_log_path)?;

        let mut rsx_buffer = Vec::new();
        let mut xmd_buffer = Vec::new();
        
        // let mut log_data = self.log_data.clone().into_values().map(|value| {
        //     value.replace(r"\", "").replace(r"\n", "")
        // }).collect::<Vec<String>>();
        
        // dbg!(&log_data);
        // 
        // log_data = self.log_data.clone().into_values().collect();
        
        serde_json::to_writer_pretty(&mut rsx_buffer, &self.log_data)?;
        serde_xml_rs::to_writer(&mut xmd_buffer, &self.log_data)?;

        rsx_log.write_all((&rsx_buffer).as_ref())?;
        xmd_log.write_all((&xmd_buffer).as_ref())?;

        Ok(())

    }
}

impl<'a> Device<'a> {
    pub(crate) fn log(&mut self, key: &'a str, data: String) -> Result<(), Box<dyn Error>> {
        // let data = data.replace(r"\", "").replace(r"\n", "");
        
        self.log.log_data.insert(key.to_string(), data);

        Ok(())

    }
}

// impl Drop for Device<'_> {
//     fn drop(&mut self) {
//     }
// }
