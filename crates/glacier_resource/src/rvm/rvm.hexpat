#pragma array_limit 0x40000
#pragma pattern_limit 0x1000000

import std.io;

struct Meta
{
    u32 maybeVersion; // 0x00000000
    u32 offset; // 0x032BC1C2
    u32 size; // 0x0004815C
    u32 maybeHash; // 0x82B10CD7
} [[static]];

struct Type
{
    u32 nameHash;
    u32 count;
    u64 index;
} [[static]];

struct Header
{
    u128 guid;
    u64 hash;
    u32 unk1;
    u32 count1;
    u32 unk3;
    u16 unk4;
    u16 typeCount;
    Type types[typeCount];
    u64 unk6;
    u16 counts[count1];
} [[static]];

struct Instance<T>
{
    T array[h.counts[current]];
    current = current + 1;
};

enum D3D11_BLEND : u8
{
  D3D11_BLEND_ZERO = 1,
  D3D11_BLEND_ONE = 2,
  D3D11_BLEND_SRC_COLOR = 3,
  D3D11_BLEND_INV_SRC_COLOR = 4,
  D3D11_BLEND_SRC_ALPHA = 5,
  D3D11_BLEND_INV_SRC_ALPHA = 6,
  D3D11_BLEND_DEST_ALPHA = 7,
  D3D11_BLEND_INV_DEST_ALPHA = 8,
  D3D11_BLEND_DEST_COLOR = 9,
  D3D11_BLEND_INV_DEST_COLOR = 10,
  D3D11_BLEND_SRC_ALPHA_SAT = 11,
  D3D11_BLEND_BLEND_FACTOR = 14,
  D3D11_BLEND_INV_BLEND_FACTOR = 15,
  D3D11_BLEND_SRC1_COLOR = 16,
  D3D11_BLEND_INV_SRC1_COLOR = 17,
  D3D11_BLEND_SRC1_ALPHA = 18,
  D3D11_BLEND_INV_SRC1_ALPHA = 19
} [[static]];

enum D3D11_BLEND_OP : u8
{
  D3D11_BLEND_OP_ADD = 1,
  D3D11_BLEND_OP_SUBTRACT = 2,
  D3D11_BLEND_OP_REV_SUBTRACT = 3,
  D3D11_BLEND_OP_MIN = 4,
  D3D11_BLEND_OP_MAX = 5
} [[static]];

struct D3D11_RENDER_TARGET_BLEND_DESC
{
    bool BlendEnable;
    D3D11_BLEND SrcBlend;
    D3D11_BLEND DestBlend;
    D3D11_BLEND_OP BlendOp;
    D3D11_BLEND SrcBlendAlpha;
    D3D11_BLEND DestBlendAlpha;
    D3D11_BLEND_OP BlendOpAlpha;
    u8 RenderTargetWriteMask;
} [[static]];

struct D3D11_BLEND_DESC
{
    bool AlphaToCoverageEnable;
    bool IndependentBlendEnable;
    padding[0x0E];
    D3D11_RENDER_TARGET_BLEND_DESC RenderTarget[8];
} [[static]];

struct RvmSerializedDb_ns_Dx11SerializedBlendState
{
    u8 data[0x30] [[sealed]];
    D3D11_BLEND_DESC description;
} [[static]];

struct Vec
{
    float x, y, z, w;
} [[static]];

