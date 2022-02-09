#![allow(non_snake_case)]

// Win32
use directx_math::*;

// Renderer
use crate::renderer::editor_camera::*;
use crate::renderer::scene_renderer::*;
use crate::renderer::texture::*;
use crate::renderer::environment::*;

// Core
use crate::core::utils::*;
use crate::core::timestep::*;

// Scene
use crate::scene::component::*;

// legion
use legion::*;

#[derive(Copy, Clone, Debug)]
pub enum SceneState
{
    Edit, Running
}

pub struct Scene
{
    m_World:        legion::World,
    m_State:        SceneState,
    m_ViewportSize: XMFLOAT2,
    m_EditorCamera: EditorCamera
}

impl Scene
{
    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn Create() -> RustyRef<Scene>
    {
        return RustyRef::CreateRef(Scene{
            m_World: legion::World::default(),
            m_State: SceneState::Edit,
            m_ViewportSize: XMFLOAT2::default(),
            m_EditorCamera: EditorCamera::Create(45.0, 16.0 / 9.0, 0.1, 10000.0)
        });
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn CreateEntity(&mut self, name: &str) -> super::entity::Entity
    {
        // Every entity must have a transform and tag components
        let transformComponent = TransformComponent::default();
        let tagComponent = TagComponent { Tag: String::from(name) };
        let entityID = self.m_World.push((tagComponent, transformComponent));

        return super::entity::Entity::Create(entityID, self);
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn DeleteEntity(&mut self, entity: super::entity::Entity)
    {
        let result: bool = self.m_World.remove(entity.GetID());
        debug_assert!(result, "Failed to delete entity!");
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn FindEntity(&mut self, name: &str) -> super::entity::Entity
    {
        let mut query = legion::Entity::query();
        
        for entity in query.iter(&self.m_World)
        {
            if let Ok(entry) = self.m_World.entry_ref(*entity)
            {
                if entry.get_component::<TagComponent>().unwrap().Tag == String::from(name)
                {
                    return super::entity::Entity::Create(*entity, self);
                }
            }
        }

        return super::entity::Entity::CreateEmpty();
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn OnSceneStart(&mut self)
    {
        
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn OnUpdate(&mut self, ts: Timestep)
    {
        self.m_EditorCamera.OnUpdate(ts);
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn OnSceneEnd(&mut self)
    {
        
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn OnEditRender(&self, renderer: &mut SceneRenderer)
    {
        // Setup the environment
        let mut environment = Environment::Create();

        // Sky light
        let mut query = <&SkyLightComponent>::query();
        for slc in query.iter(&self.m_World)
        {
            environment.SetEnvironmentMap(slc.EnvironmentMap.0.clone(), slc.EnvironmentMap.1.clone());
            break;
        }

        // Directional lights
        let mut query = <(&DirectionalLightComponent, &TransformComponent)>::query();
        for (dlc, tc) in query.iter(&self.m_World)
        {
            let light = Light::CreateDirectionalLight(dlc.Color, dlc.Intensity, XMFLOAT3::set(-tc.Position.x, -tc.Position.y, -tc.Position.z));
            environment.AddLight(light);
        }

        // Point lights
        let mut query = <(&PointLightComponent, &TransformComponent)>::query();
        for (plc, tc) in query.iter(&self.m_World)
        {
            let light = Light::CreatePointLight(plc.Color, plc.Intensity, tc.Position);
            environment.AddLight(light);
        }

        renderer.BeginScene(&self.m_EditorCamera, environment);

        let mut query = <(&MeshComponent, &TransformComponent)>::query();
        for (mc, tc) in query.iter(&self.m_World)
        {
            renderer.SubmitMesh(mc.Mesh.clone(), XMMatrixTranspose(tc.Transform()), &mc.Materials);
        }

        renderer.Flush();
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn OnRuntimeRender(&self, renderer: &mut SceneRenderer)
    {
        todo!();
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn OnViewportResize(&mut self, viewportWidth: f32, viewportHeight: f32)
    {
        self.m_ViewportSize.x = viewportWidth;
        self.m_ViewportSize.y = viewportHeight;

        self.m_EditorCamera.SetViewportSize(viewportWidth, viewportHeight);
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn SetSceneState(&mut self, state: SceneState)
    {
        self.m_State = state;
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn GetSceneState(&self) -> SceneState
    {
        return self.m_State;
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn GetEditorCamera(&mut self) -> &mut EditorCamera
    {
        return &mut self.m_EditorCamera;
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn GetWorld(&self) -> &legion::World
    {
        return &self.m_World;
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn GetWorldMut(&mut self) -> &mut legion::World
    {
        return &mut self.m_World;
    }
}
