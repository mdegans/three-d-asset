#[cfg(feature = "obj")]
mod obj;

#[cfg(feature = "gltf")]
mod gltf;

#[cfg(feature = "image")]
#[cfg_attr(docsrs, doc(cfg(feature = "image")))]
mod img;
#[cfg(feature = "image")]
#[doc(inline)]
pub use img::*;

#[cfg(feature = "vol")]
mod vol;

use crate::io::{Deserialize, RawAssets};
use crate::{Error, Model, Result, Texture2D, VoxelGrid};
use std::path::Path;

impl Deserialize for Texture2D {
    fn deserialize(raw_assets: &mut RawAssets, path: impl AsRef<std::path::Path>) -> Result<Self> {
        let bytes = raw_assets.remove(path)?;
        Self::from_bytes(&bytes)
    }
}

impl Deserialize for Model {
    fn deserialize(raw_assets: &mut RawAssets, path: impl AsRef<Path>) -> Result<Self> {
        let path = raw_assets.match_path(path)?;
        match path.extension().map(|e| e.to_str().unwrap()).unwrap_or("") {
            "gltf" | "glb" => {
                #[cfg(feature = "gltf")]
                let result = gltf::deserialize(raw_assets, path);

                #[cfg(not(feature = "gltf"))]
                let result = Err(Error::FeatureMissing(
                    "gltf".to_string(),
                    path.to_str().unwrap().to_string(),
                ));
                result
            }
            "obj" => {
                #[cfg(feature = "obj")]
                let result = obj::deserialize(raw_assets, path);

                #[cfg(not(feature = "obj"))]
                let result = Err(Error::FeatureMissing(
                    "obj".to_string(),
                    path.to_str().unwrap().to_string(),
                ));
                result
            }
            _ => Err(Error::FailedDeserialize(path.to_str().unwrap().to_string())),
        }
    }
}

impl Deserialize for VoxelGrid {
    fn deserialize(raw_assets: &mut RawAssets, path: impl AsRef<Path>) -> Result<Self> {
        let path = raw_assets.match_path(path)?;
        match path.extension().map(|e| e.to_str().unwrap()).unwrap_or("") {
            "vol" => {
                #[cfg(feature = "vol")]
                let result = vol::deserialize(raw_assets, path);

                #[cfg(not(feature = "vol"))]
                let result = Err(Error::FeatureMissing(
                    "vol".to_string(),
                    path.to_str().unwrap().to_string(),
                ));
                result
            }
            _ => Err(Error::FailedDeserialize(path.to_str().unwrap().to_string())),
        }
    }
}