struct RvmLegacyLightMapInstance
{
    u8 data[0x20] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_Dx11VsShader
{
    u8 data[0x40] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_SerializedParamDbKey
{
    u8 data[0x10] [[sealed]];
} [[static]];

struct ProjectionState
{
    u8 matrix[0x40] [[sealed]];
    u8 rest[0x50];
} [[static]];

struct StencilState
{
    u8 data[0x10] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_Dx11Sampler
{
    u8 data[0x40] [[sealed]];
} [[static]];

struct RvmLegacyLightProbes
{
    u8 data[0x90] [[sealed]];
} [[static]];

struct ViewState
{
    u8 data[0x140] [[sealed]]; // 4x4 matrix and some other stuff
} [[static]];

struct RvmSerializedDb_ns_DefaultValueRef
{
    u8 data[0x8] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_InstructionBatchRef
{
    u8 data[0x08] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_Dx11ApplyParametersBlock
{
    u8 data[0x10] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_RvmPermutation
{
    u128 guid;
    u16 index1;
    u16 index2;
    u8 unk1;
    u8 unk2;
    u16;
    u64 hash1, hash2, hash3, hash4;
} [[static]];

struct RvmSerializedDb_ns_InstructionBatch
{
    u8 data[0x18] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_RvmFunction
{
    u8 data[0x30] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchData
{
    u8 data[0x18] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_SurfaceShader
{
    u64;
    u32 nameHash;
    padding[4];
    u64;
    u32 unk1;
    u32 unk2;
    u64 hash1, hash2;
    u32 unk3; // found in sbd next to guid
    padding[0x0c];
    u128 guid; // found in sbd next to unk3
} [[static]];

struct RvmSerializedDb_ns_Dx11LegacyVertexBufferConversionInstructionData
{
    u8 data[0x28] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_SerializedParameterBlock
{
    u8 data[0x18] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_RvmFunctionInputTableIndices
{
    u8 data[0x08] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_Dx11PsShader
{
    u64;
    u64 hash;
    u64 unk1;
} [[static]];

struct RvmSerializedDb_ns_Dx11HsShader
{
    u64;
    u64 hash;
    u64 unk1;
} [[static]];

struct RvmSerializedDb_ns_SerializedParameterBlockRef
{
    u64;
} [[static]];

struct RvmSerializedDb_ns_Dx11BlendStateData
{
    u8 data[0x1000] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_DirectInputInstructionData
{
    u8 data[0x18] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_Dx11TextureConversionInstructionData
{
    u64;
    u64;
} [[static]];

struct RvmSerializedDb_ns_Dx11DsShader
{
    u64;
    u64 hash;
    u64 unk1;
} [[static]];

struct RvmSerializedDb_ns_ValueRef
{
    u64;
    u64;
} [[static]];

struct RvmSerializedDb_ns_RvmPermutationRef
{
    u64;
} [[static]];

struct RvmSerializedDb_ns_RvmPermutationSet
{
    u128 guid;
    u16;

    // probably related to shaders
    u16 unk1;
    u16 unk2;
    u16 permutationCount;
    u16 unk4;

    u16;
    u32;
    u64;

    u64 hash1, hash2, hash3, hash4;

    u64;
    u64;

    u64 hash5, hash6;
} [[static]];

struct RvmSerializedDb_ns_ShaderStreamableTextureRef
{
    u64 hash;
} [[static]];

struct RvmSerializedDb_ns_ParamDbSerializedHashView
{
    u64;
    u64;
    u64;
} [[static]];

struct RvmSerializedDb_ns_RvmDispatch
{
    u8 data[0x20] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_RvmFunctionInstance
{
    u64;
    u64;
    u64;
} [[static]];

struct RvmSerializedDb_ns_ParamDbSerializedHashViewRef
{
    u64;
} [[static]];

struct RvmSerializedDb_ns_ParamDbSerializedFilterView
{
    u64;
    u64;
} [[static]];

struct RvmSerializedDb_ns_RuntimeInstantiatedType
{
    u64;
} [[static]];

enum DXGI_FORMAT : u32
{
  DXGI_FORMAT_UNKNOWN = 0,
  DXGI_FORMAT_R32G32B32A32_TYPELESS = 1,
  DXGI_FORMAT_R32G32B32A32_FLOAT = 2,
  DXGI_FORMAT_R32G32B32A32_UINT = 3,
  DXGI_FORMAT_R32G32B32A32_SINT = 4,
  DXGI_FORMAT_R32G32B32_TYPELESS = 5,
  DXGI_FORMAT_R32G32B32_FLOAT = 6,
  DXGI_FORMAT_R32G32B32_UINT = 7,
  DXGI_FORMAT_R32G32B32_SINT = 8,
  DXGI_FORMAT_R16G16B16A16_TYPELESS = 9,
  DXGI_FORMAT_R16G16B16A16_FLOAT = 10,
  DXGI_FORMAT_R16G16B16A16_UNORM = 11,
  DXGI_FORMAT_R16G16B16A16_UINT = 12,
  DXGI_FORMAT_R16G16B16A16_SNORM = 13,
  DXGI_FORMAT_R16G16B16A16_SINT = 14,
  DXGI_FORMAT_R32G32_TYPELESS = 15,
  DXGI_FORMAT_R32G32_FLOAT = 16,
  DXGI_FORMAT_R32G32_UINT = 17,
  DXGI_FORMAT_R32G32_SINT = 18,
  DXGI_FORMAT_R32G8X24_TYPELESS = 19,
  DXGI_FORMAT_D32_FLOAT_S8X24_UINT = 20,
  DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS = 21,
  DXGI_FORMAT_X32_TYPELESS_G8X24_UINT = 22,
  DXGI_FORMAT_R10G10B10A2_TYPELESS = 23,
  DXGI_FORMAT_R10G10B10A2_UNORM = 24,
  DXGI_FORMAT_R10G10B10A2_UINT = 25,
  DXGI_FORMAT_R11G11B10_FLOAT = 26,
  DXGI_FORMAT_R8G8B8A8_TYPELESS = 27,
  DXGI_FORMAT_R8G8B8A8_UNORM = 28,
  DXGI_FORMAT_R8G8B8A8_UNORM_SRGB = 29,
  DXGI_FORMAT_R8G8B8A8_UINT = 30,
  DXGI_FORMAT_R8G8B8A8_SNORM = 31,
  DXGI_FORMAT_R8G8B8A8_SINT = 32,
  DXGI_FORMAT_R16G16_TYPELESS = 33,
  DXGI_FORMAT_R16G16_FLOAT = 34,
  DXGI_FORMAT_R16G16_UNORM = 35,
  DXGI_FORMAT_R16G16_UINT = 36,
  DXGI_FORMAT_R16G16_SNORM = 37,
  DXGI_FORMAT_R16G16_SINT = 38,
  DXGI_FORMAT_R32_TYPELESS = 39,
  DXGI_FORMAT_D32_FLOAT = 40,
  DXGI_FORMAT_R32_FLOAT = 41,
  DXGI_FORMAT_R32_UINT = 42,
  DXGI_FORMAT_R32_SINT = 43,
  DXGI_FORMAT_R24G8_TYPELESS = 44,
  DXGI_FORMAT_D24_UNORM_S8_UINT = 45,
  DXGI_FORMAT_R24_UNORM_X8_TYPELESS = 46,
  DXGI_FORMAT_X24_TYPELESS_G8_UINT = 47,
  DXGI_FORMAT_R8G8_TYPELESS = 48,
  DXGI_FORMAT_R8G8_UNORM = 49,
  DXGI_FORMAT_R8G8_UINT = 50,
  DXGI_FORMAT_R8G8_SNORM = 51,
  DXGI_FORMAT_R8G8_SINT = 52,
  DXGI_FORMAT_R16_TYPELESS = 53,
  DXGI_FORMAT_R16_FLOAT = 54,
  DXGI_FORMAT_D16_UNORM = 55,
  DXGI_FORMAT_R16_UNORM = 56,
  DXGI_FORMAT_R16_UINT = 57,
  DXGI_FORMAT_R16_SNORM = 58,
  DXGI_FORMAT_R16_SINT = 59,
  DXGI_FORMAT_R8_TYPELESS = 60,
  DXGI_FORMAT_R8_UNORM = 61,
  DXGI_FORMAT_R8_UINT = 62,
  DXGI_FORMAT_R8_SNORM = 63,
  DXGI_FORMAT_R8_SINT = 64,
  DXGI_FORMAT_A8_UNORM = 65,
  DXGI_FORMAT_R1_UNORM = 66,
  DXGI_FORMAT_R9G9B9E5_SHAREDEXP = 67,
  DXGI_FORMAT_R8G8_B8G8_UNORM = 68,
  DXGI_FORMAT_G8R8_G8B8_UNORM = 69,
  DXGI_FORMAT_BC1_TYPELESS = 70,
  DXGI_FORMAT_BC1_UNORM = 71,
  DXGI_FORMAT_BC1_UNORM_SRGB = 72,
  DXGI_FORMAT_BC2_TYPELESS = 73,
  DXGI_FORMAT_BC2_UNORM = 74,
  DXGI_FORMAT_BC2_UNORM_SRGB = 75,
  DXGI_FORMAT_BC3_TYPELESS = 76,
  DXGI_FORMAT_BC3_UNORM = 77,
  DXGI_FORMAT_BC3_UNORM_SRGB = 78,
  DXGI_FORMAT_BC4_TYPELESS = 79,
  DXGI_FORMAT_BC4_UNORM = 80,
  DXGI_FORMAT_BC4_SNORM = 81,
  DXGI_FORMAT_BC5_TYPELESS = 82,
  DXGI_FORMAT_BC5_UNORM = 83,
  DXGI_FORMAT_BC5_SNORM = 84,
  DXGI_FORMAT_B5G6R5_UNORM = 85,
  DXGI_FORMAT_B5G5R5A1_UNORM = 86,
  DXGI_FORMAT_B8G8R8A8_UNORM = 87,
  DXGI_FORMAT_B8G8R8X8_UNORM = 88,
  DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM = 89,
  DXGI_FORMAT_B8G8R8A8_TYPELESS = 90,
  DXGI_FORMAT_B8G8R8A8_UNORM_SRGB = 91,
  DXGI_FORMAT_B8G8R8X8_TYPELESS = 92,
  DXGI_FORMAT_B8G8R8X8_UNORM_SRGB = 93,
  DXGI_FORMAT_BC6H_TYPELESS = 94,
  DXGI_FORMAT_BC6H_UF16 = 95,
  DXGI_FORMAT_BC6H_SF16 = 96,
  DXGI_FORMAT_BC7_TYPELESS = 97,
  DXGI_FORMAT_BC7_UNORM = 98,
  DXGI_FORMAT_BC7_UNORM_SRGB = 99,
  DXGI_FORMAT_AYUV = 100,
  DXGI_FORMAT_Y410 = 101,
  DXGI_FORMAT_Y416 = 102,
  DXGI_FORMAT_NV12 = 103,
  DXGI_FORMAT_P010 = 104,
  DXGI_FORMAT_P016 = 105,
  DXGI_FORMAT_420_OPAQUE = 106,
  DXGI_FORMAT_YUY2 = 107,
  DXGI_FORMAT_Y210 = 108,
  DXGI_FORMAT_Y216 = 109,
  DXGI_FORMAT_NV11 = 110,
  DXGI_FORMAT_AI44 = 111,
  DXGI_FORMAT_IA44 = 112,
  DXGI_FORMAT_P8 = 113,
  DXGI_FORMAT_A8P8 = 114,
  DXGI_FORMAT_B4G4R4A4_UNORM = 115,
  DXGI_FORMAT_P208 = 130,
  DXGI_FORMAT_V208 = 131,
  DXGI_FORMAT_V408 = 132,
  DXGI_FORMAT_SAMPLER_FEEDBACK_MIN_MIP_OPAQUE,
  DXGI_FORMAT_SAMPLER_FEEDBACK_MIP_REGION_USED_OPAQUE,
  DXGI_FORMAT_FORCE_UINT = 0xffffffff
} [[static]];

enum D3D11_INPUT_CLASSIFICATION : u32
{
  D3D11_INPUT_PER_VERTEX_DATA = 0,
  D3D11_INPUT_PER_INSTANCE_DATA = 1
} [[static]];


struct RvmSerializedDb_ns_Dx11InputElement
{
    u64 SemanticName;
    u32 SemanticIndex;
    DXGI_FORMAT Format;
    u32 InputSlot;
    u32 AlignedByteOffset;
    D3D11_INPUT_CLASSIFICATION InputSlotClass;
    u32 InstanceDataStepRate;
} [[static]];

struct RvmSerializedDb_ns_CombinedSerializedParameterBlock
{
    u64;
    u64;
} [[static]];

struct RvmSerializedDb_ns_ParamDbSerializedReadView
{
    u64;
    u64;
} [[static]];

struct RvmSerializedDb_ns_Dx11ApplyParametersInstructionData
{
    u64;
    u64;
} [[static]];

struct RvmSerializedDb_ns_RvmFunctionInstanceRef
{
    u64;
} [[static]];

struct ParamDbHash
{
    u64 hash;
} [[static]];

struct RvmSerializedDb_ns_Dx11BufferConversionInstructionData
{
    u64;
    u64;
} [[static]];

struct RvmSerializedDb_ns_Settings
{
    u32;
    u32;
    u64;
    u64;
} [[static]];

struct RvmSerializedDb_ns_ShaderStreamableExternalTextureRef
{
    u64;
} [[static]];

struct RvmSerializedDb_ns_Dx11DispatchInstructionData
{
    u64;
    u64;
} [[static]];

struct RvmSerializedDb_ns_ParamDbKeyRef
{
    u64;
} [[static]];

struct RvmSerializedDb_ns_RttiType
{
    u64;
    u64;
} [[static]];

struct RvmSerializedDb_ns_TableAssemblyInstructionBatchData
{
    u64;
    u64;
    u64;
} [[static]];

struct RvmSerializedDb_ns_RvmPermutationLookupTable
{
    // probably related to the 4 shader types (vs, ps, hs, ds)
    u16 unk1;
    u16 unk2;
    u16 unk3;
    u16 unk4;
} [[static]];

enum ShaderDepthBiasGroup : u32
{
	ShaderDepthBiasGroup_Default,
	ShaderDepthBiasGroup_Decal,
	ShaderDepthBiasGroup_EmitterOcclusion,
	ShaderDepthBiasGroup_EdgeModel,
	ShaderDepthBiasGroup_TerrainDecal,
	ShaderDepthBiasGroup_TerrainDecalZPass,
	ShaderDepthBiasGroup_DistantShadowCache_LowestBias,
	ShaderDepthBiasGroup_DistantShadowCache_LowerBias,
	ShaderDepthBiasGroup_DistantShadowCache_NormalBias,
	ShaderDepthBiasGroup_DistantShadowCache_HigherBias,
	ShaderDepthBiasGroup_DistantShadowCache_HighestBias,
	ShaderDepthBiasGroup_Shadow16Bit,
	ShaderDepthBiasGroup_Shadow24Bit,
	ShaderDepthBiasGroup_Shadow32Bit,
	ShaderDepthBiasGroup_ZPass,
	ShaderDepthBiasGroup_Emissive,
	ShaderDepthBiasGroup_VelocityVector,
	ShaderDepthBiasGroupCount
} [[static]];

struct RvmSerializedDb_ns_ShaderStreamableExternalTexture
{
    float texCoordsPerMeter;
    u16 unk1;
    u16 textureType;
    u32 nameHash;
    u32 externalHash; // guessing this has sth to do with external since the non externals have 0xFFFF here
} [[static]];

enum RenderDepthMode : u32
{
	RenderDepthMode_Disabled,
	RenderDepthMode_ReadOnly,
	RenderDepthMode_ReadAndWrite,
	RenderDepthMode_WriteOnly,
	RenderDepthMode_ReadOnlyInverted
} [[static]];

enum RvmLegacyOutdoorLightStatus : u32
{
	RvmLegacyOutdoorLightStatus_Disabled,
	RvmLegacyOutdoorLightStatus_NoMethod_LowQuality,
	RvmLegacyOutdoorLightStatus_NoMethod_HighQuality,
	RvmLegacyOutdoorLightStatus_CascadedBoxMethod_LowQuality,
	RvmLegacyOutdoorLightStatus_CascadedBoxMethod_HighQuality,
	RvmLegacyOutdoorLightStatusCount
} [[static]];

enum RvmLevelOfDetail : u32
{
	RvmLevelOfDetail_Low,
	RvmLevelOfDetail_High
} [[static]];

struct RvmSerializedDb_ns_ShaderStreamableTexture
{
    float texCoordsPerMeter;
    u16 unk1; // probably pixelformat or sth
    u16 textureType;
    u32 nameHash;
    u32 externalHash; // guessing this has sth to do with external since the externals have the nameHash here
} [[static]];

struct RvmSerializedDb_ns_InstanceTableAssemblyData
{
    u8 data[0x0C] [[sealed]];
} [[static]];

enum ShaderInstancingMethod : u32
{
	ShaderInstancingMethod_None,
	ShaderInstancingMethod_ObjectTransform4x3Half,
	ShaderInstancingMethod_ObjectTransform4x3InstanceData4x1Half,
	ShaderInstancingMethod_ObjectTransform4x3InstanceData4x2Half,
	ShaderInstancingMethod_WorldTransform4x3Float,
	ShaderInstancingMethod_WorldTransform4x3FloatInstanceData4x2Half,
	ShaderInstancingMethod_PrevWorldTransform4x3FloatInstanceData4x2Half,
	ShaderInstancingMethod_ObjectTranslationScaleHalf,
	ShaderInstancingMethod_ObjectTranslationScaleHalfInstanceData4x1Half,
	ShaderInstancingMethod_ObjectTranslationScaleHalfInstanceData4x2Half,
	ShaderInstancingMethod_PositionStream,
	ShaderInstancingMethod_PositionTbnStream,
	ShaderInstancingMethod_PrevPositionStream,
	ShaderInstancingMethod_LinearMediaStreaming,
	ShaderInstancingMethod_PositionStreamAux,
	ShaderInstancingMethod_DxBuffer,
	ShaderInstancingMethod_DxBufferInstanceData4x1Float,
	ShaderInstancingMethod_DxBufferInstanceData4x2Float,
	ShaderInstancingMethod_Manual,
	ShaderInstancingMethodCount
} [[static]];

enum ShaderRenderMode : u32
{
	ShaderRenderMode_Forward,
	ShaderRenderMode_ForwardSimple,
	ShaderRenderMode_ZOnly,
	ShaderRenderMode_DeferredShadingGBufferLayout0,
	ShaderRenderMode_DeferredShadingGBufferLayout1,
	ShaderRenderMode_DeferredShadingGBufferLayout2,
	ShaderRenderMode_DeferredShadingGBufferLayout3,
	ShaderRenderMode_Raytrace,
	ShaderRenderMode_RaytraceForward,
	ShaderRenderMode_ForwardEmissive,
	ShaderRenderMode_VelocityVector,
	ShaderRenderMode_MeshPick,
	ShaderRenderMode_DistortionVector,
	ShaderRenderMode_DebugMulti,
	ShaderRenderMode_ForwardOpaque_RuntimeOnly,
	ShaderRenderMode_TextureSpace,
	ShaderRenderMode_GBufferSimple,
	ShaderRenderModeCount
} [[static]];

enum PrimitiveType : u32
{
	PrimitiveType_PointList = 0,
	PrimitiveType_LineList = 1,
	PrimitiveType_LineStrip = 2,
	PrimitiveType_TriangleList = 3,
	PrimitiveType_TriangleStrip = 5,
	PrimitiveType_QuadList = 7,
	PrimitiveType_RectList = 8,
	PrimitiveType_TrianglePatch = 9,
	PrimitiveTypeCount = 10
} [[static]];

enum ShaderSkinningMethod : u32
{
	ShaderSkinningMethod_None = 0,
	ShaderSkinningMethod_Linear1Bone = 1,
	ShaderSkinningMethod_Linear2Bone = 2,
	ShaderSkinningMethod_Linear4Bone = 4,
	ShaderSkinningMethod_Linear6Bone = 6,
	ShaderSkinningMethod_Linear8Bone = 8,
	ShaderSkinningMethod_Null = 9,
	ShaderSkinningMethod_DualQuaternion4Bone = 10,
	ShaderSkinningMethodCount = 11
} [[static]];

enum ShaderGeometrySpace : u32
{
	ShaderGeometrySpace_Object,
	ShaderGeometrySpace_ObjectScaled,
	ShaderGeometrySpace_World,
	ShaderGeometrySpace_Screen,
	ShaderGeometrySpace_Manual,
	ShaderGeometrySpaceCount
} [[static]];

struct RvmSerializedDb_ns_TableAssemblyData
{
    u8 data[0x14] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_DepthBiasGroupData
{
    u8 data[0x88] [[sealed]];
} [[static]];

enum RvmIndexBufferFormat : u32
{
	RvmIndexBufferFormat_U16,
	RvmIndexBufferFormat_U32
} [[static]];

enum QualityLevel : u32
{
	QualityLevel_Low,
	QualityLevel_Medium,
	QualityLevel_High,
	QualityLevel_Ultra,
	QualityLevel_Cinematic,
	QualityLevel_All,
	QualityLevel_Invalid
} [[static]];

struct RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionData
{
    u8 data[0x0C] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_Dx11ViewStateInstructionData
{
    u8 data[0x14] [[sealed]];
} [[static]];

struct RvmLegacyInstructions_ns_LegacyInstructionData
{
    u8 data[0x14] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_WriteOp
{
    u8 data[0x08] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_CpuToGpuMatrixInstructionData
{
    u8 data[0x08] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_LodFadeInstructionData
{
    u8 data[0x14] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_Dx11ShaderDispatchDrawInstructionData
{
    u8 data[0x24] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_DefaultValueSimpleTexture
{
    u8 data[0x04] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_PreparedVertexStream
{
    u8 data[0x0C] [[sealed]];
} [[static]];

struct RvmContextSortKeyInfo
{
    u32;
} [[static]];

struct RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionData
{
    u8 data[0x0C] [[sealed]];
} [[static]];

struct RvmSlotHandle
{
    u32;
} [[static]];

struct RvmSerializedDb_ns_VectorSubtractInstructionData
{
    u8 data[0x0C] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_TessellationParametersInstructionData
{
    u8 data[0x38] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_DefaultValueZeroMem
{
    u8 data[0x02] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_Dx11LegacyDrawStateBuilderData
{
    u8 data[0x54] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_DefaultValueSimpleBuffer
{
    u8 data[0x02] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_SliceCountInstructionData
{
    u32;
} [[static]];

struct RvmSerializedDb_ns_DefaultValueStructLegacyData
{
    u8;
} [[static]];

struct RvmSerializedDb_ns_Dx11ByteCodeElement
{
    u8 data[0x100] [[sealed]];
} [[static]];

struct RvmSerializedDb_ns_WriteOpGroup
{
    u8 data[3] [[sealed]];
} [[static]];

struct Char
{
    char;
} [[static]];

struct RvmSerializedDb_ns_BaseShaderState
{
    u8 data[6] [[sealed]];
} [[static]];

enum NvShadowMapRenderType : u32
{
    Test
} [[static]];

struct RvmSerializedDb_ns_Dx11NvViewStateInstructionData
{
    u8 test;
} [[static]];

struct RvmSerializedDb_ns_Dx11NvViewStateDepthInstructionData
{
    u16 test;
} [[static]];

struct RvmSerializedDb_ns_Dx11NvDrawStateInstructionData
{
    u8 data[24] [[sealed]];
} [[static]];

struct RvmType
{
    match (h.types[currentType].nameHash)
    {
        (2051034773): Instance<NvShadowMapRenderType> nvshadowmaprendertype[h.types[currentType].count];
        (1486634195): Instance<RvmSerializedDb_ns_Dx11NvViewStateInstructionData> nv1[h.types[currentType].count];
        (1520331774): Instance<RvmSerializedDb_ns_Dx11NvViewStateDepthInstructionData> nv2[h.types[currentType].count];
        (4275256734): Instance<RvmSerializedDb_ns_Dx11NvDrawStateInstructionData> nv3[h.types[currentType].count];
        (0x04E8CA7D): Instance<RvmSerializedDb_ns_Dx11SerializedBlendState> dx11serializedblendstate[h.types[currentType].count] [[static]];
        (0x0B87FA95): Instance<Vec> vec[h.types[currentType].count];
        (0x1CEE5632): Instance<RvmLegacyLightMapInstance> rvmlegacylightmapinstance[h.types[currentType].count];
        (0x23166F15): Instance<RvmSerializedDb_ns_Dx11VsShader> dx11vsshader[h.types[currentType].count];
        (0x2EFE3369): Instance<RvmSerializedDb_ns_SerializedParamDbKey> serializedparamdbkey[h.types[currentType].count];
        (0x7416FA4F): Instance<ProjectionState> projectionstate[h.types[currentType].count];
        (0xAB6472B8): Instance<StencilState> stencilstate[h.types[currentType].count];
        (0xC60DCB0D): Instance<RvmSerializedDb_ns_Dx11Sampler> dx11sampler[h.types[currentType].count];
        (0xDA99C37E): Instance<RvmLegacyLightProbes> rvmlegacylightprobes[h.types[currentType].count];
        (0xF0F92EDF): Instance<ViewState> viewstate[h.types[currentType].count];
        (0x01EC4994): Instance<RvmSerializedDb_ns_DefaultValueRef> defaultvalueref[h.types[currentType].count];
        (0x0247D1B0): Instance<RvmSerializedDb_ns_InstructionBatchRef> instructionbatchref[h.types[currentType].count];
        (0x057BA51E): Instance<RvmSerializedDb_ns_Dx11ApplyParametersBlock> dx11applyparametersblock[h.types[currentType].count];
        (0x0746CEFA): Instance<RvmSerializedDb_ns_RvmPermutation> rvmpermutation[h.types[currentType].count];
        (0x0CC971F4): Instance<s64> int64[h.types[currentType].count];
        (0x0D9B2921): Instance<RvmSerializedDb_ns_InstructionBatch> instructionbatch[h.types[currentType].count];
        (0x140AFC4E): Instance<RvmSerializedDb_ns_RvmFunction> rvmfunction[h.types[currentType].count];
        (0x16FDAA58): Instance<RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchData> instancetableassemblyinstructionbatchdata[h.types[currentType].count];
        (0x20149F19): Instance<RvmSerializedDb_ns_SurfaceShader> surfaceshader[h.types[currentType].count];
        (0x26B25214): Instance<RvmSerializedDb_ns_Dx11LegacyVertexBufferConversionInstructionData> dx11legacyvertexbufferconversioninstructiondata[h.types[currentType].count];
        (0x2A49CA57): Instance<RvmSerializedDb_ns_SerializedParameterBlock> serializedparameterblock[h.types[currentType].count];
        (0x364CB5D9): Instance<RvmSerializedDb_ns_RvmFunctionInputTableIndices> rvmfunctioninputtableindices[h.types[currentType].count];
        (0x500FE453): Instance<RvmSerializedDb_ns_Dx11PsShader> dx11psshader[h.types[currentType].count];
        (0x5E9D8F4B): Instance<RvmSerializedDb_ns_Dx11HsShader> dx11hsshader[h.types[currentType].count];
        (0x609E0E06): Instance<RvmSerializedDb_ns_SerializedParameterBlockRef> serializedparameterblockref[h.types[currentType].count];
        (0x687DB53F): Instance<RvmSerializedDb_ns_Dx11BlendStateData> dx11blendstatedata[h.types[currentType].count];
        (0x6D3403B6): Instance<RvmSerializedDb_ns_DirectInputInstructionData> directinputinstructiondata[h.types[currentType].count];
        (0x6FBE3136): Instance<RvmSerializedDb_ns_Dx11TextureConversionInstructionData> dx11textureconversioninstructiondata[h.types[currentType].count];
        (0x7091E0C7): Instance<RvmSerializedDb_ns_Dx11DsShader> dx11dsshader[h.types[currentType].count];
        (0x77CB507F): Instance<RvmSerializedDb_ns_ValueRef> valueref[h.types[currentType].count];
        (0x7B05DCCB): Instance<RvmSerializedDb_ns_RvmPermutationRef> rvmpermutationref[h.types[currentType].count];
        (0x7B05F918): Instance<RvmSerializedDb_ns_RvmPermutationSet> rvmpermutationset[h.types[currentType].count];
        (0x8487EB54): Instance<RvmSerializedDb_ns_ShaderStreamableTextureRef> shaderstreamabletextureref[h.types[currentType].count];
        (0x8FD7EA81): Instance<RvmSerializedDb_ns_ParamDbSerializedHashView> paramdbserializedhashview[h.types[currentType].count];
        (0x9005919C): Instance<RvmSerializedDb_ns_RvmDispatch> rvmdispatch[h.types[currentType].count];
        (0x937C3AE7): Instance<RvmSerializedDb_ns_RvmFunctionInstance> rvmfunctioninstance[h.types[currentType].count];
        (0x950FBF90): Instance<RvmSerializedDb_ns_ParamDbSerializedHashViewRef> paramdbserializedhashviewref[h.types[currentType].count];
        (0x9940EC93): Instance<RvmSerializedDb_ns_ParamDbSerializedFilterView> paramdbserializedfilterview[h.types[currentType].count];
        (0x99DB2C67): Instance<RvmSerializedDb_ns_RuntimeInstantiatedType> runtimeinstantiatedtype[h.types[currentType].count];
        (0xB46704F1): Instance<RvmSerializedDb_ns_Dx11InputElement> dx11inputelement[h.types[currentType].count];
        (0xC4450BB2): Instance<RvmSerializedDb_ns_CombinedSerializedParameterBlock> combinedserializedparameterblock[h.types[currentType].count];
        (0xC82F8081): Instance<RvmSerializedDb_ns_ParamDbSerializedReadView> paramdbserializedreadview[h.types[currentType].count];
        (0xCE79D69F): Instance<RvmSerializedDb_ns_Dx11ApplyParametersInstructionData> dx11applyparametersinstructiondata[h.types[currentType].count];
        (0xD247D436): Instance<RvmSerializedDb_ns_RvmFunctionInstanceRef> rvmfunctioninstanceref[h.types[currentType].count];
        (0xD43A4A5E): Instance<ParamDbHash> paramdbhash[h.types[currentType].count];
        (0xD5F9FA09): Instance<RvmSerializedDb_ns_Dx11BufferConversionInstructionData> dx11bufferconversioninstructiondata[h.types[currentType].count];
        (0xDAE7CC60): Instance<RvmSerializedDb_ns_Settings> settings[h.types[currentType].count];
        (0xDD2446A9): Instance<RvmSerializedDb_ns_ShaderStreamableExternalTextureRef> shaderstreamableexternaltextureref[h.types[currentType].count];
        (0xDF7CA221): Instance<RvmSerializedDb_ns_Dx11DispatchInstructionData> dx11dispatchinstructiondata[h.types[currentType].count];
        (0xDFA807CA): Instance<RvmSerializedDb_ns_ParamDbKeyRef> paramdbkeyref[h.types[currentType].count];
        (0xF17B1BA6): Instance<RvmSerializedDb_ns_RttiType> rttitype[h.types[currentType].count];
        (0xFBB62E91): Instance<RvmSerializedDb_ns_TableAssemblyInstructionBatchData> tableassemblyinstructionbatchdata[h.types[currentType].count];
        (0xFDF2A466): Instance<RvmSerializedDb_ns_RvmPermutationLookupTable> rvmpermutationlookuptable[h.types[currentType].count];
        (0x0CC97257): Instance<s32> int32[h.types[currentType].count];
        (0x191C1D07): Instance<ShaderDepthBiasGroup> shaderdepthbiasgroup[h.types[currentType].count];
        (0x1E5FBC18): Instance<RvmSerializedDb_ns_ShaderStreamableExternalTexture> shaderstreamableexternaltexture[h.types[currentType].count];
        (0x1ECE9401): Instance<RenderDepthMode> renderdepthmode[h.types[currentType].count];
        (0x2DD5FBAB): Instance<RvmLegacyOutdoorLightStatus> rvmlegacyoutdoorlightstatus[h.types[currentType].count];
        (0x3AD97822): Instance<RvmLevelOfDetail> rvmlevelofdetail[h.types[currentType].count];
        (0x518DED45): Instance<RvmSerializedDb_ns_ShaderStreamableTexture> shaderstreamabletexture[h.types[currentType].count];
        (0x6C42BB1C): Instance<RvmSerializedDb_ns_InstanceTableAssemblyData> instancetableassemblydata[h.types[currentType].count];
        (0x7F39A7B4): Instance<float> float32[h.types[currentType].count];
        (0x85EA841F): Instance<ShaderInstancingMethod> shaderinstancingmethod[h.types[currentType].count];
        (0x921C7685): Instance<ShaderRenderMode> shaderrendermode[h.types[currentType].count];
        (0x963FC9FC): Instance<PrimitiveType> primitivetype[h.types[currentType].count];
        (0xAB211D82): Instance<ShaderSkinningMethod> shaderskinningmethod[h.types[currentType].count];
        (0xB0BC3C22): Instance<u32> uint32[h.types[currentType].count];
        (0xCBEB8BF2): Instance<ShaderGeometrySpace> shadergeometryspace[h.types[currentType].count];
        (0xD80E5015): Instance<RvmSerializedDb_ns_TableAssemblyData> tableassemblydata[h.types[currentType].count];
        (0xD946931E): Instance<RvmSerializedDb_ns_DepthBiasGroupData> depthbiasgroupdata[h.types[currentType].count];
        (0xDF54A891): Instance<RvmIndexBufferFormat> rvmindexbufferformat[h.types[currentType].count];
        (0xF004D71E): Instance<QualityLevel> qualitylevel[h.types[currentType].count];
        (0x024C0AA3): Instance<RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionData> packlightmapweightintoinstanceinstructiondata[h.types[currentType].count];
        (0x30FEB5EB): Instance<RvmSerializedDb_ns_Dx11ViewStateInstructionData> dx11viewstateinstructiondata[h.types[currentType].count];
        (0x373F60B2): Instance<RvmLegacyInstructions_ns_LegacyInstructionData> rvmlegacyinstructions_ns_legacyinstructiondata[h.types[currentType].count];
        (0x38349F07): Instance<RvmSerializedDb_ns_WriteOp> writeop[h.types[currentType].count];
        (0x3EC9F4C9): Instance<RvmSerializedDb_ns_CpuToGpuMatrixInstructionData> cputogpumatrixinstructiondata[h.types[currentType].count];
        (0x3FFF936C): Instance<RvmSerializedDb_ns_LodFadeInstructionData> lodfadeinstructiondata[h.types[currentType].count];
        (0x48EF6EC8): Instance<RvmSerializedDb_ns_Dx11ShaderDispatchDrawInstructionData> dx11shaderdispatchdrawinstructiondata[h.types[currentType].count];
        (0x4CC78E74): Instance<RvmSerializedDb_ns_DefaultValueSimpleTexture> defaultvaluesimpletexture[h.types[currentType].count];
        (0x577F1194): Instance<RvmSerializedDb_ns_PreparedVertexStream> preparedvertexstream[h.types[currentType].count];
        (0x6034ECD0): Instance<RvmContextSortKeyInfo> rvmcontextsortkeyinfo[h.types[currentType].count];
        (0x64670697): Instance<RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionData> offsettranslationinmatrixinstructiondata[h.types[currentType].count];
        (0x6589C9E2): Instance<RvmSlotHandle> rvmslothandle[h.types[currentType].count];
        (0x8FD6EAC0): Instance<RvmSerializedDb_ns_VectorSubtractInstructionData> vectorsubtractinstructiondata[h.types[currentType].count];
        (0xAFD54D5E): Instance<RvmSerializedDb_ns_TessellationParametersInstructionData> tessellationparametersinstructiondata[h.types[currentType].count];
        (0xB0BC3C64): Instance<u16> uint16[h.types[currentType].count];
        (0xB1FA00E2): Instance<RvmSerializedDb_ns_DefaultValueZeroMem> defaultvaluezeromem[h.types[currentType].count];
        (0xCBC56BCA): Instance<RvmSerializedDb_ns_Dx11LegacyDrawStateBuilderData> dx11legacydrawstatebuilderdata[h.types[currentType].count];
        (0xE18E74EB): Instance<RvmSerializedDb_ns_DefaultValueSimpleBuffer> defaultvaluesimplebuffer[h.types[currentType].count];
        (0xFF1CDF1E): Instance<RvmSerializedDb_ns_SliceCountInstructionData> slicecountinstructiondata[h.types[currentType].count];
        (0x0D1CFA1B): Instance<u8> uint8[h.types[currentType].count];
        (0x5AF184F7): Instance<RvmSerializedDb_ns_DefaultValueStructLegacyData> defaultvaluestructlegacydata[h.types[currentType].count];
        (0x602A31A0): Instance<RvmSerializedDb_ns_Dx11ByteCodeElement> dx11bytecodeelement[h.types[currentType].count];
        (0x685C8B18): Instance<RvmSerializedDb_ns_WriteOpGroup> writeopgroup[h.types[currentType].count];
        (0x7C70E6DD): Instance<Char> strings[h.types[currentType].count];
        (0x920A778E): Instance<RvmSerializedDb_ns_BaseShaderState> baseshaderstate[h.types[currentType].count];
        (0x9638B221): Instance<bool> boolean[h.types[currentType].count];
        (_): std::error(std::format("{}", h.types[currentType].nameHash));
    }
    currentType = currentType + 1;
};

struct RvmDatabase
{
    RvmType types[h.typeCount];
};

u64 current = 0;
u32 currentType = 0;
Meta m @0;
Header h @(m.offset + 0x10);
RvmDatabase res @0x10;
