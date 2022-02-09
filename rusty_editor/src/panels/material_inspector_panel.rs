#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

// std
use __core::ops::RangeInclusive;
use std::borrow::Cow;

// Win32
use directx_math::*;
use windows::Win32::Graphics::Dxgi::*;
use windows::Win32::Graphics::Direct3D11::*;

// imgui
use imgui::*;

// Rusty Engine
use rusty_engine::core::utils::*;
use rusty_engine::imgui::imgui_renderer::*;
use rusty_engine::renderer::material::*;
use rusty_engine::renderer::shader::*;
use rusty_engine::renderer::shader_resource::*;
use rusty_engine::renderer::shader_uniform::*;
use rusty_engine::renderer::texture::*;
use rusty_engine::renderer::mesh::*;

pub struct MaterialInspectorPanel
{
    m_SelectedMesh: RustyRef<Mesh>,
    m_SelectedMaterialIndex: usize
}

impl MaterialInspectorPanel
{
    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn Create() -> MaterialInspectorPanel
    {
        return MaterialInspectorPanel {
            m_SelectedMesh: RustyRef::CreateEmpty(),
            m_SelectedMaterialIndex: 0
        };
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn OnImGuiRender(&mut self, ui: &Ui, renderer: &mut ImGuiRenderer)
    {
        if let Some(materialInspector) = Window::new(im_str!("Material"))
                                                .size([500.0, 400.0], imgui::Condition::FirstUseEver)
                                                .begin(&ui)
        {
            if self.m_SelectedMesh.IsValid()
            {
                let meshRef = self.m_SelectedMesh.GetRef();

                ui.text("Material");
                ui.same_line();

                let materials = meshRef.GetMaterials().as_slice();
                imgui::ComboBox::new(im_str!("##MaterialComboBox")).build_simple(&ui, &mut self.m_SelectedMaterialIndex, materials, 
                                                                                 &|material: &RustyRef<Material>|{ Cow::from(im_str!("{}", material.GetRef().GetName())) });

                let selectedMaterial = materials[self.m_SelectedMaterialIndex].clone();

                if selectedMaterial.IsValid()
                {
                    let shader: RustyRef<Shader> = selectedMaterial.GetRef().GetShader();
                    ui.text(format!("Shader: {}", shader.GetRef().GetName()));

                    let flags = TreeNodeFlags::DEFAULT_OPEN;

                    if CollapsingHeader::new(im_str!("Textures")).flags(flags).build(ui)
                    {
                        let resources = selectedMaterial.GetRef().GetResources().clone();
                        for resource in resources.iter()
                        {
                            if resource.GetType() == ShaderResourceType::Texture2D
                            {
                                ui.columns(2, im_str!("##Textures"), true);
                                ui.set_column_width(0, 150.0);
                                ui.text(resource.GetName());
                                ui.next_column();

                                let texture = selectedMaterial.GetRef().GetTextures()[resource.GetRegister() as usize].clone();
                                let textureID = imgui::TextureId::from(texture.GetRaw());

                                renderer.GetTextures().replace(textureID, texture.GetRef().CreateSRV());

                                imgui::Image::new(textureID, [64.0, 64.0]).build(&ui);

                                // If drag drop payload is accepted load the texture with the sent path 
                                if let Some(target) = imgui::DragDropTarget::new(ui) 
                                {
                                    if let Some(Ok(payloadData)) = target
                                        .accept_payload::<*const str>(im_str!("DragTexturePath"), DragDropFlags::empty())
                                    {
                                        // We know it is safe to dereference the pointer since it points to a string owned by the content browser panel
                                        // and it lives through the whole application
                                        let texturePath = unsafe { &*payloadData.data };
                                        let filename: &str = std::path::Path::new(texturePath).file_stem().unwrap().to_str().unwrap();

                                        let textureDesc = TextureDescription {
                                            Name: String::from(filename),
                                            Width: 0,
                                            Height: 0,
                                            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                                            BindFlags: D3D11_BIND_SHADER_RESOURCE,
                                            MipCount: 7,
                                            ImageData: vec![Some(rusty_engine::renderer::texture::Image::LoadFromFile(texturePath, false))]
                                        };
                                    
                                        let samplerDesc = SamplerDescription {
                                            Wrap: D3D11_TEXTURE_ADDRESS_WRAP,
                                            Filter: D3D11_FILTER_ANISOTROPIC
                                        };

                                        selectedMaterial.GetRefMut().SetTexture(resource.GetName(), Texture::CreateTexture2D(&textureDesc, &samplerDesc));
                                    }

                                    target.pop();
                                }

                                ui.columns(1, im_str!("##Textures"), true);
                            }
                        }
                    }

                    if CollapsingHeader::new(im_str!("Uniforms")).flags(flags).build(ui)
                    {
                        let uniforms = selectedMaterial.GetRef().GetUniforms().clone();
                        for uniform in uniforms.iter()
                        {
                            let labelStr: ImString = im_str!("{}", uniform.GetName());
                            let id: &ImStr = labelStr.as_ref();

                            let idToken = ui.push_id(id);

                            ui.columns(2, im_str!("##Uniforms"), true);
                            ui.set_column_width(0, 150.0);
                            ui.text(uniform.GetName());
                            ui.next_column();

                            match uniform.GetType()
                            {
                                ShaderUniformType::Float => {

                                    let sliderWidth: f32 = ui.column_width(1) * 0.2;
                                    let mut value: f32 = *(selectedMaterial.GetRef().GetUniform::<f32>(uniform.GetName()));

                                    let itemWidthToken = ui.push_item_width(sliderWidth);

                                    if imgui::Drag::new(im_str!("##X")).range(RangeInclusive::new(0.0, 1.0)).speed(0.01).build(ui, &mut value)
                                    {
                                        selectedMaterial.GetRefMut().SetUniform(uniform.GetName(), value);
                                    }

                                    itemWidthToken.pop(ui);
                                },

                                ShaderUniformType::Int | ShaderUniformType::Uint => {

                                },

                                ShaderUniformType::Bool => {

                                    let mut value: bool = *(selectedMaterial.GetRef().GetUniform::<bool>(uniform.GetName()));
                                    if ui.checkbox(im_str!("Use"), &mut value)
                                    {
                                        selectedMaterial.GetRefMut().SetUniform(uniform.GetName(), if value { 1 } else { 0 });
                                    }
                                },

                                ShaderUniformType::Vec2 => {

                                    let sliderWidth: f32 = ui.column_width(1) * 0.2;
                                    let mut values: XMFLOAT2 = *(selectedMaterial.GetRef().GetUniform::<XMFLOAT2>(uniform.GetName()));

                                    let itemWidthToken = ui.push_item_width(sliderWidth);

                                    let mut valueChanged: bool = false;
                                    valueChanged |= imgui::Drag::new(im_str!("##X")).range(RangeInclusive::new(0.0, 1.0)).speed(0.01).build(ui, &mut values.x);

                                    ui.same_line();
                                    valueChanged |=imgui::Drag::new(im_str!("##Y")).range(RangeInclusive::new(0.0, 1.0)).speed(0.01).build(ui, &mut values.y);

                                    if valueChanged
                                    {
                                        selectedMaterial.GetRefMut().SetUniform(uniform.GetName(), values);
                                    }

                                    itemWidthToken.pop(ui);
                                }

                                ShaderUniformType::Vec3 => {

                                    let sliderWidth: f32 = ui.column_width(1) * 0.2;
                                    let mut values: XMFLOAT3 = *(selectedMaterial.GetRef().GetUniform::<XMFLOAT3>(uniform.GetName()));

                                    let itemWidthToken = ui.push_item_width(sliderWidth);

                                    let mut valueChanged: bool = false;
                                    valueChanged |= imgui::Drag::new(im_str!("##X")).range(RangeInclusive::new(0.0, 1.0)).speed(0.01).build(ui, &mut values.x);

                                    ui.same_line();
                                    valueChanged |=imgui::Drag::new(im_str!("##Y")).range(RangeInclusive::new(0.0, 1.0)).speed(0.01).build(ui, &mut values.y);

                                    ui.same_line();
                                    valueChanged |=imgui::Drag::new(im_str!("##Z")).range(RangeInclusive::new(0.0, 1.0)).speed(0.01).build(ui, &mut values.z);

                                    if valueChanged
                                    {
                                        selectedMaterial.GetRefMut().SetUniform(uniform.GetName(), values);
                                    }

                                    itemWidthToken.pop(ui);
                                }

                                ShaderUniformType::Vec4 => {

                                    let sliderWidth: f32 = ui.column_width(1) * 0.2;
                                    let mut values: XMFLOAT4 = *(selectedMaterial.GetRef().GetUniform::<XMFLOAT4>(uniform.GetName()));

                                    let itemWidthToken = ui.push_item_width(sliderWidth);

                                    let mut valueChanged: bool = false;
                                    valueChanged |= imgui::Drag::new(im_str!("##X")).range(RangeInclusive::new(0.0, 1.0)).speed(0.01).build(ui, &mut values.x);

                                    ui.same_line();
                                    valueChanged |=imgui::Drag::new(im_str!("##Y")).range(RangeInclusive::new(0.0, 1.0)).speed(0.01).build(ui, &mut values.y);

                                    ui.same_line();
                                    valueChanged |=imgui::Drag::new(im_str!("##Z")).range(RangeInclusive::new(0.0, 1.0)).speed(0.01).build(ui, &mut values.z);

                                    ui.same_line();
                                    valueChanged |=imgui::Drag::new(im_str!("##W")).range(RangeInclusive::new(0.0, 1.0)).speed(0.01).build(ui, &mut values.w);

                                    if valueChanged
                                    {
                                        selectedMaterial.GetRefMut().SetUniform(uniform.GetName(), values);
                                    }

                                    itemWidthToken.pop(ui);
                                }

                                _ => {}
                            }

                            ui.columns(1, im_str!("##Uniforms"), true);
                            idToken.pop();
                        }
                    }

                    if CollapsingHeader::new(im_str!("Flags")).flags(flags).build(ui)
                    {
                        let mut flags: MaterialFlags = selectedMaterial.GetRef().GetRenderFlags();

                        ui.columns(2, im_str!("##MaterialFlags"), true);
                        ui.set_column_width(0, 150.0);
                        ui.text("Depth-tested");
                        ui.next_column();
                        let mut isDepthTested: bool = flags & MaterialFlags::DisableDepthTest == MaterialFlags::None;
                        ui.checkbox(im_str!("Use##DepthTest"), &mut isDepthTested);
                        ui.columns(1, im_str!("##MaterialFlags"), true);


                        ui.columns(2, im_str!("##MaterialFlags"), true);
                        ui.set_column_width(0, 150.0);
                        ui.text("Transparent");
                        ui.next_column();
                        let mut isTransparent: bool = flags & MaterialFlags::Transparent != MaterialFlags::None;
                        ui.checkbox(im_str!("Use##Transparent"), &mut isTransparent);
                        ui.columns(1, im_str!("##MaterialFlags"), true);

                        ui.columns(2, im_str!("##MaterialFlags"), true);
                        ui.set_column_width(0, 150.0);
                        ui.text("Two-Sided");
                        ui.next_column();
                        let mut isTwoSided: bool = flags & MaterialFlags::TwoSided != MaterialFlags::None;
                        ui.checkbox(im_str!("Use##TwoSided"), &mut isTwoSided);
                        ui.columns(1, im_str!("##MaterialFlags"), true);

                        ui.columns(2, im_str!("##MaterialFlags"), true);
                        ui.set_column_width(0, 150.0);
                        ui.text("Wireframe");
                        ui.next_column();
                        let mut isWireframe: bool = flags & MaterialFlags::Wireframe != MaterialFlags::None;
                        ui.checkbox(im_str!("Use##WireFrame"), &mut isWireframe);
                        ui.columns(1, im_str!("##MaterialFlags"), true);

                        if isDepthTested { flags &= !MaterialFlags::DisableDepthTest; } else { flags |= MaterialFlags::DisableDepthTest; }
                        if isTransparent { flags |= MaterialFlags::Transparent; } else { flags &= !MaterialFlags::Transparent; }
                        if isTwoSided    { flags |= MaterialFlags::TwoSided; } else { flags &= !MaterialFlags::TwoSided; }
                        if isWireframe   { flags |= MaterialFlags::Wireframe; } else { flags &= !MaterialFlags::Wireframe; }

                        selectedMaterial.GetRefMut().SetRenderFlags(flags);
                    }
                }
            }
            materialInspector.end();
        }
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn SetSelectedMesh(&mut self, mesh: RustyRef<Mesh>)
    {
        self.m_SelectedMesh = mesh;
        //self.m_SelectedMaterialIndex = 0;
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------
    pub fn GetSelectedMesh(&self) -> RustyRef<Mesh>
    {
        return self.m_SelectedMesh.clone();
    }
}