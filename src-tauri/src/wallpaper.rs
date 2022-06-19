use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Deserialize, Serialize, Debug)]
pub struct BingInfo {
    pub images: Vec<ImageInfo>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ImageInfo {
    pub url: Option<String>,
    pub title: Option<String>,
    pub startdate: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResultApi {
    pub code: u32,
    pub msg: String,
    pub data: String,
}

impl ResultApi {
    pub fn new(code: u32, msg: String, data: String) -> ResultApi {
        ResultApi { code, msg, data }
    }
}

#[tauri::command]
pub async fn get_bing_list() -> Result<Vec<ImageInfo>, String> {
    let bing_api = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=8&mkt=us_EN";
    let resp = reqwest::get(bing_api).await;
    let bing_info: BingInfo = match resp {
        Ok(info) => match info.json::<BingInfo>().await {
            Ok(info) => info,
            Err(error) => {
                log::error!("get images info error reason:{:?}", error);
                panic!()
            }
        },
        Err(error) => {
            log::error!("get images info error reason:{:?}", error);
            panic!()
        }
    };
    info!("bing_list:{:?}", bing_info);
    Ok(bing_info.images)
}

#[tauri::command]
pub fn set_wallpaper(image:ImageInfo) -> Result<ResultApi, String> {
    info!("set info {:#?}",image);
    let rst = match download_image(image.url, image.title, image.startdate) {
        Ok(path) => {
            info!("path {:#?}",path.as_str());
            wallpaper::set_from_path(path.as_str()).expect("set paper failed");
            ResultApi::new(
                200,
                "设置成功！".to_string(),
                "success".to_string(),
            )
        }
        Err(error) => {
            error!("set wallpaer error {:#?}", error);
            ResultApi::new(
                500,
                "设置失败！".to_string(),
                "error".to_string(),
            )
        }
    };
    Ok(rst)
}

pub fn download_image(mut url: Option<String>, title: Option<String>, date:Option<String>) -> Result<String, Box<dyn Error>> {
    if url.is_none() || title.is_none() || date.is_none() {
        panic!("some info missing")
    }

    match home::home_dir() {
        None => {
            panic!("not have home");
        }
        Some(home_path) => {
            let wallpaper_dir = home_path.join("Pictures").join("Wallpaper");
            info!("wallpaper_dir path:{:#?}", wallpaper_dir);

            if !wallpaper_dir.exists() {
                match fs::create_dir_all(&wallpaper_dir) {
                    Ok(_) => {
                        info!("dir create success")
                    }
                    Err(_) => {
                        panic!("create wallpaper dir error!")
                    }
                }
            }
            let path = wallpaper_dir.join(date.unwrap() + "-"  + ".jpg");
            let pic_path = String::from(path.to_str().unwrap());
            info!("pic path {:#?}",&pic_path);
            if !path.exists() {
                let bing_domain = "https://www.bing.com".to_string();
                let new_url = bing_domain.add(url.unwrap().replace("1920x1080", "UHD").as_ref());
                let res = reqwest::blocking::get(new_url)?;
                let mut file = File::create(&path)?;
                let stream = res.bytes()?;
                file.write_all(stream.as_ref()).unwrap();
            }
            return Ok(pic_path);
        }
    }
}
