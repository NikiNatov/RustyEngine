#![allow(non_snake_case)]

extern crate imgui;
extern crate rusty_engine;

// Win32
use directx_math::*;
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Dxgi::*;

// rusty_engine
use rusty_engine::renderer::shader_library::*;
use rusty_engine::renderer::material::*;
use rusty_engine::renderer::imgui_renderer::*;
use rusty_engine::renderer::texture::*;
use rusty_engine::renderer::editor_camera::*;
use rusty_engine::renderer::scene_renderer::*;
use rusty_engine::renderer::mesh::*;
use rusty_engine::renderer::renderer::*;
use rusty_engine::core::imgui::*;
use rusty_engine::core::input::*;
use rusty_engine::core::timestep::*;
use rusty_engine::core::timer::*;
use rusty_engine::core::event::*;
use rusty_engine::core::window_event::*;
use rusty_engine::core::keyboard_event::*;
use rusty_engine::core::window::*;
use rusty_engine::core::utils::*;

pub struct RuntimeApp
{
    m_IsRunning:       bool,
    m_Timer:           Timer,
    m_Window:          Window,
    m_SceneRenderer:   SceneRenderer,
    m_EditorCamera:    EditorCamera,
    m_ImGui:           ImGui,
    m_ImGuiRenderer:   ImGuiRenderer,

    m_CubeMesh:        RustyRef<Mesh>,
    m_CubeMaterial:    RustyRef<Material>,
    m_CubeTransform:   XMFLOAT4X4,
}

impl RuntimeApp
{
    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn Create(windowWidth: u32, windowHeight: u32) -> RuntimeApp
    {
        return RuntimeApp {
            m_IsRunning: false,
            m_Timer: Timer::Create(),
            m_Window: Window::Create("Rusty\0", windowWidth, windowHeight, true),
            m_SceneRenderer: SceneRenderer::Create(true),
            m_EditorCamera: EditorCamera::Create(45.0, windowWidth as f32 / windowHeight as f32, 0.1, 10000.0),
            m_ImGui: ImGui::Create(),
            m_ImGuiRenderer: ImGuiRenderer::Create(),

            m_CubeMesh: RustyRef::CreateEmpty(),
            m_CubeMaterial: RustyRef::CreateEmpty(),
            m_CubeTransform: XMFLOAT4X4::default(),
        };
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn Initialize(&mut self)
    {
        // Initialize systems
        self.m_Window.Initialize();
        Input::Initialize(self.m_Window.GetHandle());
        Renderer::Initialize(self.m_Window.GetGfxContext());
        self.m_SceneRenderer.Initialize();
        self.m_ImGui.Initialize(&self.m_Window);
        self.m_ImGuiRenderer.Initialize(self.m_ImGui.GetContext());

        self.m_IsRunning = true;

        // Test mesh
        self.m_CubeMesh = Mesh::CreateCube(1.0);
        XMStoreFloat4x4(&mut self.m_CubeTransform, XMMatrixIdentity());

        let textureDesc = TextureDescription {
            Width: 0,
            Height: 0,
            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
            BindFlags: D3D11_BIND_SHADER_RESOURCE,
            MipCount: 7,
            ImageData: Some(Image::LoadFromFile("assets/textures/box.jpg"))
        };

        let samplerDesc = SamplerDescription {
            Wrap: D3D11_TEXTURE_ADDRESS_MIRROR,
            Filter: D3D11_FILTER_ANISOTROPIC
        };

        self.m_CubeMaterial = Material::Create(ShaderLibrary::GetShader("mesh_pbr_shader"), MaterialFlags::None);
        self.m_CubeMaterial.GetRefMut().SetTexture("u_Texture", Texture::CreateTexture2D(&textureDesc, &samplerDesc));
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn Run(&mut self)
    {
        while self.m_IsRunning
        {
            let timestep = self.m_Timer.GetElapsedTime();
            //println!("{} fps, {:.3} ms", 1000.0 / timestep.GetMilliSeconds(), timestep.GetMilliSeconds());
            
            self.m_ImGui.UpdateDeltaTime(timestep.GetSeconds());

            self.m_Timer.Reset();

            self.OnUpdate(timestep);
            self.OnRender();

            self.m_Timer.Stop();
        }
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn GetWindow(&self) -> &Window
    {
        return &self.m_Window;
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    fn OnUpdate(&mut self, timestep: Timestep)
    {
        // Process events
        self.m_Window.ProcessMessages();

        while !self.m_Window.GetEventBuffer().is_empty()
        {
            let event = self.m_Window.GetEventBuffer().remove(0);
            self.OnEvent(event.as_ref());
        }

        self.m_EditorCamera.OnUpdate(timestep);
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn OnRender(&mut self)
    {
        self.m_SceneRenderer.BeginScene(&self.m_EditorCamera);
        self.m_SceneRenderer.SubmitMesh(self.m_CubeMesh.clone(), XMMatrixTranspose(XMLoadFloat4x4(&self.m_CubeTransform)), self.m_CubeMaterial.clone());
        self.m_SceneRenderer.Flush();

        self.OnImGuiRender();

        self.m_Window.GetGfxContext().GetRef().Present(self.m_Window.GetVSync());
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    fn OnImGuiRender(&mut self)
    {
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    fn OnEvent(&mut self, event: &dyn Event)
    {
        if event.GetType() == EventType::WindowClosed
        {
            let windowClosedEvent: &WindowClosedEvent = event.AsAny().downcast_ref::<WindowClosedEvent>().expect("Event is now a WindowClosedEvent");
            self.OnWindowClosed(windowClosedEvent);
        }
        else if event.GetType() == EventType::WindowResized
        {
            let windowResizedEvent: &WindowResizedEvent = event.AsAny().downcast_ref::<WindowResizedEvent>().expect("Event is now a WindowResizedEvent");
            self.OnWindowResized(windowResizedEvent);
        }

        self.m_ImGui.OnEvent(event);
        self.m_EditorCamera.OnEvent(event);
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    fn OnWindowClosed(&mut self, event: &WindowClosedEvent)
    {
        self.m_IsRunning = false;
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    fn OnKeyPressed(&mut self, event: &KeyPressedEvent)
    {
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    fn OnWindowResized(&mut self, event: &WindowResizedEvent)
    {
        self.m_SceneRenderer.GetCompositePassTarget().GetRefMut().Resize(event.GetWidth(), event.GetHeight());
        self.m_EditorCamera.SetViewportSize(event.GetWidth() as f32, event.GetHeight() as f32);
    }
}

fn main() 
{
    let mut app = RuntimeApp::Create(1600, 900);
    app.Initialize();
    app.Run();
}