#![allow(non_snake_case)]

// Win32
use directx_math::*;

// Core
use crate::core::utils::*;

// Renderer
use crate::renderer::mesh::*;
use crate::renderer::material::*;
use crate::renderer::texture::*;
use crate::renderer::renderer::*;

// Serialization
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;
use serde::Deserializer;

// ---------------------------------------------------------------- Tag Component ---------------------------------------------------------------- //
#[derive(Clone, Debug)]
pub struct TagComponent
{
    pub Tag: String
}

impl Default for TagComponent
{
    fn default() -> TagComponent
    {
        return TagComponent {
            Tag: String::from("Unnamed")
        }
    }
}

unsafe impl Send for TagComponent {}
unsafe impl Sync for TagComponent {}

// ---------------------------------------------------------------- Transform Component ---------------------------------------------------------- //
#[derive(Clone, Copy, Debug)]
pub struct TransformComponent
{
    pub Position: XMFLOAT3,
    pub Rotation: XMFLOAT3,
    pub Scale:    XMFLOAT3,
}

impl TransformComponent
{
    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn Transform(&self) -> XMMATRIX
    {
        let translation: XMMATRIX = XMMatrixTranslation(self.Position.x, self.Position.y, self.Position.z);
        let rotationX: XMMATRIX = XMMatrixRotationX(XMConvertToRadians(self.Rotation.x));
        let rotationY: XMMATRIX = XMMatrixRotationY(XMConvertToRadians(self.Rotation.y));
        let rotationZ: XMMATRIX = XMMatrixRotationZ(XMConvertToRadians(self.Rotation.z));
        let scale: XMMATRIX = XMMatrixScaling(self.Scale.x, self.Scale.y, self.Scale.z);

        let mut transform: XMMATRIX = XMMatrixIdentity();
        transform = XMMatrixMultiply(translation, &transform);
        transform = XMMatrixMultiply(rotationZ, &transform);
        transform = XMMatrixMultiply(rotationY, &transform);
        transform = XMMatrixMultiply(rotationX, &transform);
        transform = XMMatrixMultiply(scale, &transform);

        return transform;
    }
}

impl Default for TransformComponent
{
    fn default() -> TransformComponent
    {
        return TransformComponent {
            Position: XMFLOAT3::set(0.0, 0.0, 0.0),
            Rotation: XMFLOAT3::set(0.0, 0.0, 0.0),
            Scale: XMFLOAT3::set(1.0, 1.0, 1.0)
        }
    }
}

impl Serialize for TransformComponent 
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut transformStruct = serializer.serialize_struct("TransformComponent", 3)?;
        transformStruct.serialize_field("Position", self.Position.as_ref())?;
        transformStruct.serialize_field("Rotation", self.Rotation.as_ref())?;
        transformStruct.serialize_field("Scale", self.Scale.as_ref())?;
        transformStruct.end()
    }
}

unsafe impl Send for TransformComponent {}
unsafe impl Sync for TransformComponent {}

// ------------------------------------------------------------------- Mesh Component ------------------------------------------------------------ //
#[derive(Clone)]
pub struct MeshComponent
{
    pub Mesh:      RustyRef<Mesh>,
    pub Materials: Vec<RustyRef<Material>>
}

impl Default for MeshComponent
{
    fn default() -> MeshComponent
    {
        return MeshComponent {
            Mesh: Mesh::CreateCube(1.0),
            Materials: vec![]         
        }
    }
}

impl Serialize for MeshComponent 
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut meshStruct = serializer.serialize_struct("MeshComponent", 3)?;
        meshStruct.serialize_field("Mesh", self.Mesh.GetRef().GetFilepath())?;
        meshStruct.serialize_field("Materials", self.Mesh.GetRef().GetMaterials())?;
        meshStruct.end()
    }
}

unsafe impl Send for MeshComponent {}
unsafe impl Sync for MeshComponent {}

// ------------------------------------------------------------------- Sky Light Component ------------------------------------------------------------ //
#[derive(Clone)]
pub struct SkyLightComponent
{
    pub EnvironmentMap: (RustyRef<Texture>, RustyRef<Texture>),
    pub Intensity:      f32
}

impl Default for SkyLightComponent
{
    fn default() -> SkyLightComponent
    {
        return SkyLightComponent {
            EnvironmentMap: (Renderer::GetBlackTextureCube(), Renderer::GetBlackTextureCube()),
            Intensity: 1.0     
        }
    }
}

unsafe impl Send for SkyLightComponent {}
unsafe impl Sync for SkyLightComponent {}

// ------------------------------------------------------------------- Directional Light Component ------------------------------------------------------------ //
#[derive(Clone)]
pub struct DirectionalLightComponent
{
    pub Color:     XMFLOAT3,
    pub Intensity: f32
}

impl Default for DirectionalLightComponent
{
    fn default() -> DirectionalLightComponent
    {
        return DirectionalLightComponent {
            Color: XMFLOAT3::set(1.0, 1.0, 1.0),
            Intensity: 1.0
        }
    }
}

impl Serialize for DirectionalLightComponent 
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut dirLightStruct = serializer.serialize_struct("DirectionalLightComponent", 2)?;
        dirLightStruct.serialize_field("Color", self.Color.as_ref())?;
        dirLightStruct.serialize_field("Intensity", &self.Intensity)?;
        dirLightStruct.end()
    }
}

unsafe impl Send for DirectionalLightComponent {}
unsafe impl Sync for DirectionalLightComponent {}

// ------------------------------------------------------------------- Point Light Component ------------------------------------------------------------ //
#[derive(Clone)]
pub struct PointLightComponent
{
    pub Color:     XMFLOAT3,
    pub Intensity: f32
}

impl Default for PointLightComponent
{
    fn default() -> PointLightComponent
    {
        return PointLightComponent {
            Color: XMFLOAT3::set(1.0, 1.0, 1.0),
            Intensity: 1.0
        }
    }
}

impl Serialize for PointLightComponent 
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut pointLightStruct = serializer.serialize_struct("PointLightComponent", 2)?;
        pointLightStruct.serialize_field("Color", self.Color.as_ref())?;
        pointLightStruct.serialize_field("Intensity", &self.Intensity)?;
        pointLightStruct.end()
    }
}

unsafe impl Send for PointLightComponent {}
unsafe impl Sync for PointLightComponent {}