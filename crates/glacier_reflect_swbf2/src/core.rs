use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_core_types(registry: &mut TypeRegistry) {
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCCOMPUTEPSOINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGDISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGBUFFERINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGPARAMSINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCVERTEXBUFFERINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCSAMPLERPOINTER_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCSAMPLERTABLEWRITERINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCSHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCDISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCPSOPRELOADOP_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCSAMPLER_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12PCROOTSIGNATURE_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12VERTEXBUFFERVIEWINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(D3D12_GPU_DESCRIPTOR_HANDLE_ARRAY_TYPE_INFO);
    registry.register_type(D3D12_VERTEX_BUFFER_VIEW_ARRAY_TYPE_INFO);
    registry.register_type(DX12RVMINPUTELEMENT_ARRAY_TYPE_INFO);
    registry.register_type(DX12RVMSHADER_ARRAY_TYPE_INFO);
    registry.register_type(DX12RVMROOTSIGNATURE_ARRAY_TYPE_INFO);
    registry.register_type(D3D12_VIEWPORT_ARRAY_TYPE_INFO);
    registry.register_type(D3D12_PIPELINE_STATE_FLAGS_ARRAY_TYPE_INFO);
    registry.register_type(D3D12_CACHED_PIPELINE_STATE_ARRAY_TYPE_INFO);
    registry.register_type(DXGI_SAMPLE_DESC_ARRAY_TYPE_INFO);
    registry.register_type(D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_ARRAY_TYPE_INFO);
    registry.register_type(DXGI_FORMAT_ARRAY_TYPE_INFO);
    registry.register_type(D3D12_PRIMITIVE_TOPOLOGY_ARRAY_TYPE_INFO);
    registry.register_type(D3D12_PRIMITIVE_TOPOLOGY_TYPE_ARRAY_TYPE_INFO);
    registry.register_type(DX12RVMSODECLARATIONENTRY_ARRAY_TYPE_INFO);
    registry.register_type(DX12RVMDEPTHSTENCILDESC_ARRAY_TYPE_INFO);
    registry.register_type(DX12RVMRASTERIZERDESC_ARRAY_TYPE_INFO);
    registry.register_type(DX12RVMBLENDDESC_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYDEBUGINFO_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12BLENDSTATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12ROOTWRITEOP_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12SHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12ROOTSIGNATUREDEBUGINFO_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12TABLEDEBUGINFO_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12TABLEDEBUGENTRY_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12INPUTELEMENT_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12SHADER_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12BINARYBLOB_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12SHADERSTATE_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(D3D11_CONSERVATIVE_RASTERIZATION_MODE_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMVIEWPORTS_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMSCISSORRECTS_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMDEPTHSTENCILSTATE_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMBLENDSTATE_ARRAY_TYPE_INFO);
    registry.register_type(D3D11_CULL_MODE_ARRAY_TYPE_INFO);
    registry.register_type(D3D11_FILL_MODE_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMSAMPLER_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMVSSHADER_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMPSSHADER_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMHSSHADER_ARRAY_TYPE_INFO);
    registry.register_type(DX11RVMDSSHADER_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11BYTECODEELEMENT_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11DSSHADER_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11HSSHADER_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11PSSHADER_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11VSSHADER_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11INPUTELEMENT_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11SAMPLER_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11BLENDSTATEDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11SERIALIZEDBLENDSTATE_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11TEXTURECONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11BUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11LEGACYDRAWSTATEBUILDERDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHLEGACYDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSBLOCK_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11DISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_LODFADEINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SLICECOUNTINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_TESSELLATIONPARAMETERSINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_VECTORSUBTRACTINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_OFFSETTRANSLATIONINMATRIXINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_CPUTOGPUMATRIXINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_FLOATTOVECINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_VERTEXSTREAMPROCESSINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMPOINTER_ARRAY_TYPE_INFO);
    registry.register_type(GPUMAT4X3_ARRAY_TYPE_INFO);
    registry.register_type(IVEC4_ARRAY_TYPE_INFO);
    registry.register_type(IVEC3_ARRAY_TYPE_INFO);
    registry.register_type(IVEC2_ARRAY_TYPE_INFO);
    registry.register_type(HALF4_ARRAY_TYPE_INFO);
    registry.register_type(HALF3_ARRAY_TYPE_INFO);
    registry.register_type(HALF2_ARRAY_TYPE_INFO);
    registry.register_type(HALF_ARRAY_TYPE_INFO);
    registry.register_type(RVMLEGACYINSTRUCTIONS_NS_LEGACYINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DEPTHBIASGROUPDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_TABLEASSEMBLYINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_TABLEASSEMBLYDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_WRITEOPGROUP_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_WRITEOP_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DIRECTINPUTINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_CONSTANTVALUEINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(CHAR_ARRAY_TYPE_INFO);
    registry.register_type(PARAMDBHASH_ARRAY_TYPE_INFO);
    registry.register_type(RVMENCODEDTABLEENTRY_ARRAY_TYPE_INFO);
    registry.register_type(RVMCONTEXTSORTKEYINFO_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PREPAREDVERTEXELEMENT_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PREPAREDVERTEXSTREAM_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SERIALIZEDPARAMDBKEY_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_BASESHADERSTATE_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RUNTIMEINSTANTIATEDTYPE_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDMULTIHASHVIEW_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDREADVIEW_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEWREF_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEW_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDFILTERVIEW_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_COMBINEDSERIALIZEDPARAMETERBLOCK_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCKREF_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCK_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTUREREF_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTUREREF_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SURFACESHADERDEBUGINFO_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SURFACESHADER_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTURE_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTURE_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCEREF_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCE_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_SETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMPERMUTATIONSET_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMPERMUTATIONLOOKUPTABLE_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMPERMUTATIONTREE_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMPERMUTATIONREF_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMPERMUTATION_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMFUNCTIONINPUTTABLEINDICES_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMDISPATCHDEBUGINFO_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMFUNCTIONDEBUGINFO_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMDISPATCH_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RVMFUNCTION_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_INSTRUCTIONBATCH_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_RTTITYPE_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DEFAULTVALUEREF_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_VALUEREF_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DEFAULTVALUEZEROMEM_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DEFAULTVALUESTRUCTLEGACYDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLETEXTURE_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLEBUFFER_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_INSTRUCTIONBATCHREF_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PARAMDBKEYREF_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_PARAMDBKEYDESC_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTTLASNULLDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTTLASDYNAMICDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTTLASSTATICDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTTLASBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTSHADERTABLERECORDWRITERDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTNULLHITSHADERDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTHITCOLLECTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTSTATICBLASBUILDDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTDYNAMICBLASBUILDDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTBLASBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTHITSHADERCONSTANTDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTINDEXBUFFERDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTSORTDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTDISPATCHDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12RTCOLLECTIONPRELOADOP_ARRAY_TYPE_INFO);
    registry.register_type(RVMLEGACYLIGHTMAPINSTANCE_ARRAY_TYPE_INFO);
    registry.register_type(RVMLEGACYLIGHTPROBES_ARRAY_TYPE_INFO);
    registry.register_type(RVMLEGACYPERMUTATIONDEBUGINFO_ARRAY_TYPE_INFO);
    registry.register_type(RVMLEGACYFORWARDLIGHTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(FRUSTUMSOA_ARRAY_TYPE_INFO);
    registry.register_type(LODVIEWSTATE_ARRAY_TYPE_INFO);
    registry.register_type(TESSELLATIONVIEWSTATE_ARRAY_TYPE_INFO);
    registry.register_type(STENCILSTATE_ARRAY_TYPE_INFO);
    registry.register_type(REFLECTIONSTATE_ARRAY_TYPE_INFO);
    registry.register_type(FOGSTATE_ARRAY_TYPE_INFO);
    registry.register_type(PROJECTIONSTATE_ARRAY_TYPE_INFO);
    registry.register_type(VIEWSTATE_ARRAY_TYPE_INFO);
    registry.register_type(OUTDOORLIGHTSTATE_ARRAY_TYPE_INFO);
    registry.register_type(DX12NVRVMROOTSIGNATURE_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12NVCONSTANTBUFFERASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLE_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX12NVRVMROOTSIGNATURE_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEDEPTHINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11NVDRAWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(RADIOSITYTYPEOVERRIDE_TYPE_INFO);
    registry.register_type(RADIOSITYTYPEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(LOCALPLAYERID_TYPE_INFO);
    registry.register_type(LOCALPLAYERID_ARRAY_TYPE_INFO);
    registry.register_type(NODEGRAPH_TYPE_INFO);
    registry.register_type(NODEGRAPH_ARRAY_TYPE_INFO);
    registry.register_type(NODE_TYPE_INFO);
    registry.register_type(NODE_ARRAY_TYPE_INFO);
    registry.register_type(GAMEPLATFORM_TYPE_INFO);
    registry.register_type(GAMEPLATFORM_ARRAY_TYPE_INFO);
    registry.register_type(DESIGNERENUMERATIONSORTTYPE_TYPE_INFO);
    registry.register_type(DESIGNERENUMERATIONSORTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(CORESETTINGS_TYPE_INFO);
    registry.register_type(CORESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(LOGSETTINGS_TYPE_INFO);
    registry.register_type(LOGSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(WRITERSETTINGS_TYPE_INFO);
    registry.register_type(WRITERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(CHANNELSETTINGS_TYPE_INFO);
    registry.register_type(CHANNELSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(STREAMINSTALLFROMBLURAYTYPE_TYPE_INFO);
    registry.register_type(STREAMINSTALLFROMBLURAYTYPE_ARRAY_TYPE_INFO);
    registry.register_type(HARDWAREPROFILE_TYPE_INFO);
    registry.register_type(HARDWAREPROFILE_ARRAY_TYPE_INFO);
    registry.register_type(CORELOGLEVEL_TYPE_INFO);
    registry.register_type(CORELOGLEVEL_ARRAY_TYPE_INFO);
    registry.register_type(CONTENTPRESET_TYPE_INFO);
    registry.register_type(CONTENTPRESET_ARRAY_TYPE_INFO);
    registry.register_type(CONTENTPRESETCONTAINER_TYPE_INFO);
    registry.register_type(CONTENTPRESETCONTAINER_ARRAY_TYPE_INFO);
    registry.register_type(TIMESPAN_TYPE_INFO);
    registry.register_type(TIMESPAN_ARRAY_TYPE_INFO);
    registry.register_type(THREADSETTINGS_TYPE_INFO);
    registry.register_type(THREADSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(TEMPLATEDESCRIPTORREGISTRYASSET_TYPE_INFO);
    registry.register_type(TEMPLATEDESCRIPTORREGISTRYASSET_ARRAY_TYPE_INFO);
    registry.register_type(TEMPLATEDESCRIPTORDATA_TYPE_INFO);
    registry.register_type(TEMPLATEDESCRIPTORDATA_ARRAY_TYPE_INFO);
    registry.register_type(TEMPLATEEXPOSEDOBJECT_TYPE_INFO);
    registry.register_type(TEMPLATEEXPOSEDOBJECT_ARRAY_TYPE_INFO);
    registry.register_type(TEMPLATEASSET_TYPE_INFO);
    registry.register_type(TEMPLATEASSET_ARRAY_TYPE_INFO);
    registry.register_type(TEMPLATETARGET_TYPE_INFO);
    registry.register_type(TEMPLATETARGET_ARRAY_TYPE_INFO);
    registry.register_type(TEMPLATE_TYPE_INFO);
    registry.register_type(TEMPLATE_ARRAY_TYPE_INFO);
    registry.register_type(SUPERBUNDLELAYOUTSETTINGS_TYPE_INFO);
    registry.register_type(SUPERBUNDLELAYOUTSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(INSTALLPACKAGELAYERTYPE_TYPE_INFO);
    registry.register_type(INSTALLPACKAGELAYERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(INSTALLPACKAGETYPE_TYPE_INFO);
    registry.register_type(INSTALLPACKAGETYPE_ARRAY_TYPE_INFO);
    registry.register_type(PLATFORMSCALABLEBOOL_TYPE_INFO);
    registry.register_type(PLATFORMSCALABLEBOOL_ARRAY_TYPE_INFO);
    registry.register_type(PLATFORMSCALABLEFLOAT_TYPE_INFO);
    registry.register_type(PLATFORMSCALABLEFLOAT_ARRAY_TYPE_INFO);
    registry.register_type(PLATFORMSCALABLEINT_TYPE_INFO);
    registry.register_type(PLATFORMSCALABLEINT_ARRAY_TYPE_INFO);
    registry.register_type(QUALITYSCALABLEENABLED_TYPE_INFO);
    registry.register_type(QUALITYSCALABLEENABLED_ARRAY_TYPE_INFO);
    registry.register_type(QUALITYSCALABLEBOOL_TYPE_INFO);
    registry.register_type(QUALITYSCALABLEBOOL_ARRAY_TYPE_INFO);
    registry.register_type(QUALITYSCALABLEFLOAT_TYPE_INFO);
    registry.register_type(QUALITYSCALABLEFLOAT_ARRAY_TYPE_INFO);
    registry.register_type(QUALITYSCALABLEINT_TYPE_INFO);
    registry.register_type(QUALITYSCALABLEINT_ARRAY_TYPE_INFO);
    registry.register_type(QUALITYLEVEL_TYPE_INFO);
    registry.register_type(QUALITYLEVEL_ARRAY_TYPE_INFO);
    registry.register_type(RESOURCEMANAGERSETTINGS_TYPE_INFO);
    registry.register_type(RESOURCEMANAGERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(RENDERINGOVERRIDES_TYPE_INFO);
    registry.register_type(RENDERINGOVERRIDES_ARRAY_TYPE_INFO);
    registry.register_type(CHUNKONLYRESOURCETYPE_TYPE_INFO);
    registry.register_type(CHUNKONLYRESOURCETYPE_ARRAY_TYPE_INFO);
    registry.register_type(IRESOURCEOBJECT_TYPE_INFO);
    registry.register_type(IRESOURCEOBJECT_ARRAY_TYPE_INFO);
    registry.register_type(PROXYRESOURCEOBJECT_TYPE_INFO);
    registry.register_type(PROXYRESOURCEOBJECT_ARRAY_TYPE_INFO);
    registry.register_type(BOOLOVERRIDE_TYPE_INFO);
    registry.register_type(BOOLOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(DATACONTAINERPOLICYPIPELINERESULTBASE_TYPE_INFO);
    registry.register_type(DATACONTAINERPOLICYPIPELINERESULTBASE_ARRAY_TYPE_INFO);
    registry.register_type(SETDATARESULTLISTENER_TYPE_INFO);
    registry.register_type(SETDATARESULTLISTENER_ARRAY_TYPE_INFO);
    registry.register_type(DATABUSDATA_TYPE_INFO);
    registry.register_type(DATABUSDATA_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICDATACONTAINER_TYPE_INFO);
    registry.register_type(DYNAMICDATACONTAINER_ARRAY_TYPE_INFO);
    registry.register_type(DATAFIELD_TYPE_INFO);
    registry.register_type(DATAFIELD_ARRAY_TYPE_INFO);
    registry.register_type(FIELDACCESSTYPE_TYPE_INFO);
    registry.register_type(FIELDACCESSTYPE_ARRAY_TYPE_INFO);
    registry.register_type(LINKCONNECTION_TYPE_INFO);
    registry.register_type(LINKCONNECTION_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYCONNECTION_TYPE_INFO);
    registry.register_type(PROPERTYCONNECTION_ARRAY_TYPE_INFO);
    registry.register_type(INPUTPROPERTYTYPE_TYPE_INFO);
    registry.register_type(INPUTPROPERTYTYPE_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYCONNECTIONTARGETTYPE_TYPE_INFO);
    registry.register_type(PROPERTYCONNECTIONTARGETTYPE_ARRAY_TYPE_INFO);
    registry.register_type(SETTINGSBUNDLEASSET_TYPE_INFO);
    registry.register_type(SETTINGSBUNDLEASSET_ARRAY_TYPE_INFO);
    registry.register_type(SYSTEMSETTINGS_TYPE_INFO);
    registry.register_type(SYSTEMSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(DATABUSPEER_TYPE_INFO);
    registry.register_type(DATABUSPEER_ARRAY_TYPE_INFO);
    registry.register_type(GAMEDATACONTAINER_TYPE_INFO);
    registry.register_type(GAMEDATACONTAINER_ARRAY_TYPE_INFO);
    registry.register_type(REALM_TYPE_INFO);
    registry.register_type(REALM_ARRAY_TYPE_INFO);
    registry.register_type(LANGUAGEFORMAT_TYPE_INFO);
    registry.register_type(LANGUAGEFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(DATACONTAINERPOLICYASSET_TYPE_INFO);
    registry.register_type(DATACONTAINERPOLICYASSET_ARRAY_TYPE_INFO);
    registry.register_type(ASSET_TYPE_INFO);
    registry.register_type(ASSET_ARRAY_TYPE_INFO);
    registry.register_type(NATIVEFUNCTIONTYPEINFOASSET_TYPE_INFO);
    registry.register_type(NATIVEFUNCTIONTYPEINFOASSET_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDSTRUCTVALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDSTRUCTVALUE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDREFVALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDREFVALUE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDTYPEREFVALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDTYPEREFVALUE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDSHA1VALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDSHA1VALUE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDGUIDVALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDGUIDVALUE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDENUMVALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDENUMVALUE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDSTRINGVALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDSTRINGVALUE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDFLOAT64VALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDFLOAT64VALUE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDFLOAT32VALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDFLOAT32VALUE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDUINT64VALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDUINT64VALUE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDUINT32VALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDUINT32VALUE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDUINT16VALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDUINT16VALUE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDUINT8VALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDUINT8VALUE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDINT64VALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDINT64VALUE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDINT32VALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDINT32VALUE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDINT16VALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDINT16VALUE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDINT8VALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDINT8VALUE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDBOOLEANVALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDBOOLEANVALUE_ARRAY_TYPE_INFO);
    registry.register_type(PRIMITIVETYPEINFOASSET_TYPE_INFO);
    registry.register_type(PRIMITIVETYPEINFOASSET_ARRAY_TYPE_INFO);
    registry.register_type(ENUMTYPEINFOASSET_TYPE_INFO);
    registry.register_type(ENUMTYPEINFOASSET_ARRAY_TYPE_INFO);
    registry.register_type(ENUMTYPEENUMERATORDATA_TYPE_INFO);
    registry.register_type(ENUMTYPEENUMERATORDATA_ARRAY_TYPE_INFO);
    registry.register_type(FUNCTIONTYPEINFOASSET_TYPE_INFO);
    registry.register_type(FUNCTIONTYPEINFOASSET_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOPARAMETERDATACONTAINER_TYPE_INFO);
    registry.register_type(TYPEINFOPARAMETERDATACONTAINER_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOPARAMETERQUALIFIER_TYPE_INFO);
    registry.register_type(TYPEINFOPARAMETERQUALIFIER_ARRAY_TYPE_INFO);
    registry.register_type(FUNCTIONTYPEINFOREF_TYPE_INFO);
    registry.register_type(FUNCTIONTYPEINFOREF_ARRAY_TYPE_INFO);
    registry.register_type(VALUETYPEINFOASSET_TYPE_INFO);
    registry.register_type(VALUETYPEINFOASSET_ARRAY_TYPE_INFO);
    registry.register_type(CLASSINFOASSET_TYPE_INFO);
    registry.register_type(CLASSINFOASSET_ARRAY_TYPE_INFO);
    registry.register_type(CLASSINFOREF_TYPE_INFO);
    registry.register_type(CLASSINFOREF_ARRAY_TYPE_INFO);
    registry.register_type(COMPLEXTYPEINFOASSET_TYPE_INFO);
    registry.register_type(COMPLEXTYPEINFOASSET_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDCOLLECTIONREF_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDCOLLECTIONREF_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDCOLLECTION_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDCOLLECTION_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDVALUE_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDVALUE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDDATA_TYPE_INFO);
    registry.register_type(TYPEINFOFIELDDATA_ARRAY_TYPE_INFO);
    registry.register_type(ACCESSTYPE_TYPE_INFO);
    registry.register_type(ACCESSTYPE_ARRAY_TYPE_INFO);
    registry.register_type(PROTECTIONLEVEL_TYPE_INFO);
    registry.register_type(PROTECTIONLEVEL_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOREF_TYPE_INFO);
    registry.register_type(TYPEINFOREF_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOASSET_TYPE_INFO);
    registry.register_type(TYPEINFOASSET_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOATTRIBUTE_TYPE_INFO);
    registry.register_type(TYPEINFOATTRIBUTE_ARRAY_TYPE_INFO);
    registry.register_type(TYPEINFOATTRIBUTEARGUMENT_TYPE_INFO);
    registry.register_type(TYPEINFOATTRIBUTEARGUMENT_ARRAY_TYPE_INFO);
    registry.register_type(FUNCTIONDELEGATEREF_TYPE_INFO);
    registry.register_type(FUNCTIONDELEGATEREF_ARRAY_TYPE_INFO);
    registry.register_type(EVENTDELAY_TYPE_INFO);
    registry.register_type(EVENTDELAY_ARRAY_TYPE_INFO);
    registry.register_type(TIMINGVIEWSETTINGS_TYPE_INFO);
    registry.register_type(TIMINGVIEWSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(PERFJOURNALSETTINGS_TYPE_INFO);
    registry.register_type(PERFJOURNALSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(PERFHUDSETTINGS_TYPE_INFO);
    registry.register_type(PERFHUDSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(CORELOGUPDATESETTINGSMESSAGE_TYPE_INFO);
    registry.register_type(SPLITSCREENSETPRIMARYLOCALPLAYERIDMESSAGE_TYPE_INFO);
    registry.register_type(STREAMINSTALLREQUESTRESUMEMESSAGE_TYPE_INFO);
    registry.register_type(STREAMINSTALLREQUESTSUSPENDMESSAGE_TYPE_INFO);
    registry.register_type(STREAMINSTALLGROUPINSTALLATIONPROGRESSMESSAGE_TYPE_INFO);
    registry.register_type(STREAMINSTALLGROUPINSTALLEDMESSAGE_TYPE_INFO);
    registry.register_type(STREAMINSTALLINSTALLATIONPROGRESSMESSAGE_TYPE_INFO);
    registry.register_type(STREAMINSTALLINSTALLATIONFAILEDMESSAGE_TYPE_INFO);
    registry.register_type(STREAMINSTALLINSTALLATIONCOMPLETEDMESSAGE_TYPE_INFO);
    registry.register_type(COREAPPLICATIONACTIVATIONMESSAGE_TYPE_INFO);
    registry.register_type(APPLICATIONACTIVATIONSTATE_TYPE_INFO);
    registry.register_type(APPLICATIONACTIVATIONSTATE_ARRAY_TYPE_INFO);
    registry.register_type(CORECLEANUPMESSAGE_TYPE_INFO);
    registry.register_type(CORESUBLEVELSTARTSTREAMOUTMESSAGE_TYPE_INFO);
    registry.register_type(COREPANICMESSAGE_TYPE_INFO);
    registry.register_type(COREQUITMESSAGE_TYPE_INFO);
    registry.register_type(COREUPDATECLIPBOARDMESSAGE_TYPE_INFO);
    registry.register_type(COREMAINTHREADINITMESSAGE_TYPE_INFO);
    registry.register_type(COREHIBERNATEMESSAGE_TYPE_INFO);
    registry.register_type(COREQUITTINGINITIATEDMESSAGE_TYPE_INFO);
    registry.register_type(SPARSETRANSFORMARRAY_TYPE_INFO);
    registry.register_type(SPARSETRANSFORMARRAY_ARRAY_TYPE_INFO);
    registry.register_type(SPLINECURVE_TYPE_INFO);
    registry.register_type(SPLINECURVE_ARRAY_TYPE_INFO);
    registry.register_type(SPLINETYPE_TYPE_INFO);
    registry.register_type(SPLINETYPE_ARRAY_TYPE_INFO);
    registry.register_type(EVALUATEFLOATCURVE_FLOAT32_FLOAT32_FLOATCURVE__TYPE_INFO);
    registry.register_type(EVALUATEAUDIOCURVE_FLOAT32_FLOAT32_AUDIOCURVE__TYPE_INFO);
    registry.register_type(FLOATCURVECOLLECTIONASSET_TYPE_INFO);
    registry.register_type(FLOATCURVECOLLECTIONASSET_ARRAY_TYPE_INFO);
    registry.register_type(FLOATCURVE_TYPE_INFO);
    registry.register_type(FLOATCURVE_ARRAY_TYPE_INFO);
    registry.register_type(FLOATCURVETYPE_TYPE_INFO);
    registry.register_type(FLOATCURVETYPE_ARRAY_TYPE_INFO);
    registry.register_type(FLOATCURVEPOINT_TYPE_INFO);
    registry.register_type(FLOATCURVEPOINT_ARRAY_TYPE_INFO);
    registry.register_type(EFFECTCURVEPOINT_TYPE_INFO);
    registry.register_type(EFFECTCURVEPOINT_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOCURVE_TYPE_INFO);
    registry.register_type(AUDIOCURVE_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOCURVETYPE_TYPE_INFO);
    registry.register_type(AUDIOCURVETYPE_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOCURVEPOINT_TYPE_INFO);
    registry.register_type(AUDIOCURVEPOINT_ARRAY_TYPE_INFO);
    registry.register_type(FBASSERT_BOOLEAN__TYPE_INFO);
    registry.register_type(DEBUGRENDERSETTINGS_TYPE_INFO);
    registry.register_type(DEBUGRENDERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(GETSCREENHEIGHT_UINT32__TYPE_INFO);
    registry.register_type(GETSCREENWIDTH_UINT32__TYPE_INFO);
    registry.register_type(DRAWTEXT2D_INT32_INT32_FLOAT32_CSTRING__TYPE_INFO);
    registry.register_type(RUNTIMEMODULE_TYPE_INFO);
    registry.register_type(RUNTIMEMODULE_ARRAY_TYPE_INFO);
    registry.register_type(BOXEDVALUEREF_ARRAY_TYPE_INFO);
    registry.register_type(TYPEREF_ARRAY_TYPE_INFO);
    registry.register_type(DBOBJECT_ARRAY_TYPE_INFO);
    registry.register_type(DBOBJECT_TYPE_INFO);
    registry.register_type(FILEREF_ARRAY_TYPE_INFO);
    registry.register_type(STRING_ARRAY_TYPE_INFO);
    registry.register_type(STRING_TYPE_INFO);
    registry.register_type(CSTRING_ARRAY_TYPE_INFO);
    registry.register_type(VOID_ARRAY_TYPE_INFO);
    registry.register_type(VOID_TYPE_INFO);
    registry.register_type(RESOURCEREF_ARRAY_TYPE_INFO);
    registry.register_type(SHA1_ARRAY_TYPE_INFO);
    registry.register_type(GUID_ARRAY_TYPE_INFO);
    registry.register_type(FLOAT64_ARRAY_TYPE_INFO);
    registry.register_type(FLOAT32_ARRAY_TYPE_INFO);
    registry.register_type(INT64_ARRAY_TYPE_INFO);
    registry.register_type(UINT64_ARRAY_TYPE_INFO);
    registry.register_type(INT32_ARRAY_TYPE_INFO);
    registry.register_type(UINT32_ARRAY_TYPE_INFO);
    registry.register_type(INT16_ARRAY_TYPE_INFO);
    registry.register_type(UINT16_ARRAY_TYPE_INFO);
    registry.register_type(INT8_ARRAY_TYPE_INFO);
    registry.register_type(UINT8_ARRAY_TYPE_INFO);
    registry.register_type(BOOLEAN_ARRAY_TYPE_INFO);
    registry.register_type(AXISALIGNEDBOX_TYPE_INFO);
    registry.register_type(AXISALIGNEDBOX_ARRAY_TYPE_INFO);
    registry.register_type(MAT4_TYPE_INFO);
    registry.register_type(MAT4_ARRAY_TYPE_INFO);
    registry.register_type(LINEARTRANSFORM_TYPE_INFO);
    registry.register_type(LINEARTRANSFORM_ARRAY_TYPE_INFO);
    registry.register_type(PLANE_TYPE_INFO);
    registry.register_type(PLANE_ARRAY_TYPE_INFO);
    registry.register_type(QUAT_TYPE_INFO);
    registry.register_type(QUAT_ARRAY_TYPE_INFO);
    registry.register_type(VEC4_TYPE_INFO);
    registry.register_type(VEC4_ARRAY_TYPE_INFO);
    registry.register_type(VEC3_TYPE_INFO);
    registry.register_type(VEC3_ARRAY_TYPE_INFO);
    registry.register_type(VEC2_TYPE_INFO);
    registry.register_type(VEC2_ARRAY_TYPE_INFO);
    registry.register_type(VEC_TYPE_INFO);
    registry.register_type(VEC_ARRAY_TYPE_INFO);
    registry.register_type(EVENTDISPATCHER_TYPE_INFO);
    registry.register_type(EVENTDISPATCHER_ARRAY_TYPE_INFO);
    registry.register_type(DATACONTAINER_TYPE_INFO);
    registry.register_type(DATACONTAINER_ARRAY_TYPE_INFO);
}


pub static RVMSERIALIZEDDB_NS_DX12PCCOMPUTEPSOINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcComputePsoInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcComputePsoInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGDISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcCsSkinningDispatchInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcCsSkinningDispatchInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGBUFFERINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcCsSkinningBufferInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcCsSkinningBufferInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGPARAMSINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcCsSkinningParamsInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcCsSkinningParamsInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCVERTEXBUFFERINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcVertexBufferInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcVertexBufferInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCSAMPLERPOINTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcSamplerPointer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcSamplerPointer"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCSAMPLERTABLEWRITERINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcSamplerTableWriterInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcSamplerTableWriterInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcRvmDescriptorTableAssemblyInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcRvmDescriptorTableAssemblyInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCSHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcShaderDispatchDrawInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcShaderDispatchDrawInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCDISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcDispatchInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcDispatchInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCPSOPRELOADOP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcPSOPreloadOp-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcPSOPreloadOp"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCSAMPLER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcSampler-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcSampler"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCROOTSIGNATURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcRootSignature-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcRootSignature"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12VERTEXBUFFERVIEWINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12VertexBufferViewInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12VertexBufferViewInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12ViewStateInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12ViewStateInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static D3D12_GPU_DESCRIPTOR_HANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_GPU_DESCRIPTOR_HANDLE-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_GPU_DESCRIPTOR_HANDLE"),
    array_type: None,
    alignment: 8,
};



pub static D3D12_VERTEX_BUFFER_VIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_VERTEX_BUFFER_VIEW-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_VERTEX_BUFFER_VIEW"),
    array_type: None,
    alignment: 8,
};



pub static DX12RVMINPUTELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmInputElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmInputElement"),
    array_type: None,
    alignment: 8,
};



pub static DX12RVMSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmShader"),
    array_type: None,
    alignment: 8,
};



pub static DX12RVMROOTSIGNATURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmRootSignature-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmRootSignature"),
    array_type: None,
    alignment: 8,
};



pub static D3D12_VIEWPORT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_VIEWPORT-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_VIEWPORT"),
    array_type: None,
    alignment: 8,
};



pub static D3D12_PIPELINE_STATE_FLAGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_PIPELINE_STATE_FLAGS-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_PIPELINE_STATE_FLAGS"),
    array_type: None,
    alignment: 8,
};



pub static D3D12_CACHED_PIPELINE_STATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_CACHED_PIPELINE_STATE-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_CACHED_PIPELINE_STATE"),
    array_type: None,
    alignment: 8,
};



pub static DXGI_SAMPLE_DESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DXGI_SAMPLE_DESC-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DXGI_SAMPLE_DESC"),
    array_type: None,
    alignment: 8,
};



pub static D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_INDEX_BUFFER_STRIP_CUT_VALUE-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_INDEX_BUFFER_STRIP_CUT_VALUE"),
    array_type: None,
    alignment: 8,
};



pub static DXGI_FORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DXGI_FORMAT-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DXGI_FORMAT"),
    array_type: None,
    alignment: 8,
};



pub static D3D12_PRIMITIVE_TOPOLOGY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_PRIMITIVE_TOPOLOGY-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_PRIMITIVE_TOPOLOGY"),
    array_type: None,
    alignment: 8,
};



pub static D3D12_PRIMITIVE_TOPOLOGY_TYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_PRIMITIVE_TOPOLOGY_TYPE-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_PRIMITIVE_TOPOLOGY_TYPE"),
    array_type: None,
    alignment: 8,
};



pub static DX12RVMSODECLARATIONENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmSoDeclarationEntry-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmSoDeclarationEntry"),
    array_type: None,
    alignment: 8,
};



pub static DX12RVMDEPTHSTENCILDESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmDepthStencilDesc-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmDepthStencilDesc"),
    array_type: None,
    alignment: 8,
};



pub static DX12RVMRASTERIZERDESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmRasterizerDesc-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmRasterizerDesc"),
    array_type: None,
    alignment: 8,
};



pub static DX12RVMBLENDDESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmBlendDesc-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmBlendDesc"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyDebugInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyDebugInfo"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12BLENDSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12BlendStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12BlendStateData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12ROOTWRITEOP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootWriteOp-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RootWriteOp"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionBatchData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionBatchData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12SHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12ShaderDispatchDrawInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12ShaderDispatchDrawInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12ROOTSIGNATUREDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootSignatureDebugInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RootSignatureDebugInfo"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12TABLEDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12TableDebugInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12TableDebugInfo"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12TABLEDEBUGENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12TableDebugEntry-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12TableDebugEntry"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12INPUTELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12InputElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12InputElement"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12SHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12Shader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12Shader"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12BINARYBLOB_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12BinaryBlob-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12BinaryBlob"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12SHADERSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12ShaderState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12ShaderState"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ShaderDispatchDrawInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11ShaderDispatchDrawInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11LegacyVertexBufferConversionInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11LegacyVertexBufferConversionInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ViewStateInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11ViewStateInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static D3D11_CONSERVATIVE_RASTERIZATION_MODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D11_CONSERVATIVE_RASTERIZATION_MODE-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D11_CONSERVATIVE_RASTERIZATION_MODE"),
    array_type: None,
    alignment: 8,
};



pub static DX11RVMVIEWPORTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmViewports-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmViewports"),
    array_type: None,
    alignment: 8,
};



pub static DX11RVMSCISSORRECTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmScissorRects-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmScissorRects"),
    array_type: None,
    alignment: 8,
};



pub static DX11RVMDEPTHSTENCILSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmDepthStencilState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmDepthStencilState"),
    array_type: None,
    alignment: 8,
};



pub static DX11RVMBLENDSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBlendState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmBlendState"),
    array_type: None,
    alignment: 8,
};



pub static D3D11_CULL_MODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D11_CULL_MODE-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D11_CULL_MODE"),
    array_type: None,
    alignment: 8,
};



pub static D3D11_FILL_MODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D11_FILL_MODE-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D11_FILL_MODE"),
    array_type: None,
    alignment: 8,
};



pub static DX11RVMSAMPLER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmSampler-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmSampler"),
    array_type: None,
    alignment: 8,
};



pub static DX11RVMVSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmVsShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmVsShader"),
    array_type: None,
    alignment: 8,
};



pub static DX11RVMPSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmPsShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmPsShader"),
    array_type: None,
    alignment: 8,
};



pub static DX11RVMHSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmHsShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmHsShader"),
    array_type: None,
    alignment: 8,
};



pub static DX11RVMDSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmDsShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmDsShader"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11BYTECODEELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ByteCodeElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11ByteCodeElement"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11DSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11DsShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11DsShader"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11HSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11HsShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11HsShader"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11PSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11PsShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11PsShader"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11VSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11VsShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11VsShader"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11INPUTELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11InputElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11InputElement"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11SAMPLER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11Sampler-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11Sampler"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11BLENDSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11BlendStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11BlendStateData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11SERIALIZEDBLENDSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11SerializedBlendState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11SerializedBlendState"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11TEXTURECONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11TextureConversionInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11TextureConversionInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11BUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11BufferConversionInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11BufferConversionInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11LEGACYDRAWSTATEBUILDERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11LegacyDrawStateBuilderData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11LegacyDrawStateBuilderData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHLEGACYDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ShaderDispatchLegacyDrawInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11ShaderDispatchLegacyDrawInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ApplyParametersInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11ApplyParametersInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSBLOCK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ApplyParametersBlock-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11ApplyParametersBlock"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11DISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11DispatchInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11DispatchInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_LODFADEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_LodFadeInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_LodFadeInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SLICECOUNTINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SliceCountInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_SliceCountInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_TESSELLATIONPARAMETERSINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_TessellationParametersInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_TessellationParametersInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_VECTORSUBTRACTINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_VectorSubtractInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_VectorSubtractInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_OFFSETTRANSLATIONINMATRIXINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_CPUTOGPUMATRIXINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_CpuToGpuMatrixInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_CpuToGpuMatrixInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_FLOATTOVECINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_FloatToVecInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_FloatToVecInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_LegacyVertexBufferConversionInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_LegacyVertexBufferConversionInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_VERTEXSTREAMPROCESSINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_VertexStreamProcessInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_VertexStreamProcessInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ViewStateInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ViewStateInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMPOINTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmPointer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmPointer"),
    array_type: None,
    alignment: 8,
};



pub static GPUMAT4X3_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GpuMat4x3-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("GpuMat4x3"),
    array_type: None,
    alignment: 8,
};



pub static IVEC4_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVec4-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("IVec4"),
    array_type: None,
    alignment: 8,
};



pub static IVEC3_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVec3-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("IVec3"),
    array_type: None,
    alignment: 8,
};



pub static IVEC2_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVec2-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("IVec2"),
    array_type: None,
    alignment: 8,
};



pub static HALF4_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half4-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Half4"),
    array_type: None,
    alignment: 8,
};



pub static HALF3_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half3-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Half3"),
    array_type: None,
    alignment: 8,
};



pub static HALF2_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half2-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Half2"),
    array_type: None,
    alignment: 8,
};



pub static HALF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Half"),
    array_type: None,
    alignment: 8,
};



pub static RVMLEGACYINSTRUCTIONS_NS_LEGACYINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyInstructions_ns_LegacyInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmLegacyInstructions_ns_LegacyInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstanceTableAssemblyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_InstanceTableAssemblyData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DEPTHBIASGROUPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DepthBiasGroupData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DepthBiasGroupData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_TABLEASSEMBLYINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_TableAssemblyInstructionBatchData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_TableAssemblyInstructionBatchData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_TABLEASSEMBLYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_TableAssemblyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_TableAssemblyData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_WRITEOPGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_WriteOpGroup-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_WriteOpGroup"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_WRITEOP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_WriteOp-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_WriteOp"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DrawStateBuilderInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DrawStateBuilderInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DispatchInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DispatchInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DIRECTINPUTINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DirectInputInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DirectInputInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_CONSTANTVALUEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ConstantValueInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ConstantValueInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static CHAR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "char-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("char"),
    array_type: None,
    alignment: 8,
};



pub static PARAMDBHASH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParamDbHash-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ParamDbHash"),
    array_type: None,
    alignment: 8,
};



pub static RVMENCODEDTABLEENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmEncodedTableEntry-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmEncodedTableEntry"),
    array_type: None,
    alignment: 8,
};



pub static RVMCONTEXTSORTKEYINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmContextSortKeyInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmContextSortKeyInfo"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PREPAREDVERTEXELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_PreparedVertexElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_PreparedVertexElement"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PREPAREDVERTEXSTREAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_PreparedVertexStream-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_PreparedVertexStream"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SERIALIZEDPARAMDBKEY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SerializedParamDbKey-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_SerializedParamDbKey"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_BASESHADERSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_BaseShaderState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_BaseShaderState"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RUNTIMEINSTANTIATEDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RuntimeInstantiatedType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RuntimeInstantiatedType"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDMULTIHASHVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedMultiHashView-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbSerializedMultiHashView"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDREADVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedReadView-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbSerializedReadView"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEWREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedHashViewRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbSerializedHashViewRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedHashView-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbSerializedHashView"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDFILTERVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedFilterView-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbSerializedFilterView"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_COMBINEDSERIALIZEDPARAMETERBLOCK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_CombinedSerializedParameterBlock-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_CombinedSerializedParameterBlock"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCKREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SerializedParameterBlockRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_SerializedParameterBlockRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SerializedParameterBlock-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_SerializedParameterBlock"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTUREREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableExternalTextureRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ShaderStreamableExternalTextureRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTUREREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableTextureRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ShaderStreamableTextureRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SURFACESHADERDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SurfaceShaderDebugInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_SurfaceShaderDebugInfo"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SURFACESHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SurfaceShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_SurfaceShader"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ShaderStreamableTexture"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableExternalTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ShaderStreamableExternalTexture"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionInstanceRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmFunctionInstanceRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionInstance-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmFunctionInstance"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Settings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Settings"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMPERMUTATIONSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationSet-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmPermutationSet"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMPERMUTATIONLOOKUPTABLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationLookupTable-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmPermutationLookupTable"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMPERMUTATIONTREE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationTree-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmPermutationTree"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMPERMUTATIONREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmPermutationRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMPERMUTATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutation-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmPermutation"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMFUNCTIONINPUTTABLEINDICES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionInputTableIndices-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmFunctionInputTableIndices"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMDISPATCHDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmDispatchDebugInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmDispatchDebugInfo"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMFUNCTIONDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionDebugInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmFunctionDebugInfo"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMDISPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmDispatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmDispatch"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMFUNCTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunction-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmFunction"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_INSTRUCTIONBATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstructionBatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_InstructionBatch"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RTTITYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RttiType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RttiType"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DEFAULTVALUEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DefaultValueRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_VALUEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ValueRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ValueRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DEFAULTVALUEZEROMEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueZeroMem-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DefaultValueZeroMem"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DEFAULTVALUESTRUCTLEGACYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueStructLegacyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DefaultValueStructLegacyData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLETEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueSimpleTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DefaultValueSimpleTexture"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLEBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueSimpleBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DefaultValueSimpleBuffer"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_INSTRUCTIONBATCHREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstructionBatchRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_InstructionBatchRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PARAMDBKEYREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbKeyRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbKeyRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PARAMDBKEYDESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbKeyDesc-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbKeyDesc"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTTLASNULLDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasNullData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtTlasNullData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTTLASDYNAMICDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasDynamicData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtTlasDynamicData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTTLASSTATICDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasStaticData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtTlasStaticData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTTLASBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtTlasBaseData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTSHADERTABLERECORDWRITERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtShaderTableRecordWriterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtShaderTableRecordWriterData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTNULLHITSHADERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtNullHitShaderData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtNullHitShaderData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTHITCOLLECTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtHitCollectionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtHitCollectionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTSTATICBLASBUILDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtStaticBlasBuildData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtStaticBlasBuildData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTDYNAMICBLASBUILDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtDynamicBlasBuildData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtDynamicBlasBuildData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTBLASBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtBlasBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtBlasBaseData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTHITSHADERCONSTANTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtHitShaderConstantData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtHitShaderConstantData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTINDEXBUFFERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtIndexBufferData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtIndexBufferData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTSORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtSortData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtSortData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTDISPATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtDispatchData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtDispatchData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTCOLLECTIONPRELOADOP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtCollectionPreloadOp-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtCollectionPreloadOp"),
    array_type: None,
    alignment: 8,
};



pub static RVMLEGACYLIGHTMAPINSTANCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyLightMapInstance-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmLegacyLightMapInstance"),
    array_type: None,
    alignment: 8,
};



pub static RVMLEGACYLIGHTPROBES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyLightProbes-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmLegacyLightProbes"),
    array_type: None,
    alignment: 8,
};



pub static RVMLEGACYPERMUTATIONDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyPermutationDebugInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmLegacyPermutationDebugInfo"),
    array_type: None,
    alignment: 8,
};



pub static RVMLEGACYFORWARDLIGHTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyForwardLightState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmLegacyForwardLightState"),
    array_type: None,
    alignment: 8,
};



pub static FRUSTUMSOA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FrustumSoA-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FrustumSoA"),
    array_type: None,
    alignment: 8,
};



pub static LODVIEWSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LodViewState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("LodViewState"),
    array_type: None,
    alignment: 8,
};



pub static TESSELLATIONVIEWSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TessellationViewState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TessellationViewState"),
    array_type: None,
    alignment: 8,
};



pub static STENCILSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StencilState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("StencilState"),
    array_type: None,
    alignment: 8,
};



pub static REFLECTIONSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReflectionState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ReflectionState"),
    array_type: None,
    alignment: 8,
};



pub static FOGSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FogState"),
    array_type: None,
    alignment: 8,
};



pub static PROJECTIONSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProjectionState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ProjectionState"),
    array_type: None,
    alignment: 8,
};



pub static VIEWSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ViewState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ViewState"),
    array_type: None,
    alignment: 8,
};



pub static OUTDOORLIGHTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutdoorLightState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("OutdoorLightState"),
    array_type: None,
    alignment: 8,
};



pub static DX12NVRVMROOTSIGNATURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmRootSignature-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12NvRvmRootSignature"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvDescriptorTableAssemblyInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12NvDescriptorTableAssemblyInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionBatchData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionBatchData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12NVCONSTANTBUFFERASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvConstantBufferAssemblyInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12NvConstantBufferAssemblyInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvDescriptorTable-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12NvDescriptorTable"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12NVRVMROOTSIGNATURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvRvmRootSignature-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12NvRvmRootSignature"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEDEPTHINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11NvViewStateDepthInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11NvViewStateDepthInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11NVDRAWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11NvDrawStateInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11NvDrawStateInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11NvViewStateInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11NvViewStateInstructionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum RadiosityTypeOverride {
    #[default]
    RadiosityTypeOverride_None = 0,
    RadiosityTypeOverride_Dynamic = 1,
    RadiosityTypeOverride_LightProbe = 2,
    RadiosityTypeOverride_Static = 3,
    RadiosityTypeOverride_TerrainProjected = 4,
    RadiosityTypeOverride_Proxy = 5,
}

pub static RADIOSITYTYPEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityTypeOverride",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(RADIOSITYTYPEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RadiosityTypeOverride {
    fn type_info(&self) -> &'static TypeInfo {
        RADIOSITYTYPEOVERRIDE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RADIOSITYTYPEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityTypeOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RadiosityTypeOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LocalPlayerId {
    #[default]
    LocalPlayerId_0 = 0,
    LocalPlayerId_1 = 1,
    LocalPlayerId_2 = 2,
    LocalPlayerId_3 = 3,
    LocalPlayerId_4 = 4,
    LocalPlayerId_5 = 5,
    LocalPlayerId_6 = 6,
    LocalPlayerId_7 = 7,
    LocalPlayerId_Any = 253,
    LocalPlayerId_All = 254,
    LocalPlayerId_Invalid = 255,
}

pub static LOCALPLAYERID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlayerId",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(LOCALPLAYERID_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LocalPlayerId {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALPLAYERID_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LOCALPLAYERID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlayerId-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("LocalPlayerId"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct NodeGraph {
    pub _glacier_base: Asset,
}

pub trait NodeGraphTrait: AssetTrait {
}

impl NodeGraphTrait for NodeGraph {
}

impl AssetTrait for NodeGraph {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl DataContainerTrait for NodeGraph {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static NODEGRAPH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NodeGraph",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NodeGraph as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(NODEGRAPH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NodeGraph {
    fn type_info(&self) -> &'static TypeInfo {
        NODEGRAPH_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static NODEGRAPH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NodeGraph-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("NodeGraph"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Node {
    pub _glacier_base: DataContainer,
}

pub trait NodeTrait: DataContainerTrait {
}

impl NodeTrait for Node {
}

impl DataContainerTrait for Node {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static NODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Node",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Node as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(NODE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Node {
    fn type_info(&self) -> &'static TypeInfo {
        NODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static NODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Node-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Node"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum GamePlatform {
    #[default]
    GamePlatform_Win32 = 0,
    GamePlatform_Gen4a = 1,
    GamePlatform_Gen4b = 2,
    GamePlatform_Android = 3,
    GamePlatform_iOS = 4,
    GamePlatform_OSX = 5,
    GamePlatform_Linux = 6,
    GamePlatform_Any = 7,
    GamePlatform_Invalid = 8,
    GamePlatformCount = 9,
}

pub static GAMEPLATFORM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GamePlatform",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(GAMEPLATFORM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GamePlatform {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEPLATFORM_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GAMEPLATFORM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GamePlatform-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("GamePlatform"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum DesignerEnumerationSortType {
    #[default]
    DesignerEnumSort_None = 0,
    DesignerEnumSort_ByName = 1,
    DesignerEnumSort_ById = 2,
}

pub static DESIGNERENUMERATIONSORTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DesignerEnumerationSortType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(DESIGNERENUMERATIONSORTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DesignerEnumerationSortType {
    fn type_info(&self) -> &'static TypeInfo {
        DESIGNERENUMERATIONSORTTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DESIGNERENUMERATIONSORTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DesignerEnumerationSortType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DesignerEnumerationSortType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CoreSettings {
    pub live_editing_enable: bool,
    pub disable_safe_state_for_refresh: bool,
    pub host: String,
    pub host_user: String,
    pub host_user_domain: String,
    pub init_seed: String,
    pub trace_host: String,
    pub forward_trace: bool,
    pub forward_trace_host: String,
    pub forward_trace_port: i32,
    pub perf_track_enabled: bool,
    pub log_level: CoreLogLevel,
    pub dialog_level: CoreLogLevel,
    pub user_log_enabled: bool,
    pub log_settings: LogSettings,
    pub allow_assign_vars_from_lua: bool,
    pub display_asserts: bool,
    pub debug_output_mode: u32,
    pub crash_on_fatal_errors: bool,
    pub enable_memory_snapshots: bool,
    pub breakpad_mode: u32,
    pub random_time_seed: i32,
    pub random_tick_seed: i32,
    pub random_session_id: i32,
    pub local_player_id_to_command: LocalPlayerId,
    pub game_configuration_name: String,
    pub use_storage_server: bool,
    pub hardware_profile: HardwareProfile,
    pub hardware_cpu_bias: f32,
    pub hardware_gpu_bias: f32,
    pub profile_directory_name: String,
    pub resource_auto_job_thread: bool,
    pub resource_auto_job_thread_affinity_mask: u32,
    pub enable_localization: bool,
    pub available_languages: String,
    pub enable_spike_packet_buffering: bool,
    pub world_size: f32,
    pub stream_install_allow_play_from_bluray: StreamInstallFromBlurayType,
    pub stream_install_enable_delayed_superbundle_mount: bool,
    pub stream_install_all_languages: bool,
    pub operational_context_data_keys: Vec<String>,
}

pub trait CoreSettingsTrait: TypeObject {
    fn live_editing_enable(&self) -> &bool;
    fn disable_safe_state_for_refresh(&self) -> &bool;
    fn host(&self) -> &String;
    fn host_user(&self) -> &String;
    fn host_user_domain(&self) -> &String;
    fn init_seed(&self) -> &String;
    fn trace_host(&self) -> &String;
    fn forward_trace(&self) -> &bool;
    fn forward_trace_host(&self) -> &String;
    fn forward_trace_port(&self) -> &i32;
    fn perf_track_enabled(&self) -> &bool;
    fn log_level(&self) -> &CoreLogLevel;
    fn dialog_level(&self) -> &CoreLogLevel;
    fn user_log_enabled(&self) -> &bool;
    fn log_settings(&self) -> &LogSettings;
    fn allow_assign_vars_from_lua(&self) -> &bool;
    fn display_asserts(&self) -> &bool;
    fn debug_output_mode(&self) -> &u32;
    fn crash_on_fatal_errors(&self) -> &bool;
    fn enable_memory_snapshots(&self) -> &bool;
    fn breakpad_mode(&self) -> &u32;
    fn random_time_seed(&self) -> &i32;
    fn random_tick_seed(&self) -> &i32;
    fn random_session_id(&self) -> &i32;
    fn local_player_id_to_command(&self) -> &LocalPlayerId;
    fn game_configuration_name(&self) -> &String;
    fn use_storage_server(&self) -> &bool;
    fn hardware_profile(&self) -> &HardwareProfile;
    fn hardware_cpu_bias(&self) -> &f32;
    fn hardware_gpu_bias(&self) -> &f32;
    fn profile_directory_name(&self) -> &String;
    fn resource_auto_job_thread(&self) -> &bool;
    fn resource_auto_job_thread_affinity_mask(&self) -> &u32;
    fn enable_localization(&self) -> &bool;
    fn available_languages(&self) -> &String;
    fn enable_spike_packet_buffering(&self) -> &bool;
    fn world_size(&self) -> &f32;
    fn stream_install_allow_play_from_bluray(&self) -> &StreamInstallFromBlurayType;
    fn stream_install_enable_delayed_superbundle_mount(&self) -> &bool;
    fn stream_install_all_languages(&self) -> &bool;
    fn operational_context_data_keys(&self) -> &Vec<String>;
}

impl CoreSettingsTrait for CoreSettings {
    fn live_editing_enable(&self) -> &bool {
        &self.live_editing_enable
    }
    fn disable_safe_state_for_refresh(&self) -> &bool {
        &self.disable_safe_state_for_refresh
    }
    fn host(&self) -> &String {
        &self.host
    }
    fn host_user(&self) -> &String {
        &self.host_user
    }
    fn host_user_domain(&self) -> &String {
        &self.host_user_domain
    }
    fn init_seed(&self) -> &String {
        &self.init_seed
    }
    fn trace_host(&self) -> &String {
        &self.trace_host
    }
    fn forward_trace(&self) -> &bool {
        &self.forward_trace
    }
    fn forward_trace_host(&self) -> &String {
        &self.forward_trace_host
    }
    fn forward_trace_port(&self) -> &i32 {
        &self.forward_trace_port
    }
    fn perf_track_enabled(&self) -> &bool {
        &self.perf_track_enabled
    }
    fn log_level(&self) -> &CoreLogLevel {
        &self.log_level
    }
    fn dialog_level(&self) -> &CoreLogLevel {
        &self.dialog_level
    }
    fn user_log_enabled(&self) -> &bool {
        &self.user_log_enabled
    }
    fn log_settings(&self) -> &LogSettings {
        &self.log_settings
    }
    fn allow_assign_vars_from_lua(&self) -> &bool {
        &self.allow_assign_vars_from_lua
    }
    fn display_asserts(&self) -> &bool {
        &self.display_asserts
    }
    fn debug_output_mode(&self) -> &u32 {
        &self.debug_output_mode
    }
    fn crash_on_fatal_errors(&self) -> &bool {
        &self.crash_on_fatal_errors
    }
    fn enable_memory_snapshots(&self) -> &bool {
        &self.enable_memory_snapshots
    }
    fn breakpad_mode(&self) -> &u32 {
        &self.breakpad_mode
    }
    fn random_time_seed(&self) -> &i32 {
        &self.random_time_seed
    }
    fn random_tick_seed(&self) -> &i32 {
        &self.random_tick_seed
    }
    fn random_session_id(&self) -> &i32 {
        &self.random_session_id
    }
    fn local_player_id_to_command(&self) -> &LocalPlayerId {
        &self.local_player_id_to_command
    }
    fn game_configuration_name(&self) -> &String {
        &self.game_configuration_name
    }
    fn use_storage_server(&self) -> &bool {
        &self.use_storage_server
    }
    fn hardware_profile(&self) -> &HardwareProfile {
        &self.hardware_profile
    }
    fn hardware_cpu_bias(&self) -> &f32 {
        &self.hardware_cpu_bias
    }
    fn hardware_gpu_bias(&self) -> &f32 {
        &self.hardware_gpu_bias
    }
    fn profile_directory_name(&self) -> &String {
        &self.profile_directory_name
    }
    fn resource_auto_job_thread(&self) -> &bool {
        &self.resource_auto_job_thread
    }
    fn resource_auto_job_thread_affinity_mask(&self) -> &u32 {
        &self.resource_auto_job_thread_affinity_mask
    }
    fn enable_localization(&self) -> &bool {
        &self.enable_localization
    }
    fn available_languages(&self) -> &String {
        &self.available_languages
    }
    fn enable_spike_packet_buffering(&self) -> &bool {
        &self.enable_spike_packet_buffering
    }
    fn world_size(&self) -> &f32 {
        &self.world_size
    }
    fn stream_install_allow_play_from_bluray(&self) -> &StreamInstallFromBlurayType {
        &self.stream_install_allow_play_from_bluray
    }
    fn stream_install_enable_delayed_superbundle_mount(&self) -> &bool {
        &self.stream_install_enable_delayed_superbundle_mount
    }
    fn stream_install_all_languages(&self) -> &bool {
        &self.stream_install_all_languages
    }
    fn operational_context_data_keys(&self) -> &Vec<String> {
        &self.operational_context_data_keys
    }
}

pub static CORESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreSettings",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LiveEditingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, live_editing_enable),
            },
            FieldInfoData {
                name: "DisableSafeStateForRefresh",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, disable_safe_state_for_refresh),
            },
            FieldInfoData {
                name: "Host",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoreSettings, host),
            },
            FieldInfoData {
                name: "HostUser",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoreSettings, host_user),
            },
            FieldInfoData {
                name: "HostUserDomain",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoreSettings, host_user_domain),
            },
            FieldInfoData {
                name: "InitSeed",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoreSettings, init_seed),
            },
            FieldInfoData {
                name: "TraceHost",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoreSettings, trace_host),
            },
            FieldInfoData {
                name: "ForwardTrace",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, forward_trace),
            },
            FieldInfoData {
                name: "ForwardTraceHost",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoreSettings, forward_trace_host),
            },
            FieldInfoData {
                name: "ForwardTracePort",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CoreSettings, forward_trace_port),
            },
            FieldInfoData {
                name: "PerfTrackEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, perf_track_enabled),
            },
            FieldInfoData {
                name: "LogLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "CoreLogLevel",
                rust_offset: offset_of!(CoreSettings, log_level),
            },
            FieldInfoData {
                name: "DialogLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "CoreLogLevel",
                rust_offset: offset_of!(CoreSettings, dialog_level),
            },
            FieldInfoData {
                name: "UserLogEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, user_log_enabled),
            },
            FieldInfoData {
                name: "LogSettings",
                flags: MemberInfoFlags::new(0),
                field_type: "LogSettings",
                rust_offset: offset_of!(CoreSettings, log_settings),
            },
            FieldInfoData {
                name: "AllowAssignVarsFromLua",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, allow_assign_vars_from_lua),
            },
            FieldInfoData {
                name: "DisplayAsserts",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, display_asserts),
            },
            FieldInfoData {
                name: "DebugOutputMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(CoreSettings, debug_output_mode),
            },
            FieldInfoData {
                name: "CrashOnFatalErrors",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, crash_on_fatal_errors),
            },
            FieldInfoData {
                name: "EnableMemorySnapshots",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, enable_memory_snapshots),
            },
            FieldInfoData {
                name: "BreakpadMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(CoreSettings, breakpad_mode),
            },
            FieldInfoData {
                name: "RandomTimeSeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CoreSettings, random_time_seed),
            },
            FieldInfoData {
                name: "RandomTickSeed",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CoreSettings, random_tick_seed),
            },
            FieldInfoData {
                name: "RandomSessionId",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CoreSettings, random_session_id),
            },
            FieldInfoData {
                name: "LocalPlayerIdToCommand",
                flags: MemberInfoFlags::new(0),
                field_type: "LocalPlayerId",
                rust_offset: offset_of!(CoreSettings, local_player_id_to_command),
            },
            FieldInfoData {
                name: "GameConfigurationName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoreSettings, game_configuration_name),
            },
            FieldInfoData {
                name: "UseStorageServer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, use_storage_server),
            },
            FieldInfoData {
                name: "HardwareProfile",
                flags: MemberInfoFlags::new(0),
                field_type: "HardwareProfile",
                rust_offset: offset_of!(CoreSettings, hardware_profile),
            },
            FieldInfoData {
                name: "HardwareCpuBias",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CoreSettings, hardware_cpu_bias),
            },
            FieldInfoData {
                name: "HardwareGpuBias",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CoreSettings, hardware_gpu_bias),
            },
            FieldInfoData {
                name: "ProfileDirectoryName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoreSettings, profile_directory_name),
            },
            FieldInfoData {
                name: "ResourceAutoJobThread",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, resource_auto_job_thread),
            },
            FieldInfoData {
                name: "ResourceAutoJobThreadAffinityMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(CoreSettings, resource_auto_job_thread_affinity_mask),
            },
            FieldInfoData {
                name: "EnableLocalization",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, enable_localization),
            },
            FieldInfoData {
                name: "AvailableLanguages",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoreSettings, available_languages),
            },
            FieldInfoData {
                name: "EnableSpikePacketBuffering",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, enable_spike_packet_buffering),
            },
            FieldInfoData {
                name: "WorldSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CoreSettings, world_size),
            },
            FieldInfoData {
                name: "StreamInstallAllowPlayFromBluray",
                flags: MemberInfoFlags::new(0),
                field_type: "StreamInstallFromBlurayType",
                rust_offset: offset_of!(CoreSettings, stream_install_allow_play_from_bluray),
            },
            FieldInfoData {
                name: "StreamInstallEnableDelayedSuperbundleMount",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, stream_install_enable_delayed_superbundle_mount),
            },
            FieldInfoData {
                name: "StreamInstallAllLanguages",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, stream_install_all_languages),
            },
            FieldInfoData {
                name: "OperationalContextDataKeys",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(CoreSettings, operational_context_data_keys),
            },
        ],
    }),
    array_type: Some(CORESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CoreSettings {
    fn type_info(&self) -> &'static TypeInfo {
        CORESETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CORESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("CoreSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LogSettings {
    pub channels: Vec<ChannelSettings>,
    pub writers: Vec<WriterSettings>,
}

pub trait LogSettingsTrait: TypeObject {
    fn channels(&self) -> &Vec<ChannelSettings>;
    fn writers(&self) -> &Vec<WriterSettings>;
}

impl LogSettingsTrait for LogSettings {
    fn channels(&self) -> &Vec<ChannelSettings> {
        &self.channels
    }
    fn writers(&self) -> &Vec<WriterSettings> {
        &self.writers
    }
}

pub static LOGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogSettings",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LogSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Channels",
                flags: MemberInfoFlags::new(144),
                field_type: "ChannelSettings-Array",
                rust_offset: offset_of!(LogSettings, channels),
            },
            FieldInfoData {
                name: "Writers",
                flags: MemberInfoFlags::new(144),
                field_type: "WriterSettings-Array",
                rust_offset: offset_of!(LogSettings, writers),
            },
        ],
    }),
    array_type: Some(LOGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LogSettings {
    fn type_info(&self) -> &'static TypeInfo {
        LOGSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LOGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("LogSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WriterSettings {
    pub id: String,
    pub class: String,
    pub params: String,
}

pub trait WriterSettingsTrait: TypeObject {
    fn id(&self) -> &String;
    fn class(&self) -> &String;
    fn params(&self) -> &String;
}

impl WriterSettingsTrait for WriterSettings {
    fn id(&self) -> &String {
        &self.id
    }
    fn class(&self) -> &String {
        &self.class
    }
    fn params(&self) -> &String {
        &self.params
    }
}

pub static WRITERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WriterSettings",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WriterSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WriterSettings, id),
            },
            FieldInfoData {
                name: "Class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WriterSettings, class),
            },
            FieldInfoData {
                name: "Params",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WriterSettings, params),
            },
        ],
    }),
    array_type: Some(WRITERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WriterSettings {
    fn type_info(&self) -> &'static TypeInfo {
        WRITERSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WRITERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WriterSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("WriterSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ChannelSettings {
    pub id: String,
    pub inherits_level: bool,
    pub inherits_writers: bool,
    pub level: CoreLogLevel,
    pub writers: Vec<String>,
}

pub trait ChannelSettingsTrait: TypeObject {
    fn id(&self) -> &String;
    fn inherits_level(&self) -> &bool;
    fn inherits_writers(&self) -> &bool;
    fn level(&self) -> &CoreLogLevel;
    fn writers(&self) -> &Vec<String>;
}

impl ChannelSettingsTrait for ChannelSettings {
    fn id(&self) -> &String {
        &self.id
    }
    fn inherits_level(&self) -> &bool {
        &self.inherits_level
    }
    fn inherits_writers(&self) -> &bool {
        &self.inherits_writers
    }
    fn level(&self) -> &CoreLogLevel {
        &self.level
    }
    fn writers(&self) -> &Vec<String> {
        &self.writers
    }
}

pub static CHANNELSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChannelSettings",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ChannelSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ChannelSettings, id),
            },
            FieldInfoData {
                name: "InheritsLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ChannelSettings, inherits_level),
            },
            FieldInfoData {
                name: "InheritsWriters",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ChannelSettings, inherits_writers),
            },
            FieldInfoData {
                name: "Level",
                flags: MemberInfoFlags::new(0),
                field_type: "CoreLogLevel",
                rust_offset: offset_of!(ChannelSettings, level),
            },
            FieldInfoData {
                name: "Writers",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(ChannelSettings, writers),
            },
        ],
    }),
    array_type: Some(CHANNELSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ChannelSettings {
    fn type_info(&self) -> &'static TypeInfo {
        CHANNELSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CHANNELSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChannelSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ChannelSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum StreamInstallFromBlurayType {
    #[default]
    StreamInstall_DisablePlayFromBluray = 0,
    StreamInstall_InstallChunkPlayFromBluray = 1,
    StreamInstall_FullPlayFromBluray = 2,
}

pub static STREAMINSTALLFROMBLURAYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallFromBlurayType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(STREAMINSTALLFROMBLURAYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StreamInstallFromBlurayType {
    fn type_info(&self) -> &'static TypeInfo {
        STREAMINSTALLFROMBLURAYTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static STREAMINSTALLFROMBLURAYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallFromBlurayType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("StreamInstallFromBlurayType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum HardwareProfile {
    #[default]
    Hardware_Autodetect = 0,
    Hardware_LowEnd = 1,
    Hardware_Medium = 2,
    Hardware_HighEnd = 3,
    Hardware_Maximum = 4,
}

pub static HARDWAREPROFILE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HardwareProfile",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(HARDWAREPROFILE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for HardwareProfile {
    fn type_info(&self) -> &'static TypeInfo {
        HARDWAREPROFILE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static HARDWAREPROFILE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HardwareProfile-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("HardwareProfile"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CoreLogLevel {
    #[default]
    CllNone = 0,
    CllEventRecord = 1,
    CllCrash = 2,
    CllError = 3,
    CllAssert = 4,
    CllFatalAssert = 5,
    CllValidate = 6,
    CllWarning = 7,
    CllInfo = 8,
    CllOutput = 9,
    CllDebug = 10,
}

pub static CORELOGLEVEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreLogLevel",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(CORELOGLEVEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CoreLogLevel {
    fn type_info(&self) -> &'static TypeInfo {
        CORELOGLEVEL_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CORELOGLEVEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreLogLevel-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("CoreLogLevel"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ContentPreset {
    pub _glacier_base: DataContainer,
    pub content_name: String,
    pub content_type: String,
    pub content_parameters: Option<Arc<Mutex<dyn DataContainerTrait>>>,
}

pub trait ContentPresetTrait: DataContainerTrait {
    fn content_name(&self) -> &String;
    fn content_type(&self) -> &String;
    fn content_parameters(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>>;
}

impl ContentPresetTrait for ContentPreset {
    fn content_name(&self) -> &String {
        &self.content_name
    }
    fn content_type(&self) -> &String {
        &self.content_type
    }
    fn content_parameters(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>> {
        &self.content_parameters
    }
}

impl DataContainerTrait for ContentPreset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CONTENTPRESET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ContentPreset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ContentPreset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ContentName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ContentPreset, content_name),
            },
            FieldInfoData {
                name: "ContentType",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ContentPreset, content_type),
            },
            FieldInfoData {
                name: "ContentParameters",
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(ContentPreset, content_parameters),
            },
        ],
    }),
    array_type: Some(CONTENTPRESET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ContentPreset {
    fn type_info(&self) -> &'static TypeInfo {
        CONTENTPRESET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CONTENTPRESET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ContentPreset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ContentPreset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ContentPresetContainer {
    pub _glacier_base: Asset,
    pub presets: Vec<Option<Arc<Mutex<dyn ContentPresetTrait>>>>,
}

pub trait ContentPresetContainerTrait: AssetTrait {
    fn presets(&self) -> &Vec<Option<Arc<Mutex<dyn ContentPresetTrait>>>>;
}

impl ContentPresetContainerTrait for ContentPresetContainer {
    fn presets(&self) -> &Vec<Option<Arc<Mutex<dyn ContentPresetTrait>>>> {
        &self.presets
    }
}

impl AssetTrait for ContentPresetContainer {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl DataContainerTrait for ContentPresetContainer {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CONTENTPRESETCONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ContentPresetContainer",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ContentPresetContainer as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Presets",
                flags: MemberInfoFlags::new(144),
                field_type: "ContentPreset-Array",
                rust_offset: offset_of!(ContentPresetContainer, presets),
            },
        ],
    }),
    array_type: Some(CONTENTPRESETCONTAINER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ContentPresetContainer {
    fn type_info(&self) -> &'static TypeInfo {
        CONTENTPRESETCONTAINER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CONTENTPRESETCONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ContentPresetContainer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ContentPresetContainer"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TimeSpan {
    pub internal: i64,
}

pub trait TimeSpanTrait: TypeObject {
    fn internal(&self) -> &i64;
}

impl TimeSpanTrait for TimeSpan {
    fn internal(&self) -> &i64 {
        &self.internal
    }
}

pub static TIMESPAN_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimeSpan",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TimeSpan as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Internal",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TimeSpan, internal),
            },
        ],
    }),
    array_type: Some(TIMESPAN_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimeSpan {
    fn type_info(&self) -> &'static TypeInfo {
        TIMESPAN_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TIMESPAN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimeSpan-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TimeSpan"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ThreadSettings {
    pub _glacier_base: DataContainer,
    pub processor_count: i32,
    pub max_live_edit_processor_count: i32,
    pub job_thread_priority: i32,
    pub free_processor_count: i32,
}

pub trait ThreadSettingsTrait: DataContainerTrait {
    fn processor_count(&self) -> &i32;
    fn max_live_edit_processor_count(&self) -> &i32;
    fn job_thread_priority(&self) -> &i32;
    fn free_processor_count(&self) -> &i32;
}

impl ThreadSettingsTrait for ThreadSettings {
    fn processor_count(&self) -> &i32 {
        &self.processor_count
    }
    fn max_live_edit_processor_count(&self) -> &i32 {
        &self.max_live_edit_processor_count
    }
    fn job_thread_priority(&self) -> &i32 {
        &self.job_thread_priority
    }
    fn free_processor_count(&self) -> &i32 {
        &self.free_processor_count
    }
}

impl DataContainerTrait for ThreadSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static THREADSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ThreadSettings",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ThreadSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ProcessorCount",
                flags: MemberInfoFlags::new(8192),
                field_type: "Int32",
                rust_offset: offset_of!(ThreadSettings, processor_count),
            },
            FieldInfoData {
                name: "MaxLiveEditProcessorCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ThreadSettings, max_live_edit_processor_count),
            },
            FieldInfoData {
                name: "JobThreadPriority",
                flags: MemberInfoFlags::new(8192),
                field_type: "Int32",
                rust_offset: offset_of!(ThreadSettings, job_thread_priority),
            },
            FieldInfoData {
                name: "FreeProcessorCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ThreadSettings, free_processor_count),
            },
        ],
    }),
    array_type: Some(THREADSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ThreadSettings {
    fn type_info(&self) -> &'static TypeInfo {
        THREADSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static THREADSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ThreadSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ThreadSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TemplateDescriptorRegistryAsset {
    pub _glacier_base: Asset,
    pub descriptors: Vec<Option<Arc<Mutex<dyn TemplateDescriptorDataTrait>>>>,
}

pub trait TemplateDescriptorRegistryAssetTrait: AssetTrait {
    fn descriptors(&self) -> &Vec<Option<Arc<Mutex<dyn TemplateDescriptorDataTrait>>>>;
}

impl TemplateDescriptorRegistryAssetTrait for TemplateDescriptorRegistryAsset {
    fn descriptors(&self) -> &Vec<Option<Arc<Mutex<dyn TemplateDescriptorDataTrait>>>> {
        &self.descriptors
    }
}

impl AssetTrait for TemplateDescriptorRegistryAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl DataContainerTrait for TemplateDescriptorRegistryAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TEMPLATEDESCRIPTORREGISTRYASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateDescriptorRegistryAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TemplateDescriptorRegistryAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Descriptors",
                flags: MemberInfoFlags::new(144),
                field_type: "TemplateDescriptorData-Array",
                rust_offset: offset_of!(TemplateDescriptorRegistryAsset, descriptors),
            },
        ],
    }),
    array_type: Some(TEMPLATEDESCRIPTORREGISTRYASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TemplateDescriptorRegistryAsset {
    fn type_info(&self) -> &'static TypeInfo {
        TEMPLATEDESCRIPTORREGISTRYASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TEMPLATEDESCRIPTORREGISTRYASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateDescriptorRegistryAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TemplateDescriptorRegistryAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TemplateDescriptorData {
    pub _glacier_base: DataContainer,
    pub target_asset: Option<Arc<Mutex<dyn AssetTrait>>>,
    pub exposed_objects: Vec<TemplateExposedObject>,
}

pub trait TemplateDescriptorDataTrait: DataContainerTrait {
    fn target_asset(&self) -> &Option<Arc<Mutex<dyn AssetTrait>>>;
    fn exposed_objects(&self) -> &Vec<TemplateExposedObject>;
}

impl TemplateDescriptorDataTrait for TemplateDescriptorData {
    fn target_asset(&self) -> &Option<Arc<Mutex<dyn AssetTrait>>> {
        &self.target_asset
    }
    fn exposed_objects(&self) -> &Vec<TemplateExposedObject> {
        &self.exposed_objects
    }
}

impl DataContainerTrait for TemplateDescriptorData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TEMPLATEDESCRIPTORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateDescriptorData",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TemplateDescriptorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TargetAsset",
                flags: MemberInfoFlags::new(0),
                field_type: "Asset",
                rust_offset: offset_of!(TemplateDescriptorData, target_asset),
            },
            FieldInfoData {
                name: "ExposedObjects",
                flags: MemberInfoFlags::new(144),
                field_type: "TemplateExposedObject-Array",
                rust_offset: offset_of!(TemplateDescriptorData, exposed_objects),
            },
        ],
    }),
    array_type: Some(TEMPLATEDESCRIPTORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TemplateDescriptorData {
    fn type_info(&self) -> &'static TypeInfo {
        TEMPLATEDESCRIPTORDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TEMPLATEDESCRIPTORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateDescriptorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TemplateDescriptorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TemplateExposedObject {
    pub object: Option<Arc<Mutex<dyn DataContainerTrait>>>,
    pub object_ref: glacier_util::guid::Guid,
}

pub trait TemplateExposedObjectTrait: TypeObject {
    fn object(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>>;
    fn object_ref(&self) -> &glacier_util::guid::Guid;
}

impl TemplateExposedObjectTrait for TemplateExposedObject {
    fn object(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>> {
        &self.object
    }
    fn object_ref(&self) -> &glacier_util::guid::Guid {
        &self.object_ref
    }
}

pub static TEMPLATEEXPOSEDOBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateExposedObject",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TemplateExposedObject as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Object",
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(TemplateExposedObject, object),
            },
            FieldInfoData {
                name: "ObjectRef",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(TemplateExposedObject, object_ref),
            },
        ],
    }),
    array_type: Some(TEMPLATEEXPOSEDOBJECT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TemplateExposedObject {
    fn type_info(&self) -> &'static TypeInfo {
        TEMPLATEEXPOSEDOBJECT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TEMPLATEEXPOSEDOBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateExposedObject-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TemplateExposedObject"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TemplateAsset {
    pub _glacier_base: Asset,
    pub objects: Vec<TemplateTarget>,
}

pub trait TemplateAssetTrait: AssetTrait {
    fn objects(&self) -> &Vec<TemplateTarget>;
}

impl TemplateAssetTrait for TemplateAsset {
    fn objects(&self) -> &Vec<TemplateTarget> {
        &self.objects
    }
}

impl AssetTrait for TemplateAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl DataContainerTrait for TemplateAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TEMPLATEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TemplateAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Objects",
                flags: MemberInfoFlags::new(144),
                field_type: "TemplateTarget-Array",
                rust_offset: offset_of!(TemplateAsset, objects),
            },
        ],
    }),
    array_type: Some(TEMPLATEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TemplateAsset {
    fn type_info(&self) -> &'static TypeInfo {
        TEMPLATEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TEMPLATEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TemplateAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TemplateTarget {
    pub target_ref: glacier_util::guid::Guid,
    pub template: Option<Arc<Mutex<dyn TemplateTrait>>>,
}

pub trait TemplateTargetTrait: TypeObject {
    fn target_ref(&self) -> &glacier_util::guid::Guid;
    fn template(&self) -> &Option<Arc<Mutex<dyn TemplateTrait>>>;
}

impl TemplateTargetTrait for TemplateTarget {
    fn target_ref(&self) -> &glacier_util::guid::Guid {
        &self.target_ref
    }
    fn template(&self) -> &Option<Arc<Mutex<dyn TemplateTrait>>> {
        &self.template
    }
}

pub static TEMPLATETARGET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateTarget",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TemplateTarget as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TargetRef",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(TemplateTarget, target_ref),
            },
            FieldInfoData {
                name: "Template",
                flags: MemberInfoFlags::new(0),
                field_type: "Template",
                rust_offset: offset_of!(TemplateTarget, template),
            },
        ],
    }),
    array_type: Some(TEMPLATETARGET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TemplateTarget {
    fn type_info(&self) -> &'static TypeInfo {
        TEMPLATETARGET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TEMPLATETARGET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateTarget-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TemplateTarget"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Template {
    pub _glacier_base: DataContainer,
}

pub trait TemplateTrait: DataContainerTrait {
}

impl TemplateTrait for Template {
}

impl DataContainerTrait for Template {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TEMPLATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Template",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Template as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TEMPLATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Template {
    fn type_info(&self) -> &'static TypeInfo {
        TEMPLATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TEMPLATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Template-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Template"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SuperbundleLayoutSettings {
    pub _glacier_base: SystemSettings,
}

pub trait SuperbundleLayoutSettingsTrait: SystemSettingsTrait {
}

impl SuperbundleLayoutSettingsTrait for SuperbundleLayoutSettings {
}

impl SystemSettingsTrait for SuperbundleLayoutSettings {
    fn platform(&self) -> &GamePlatform {
        self._glacier_base.platform()
    }
}

impl DataContainerTrait for SuperbundleLayoutSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SUPERBUNDLELAYOUTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SuperbundleLayoutSettings",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SuperbundleLayoutSettings as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SUPERBUNDLELAYOUTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SuperbundleLayoutSettings {
    fn type_info(&self) -> &'static TypeInfo {
        SUPERBUNDLELAYOUTSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SUPERBUNDLELAYOUTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SuperbundleLayoutSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SuperbundleLayoutSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum InstallPackageLayerType {
    #[default]
    InstallPackageLayerType_Any = 4294967295,
    InstallPackageLayerType_Zero = 0,
    InstallPackageLayerType_One = 1,
}

pub static INSTALLPACKAGELAYERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InstallPackageLayerType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(INSTALLPACKAGELAYERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InstallPackageLayerType {
    fn type_info(&self) -> &'static TypeInfo {
        INSTALLPACKAGELAYERTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INSTALLPACKAGELAYERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InstallPackageLayerType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("InstallPackageLayerType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum InstallPackageType {
    #[default]
    InstallPackageType_Default = 0,
    InstallPackageType_AlwaysInstalled = 1,
    InstallPackageType_MandatoryDLC = 2,
    InstallPackageType_OptionalDLC = 3,
    InstallPackageType_Exclude = 4,
}

pub static INSTALLPACKAGETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InstallPackageType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(INSTALLPACKAGETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InstallPackageType {
    fn type_info(&self) -> &'static TypeInfo {
        INSTALLPACKAGETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INSTALLPACKAGETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InstallPackageType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("InstallPackageType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PlatformScalableBool {
    pub default: bool,
    pub xenon: bool,
    pub ps3: bool,
    pub gen4a: bool,
    pub gen4b: bool,
    pub android: bool,
    pub i_o_s: bool,
    pub o_s_x: bool,
    pub linux: bool,
}

pub trait PlatformScalableBoolTrait: TypeObject {
    fn default(&self) -> &bool;
    fn xenon(&self) -> &bool;
    fn ps3(&self) -> &bool;
    fn gen4a(&self) -> &bool;
    fn gen4b(&self) -> &bool;
    fn android(&self) -> &bool;
    fn i_o_s(&self) -> &bool;
    fn o_s_x(&self) -> &bool;
    fn linux(&self) -> &bool;
}

impl PlatformScalableBoolTrait for PlatformScalableBool {
    fn default(&self) -> &bool {
        &self.default
    }
    fn xenon(&self) -> &bool {
        &self.xenon
    }
    fn ps3(&self) -> &bool {
        &self.ps3
    }
    fn gen4a(&self) -> &bool {
        &self.gen4a
    }
    fn gen4b(&self) -> &bool {
        &self.gen4b
    }
    fn android(&self) -> &bool {
        &self.android
    }
    fn i_o_s(&self) -> &bool {
        &self.i_o_s
    }
    fn o_s_x(&self) -> &bool {
        &self.o_s_x
    }
    fn linux(&self) -> &bool {
        &self.linux
    }
}

pub static PLATFORMSCALABLEBOOL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformScalableBool",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlatformScalableBool as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Default",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlatformScalableBool, default),
            },
            FieldInfoData {
                name: "Xenon",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlatformScalableBool, xenon),
            },
            FieldInfoData {
                name: "Ps3",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlatformScalableBool, ps3),
            },
            FieldInfoData {
                name: "Gen4a",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlatformScalableBool, gen4a),
            },
            FieldInfoData {
                name: "Gen4b",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlatformScalableBool, gen4b),
            },
            FieldInfoData {
                name: "Android",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlatformScalableBool, android),
            },
            FieldInfoData {
                name: "iOS",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlatformScalableBool, i_o_s),
            },
            FieldInfoData {
                name: "OSX",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlatformScalableBool, o_s_x),
            },
            FieldInfoData {
                name: "Linux",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlatformScalableBool, linux),
            },
        ],
    }),
    array_type: Some(PLATFORMSCALABLEBOOL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PlatformScalableBool {
    fn type_info(&self) -> &'static TypeInfo {
        PLATFORMSCALABLEBOOL_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PLATFORMSCALABLEBOOL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformScalableBool-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PlatformScalableBool"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PlatformScalableFloat {
    pub default: f32,
    pub xenon: f32,
    pub ps3: f32,
    pub gen4a: f32,
    pub gen4b: f32,
    pub android: f32,
    pub i_o_s: f32,
    pub o_s_x: f32,
    pub linux: f32,
}

pub trait PlatformScalableFloatTrait: TypeObject {
    fn default(&self) -> &f32;
    fn xenon(&self) -> &f32;
    fn ps3(&self) -> &f32;
    fn gen4a(&self) -> &f32;
    fn gen4b(&self) -> &f32;
    fn android(&self) -> &f32;
    fn i_o_s(&self) -> &f32;
    fn o_s_x(&self) -> &f32;
    fn linux(&self) -> &f32;
}

impl PlatformScalableFloatTrait for PlatformScalableFloat {
    fn default(&self) -> &f32 {
        &self.default
    }
    fn xenon(&self) -> &f32 {
        &self.xenon
    }
    fn ps3(&self) -> &f32 {
        &self.ps3
    }
    fn gen4a(&self) -> &f32 {
        &self.gen4a
    }
    fn gen4b(&self) -> &f32 {
        &self.gen4b
    }
    fn android(&self) -> &f32 {
        &self.android
    }
    fn i_o_s(&self) -> &f32 {
        &self.i_o_s
    }
    fn o_s_x(&self) -> &f32 {
        &self.o_s_x
    }
    fn linux(&self) -> &f32 {
        &self.linux
    }
}

pub static PLATFORMSCALABLEFLOAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformScalableFloat",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlatformScalableFloat as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Default",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlatformScalableFloat, default),
            },
            FieldInfoData {
                name: "Xenon",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlatformScalableFloat, xenon),
            },
            FieldInfoData {
                name: "Ps3",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlatformScalableFloat, ps3),
            },
            FieldInfoData {
                name: "Gen4a",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlatformScalableFloat, gen4a),
            },
            FieldInfoData {
                name: "Gen4b",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlatformScalableFloat, gen4b),
            },
            FieldInfoData {
                name: "Android",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlatformScalableFloat, android),
            },
            FieldInfoData {
                name: "iOS",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlatformScalableFloat, i_o_s),
            },
            FieldInfoData {
                name: "OSX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlatformScalableFloat, o_s_x),
            },
            FieldInfoData {
                name: "Linux",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlatformScalableFloat, linux),
            },
        ],
    }),
    array_type: Some(PLATFORMSCALABLEFLOAT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PlatformScalableFloat {
    fn type_info(&self) -> &'static TypeInfo {
        PLATFORMSCALABLEFLOAT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PLATFORMSCALABLEFLOAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformScalableFloat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PlatformScalableFloat"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PlatformScalableInt {
    pub default: i32,
    pub xenon: i32,
    pub ps3: i32,
    pub gen4a: i32,
    pub gen4b: i32,
    pub android: i32,
    pub i_o_s: i32,
    pub o_s_x: i32,
    pub linux: i32,
}

pub trait PlatformScalableIntTrait: TypeObject {
    fn default(&self) -> &i32;
    fn xenon(&self) -> &i32;
    fn ps3(&self) -> &i32;
    fn gen4a(&self) -> &i32;
    fn gen4b(&self) -> &i32;
    fn android(&self) -> &i32;
    fn i_o_s(&self) -> &i32;
    fn o_s_x(&self) -> &i32;
    fn linux(&self) -> &i32;
}

impl PlatformScalableIntTrait for PlatformScalableInt {
    fn default(&self) -> &i32 {
        &self.default
    }
    fn xenon(&self) -> &i32 {
        &self.xenon
    }
    fn ps3(&self) -> &i32 {
        &self.ps3
    }
    fn gen4a(&self) -> &i32 {
        &self.gen4a
    }
    fn gen4b(&self) -> &i32 {
        &self.gen4b
    }
    fn android(&self) -> &i32 {
        &self.android
    }
    fn i_o_s(&self) -> &i32 {
        &self.i_o_s
    }
    fn o_s_x(&self) -> &i32 {
        &self.o_s_x
    }
    fn linux(&self) -> &i32 {
        &self.linux
    }
}

pub static PLATFORMSCALABLEINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformScalableInt",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlatformScalableInt as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Default",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PlatformScalableInt, default),
            },
            FieldInfoData {
                name: "Xenon",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PlatformScalableInt, xenon),
            },
            FieldInfoData {
                name: "Ps3",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PlatformScalableInt, ps3),
            },
            FieldInfoData {
                name: "Gen4a",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PlatformScalableInt, gen4a),
            },
            FieldInfoData {
                name: "Gen4b",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PlatformScalableInt, gen4b),
            },
            FieldInfoData {
                name: "Android",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PlatformScalableInt, android),
            },
            FieldInfoData {
                name: "iOS",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PlatformScalableInt, i_o_s),
            },
            FieldInfoData {
                name: "OSX",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PlatformScalableInt, o_s_x),
            },
            FieldInfoData {
                name: "Linux",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PlatformScalableInt, linux),
            },
        ],
    }),
    array_type: Some(PLATFORMSCALABLEINT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PlatformScalableInt {
    fn type_info(&self) -> &'static TypeInfo {
        PLATFORMSCALABLEINT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PLATFORMSCALABLEINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformScalableInt-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PlatformScalableInt"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum QualityScalableEnabled {
    #[default]
    QualityScalableEnabled_Low = 0,
    QualityScalableEnabled_Medium = 1,
    QualityScalableEnabled_High = 2,
    QualityScalableEnabled_Ultra = 3,
    QualityScalableEnabled_Cinematic = 4,
    QualityScalableEnabled_Disabled = 5,
}

pub static QUALITYSCALABLEENABLED_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableEnabled",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(QUALITYSCALABLEENABLED_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for QualityScalableEnabled {
    fn type_info(&self) -> &'static TypeInfo {
        QUALITYSCALABLEENABLED_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static QUALITYSCALABLEENABLED_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableEnabled-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("QualityScalableEnabled"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct QualityScalableBool {
    pub low: bool,
    pub medium: bool,
    pub high: bool,
    pub ultra: bool,
    pub cinematic: bool,
}

pub trait QualityScalableBoolTrait: TypeObject {
    fn low(&self) -> &bool;
    fn medium(&self) -> &bool;
    fn high(&self) -> &bool;
    fn ultra(&self) -> &bool;
    fn cinematic(&self) -> &bool;
}

impl QualityScalableBoolTrait for QualityScalableBool {
    fn low(&self) -> &bool {
        &self.low
    }
    fn medium(&self) -> &bool {
        &self.medium
    }
    fn high(&self) -> &bool {
        &self.high
    }
    fn ultra(&self) -> &bool {
        &self.ultra
    }
    fn cinematic(&self) -> &bool {
        &self.cinematic
    }
}

pub static QUALITYSCALABLEBOOL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableBool",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QualityScalableBool as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Low",
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(QualityScalableBool, low),
            },
            FieldInfoData {
                name: "Medium",
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(QualityScalableBool, medium),
            },
            FieldInfoData {
                name: "High",
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(QualityScalableBool, high),
            },
            FieldInfoData {
                name: "Ultra",
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(QualityScalableBool, ultra),
            },
            FieldInfoData {
                name: "Cinematic",
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(QualityScalableBool, cinematic),
            },
        ],
    }),
    array_type: Some(QUALITYSCALABLEBOOL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for QualityScalableBool {
    fn type_info(&self) -> &'static TypeInfo {
        QUALITYSCALABLEBOOL_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static QUALITYSCALABLEBOOL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableBool-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("QualityScalableBool"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct QualityScalableFloat {
    pub low: f32,
    pub medium: f32,
    pub high: f32,
    pub ultra: f32,
    pub cinematic: f32,
}

pub trait QualityScalableFloatTrait: TypeObject {
    fn low(&self) -> &f32;
    fn medium(&self) -> &f32;
    fn high(&self) -> &f32;
    fn ultra(&self) -> &f32;
    fn cinematic(&self) -> &f32;
}

impl QualityScalableFloatTrait for QualityScalableFloat {
    fn low(&self) -> &f32 {
        &self.low
    }
    fn medium(&self) -> &f32 {
        &self.medium
    }
    fn high(&self) -> &f32 {
        &self.high
    }
    fn ultra(&self) -> &f32 {
        &self.ultra
    }
    fn cinematic(&self) -> &f32 {
        &self.cinematic
    }
}

pub static QUALITYSCALABLEFLOAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableFloat",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QualityScalableFloat as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Low",
                flags: MemberInfoFlags::new(8192),
                field_type: "Float32",
                rust_offset: offset_of!(QualityScalableFloat, low),
            },
            FieldInfoData {
                name: "Medium",
                flags: MemberInfoFlags::new(8192),
                field_type: "Float32",
                rust_offset: offset_of!(QualityScalableFloat, medium),
            },
            FieldInfoData {
                name: "High",
                flags: MemberInfoFlags::new(8192),
                field_type: "Float32",
                rust_offset: offset_of!(QualityScalableFloat, high),
            },
            FieldInfoData {
                name: "Ultra",
                flags: MemberInfoFlags::new(8192),
                field_type: "Float32",
                rust_offset: offset_of!(QualityScalableFloat, ultra),
            },
            FieldInfoData {
                name: "Cinematic",
                flags: MemberInfoFlags::new(8192),
                field_type: "Float32",
                rust_offset: offset_of!(QualityScalableFloat, cinematic),
            },
        ],
    }),
    array_type: Some(QUALITYSCALABLEFLOAT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for QualityScalableFloat {
    fn type_info(&self) -> &'static TypeInfo {
        QUALITYSCALABLEFLOAT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static QUALITYSCALABLEFLOAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableFloat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("QualityScalableFloat"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct QualityScalableInt {
    pub low: i32,
    pub medium: i32,
    pub high: i32,
    pub ultra: i32,
    pub cinematic: i32,
}

pub trait QualityScalableIntTrait: TypeObject {
    fn low(&self) -> &i32;
    fn medium(&self) -> &i32;
    fn high(&self) -> &i32;
    fn ultra(&self) -> &i32;
    fn cinematic(&self) -> &i32;
}

impl QualityScalableIntTrait for QualityScalableInt {
    fn low(&self) -> &i32 {
        &self.low
    }
    fn medium(&self) -> &i32 {
        &self.medium
    }
    fn high(&self) -> &i32 {
        &self.high
    }
    fn ultra(&self) -> &i32 {
        &self.ultra
    }
    fn cinematic(&self) -> &i32 {
        &self.cinematic
    }
}

pub static QUALITYSCALABLEINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableInt",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QualityScalableInt as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Low",
                flags: MemberInfoFlags::new(8192),
                field_type: "Int32",
                rust_offset: offset_of!(QualityScalableInt, low),
            },
            FieldInfoData {
                name: "Medium",
                flags: MemberInfoFlags::new(8192),
                field_type: "Int32",
                rust_offset: offset_of!(QualityScalableInt, medium),
            },
            FieldInfoData {
                name: "High",
                flags: MemberInfoFlags::new(8192),
                field_type: "Int32",
                rust_offset: offset_of!(QualityScalableInt, high),
            },
            FieldInfoData {
                name: "Ultra",
                flags: MemberInfoFlags::new(8192),
                field_type: "Int32",
                rust_offset: offset_of!(QualityScalableInt, ultra),
            },
            FieldInfoData {
                name: "Cinematic",
                flags: MemberInfoFlags::new(8192),
                field_type: "Int32",
                rust_offset: offset_of!(QualityScalableInt, cinematic),
            },
        ],
    }),
    array_type: Some(QUALITYSCALABLEINT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for QualityScalableInt {
    fn type_info(&self) -> &'static TypeInfo {
        QUALITYSCALABLEINT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static QUALITYSCALABLEINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableInt-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("QualityScalableInt"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum QualityLevel {
    #[default]
    QualityLevel_Low = 0,
    QualityLevel_Medium = 1,
    QualityLevel_High = 2,
    QualityLevel_Ultra = 3,
    QualityLevel_Cinematic = 4,
    QualityLevel_All = 5,
    QualityLevel_Invalid = 6,
}

pub static QUALITYLEVEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityLevel",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(QUALITYLEVEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for QualityLevel {
    fn type_info(&self) -> &'static TypeInfo {
        QUALITYLEVEL_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static QUALITYLEVEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityLevel-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("QualityLevel"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ResourceManagerSettings {
    pub _glacier_base: SystemSettings,
    pub cas_bundle_read_buffer_size_kb: i32,
    pub cas_bundle_decompress_buffer_size_kb: i32,
    pub cas_bundle_decompress_buffer_count: i32,
    pub bundle_profiling_enable: bool,
}

pub trait ResourceManagerSettingsTrait: SystemSettingsTrait {
    fn cas_bundle_read_buffer_size_kb(&self) -> &i32;
    fn cas_bundle_decompress_buffer_size_kb(&self) -> &i32;
    fn cas_bundle_decompress_buffer_count(&self) -> &i32;
    fn bundle_profiling_enable(&self) -> &bool;
}

impl ResourceManagerSettingsTrait for ResourceManagerSettings {
    fn cas_bundle_read_buffer_size_kb(&self) -> &i32 {
        &self.cas_bundle_read_buffer_size_kb
    }
    fn cas_bundle_decompress_buffer_size_kb(&self) -> &i32 {
        &self.cas_bundle_decompress_buffer_size_kb
    }
    fn cas_bundle_decompress_buffer_count(&self) -> &i32 {
        &self.cas_bundle_decompress_buffer_count
    }
    fn bundle_profiling_enable(&self) -> &bool {
        &self.bundle_profiling_enable
    }
}

impl SystemSettingsTrait for ResourceManagerSettings {
    fn platform(&self) -> &GamePlatform {
        self._glacier_base.platform()
    }
}

impl DataContainerTrait for ResourceManagerSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static RESOURCEMANAGERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceManagerSettings",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ResourceManagerSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CasBundleReadBufferSizeKb",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ResourceManagerSettings, cas_bundle_read_buffer_size_kb),
            },
            FieldInfoData {
                name: "CasBundleDecompressBufferSizeKb",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ResourceManagerSettings, cas_bundle_decompress_buffer_size_kb),
            },
            FieldInfoData {
                name: "CasBundleDecompressBufferCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ResourceManagerSettings, cas_bundle_decompress_buffer_count),
            },
            FieldInfoData {
                name: "BundleProfilingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ResourceManagerSettings, bundle_profiling_enable),
            },
        ],
    }),
    array_type: Some(RESOURCEMANAGERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ResourceManagerSettings {
    fn type_info(&self) -> &'static TypeInfo {
        RESOURCEMANAGERSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RESOURCEMANAGERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceManagerSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ResourceManagerSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RenderingOverrides {
    pub shadow_enable: BoolOverride,
    pub sun_shadow_enable: BoolOverride,
    pub local_shadow_enable: BoolOverride,
    pub dynamic_reflection_enable: BoolOverride,
    pub static_reflection_enable: BoolOverride,
    pub planar_shadow_enable: BoolOverride,
    pub hologram_enable: BoolOverride,
    pub hologram_projector_index: u32,
    pub distant_shadow_cache_enable: BoolOverride,
    pub dynamic_distant_shadow_cache_enable: BoolOverride,
    pub local_shadow_cache_enable: BoolOverride,
    pub root_shader_effect: BoolOverride,
}

pub trait RenderingOverridesTrait: TypeObject {
    fn shadow_enable(&self) -> &BoolOverride;
    fn sun_shadow_enable(&self) -> &BoolOverride;
    fn local_shadow_enable(&self) -> &BoolOverride;
    fn dynamic_reflection_enable(&self) -> &BoolOverride;
    fn static_reflection_enable(&self) -> &BoolOverride;
    fn planar_shadow_enable(&self) -> &BoolOverride;
    fn hologram_enable(&self) -> &BoolOverride;
    fn hologram_projector_index(&self) -> &u32;
    fn distant_shadow_cache_enable(&self) -> &BoolOverride;
    fn dynamic_distant_shadow_cache_enable(&self) -> &BoolOverride;
    fn local_shadow_cache_enable(&self) -> &BoolOverride;
    fn root_shader_effect(&self) -> &BoolOverride;
}

impl RenderingOverridesTrait for RenderingOverrides {
    fn shadow_enable(&self) -> &BoolOverride {
        &self.shadow_enable
    }
    fn sun_shadow_enable(&self) -> &BoolOverride {
        &self.sun_shadow_enable
    }
    fn local_shadow_enable(&self) -> &BoolOverride {
        &self.local_shadow_enable
    }
    fn dynamic_reflection_enable(&self) -> &BoolOverride {
        &self.dynamic_reflection_enable
    }
    fn static_reflection_enable(&self) -> &BoolOverride {
        &self.static_reflection_enable
    }
    fn planar_shadow_enable(&self) -> &BoolOverride {
        &self.planar_shadow_enable
    }
    fn hologram_enable(&self) -> &BoolOverride {
        &self.hologram_enable
    }
    fn hologram_projector_index(&self) -> &u32 {
        &self.hologram_projector_index
    }
    fn distant_shadow_cache_enable(&self) -> &BoolOverride {
        &self.distant_shadow_cache_enable
    }
    fn dynamic_distant_shadow_cache_enable(&self) -> &BoolOverride {
        &self.dynamic_distant_shadow_cache_enable
    }
    fn local_shadow_cache_enable(&self) -> &BoolOverride {
        &self.local_shadow_cache_enable
    }
    fn root_shader_effect(&self) -> &BoolOverride {
        &self.root_shader_effect
    }
}

pub static RENDERINGOVERRIDES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderingOverrides",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RenderingOverrides as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, shadow_enable),
            },
            FieldInfoData {
                name: "SunShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, sun_shadow_enable),
            },
            FieldInfoData {
                name: "LocalShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, local_shadow_enable),
            },
            FieldInfoData {
                name: "DynamicReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, dynamic_reflection_enable),
            },
            FieldInfoData {
                name: "StaticReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, static_reflection_enable),
            },
            FieldInfoData {
                name: "PlanarShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, planar_shadow_enable),
            },
            FieldInfoData {
                name: "HologramEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, hologram_enable),
            },
            FieldInfoData {
                name: "HologramProjectorIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(RenderingOverrides, hologram_projector_index),
            },
            FieldInfoData {
                name: "DistantShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, distant_shadow_cache_enable),
            },
            FieldInfoData {
                name: "DynamicDistantShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, dynamic_distant_shadow_cache_enable),
            },
            FieldInfoData {
                name: "LocalShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, local_shadow_cache_enable),
            },
            FieldInfoData {
                name: "RootShaderEffect",
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, root_shader_effect),
            },
        ],
    }),
    array_type: Some(RENDERINGOVERRIDES_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RenderingOverrides {
    fn type_info(&self) -> &'static TypeInfo {
        RENDERINGOVERRIDES_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RENDERINGOVERRIDES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderingOverrides-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RenderingOverrides"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ChunkOnlyResourceType {
}

pub trait ChunkOnlyResourceTypeTrait: TypeObject {
}

impl ChunkOnlyResourceTypeTrait for ChunkOnlyResourceType {
}

pub static CHUNKONLYRESOURCETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChunkOnlyResourceType",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ChunkOnlyResourceType as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CHUNKONLYRESOURCETYPE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ChunkOnlyResourceType {
    fn type_info(&self) -> &'static TypeInfo {
        CHUNKONLYRESOURCETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CHUNKONLYRESOURCETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChunkOnlyResourceType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ChunkOnlyResourceType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IResourceObject {
}

pub trait IResourceObjectTrait: TypeObject {
}

impl IResourceObjectTrait for IResourceObject {
}

pub static IRESOURCEOBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IResourceObject",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IResourceObject as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IRESOURCEOBJECT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IResourceObject {
    fn type_info(&self) -> &'static TypeInfo {
        IRESOURCEOBJECT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IRESOURCEOBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IResourceObject-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("IResourceObject"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ProxyResourceObject {
}

pub trait ProxyResourceObjectTrait: TypeObject {
}

impl ProxyResourceObjectTrait for ProxyResourceObject {
}

pub static PROXYRESOURCEOBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProxyResourceObject",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProxyResourceObject as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PROXYRESOURCEOBJECT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ProxyResourceObject {
    fn type_info(&self) -> &'static TypeInfo {
        PROXYRESOURCEOBJECT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PROXYRESOURCEOBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProxyResourceObject-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ProxyResourceObject"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum BoolOverride {
    #[default]
    BoolOverride_Inherit = 0,
    BoolOverride_Enable = 1,
    BoolOverride_Disable = 2,
}

pub static BOOLOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolOverride",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(BOOLOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BoolOverride {
    fn type_info(&self) -> &'static TypeInfo {
        BOOLOVERRIDE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BOOLOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("BoolOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DataContainerPolicyPipelineResultBase {
    pub _glacier_base: SetDataResultListener,
    pub secondary_result: Option<Arc<Mutex<dyn DataContainerTrait>>>,
}

pub trait DataContainerPolicyPipelineResultBaseTrait: SetDataResultListenerTrait {
    fn secondary_result(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>>;
}

impl DataContainerPolicyPipelineResultBaseTrait for DataContainerPolicyPipelineResultBase {
    fn secondary_result(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>> {
        &self.secondary_result
    }
}

impl SetDataResultListenerTrait for DataContainerPolicyPipelineResultBase {
}

impl DataContainerTrait for DataContainerPolicyPipelineResultBase {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DATACONTAINERPOLICYPIPELINERESULTBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainerPolicyPipelineResultBase",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SETDATARESULTLISTENER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DataContainerPolicyPipelineResultBase as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SecondaryResult",
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(DataContainerPolicyPipelineResultBase, secondary_result),
            },
        ],
    }),
    array_type: Some(DATACONTAINERPOLICYPIPELINERESULTBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DataContainerPolicyPipelineResultBase {
    fn type_info(&self) -> &'static TypeInfo {
        DATACONTAINERPOLICYPIPELINERESULTBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DATACONTAINERPOLICYPIPELINERESULTBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainerPolicyPipelineResultBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DataContainerPolicyPipelineResultBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SetDataResultListener {
    pub _glacier_base: DataContainer,
}

pub trait SetDataResultListenerTrait: DataContainerTrait {
}

impl SetDataResultListenerTrait for SetDataResultListener {
}

impl DataContainerTrait for SetDataResultListener {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SETDATARESULTLISTENER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetDataResultListener",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SetDataResultListener as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SETDATARESULTLISTENER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SetDataResultListener {
    fn type_info(&self) -> &'static TypeInfo {
        SETDATARESULTLISTENER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SETDATARESULTLISTENER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetDataResultListener-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SetDataResultListener"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DataBusData {
    pub _glacier_base: Asset,
    pub flags: u16,
    pub property_connections: Vec<PropertyConnection>,
    pub link_connections: Vec<LinkConnection>,
    pub interface: Option<Arc<Mutex<dyn DynamicDataContainerTrait>>>,
}

pub trait DataBusDataTrait: AssetTrait {
    fn flags(&self) -> &u16;
    fn property_connections(&self) -> &Vec<PropertyConnection>;
    fn link_connections(&self) -> &Vec<LinkConnection>;
    fn interface(&self) -> &Option<Arc<Mutex<dyn DynamicDataContainerTrait>>>;
}

impl DataBusDataTrait for DataBusData {
    fn flags(&self) -> &u16 {
        &self.flags
    }
    fn property_connections(&self) -> &Vec<PropertyConnection> {
        &self.property_connections
    }
    fn link_connections(&self) -> &Vec<LinkConnection> {
        &self.link_connections
    }
    fn interface(&self) -> &Option<Arc<Mutex<dyn DynamicDataContainerTrait>>> {
        &self.interface
    }
}

impl AssetTrait for DataBusData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl DataContainerTrait for DataBusData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DATABUSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataBusData",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DataBusData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Flags",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(DataBusData, flags),
            },
            FieldInfoData {
                name: "PropertyConnections",
                flags: MemberInfoFlags::new(144),
                field_type: "PropertyConnection-Array",
                rust_offset: offset_of!(DataBusData, property_connections),
            },
            FieldInfoData {
                name: "LinkConnections",
                flags: MemberInfoFlags::new(144),
                field_type: "LinkConnection-Array",
                rust_offset: offset_of!(DataBusData, link_connections),
            },
            FieldInfoData {
                name: "Interface",
                flags: MemberInfoFlags::new(0),
                field_type: "DynamicDataContainer",
                rust_offset: offset_of!(DataBusData, interface),
            },
        ],
    }),
    array_type: Some(DATABUSDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DataBusData {
    fn type_info(&self) -> &'static TypeInfo {
        DATABUSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DATABUSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataBusData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DataBusData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DynamicDataContainer {
    pub _glacier_base: DataContainer,
    pub fields: Vec<DataField>,
}

pub trait DynamicDataContainerTrait: DataContainerTrait {
    fn fields(&self) -> &Vec<DataField>;
}

impl DynamicDataContainerTrait for DynamicDataContainer {
    fn fields(&self) -> &Vec<DataField> {
        &self.fields
    }
}

impl DataContainerTrait for DynamicDataContainer {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DYNAMICDATACONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicDataContainer",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DynamicDataContainer as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Fields",
                flags: MemberInfoFlags::new(144),
                field_type: "DataField-Array",
                rust_offset: offset_of!(DynamicDataContainer, fields),
            },
        ],
    }),
    array_type: Some(DYNAMICDATACONTAINER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DynamicDataContainer {
    fn type_info(&self) -> &'static TypeInfo {
        DYNAMICDATACONTAINER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DYNAMICDATACONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicDataContainer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DynamicDataContainer"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DataField {
    pub value: String,
    pub value_ref: Option<Arc<Mutex<dyn DataContainerTrait>>>,
    pub id: i32,
    pub access_type: FieldAccessType,
}

pub trait DataFieldTrait: TypeObject {
    fn value(&self) -> &String;
    fn value_ref(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>>;
    fn id(&self) -> &i32;
    fn access_type(&self) -> &FieldAccessType;
}

impl DataFieldTrait for DataField {
    fn value(&self) -> &String {
        &self.value
    }
    fn value_ref(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>> {
        &self.value_ref
    }
    fn id(&self) -> &i32 {
        &self.id
    }
    fn access_type(&self) -> &FieldAccessType {
        &self.access_type
    }
}

pub static DATAFIELD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataField",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DataField as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(DataField, value),
            },
            FieldInfoData {
                name: "ValueRef",
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(DataField, value_ref),
            },
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DataField, id),
            },
            FieldInfoData {
                name: "AccessType",
                flags: MemberInfoFlags::new(0),
                field_type: "FieldAccessType",
                rust_offset: offset_of!(DataField, access_type),
            },
        ],
    }),
    array_type: Some(DATAFIELD_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DataField {
    fn type_info(&self) -> &'static TypeInfo {
        DATAFIELD_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DATAFIELD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataField-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DataField"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum FieldAccessType {
    #[default]
    FieldAccessType_Source = 0,
    FieldAccessType_Target = 1,
    FieldAccessType_SourceAndTarget = 2,
}

pub static FIELDACCESSTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FieldAccessType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(FIELDACCESSTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FieldAccessType {
    fn type_info(&self) -> &'static TypeInfo {
        FIELDACCESSTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FIELDACCESSTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FieldAccessType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FieldAccessType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinkConnection {
    pub source: Option<Arc<Mutex<dyn DataContainerTrait>>>,
    pub target: Option<Arc<Mutex<dyn DataContainerTrait>>>,
    pub source_field_id: i32,
    pub target_field_id: i32,
}

pub trait LinkConnectionTrait: TypeObject {
    fn source(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>>;
    fn target(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>>;
    fn source_field_id(&self) -> &i32;
    fn target_field_id(&self) -> &i32;
}

impl LinkConnectionTrait for LinkConnection {
    fn source(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>> {
        &self.source
    }
    fn target(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>> {
        &self.target
    }
    fn source_field_id(&self) -> &i32 {
        &self.source_field_id
    }
    fn target_field_id(&self) -> &i32 {
        &self.target_field_id
    }
}

pub static LINKCONNECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkConnection",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinkConnection as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Source",
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(LinkConnection, source),
            },
            FieldInfoData {
                name: "Target",
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(LinkConnection, target),
            },
            FieldInfoData {
                name: "SourceFieldId",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(LinkConnection, source_field_id),
            },
            FieldInfoData {
                name: "TargetFieldId",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(LinkConnection, target_field_id),
            },
        ],
    }),
    array_type: Some(LINKCONNECTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinkConnection {
    fn type_info(&self) -> &'static TypeInfo {
        LINKCONNECTION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LINKCONNECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkConnection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("LinkConnection"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PropertyConnection {
    pub source: Option<Arc<Mutex<dyn DataContainerTrait>>>,
    pub target: Option<Arc<Mutex<dyn DataContainerTrait>>>,
    pub source_field_id: i32,
    pub target_field_id: i32,
    pub flags: u32,
}

pub trait PropertyConnectionTrait: TypeObject {
    fn source(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>>;
    fn target(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>>;
    fn source_field_id(&self) -> &i32;
    fn target_field_id(&self) -> &i32;
    fn flags(&self) -> &u32;
}

impl PropertyConnectionTrait for PropertyConnection {
    fn source(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>> {
        &self.source
    }
    fn target(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>> {
        &self.target
    }
    fn source_field_id(&self) -> &i32 {
        &self.source_field_id
    }
    fn target_field_id(&self) -> &i32 {
        &self.target_field_id
    }
    fn flags(&self) -> &u32 {
        &self.flags
    }
}

pub static PROPERTYCONNECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyConnection",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PropertyConnection as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Source",
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(PropertyConnection, source),
            },
            FieldInfoData {
                name: "Target",
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(PropertyConnection, target),
            },
            FieldInfoData {
                name: "SourceFieldId",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PropertyConnection, source_field_id),
            },
            FieldInfoData {
                name: "TargetFieldId",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PropertyConnection, target_field_id),
            },
            FieldInfoData {
                name: "Flags",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(PropertyConnection, flags),
            },
        ],
    }),
    array_type: Some(PROPERTYCONNECTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PropertyConnection {
    fn type_info(&self) -> &'static TypeInfo {
        PROPERTYCONNECTION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PROPERTYCONNECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyConnection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PropertyConnection"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum InputPropertyType {
    #[default]
    InputPropertyType_Default = 0,
    InputPropertyType_Interface = 1,
    InputPropertyType_Exposed = 2,
    InputPropertyType_Invalid = 3,
}

pub static INPUTPROPERTYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputPropertyType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(INPUTPROPERTYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InputPropertyType {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTPROPERTYTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INPUTPROPERTYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputPropertyType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("InputPropertyType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PropertyConnectionTargetType {
    #[default]
    PropertyConnectionTargetType_Invalid = 0,
    PropertyConnectionTargetType_ClientAndServer = 1,
    PropertyConnectionTargetType_Client = 2,
    PropertyConnectionTargetType_Server = 3,
    PropertyConnectionTargetType_NetworkedClient = 4,
    PropertyConnectionTargetType_NetworkedClientAndServer = 5,
}

pub static PROPERTYCONNECTIONTARGETTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyConnectionTargetType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(PROPERTYCONNECTIONTARGETTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PropertyConnectionTargetType {
    fn type_info(&self) -> &'static TypeInfo {
        PROPERTYCONNECTIONTARGETTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PROPERTYCONNECTIONTARGETTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyConnectionTargetType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PropertyConnectionTargetType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SettingsBundleAsset {
    pub _glacier_base: Asset,
    pub is_dedicated_server_bundle: bool,
    pub settings: Vec<Option<Arc<Mutex<dyn SystemSettingsTrait>>>>,
}

pub trait SettingsBundleAssetTrait: AssetTrait {
    fn is_dedicated_server_bundle(&self) -> &bool;
    fn settings(&self) -> &Vec<Option<Arc<Mutex<dyn SystemSettingsTrait>>>>;
}

impl SettingsBundleAssetTrait for SettingsBundleAsset {
    fn is_dedicated_server_bundle(&self) -> &bool {
        &self.is_dedicated_server_bundle
    }
    fn settings(&self) -> &Vec<Option<Arc<Mutex<dyn SystemSettingsTrait>>>> {
        &self.settings
    }
}

impl AssetTrait for SettingsBundleAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl DataContainerTrait for SettingsBundleAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SETTINGSBUNDLEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SettingsBundleAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SettingsBundleAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "IsDedicatedServerBundle",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SettingsBundleAsset, is_dedicated_server_bundle),
            },
            FieldInfoData {
                name: "Settings",
                flags: MemberInfoFlags::new(144),
                field_type: "SystemSettings-Array",
                rust_offset: offset_of!(SettingsBundleAsset, settings),
            },
        ],
    }),
    array_type: Some(SETTINGSBUNDLEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SettingsBundleAsset {
    fn type_info(&self) -> &'static TypeInfo {
        SETTINGSBUNDLEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SETTINGSBUNDLEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SettingsBundleAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SettingsBundleAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SystemSettings {
    pub _glacier_base: DataContainer,
    pub platform: GamePlatform,
}

pub trait SystemSettingsTrait: DataContainerTrait {
    fn platform(&self) -> &GamePlatform;
}

impl SystemSettingsTrait for SystemSettings {
    fn platform(&self) -> &GamePlatform {
        &self.platform
    }
}

impl DataContainerTrait for SystemSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SystemSettings",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SystemSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: "GamePlatform",
                rust_offset: offset_of!(SystemSettings, platform),
            },
        ],
    }),
    array_type: Some(SYSTEMSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SystemSettings {
    fn type_info(&self) -> &'static TypeInfo {
        SYSTEMSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SystemSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SystemSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DataBusPeer {
    pub _glacier_base: GameDataContainer,
    pub flags: u32,
}

pub trait DataBusPeerTrait: GameDataContainerTrait {
    fn flags(&self) -> &u32;
}

impl DataBusPeerTrait for DataBusPeer {
    fn flags(&self) -> &u32 {
        &self.flags
    }
}

impl GameDataContainerTrait for DataBusPeer {
}

impl DataContainerTrait for DataBusPeer {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DATABUSPEER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataBusPeer",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEDATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DataBusPeer as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Flags",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DataBusPeer, flags),
            },
        ],
    }),
    array_type: Some(DATABUSPEER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DataBusPeer {
    fn type_info(&self) -> &'static TypeInfo {
        DATABUSPEER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DATABUSPEER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataBusPeer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DataBusPeer"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GameDataContainer {
    pub _glacier_base: DataContainer,
}

pub trait GameDataContainerTrait: DataContainerTrait {
}

impl GameDataContainerTrait for GameDataContainer {
}

impl DataContainerTrait for GameDataContainer {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static GAMEDATACONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameDataContainer",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameDataContainer as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(GAMEDATACONTAINER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameDataContainer {
    fn type_info(&self) -> &'static TypeInfo {
        GAMEDATACONTAINER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static GAMEDATACONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameDataContainer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("GameDataContainer"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum Realm {
    #[default]
    Realm_Client = 0,
    Realm_Server = 1,
    Realm_ClientAndServer = 2,
    Realm_None = 3,
    Realm_Pipeline = 4,
}

pub static REALM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Realm",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(REALM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for Realm {
    fn type_info(&self) -> &'static TypeInfo {
        REALM_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static REALM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Realm-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Realm"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LanguageFormat {
    #[default]
    LanguageFormat_English = 0,
    LanguageFormat_French = 1,
    LanguageFormat_German = 2,
    LanguageFormat_Spanish = 3,
    LanguageFormat_SpanishMex = 4,
    LanguageFormat_Italian = 5,
    LanguageFormat_Japanese = 6,
    LanguageFormat_Russian = 7,
    LanguageFormat_Polish = 8,
    LanguageFormat_Dutch = 9,
    LanguageFormat_Portuguese = 10,
    LanguageFormat_TraditionalChinese = 11,
    LanguageFormat_Korean = 12,
    LanguageFormat_Czech = 13,
    LanguageFormat_BrazilianPortuguese = 14,
    LanguageFormat_ArabicSA = 15,
    LanguageFormat_Swedish = 16,
    LanguageFormat_Norwegian = 17,
    LanguageFormat_Danish = 18,
    LanguageFormat_Turkish = 19,
    LanguageFormat_SimplifiedChinese = 20,
    LanguageFormat_WorstCase = 21,
    LanguageFormat_Count = 22,
    LanguageFormat_Undefined = 23,
}

pub static LANGUAGEFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanguageFormat",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(LANGUAGEFORMAT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LanguageFormat {
    fn type_info(&self) -> &'static TypeInfo {
        LANGUAGEFORMAT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LANGUAGEFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanguageFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("LanguageFormat"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DataContainerPolicyAsset {
    pub _glacier_base: Asset,
}

pub trait DataContainerPolicyAssetTrait: AssetTrait {
}

impl DataContainerPolicyAssetTrait for DataContainerPolicyAsset {
}

impl AssetTrait for DataContainerPolicyAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl DataContainerTrait for DataContainerPolicyAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DATACONTAINERPOLICYASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainerPolicyAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DataContainerPolicyAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DATACONTAINERPOLICYASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DataContainerPolicyAsset {
    fn type_info(&self) -> &'static TypeInfo {
        DATACONTAINERPOLICYASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DATACONTAINERPOLICYASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainerPolicyAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DataContainerPolicyAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Asset {
    pub _glacier_base: DataContainer,
    pub name: String,
}

pub trait AssetTrait: DataContainerTrait {
    fn name(&self) -> &String;
}

impl AssetTrait for Asset {
    fn name(&self) -> &String {
        &self.name
    }
}

impl DataContainerTrait for Asset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Asset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Asset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(Asset, name),
            },
        ],
    }),
    array_type: Some(ASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Asset {
    fn type_info(&self) -> &'static TypeInfo {
        ASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Asset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Asset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct NativeFunctionTypeInfoAsset {
    pub _glacier_base: FunctionTypeInfoAsset,
    pub function: glacier_reflect::builtin::TypeRef,
}

pub trait NativeFunctionTypeInfoAssetTrait: FunctionTypeInfoAssetTrait {
    fn function(&self) -> &glacier_reflect::builtin::TypeRef;
}

impl NativeFunctionTypeInfoAssetTrait for NativeFunctionTypeInfoAsset {
    fn function(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.function
    }
}

impl FunctionTypeInfoAssetTrait for NativeFunctionTypeInfoAsset {
    fn parameters(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoParameterDataContainerTrait>>>> {
        self._glacier_base.parameters()
    }
    fn owner(&self) -> &Option<Arc<Mutex<dyn ClassInfoAssetTrait>>> {
        self._glacier_base.owner()
    }
}

impl TypeInfoAssetTrait for NativeFunctionTypeInfoAsset {
    fn module_name(&self) -> &String {
        self._glacier_base.module_name()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn attributes(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoAttributeTrait>>>> {
        self._glacier_base.attributes()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
}

impl AssetTrait for NativeFunctionTypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl DataContainerTrait for NativeFunctionTypeInfoAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static NATIVEFUNCTIONTYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NativeFunctionTypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FUNCTIONTYPEINFOASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NativeFunctionTypeInfoAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Function",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(NativeFunctionTypeInfoAsset, function),
            },
        ],
    }),
    array_type: Some(NATIVEFUNCTIONTYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NativeFunctionTypeInfoAsset {
    fn type_info(&self) -> &'static TypeInfo {
        NATIVEFUNCTIONTYPEINFOASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static NATIVEFUNCTIONTYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NativeFunctionTypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("NativeFunctionTypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldStructValue {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: glacier_reflect::builtin::BoxedValueRef,
}

pub trait TypeInfoFieldStructValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &glacier_reflect::builtin::BoxedValueRef;
}

impl TypeInfoFieldStructValueTrait for TypeInfoFieldStructValue {
    fn value(&self) -> &glacier_reflect::builtin::BoxedValueRef {
        &self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldStructValue {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
}

impl DataContainerTrait for TypeInfoFieldStructValue {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDSTRUCTVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldStructValue",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldStructValue as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "BoxedValueRef",
                rust_offset: offset_of!(TypeInfoFieldStructValue, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDSTRUCTVALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldStructValue {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDSTRUCTVALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDSTRUCTVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldStructValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldStructValue"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldRefValue {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: Option<Arc<Mutex<dyn DataContainerTrait>>>,
}

pub trait TypeInfoFieldRefValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>>;
}

impl TypeInfoFieldRefValueTrait for TypeInfoFieldRefValue {
    fn value(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>> {
        &self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldRefValue {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
}

impl DataContainerTrait for TypeInfoFieldRefValue {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDREFVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldRefValue",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldRefValue as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(TypeInfoFieldRefValue, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDREFVALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldRefValue {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDREFVALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDREFVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldRefValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldRefValue"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldTypeRefValue {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: glacier_reflect::builtin::TypeRef,
}

pub trait TypeInfoFieldTypeRefValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &glacier_reflect::builtin::TypeRef;
}

impl TypeInfoFieldTypeRefValueTrait for TypeInfoFieldTypeRefValue {
    fn value(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldTypeRefValue {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
}

impl DataContainerTrait for TypeInfoFieldTypeRefValue {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDTYPEREFVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldTypeRefValue",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldTypeRefValue as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(TypeInfoFieldTypeRefValue, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDTYPEREFVALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldTypeRefValue {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDTYPEREFVALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDTYPEREFVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldTypeRefValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldTypeRefValue"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldSHA1Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: glacier_reflect::builtin::SHA1,
}

pub trait TypeInfoFieldSHA1ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &glacier_reflect::builtin::SHA1;
}

impl TypeInfoFieldSHA1ValueTrait for TypeInfoFieldSHA1Value {
    fn value(&self) -> &glacier_reflect::builtin::SHA1 {
        &self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldSHA1Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
}

impl DataContainerTrait for TypeInfoFieldSHA1Value {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDSHA1VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldSHA1Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldSHA1Value as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "SHA1",
                rust_offset: offset_of!(TypeInfoFieldSHA1Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDSHA1VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldSHA1Value {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDSHA1VALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDSHA1VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldSHA1Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldSHA1Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldGuidValue {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: glacier_util::guid::Guid,
}

pub trait TypeInfoFieldGuidValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &glacier_util::guid::Guid;
}

impl TypeInfoFieldGuidValueTrait for TypeInfoFieldGuidValue {
    fn value(&self) -> &glacier_util::guid::Guid {
        &self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldGuidValue {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
}

impl DataContainerTrait for TypeInfoFieldGuidValue {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDGUIDVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldGuidValue",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldGuidValue as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(TypeInfoFieldGuidValue, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDGUIDVALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldGuidValue {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDGUIDVALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDGUIDVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldGuidValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldGuidValue"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldEnumValue {
    pub _glacier_base: TypeInfoFieldValue,
    pub enum_info: glacier_reflect::builtin::TypeRef,
    pub value: u32,
}

pub trait TypeInfoFieldEnumValueTrait: TypeInfoFieldValueTrait {
    fn enum_info(&self) -> &glacier_reflect::builtin::TypeRef;
    fn value(&self) -> &u32;
}

impl TypeInfoFieldEnumValueTrait for TypeInfoFieldEnumValue {
    fn enum_info(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.enum_info
    }
    fn value(&self) -> &u32 {
        &self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldEnumValue {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
}

impl DataContainerTrait for TypeInfoFieldEnumValue {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDENUMVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldEnumValue",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldEnumValue as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EnumInfo",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(TypeInfoFieldEnumValue, enum_info),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TypeInfoFieldEnumValue, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDENUMVALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldEnumValue {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDENUMVALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDENUMVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldEnumValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldEnumValue"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldStringValue {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: String,
}

pub trait TypeInfoFieldStringValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &String;
}

impl TypeInfoFieldStringValueTrait for TypeInfoFieldStringValue {
    fn value(&self) -> &String {
        &self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldStringValue {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
}

impl DataContainerTrait for TypeInfoFieldStringValue {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDSTRINGVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldStringValue",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldStringValue as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TypeInfoFieldStringValue, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDSTRINGVALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldStringValue {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDSTRINGVALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDSTRINGVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldStringValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldStringValue"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldFloat64Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: f64,
}

pub trait TypeInfoFieldFloat64ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &f64;
}

impl TypeInfoFieldFloat64ValueTrait for TypeInfoFieldFloat64Value {
    fn value(&self) -> &f64 {
        &self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldFloat64Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
}

impl DataContainerTrait for TypeInfoFieldFloat64Value {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDFLOAT64VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldFloat64Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldFloat64Value as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Float64",
                rust_offset: offset_of!(TypeInfoFieldFloat64Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDFLOAT64VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldFloat64Value {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDFLOAT64VALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDFLOAT64VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldFloat64Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldFloat64Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldFloat32Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: f32,
}

pub trait TypeInfoFieldFloat32ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &f32;
}

impl TypeInfoFieldFloat32ValueTrait for TypeInfoFieldFloat32Value {
    fn value(&self) -> &f32 {
        &self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldFloat32Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
}

impl DataContainerTrait for TypeInfoFieldFloat32Value {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDFLOAT32VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldFloat32Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldFloat32Value as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TypeInfoFieldFloat32Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDFLOAT32VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldFloat32Value {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDFLOAT32VALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDFLOAT32VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldFloat32Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldFloat32Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldUint64Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: u64,
}

pub trait TypeInfoFieldUint64ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &u64;
}

impl TypeInfoFieldUint64ValueTrait for TypeInfoFieldUint64Value {
    fn value(&self) -> &u64 {
        &self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldUint64Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
}

impl DataContainerTrait for TypeInfoFieldUint64Value {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDUINT64VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint64Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldUint64Value as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(TypeInfoFieldUint64Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDUINT64VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldUint64Value {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDUINT64VALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDUINT64VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint64Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldUint64Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldUint32Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: u32,
}

pub trait TypeInfoFieldUint32ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &u32;
}

impl TypeInfoFieldUint32ValueTrait for TypeInfoFieldUint32Value {
    fn value(&self) -> &u32 {
        &self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldUint32Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
}

impl DataContainerTrait for TypeInfoFieldUint32Value {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDUINT32VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint32Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldUint32Value as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TypeInfoFieldUint32Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDUINT32VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldUint32Value {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDUINT32VALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDUINT32VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint32Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldUint32Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldUint16Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: u16,
}

pub trait TypeInfoFieldUint16ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &u16;
}

impl TypeInfoFieldUint16ValueTrait for TypeInfoFieldUint16Value {
    fn value(&self) -> &u16 {
        &self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldUint16Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
}

impl DataContainerTrait for TypeInfoFieldUint16Value {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDUINT16VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint16Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldUint16Value as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TypeInfoFieldUint16Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDUINT16VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldUint16Value {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDUINT16VALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDUINT16VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint16Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldUint16Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldUint8Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: u8,
}

pub trait TypeInfoFieldUint8ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &u8;
}

impl TypeInfoFieldUint8ValueTrait for TypeInfoFieldUint8Value {
    fn value(&self) -> &u8 {
        &self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldUint8Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
}

impl DataContainerTrait for TypeInfoFieldUint8Value {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDUINT8VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint8Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldUint8Value as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TypeInfoFieldUint8Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDUINT8VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldUint8Value {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDUINT8VALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDUINT8VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint8Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldUint8Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldInt64Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: i64,
}

pub trait TypeInfoFieldInt64ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &i64;
}

impl TypeInfoFieldInt64ValueTrait for TypeInfoFieldInt64Value {
    fn value(&self) -> &i64 {
        &self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldInt64Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
}

impl DataContainerTrait for TypeInfoFieldInt64Value {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDINT64VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt64Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldInt64Value as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TypeInfoFieldInt64Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDINT64VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldInt64Value {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDINT64VALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDINT64VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt64Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldInt64Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldInt32Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: i32,
}

pub trait TypeInfoFieldInt32ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &i32;
}

impl TypeInfoFieldInt32ValueTrait for TypeInfoFieldInt32Value {
    fn value(&self) -> &i32 {
        &self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldInt32Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
}

impl DataContainerTrait for TypeInfoFieldInt32Value {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDINT32VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt32Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldInt32Value as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TypeInfoFieldInt32Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDINT32VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldInt32Value {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDINT32VALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDINT32VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt32Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldInt32Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldInt16Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: i16,
}

pub trait TypeInfoFieldInt16ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &i16;
}

impl TypeInfoFieldInt16ValueTrait for TypeInfoFieldInt16Value {
    fn value(&self) -> &i16 {
        &self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldInt16Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
}

impl DataContainerTrait for TypeInfoFieldInt16Value {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDINT16VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt16Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldInt16Value as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Int16",
                rust_offset: offset_of!(TypeInfoFieldInt16Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDINT16VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldInt16Value {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDINT16VALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDINT16VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt16Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldInt16Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldInt8Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: i8,
}

pub trait TypeInfoFieldInt8ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &i8;
}

impl TypeInfoFieldInt8ValueTrait for TypeInfoFieldInt8Value {
    fn value(&self) -> &i8 {
        &self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldInt8Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
}

impl DataContainerTrait for TypeInfoFieldInt8Value {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDINT8VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt8Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldInt8Value as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Int8",
                rust_offset: offset_of!(TypeInfoFieldInt8Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDINT8VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldInt8Value {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDINT8VALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDINT8VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt8Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldInt8Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldBooleanValue {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: bool,
}

pub trait TypeInfoFieldBooleanValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &bool;
}

impl TypeInfoFieldBooleanValueTrait for TypeInfoFieldBooleanValue {
    fn value(&self) -> &bool {
        &self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldBooleanValue {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
}

impl DataContainerTrait for TypeInfoFieldBooleanValue {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDBOOLEANVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldBooleanValue",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldBooleanValue as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TypeInfoFieldBooleanValue, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDBOOLEANVALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldBooleanValue {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDBOOLEANVALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDBOOLEANVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldBooleanValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldBooleanValue"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PrimitiveTypeInfoAsset {
    pub _glacier_base: TypeInfoAsset,
}

pub trait PrimitiveTypeInfoAssetTrait: TypeInfoAssetTrait {
}

impl PrimitiveTypeInfoAssetTrait for PrimitiveTypeInfoAsset {
}

impl TypeInfoAssetTrait for PrimitiveTypeInfoAsset {
    fn module_name(&self) -> &String {
        self._glacier_base.module_name()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn attributes(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoAttributeTrait>>>> {
        self._glacier_base.attributes()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
}

impl AssetTrait for PrimitiveTypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl DataContainerTrait for PrimitiveTypeInfoAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PRIMITIVETYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrimitiveTypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PrimitiveTypeInfoAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRIMITIVETYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PrimitiveTypeInfoAsset {
    fn type_info(&self) -> &'static TypeInfo {
        PRIMITIVETYPEINFOASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PRIMITIVETYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrimitiveTypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PrimitiveTypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EnumTypeInfoAsset {
    pub _glacier_base: TypeInfoAsset,
    pub enumerators: Vec<EnumTypeEnumeratorData>,
}

pub trait EnumTypeInfoAssetTrait: TypeInfoAssetTrait {
    fn enumerators(&self) -> &Vec<EnumTypeEnumeratorData>;
}

impl EnumTypeInfoAssetTrait for EnumTypeInfoAsset {
    fn enumerators(&self) -> &Vec<EnumTypeEnumeratorData> {
        &self.enumerators
    }
}

impl TypeInfoAssetTrait for EnumTypeInfoAsset {
    fn module_name(&self) -> &String {
        self._glacier_base.module_name()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn attributes(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoAttributeTrait>>>> {
        self._glacier_base.attributes()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
}

impl AssetTrait for EnumTypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl DataContainerTrait for EnumTypeInfoAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ENUMTYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumTypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnumTypeInfoAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enumerators",
                flags: MemberInfoFlags::new(144),
                field_type: "EnumTypeEnumeratorData-Array",
                rust_offset: offset_of!(EnumTypeInfoAsset, enumerators),
            },
        ],
    }),
    array_type: Some(ENUMTYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnumTypeInfoAsset {
    fn type_info(&self) -> &'static TypeInfo {
        ENUMTYPEINFOASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENUMTYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumTypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("EnumTypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EnumTypeEnumeratorData {
    pub name: String,
    pub value: i32,
    pub attributes: Vec<Option<Arc<Mutex<dyn TypeInfoAttributeTrait>>>>,
}

pub trait EnumTypeEnumeratorDataTrait: TypeObject {
    fn name(&self) -> &String;
    fn value(&self) -> &i32;
    fn attributes(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoAttributeTrait>>>>;
}

impl EnumTypeEnumeratorDataTrait for EnumTypeEnumeratorData {
    fn name(&self) -> &String {
        &self.name
    }
    fn value(&self) -> &i32 {
        &self.value
    }
    fn attributes(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoAttributeTrait>>>> {
        &self.attributes
    }
}

pub static ENUMTYPEENUMERATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumTypeEnumeratorData",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnumTypeEnumeratorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EnumTypeEnumeratorData, name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EnumTypeEnumeratorData, value),
            },
            FieldInfoData {
                name: "Attributes",
                flags: MemberInfoFlags::new(144),
                field_type: "TypeInfoAttribute-Array",
                rust_offset: offset_of!(EnumTypeEnumeratorData, attributes),
            },
        ],
    }),
    array_type: Some(ENUMTYPEENUMERATORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnumTypeEnumeratorData {
    fn type_info(&self) -> &'static TypeInfo {
        ENUMTYPEENUMERATORDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENUMTYPEENUMERATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumTypeEnumeratorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("EnumTypeEnumeratorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FunctionTypeInfoAsset {
    pub _glacier_base: TypeInfoAsset,
    pub parameters: Vec<Option<Arc<Mutex<dyn TypeInfoParameterDataContainerTrait>>>>,
    pub owner: Option<Arc<Mutex<dyn ClassInfoAssetTrait>>>,
}

pub trait FunctionTypeInfoAssetTrait: TypeInfoAssetTrait {
    fn parameters(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoParameterDataContainerTrait>>>>;
    fn owner(&self) -> &Option<Arc<Mutex<dyn ClassInfoAssetTrait>>>;
}

impl FunctionTypeInfoAssetTrait for FunctionTypeInfoAsset {
    fn parameters(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoParameterDataContainerTrait>>>> {
        &self.parameters
    }
    fn owner(&self) -> &Option<Arc<Mutex<dyn ClassInfoAssetTrait>>> {
        &self.owner
    }
}

impl TypeInfoAssetTrait for FunctionTypeInfoAsset {
    fn module_name(&self) -> &String {
        self._glacier_base.module_name()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn attributes(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoAttributeTrait>>>> {
        self._glacier_base.attributes()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
}

impl AssetTrait for FunctionTypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl DataContainerTrait for FunctionTypeInfoAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FUNCTIONTYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FunctionTypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FunctionTypeInfoAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Parameters",
                flags: MemberInfoFlags::new(144),
                field_type: "TypeInfoParameterDataContainer-Array",
                rust_offset: offset_of!(FunctionTypeInfoAsset, parameters),
            },
            FieldInfoData {
                name: "Owner",
                flags: MemberInfoFlags::new(0),
                field_type: "ClassInfoAsset",
                rust_offset: offset_of!(FunctionTypeInfoAsset, owner),
            },
        ],
    }),
    array_type: Some(FUNCTIONTYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FunctionTypeInfoAsset {
    fn type_info(&self) -> &'static TypeInfo {
        FUNCTIONTYPEINFOASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FUNCTIONTYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FunctionTypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FunctionTypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoParameterDataContainer {
    pub _glacier_base: DataContainer,
    pub name: String,
    pub type_ref: TypeInfoRef,
    pub is_array: bool,
    pub qualifier: TypeInfoParameterQualifier,
    pub default_value: Option<Arc<Mutex<dyn DataContainerTrait>>>,
}

pub trait TypeInfoParameterDataContainerTrait: DataContainerTrait {
    fn name(&self) -> &String;
    fn type_ref(&self) -> &TypeInfoRef;
    fn is_array(&self) -> &bool;
    fn qualifier(&self) -> &TypeInfoParameterQualifier;
    fn default_value(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>>;
}

impl TypeInfoParameterDataContainerTrait for TypeInfoParameterDataContainer {
    fn name(&self) -> &String {
        &self.name
    }
    fn type_ref(&self) -> &TypeInfoRef {
        &self.type_ref
    }
    fn is_array(&self) -> &bool {
        &self.is_array
    }
    fn qualifier(&self) -> &TypeInfoParameterQualifier {
        &self.qualifier
    }
    fn default_value(&self) -> &Option<Arc<Mutex<dyn DataContainerTrait>>> {
        &self.default_value
    }
}

impl DataContainerTrait for TypeInfoParameterDataContainer {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOPARAMETERDATACONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoParameterDataContainer",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoParameterDataContainer as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TypeInfoParameterDataContainer, name),
            },
            FieldInfoData {
                name: "TypeRef",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeInfoRef",
                rust_offset: offset_of!(TypeInfoParameterDataContainer, type_ref),
            },
            FieldInfoData {
                name: "IsArray",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TypeInfoParameterDataContainer, is_array),
            },
            FieldInfoData {
                name: "Qualifier",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeInfoParameterQualifier",
                rust_offset: offset_of!(TypeInfoParameterDataContainer, qualifier),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(TypeInfoParameterDataContainer, default_value),
            },
        ],
    }),
    array_type: Some(TYPEINFOPARAMETERDATACONTAINER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoParameterDataContainer {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOPARAMETERDATACONTAINER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOPARAMETERDATACONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoParameterDataContainer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoParameterDataContainer"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TypeInfoParameterQualifier {
    #[default]
    TypeInfoParameterQualifier_In = 0,
    TypeInfoParameterQualifier_Out = 1,
    TypeInfoParameterQualifier_InRef = 2,
    TypeInfoParameterQualifier_OutRef = 3,
}

pub static TYPEINFOPARAMETERQUALIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoParameterQualifier",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(TYPEINFOPARAMETERQUALIFIER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TypeInfoParameterQualifier {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOPARAMETERQUALIFIER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOPARAMETERQUALIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoParameterQualifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoParameterQualifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FunctionTypeInfoRef {
    pub asset: Option<Arc<Mutex<dyn FunctionTypeInfoAssetTrait>>>,
    pub type_info: glacier_reflect::builtin::TypeRef,
}

pub trait FunctionTypeInfoRefTrait: TypeObject {
    fn asset(&self) -> &Option<Arc<Mutex<dyn FunctionTypeInfoAssetTrait>>>;
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef;
}

impl FunctionTypeInfoRefTrait for FunctionTypeInfoRef {
    fn asset(&self) -> &Option<Arc<Mutex<dyn FunctionTypeInfoAssetTrait>>> {
        &self.asset
    }
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.type_info
    }
}

pub static FUNCTIONTYPEINFOREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FunctionTypeInfoRef",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FunctionTypeInfoRef as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: "FunctionTypeInfoAsset",
                rust_offset: offset_of!(FunctionTypeInfoRef, asset),
            },
            FieldInfoData {
                name: "TypeInfo",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(FunctionTypeInfoRef, type_info),
            },
        ],
    }),
    array_type: Some(FUNCTIONTYPEINFOREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FunctionTypeInfoRef {
    fn type_info(&self) -> &'static TypeInfo {
        FUNCTIONTYPEINFOREF_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FUNCTIONTYPEINFOREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FunctionTypeInfoRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FunctionTypeInfoRef"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ValueTypeInfoAsset {
    pub _glacier_base: ComplexTypeInfoAsset,
}

pub trait ValueTypeInfoAssetTrait: ComplexTypeInfoAssetTrait {
}

impl ValueTypeInfoAssetTrait for ValueTypeInfoAsset {
}

impl ComplexTypeInfoAssetTrait for ValueTypeInfoAsset {
    fn field_collections(&self) -> &Vec<TypeInfoFieldCollectionRef> {
        self._glacier_base.field_collections()
    }
    fn alignment(&self) -> &u32 {
        self._glacier_base.alignment()
    }
}

impl TypeInfoAssetTrait for ValueTypeInfoAsset {
    fn module_name(&self) -> &String {
        self._glacier_base.module_name()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn attributes(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoAttributeTrait>>>> {
        self._glacier_base.attributes()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
}

impl AssetTrait for ValueTypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl DataContainerTrait for ValueTypeInfoAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static VALUETYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValueTypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPLEXTYPEINFOASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ValueTypeInfoAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VALUETYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ValueTypeInfoAsset {
    fn type_info(&self) -> &'static TypeInfo {
        VALUETYPEINFOASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VALUETYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValueTypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ValueTypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClassInfoAsset {
    pub _glacier_base: ComplexTypeInfoAsset,
    pub super_class_ref: ClassInfoRef,
    pub is_abstract: bool,
    pub is_sealed: bool,
}

pub trait ClassInfoAssetTrait: ComplexTypeInfoAssetTrait {
    fn super_class_ref(&self) -> &ClassInfoRef;
    fn is_abstract(&self) -> &bool;
    fn is_sealed(&self) -> &bool;
}

impl ClassInfoAssetTrait for ClassInfoAsset {
    fn super_class_ref(&self) -> &ClassInfoRef {
        &self.super_class_ref
    }
    fn is_abstract(&self) -> &bool {
        &self.is_abstract
    }
    fn is_sealed(&self) -> &bool {
        &self.is_sealed
    }
}

impl ComplexTypeInfoAssetTrait for ClassInfoAsset {
    fn field_collections(&self) -> &Vec<TypeInfoFieldCollectionRef> {
        self._glacier_base.field_collections()
    }
    fn alignment(&self) -> &u32 {
        self._glacier_base.alignment()
    }
}

impl TypeInfoAssetTrait for ClassInfoAsset {
    fn module_name(&self) -> &String {
        self._glacier_base.module_name()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn attributes(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoAttributeTrait>>>> {
        self._glacier_base.attributes()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
}

impl AssetTrait for ClassInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl DataContainerTrait for ClassInfoAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CLASSINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClassInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPLEXTYPEINFOASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClassInfoAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SuperClassRef",
                flags: MemberInfoFlags::new(0),
                field_type: "ClassInfoRef",
                rust_offset: offset_of!(ClassInfoAsset, super_class_ref),
            },
            FieldInfoData {
                name: "IsAbstract",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClassInfoAsset, is_abstract),
            },
            FieldInfoData {
                name: "IsSealed",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClassInfoAsset, is_sealed),
            },
        ],
    }),
    array_type: Some(CLASSINFOASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClassInfoAsset {
    fn type_info(&self) -> &'static TypeInfo {
        CLASSINFOASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLASSINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClassInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ClassInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClassInfoRef {
    pub asset: Option<Arc<Mutex<dyn ClassInfoAssetTrait>>>,
    pub type_info: glacier_reflect::builtin::TypeRef,
}

pub trait ClassInfoRefTrait: TypeObject {
    fn asset(&self) -> &Option<Arc<Mutex<dyn ClassInfoAssetTrait>>>;
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef;
}

impl ClassInfoRefTrait for ClassInfoRef {
    fn asset(&self) -> &Option<Arc<Mutex<dyn ClassInfoAssetTrait>>> {
        &self.asset
    }
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.type_info
    }
}

pub static CLASSINFOREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClassInfoRef",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClassInfoRef as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: "ClassInfoAsset",
                rust_offset: offset_of!(ClassInfoRef, asset),
            },
            FieldInfoData {
                name: "TypeInfo",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(ClassInfoRef, type_info),
            },
        ],
    }),
    array_type: Some(CLASSINFOREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClassInfoRef {
    fn type_info(&self) -> &'static TypeInfo {
        CLASSINFOREF_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLASSINFOREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClassInfoRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ClassInfoRef"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ComplexTypeInfoAsset {
    pub _glacier_base: TypeInfoAsset,
    pub field_collections: Vec<TypeInfoFieldCollectionRef>,
    pub alignment: u32,
}

pub trait ComplexTypeInfoAssetTrait: TypeInfoAssetTrait {
    fn field_collections(&self) -> &Vec<TypeInfoFieldCollectionRef>;
    fn alignment(&self) -> &u32;
}

impl ComplexTypeInfoAssetTrait for ComplexTypeInfoAsset {
    fn field_collections(&self) -> &Vec<TypeInfoFieldCollectionRef> {
        &self.field_collections
    }
    fn alignment(&self) -> &u32 {
        &self.alignment
    }
}

impl TypeInfoAssetTrait for ComplexTypeInfoAsset {
    fn module_name(&self) -> &String {
        self._glacier_base.module_name()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn attributes(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoAttributeTrait>>>> {
        self._glacier_base.attributes()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
}

impl AssetTrait for ComplexTypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl DataContainerTrait for ComplexTypeInfoAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static COMPLEXTYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComplexTypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ComplexTypeInfoAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FieldCollections",
                flags: MemberInfoFlags::new(144),
                field_type: "TypeInfoFieldCollectionRef-Array",
                rust_offset: offset_of!(ComplexTypeInfoAsset, field_collections),
            },
            FieldInfoData {
                name: "Alignment",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ComplexTypeInfoAsset, alignment),
            },
        ],
    }),
    array_type: Some(COMPLEXTYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ComplexTypeInfoAsset {
    fn type_info(&self) -> &'static TypeInfo {
        COMPLEXTYPEINFOASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static COMPLEXTYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComplexTypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ComplexTypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldCollectionRef {
    pub collection: Option<Arc<Mutex<dyn TypeInfoFieldCollectionTrait>>>,
}

pub trait TypeInfoFieldCollectionRefTrait: TypeObject {
    fn collection(&self) -> &Option<Arc<Mutex<dyn TypeInfoFieldCollectionTrait>>>;
}

impl TypeInfoFieldCollectionRefTrait for TypeInfoFieldCollectionRef {
    fn collection(&self) -> &Option<Arc<Mutex<dyn TypeInfoFieldCollectionTrait>>> {
        &self.collection
    }
}

pub static TYPEINFOFIELDCOLLECTIONREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldCollectionRef",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldCollectionRef as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Collection",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeInfoFieldCollection",
                rust_offset: offset_of!(TypeInfoFieldCollectionRef, collection),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDCOLLECTIONREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldCollectionRef {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDCOLLECTIONREF_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDCOLLECTIONREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldCollectionRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldCollectionRef"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldCollection {
    pub _glacier_base: DataContainer,
    pub fields: Vec<Option<Arc<Mutex<dyn TypeInfoFieldDataTrait>>>>,
    pub default_values: Vec<Option<Arc<Mutex<dyn TypeInfoFieldValueTrait>>>>,
}

pub trait TypeInfoFieldCollectionTrait: DataContainerTrait {
    fn fields(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoFieldDataTrait>>>>;
    fn default_values(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoFieldValueTrait>>>>;
}

impl TypeInfoFieldCollectionTrait for TypeInfoFieldCollection {
    fn fields(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoFieldDataTrait>>>> {
        &self.fields
    }
    fn default_values(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoFieldValueTrait>>>> {
        &self.default_values
    }
}

impl DataContainerTrait for TypeInfoFieldCollection {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDCOLLECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldCollection",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldCollection as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Fields",
                flags: MemberInfoFlags::new(144),
                field_type: "TypeInfoFieldData-Array",
                rust_offset: offset_of!(TypeInfoFieldCollection, fields),
            },
            FieldInfoData {
                name: "DefaultValues",
                flags: MemberInfoFlags::new(144),
                field_type: "TypeInfoFieldValue-Array",
                rust_offset: offset_of!(TypeInfoFieldCollection, default_values),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDCOLLECTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldCollection {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDCOLLECTION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDCOLLECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldCollection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldCollection"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldValue {
    pub _glacier_base: DataContainer,
    pub field: String,
}

pub trait TypeInfoFieldValueTrait: DataContainerTrait {
    fn field(&self) -> &String;
}

impl TypeInfoFieldValueTrait for TypeInfoFieldValue {
    fn field(&self) -> &String {
        &self.field
    }
}

impl DataContainerTrait for TypeInfoFieldValue {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldValue",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldValue as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Field",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TypeInfoFieldValue, field),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDVALUE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TypeInfoFieldValue {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDVALUE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldValue"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoFieldData {
    pub _glacier_base: DataContainer,
    pub name: String,
    pub type_ref: TypeInfoRef,
    pub is_array: bool,
    pub protection_level: ProtectionLevel,
    pub memory_sort_index: u32,
    pub is_meta: bool,
    pub is_exposed: bool,
    pub access_type: AccessType,
    pub always_persist: bool,
    pub attributes: Vec<Option<Arc<Mutex<dyn TypeInfoAttributeTrait>>>>,
}

pub trait TypeInfoFieldDataTrait: DataContainerTrait {
    fn name(&self) -> &String;
    fn type_ref(&self) -> &TypeInfoRef;
    fn is_array(&self) -> &bool;
    fn protection_level(&self) -> &ProtectionLevel;
    fn memory_sort_index(&self) -> &u32;
    fn is_meta(&self) -> &bool;
    fn is_exposed(&self) -> &bool;
    fn access_type(&self) -> &AccessType;
    fn always_persist(&self) -> &bool;
    fn attributes(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoAttributeTrait>>>>;
}

impl TypeInfoFieldDataTrait for TypeInfoFieldData {
    fn name(&self) -> &String {
        &self.name
    }
    fn type_ref(&self) -> &TypeInfoRef {
        &self.type_ref
    }
    fn is_array(&self) -> &bool {
        &self.is_array
    }
    fn protection_level(&self) -> &ProtectionLevel {
        &self.protection_level
    }
    fn memory_sort_index(&self) -> &u32 {
        &self.memory_sort_index
    }
    fn is_meta(&self) -> &bool {
        &self.is_meta
    }
    fn is_exposed(&self) -> &bool {
        &self.is_exposed
    }
    fn access_type(&self) -> &AccessType {
        &self.access_type
    }
    fn always_persist(&self) -> &bool {
        &self.always_persist
    }
    fn attributes(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoAttributeTrait>>>> {
        &self.attributes
    }
}

impl DataContainerTrait for TypeInfoFieldData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOFIELDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldData",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TypeInfoFieldData, name),
            },
            FieldInfoData {
                name: "TypeRef",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeInfoRef",
                rust_offset: offset_of!(TypeInfoFieldData, type_ref),
            },
            FieldInfoData {
                name: "IsArray",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TypeInfoFieldData, is_array),
            },
            FieldInfoData {
                name: "ProtectionLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "ProtectionLevel",
                rust_offset: offset_of!(TypeInfoFieldData, protection_level),
            },
            FieldInfoData {
                name: "MemorySortIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TypeInfoFieldData, memory_sort_index),
            },
            FieldInfoData {
                name: "IsMeta",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TypeInfoFieldData, is_meta),
            },
            FieldInfoData {
                name: "IsExposed",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TypeInfoFieldData, is_exposed),
            },
            FieldInfoData {
                name: "AccessType",
                flags: MemberInfoFlags::new(0),
                field_type: "AccessType",
                rust_offset: offset_of!(TypeInfoFieldData, access_type),
            },
            FieldInfoData {
                name: "AlwaysPersist",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TypeInfoFieldData, always_persist),
            },
            FieldInfoData {
                name: "Attributes",
                flags: MemberInfoFlags::new(144),
                field_type: "TypeInfoAttribute-Array",
                rust_offset: offset_of!(TypeInfoFieldData, attributes),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldData {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOFIELDDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOFIELDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AccessType {
    #[default]
    AccessType_Member = 0,
    AccessType_Const = 1,
}

pub static ACCESSTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AccessType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(ACCESSTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AccessType {
    fn type_info(&self) -> &'static TypeInfo {
        ACCESSTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ACCESSTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AccessType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("AccessType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ProtectionLevel {
    #[default]
    ProtectionLevel_Private = 0,
    ProtectionLevel_Protected = 1,
    ProtectionLevel_Public = 2,
}

pub static PROTECTIONLEVEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProtectionLevel",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(PROTECTIONLEVEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ProtectionLevel {
    fn type_info(&self) -> &'static TypeInfo {
        PROTECTIONLEVEL_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PROTECTIONLEVEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProtectionLevel-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ProtectionLevel"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoRef {
    pub asset: Option<Arc<Mutex<dyn TypeInfoAssetTrait>>>,
    pub type_info: glacier_reflect::builtin::TypeRef,
}

pub trait TypeInfoRefTrait: TypeObject {
    fn asset(&self) -> &Option<Arc<Mutex<dyn TypeInfoAssetTrait>>>;
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef;
}

impl TypeInfoRefTrait for TypeInfoRef {
    fn asset(&self) -> &Option<Arc<Mutex<dyn TypeInfoAssetTrait>>> {
        &self.asset
    }
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.type_info
    }
}

pub static TYPEINFOREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoRef",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoRef as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeInfoAsset",
                rust_offset: offset_of!(TypeInfoRef, asset),
            },
            FieldInfoData {
                name: "TypeInfo",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(TypeInfoRef, type_info),
            },
        ],
    }),
    array_type: Some(TYPEINFOREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoRef {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOREF_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoRef"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoAsset {
    pub _glacier_base: Asset,
    pub module_name: String,
    pub type_name: String,
    pub is_meta: bool,
    pub attributes: Vec<Option<Arc<Mutex<dyn TypeInfoAttributeTrait>>>>,
    pub is_native: bool,
}

pub trait TypeInfoAssetTrait: AssetTrait {
    fn module_name(&self) -> &String;
    fn type_name(&self) -> &String;
    fn is_meta(&self) -> &bool;
    fn attributes(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoAttributeTrait>>>>;
    fn is_native(&self) -> &bool;
}

impl TypeInfoAssetTrait for TypeInfoAsset {
    fn module_name(&self) -> &String {
        &self.module_name
    }
    fn type_name(&self) -> &String {
        &self.type_name
    }
    fn is_meta(&self) -> &bool {
        &self.is_meta
    }
    fn attributes(&self) -> &Vec<Option<Arc<Mutex<dyn TypeInfoAttributeTrait>>>> {
        &self.attributes
    }
    fn is_native(&self) -> &bool {
        &self.is_native
    }
}

impl AssetTrait for TypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl DataContainerTrait for TypeInfoAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ModuleName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TypeInfoAsset, module_name),
            },
            FieldInfoData {
                name: "TypeName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TypeInfoAsset, type_name),
            },
            FieldInfoData {
                name: "IsMeta",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TypeInfoAsset, is_meta),
            },
            FieldInfoData {
                name: "Attributes",
                flags: MemberInfoFlags::new(144),
                field_type: "TypeInfoAttribute-Array",
                rust_offset: offset_of!(TypeInfoAsset, attributes),
            },
            FieldInfoData {
                name: "IsNative",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TypeInfoAsset, is_native),
            },
        ],
    }),
    array_type: Some(TYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TypeInfoAsset {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoAttribute {
    pub _glacier_base: DataContainer,
    pub name: String,
    pub arguments: Vec<TypeInfoAttributeArgument>,
    pub is_native: bool,
}

pub trait TypeInfoAttributeTrait: DataContainerTrait {
    fn name(&self) -> &String;
    fn arguments(&self) -> &Vec<TypeInfoAttributeArgument>;
    fn is_native(&self) -> &bool;
}

impl TypeInfoAttributeTrait for TypeInfoAttribute {
    fn name(&self) -> &String {
        &self.name
    }
    fn arguments(&self) -> &Vec<TypeInfoAttributeArgument> {
        &self.arguments
    }
    fn is_native(&self) -> &bool {
        &self.is_native
    }
}

impl DataContainerTrait for TypeInfoAttribute {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TYPEINFOATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoAttribute",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoAttribute as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TypeInfoAttribute, name),
            },
            FieldInfoData {
                name: "Arguments",
                flags: MemberInfoFlags::new(144),
                field_type: "TypeInfoAttributeArgument-Array",
                rust_offset: offset_of!(TypeInfoAttribute, arguments),
            },
            FieldInfoData {
                name: "IsNative",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TypeInfoAttribute, is_native),
            },
        ],
    }),
    array_type: Some(TYPEINFOATTRIBUTE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoAttribute {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOATTRIBUTE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoAttribute-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoAttribute"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TypeInfoAttributeArgument {
    pub name: String,
    pub value: String,
}

pub trait TypeInfoAttributeArgumentTrait: TypeObject {
    fn name(&self) -> &String;
    fn value(&self) -> &String;
}

impl TypeInfoAttributeArgumentTrait for TypeInfoAttributeArgument {
    fn name(&self) -> &String {
        &self.name
    }
    fn value(&self) -> &String {
        &self.value
    }
}

pub static TYPEINFOATTRIBUTEARGUMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoAttributeArgument",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoAttributeArgument as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TypeInfoAttributeArgument, name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TypeInfoAttributeArgument, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOATTRIBUTEARGUMENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoAttributeArgument {
    fn type_info(&self) -> &'static TypeInfo {
        TYPEINFOATTRIBUTEARGUMENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TYPEINFOATTRIBUTEARGUMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoAttributeArgument-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoAttributeArgument"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FunctionDelegateRef {
    pub type_info: glacier_reflect::builtin::TypeRef,
}

pub trait FunctionDelegateRefTrait: TypeObject {
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef;
}

impl FunctionDelegateRefTrait for FunctionDelegateRef {
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.type_info
    }
}

pub static FUNCTIONDELEGATEREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FunctionDelegateRef",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FunctionDelegateRef as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TypeInfo",
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(FunctionDelegateRef, type_info),
            },
        ],
    }),
    array_type: Some(FUNCTIONDELEGATEREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FunctionDelegateRef {
    fn type_info(&self) -> &'static TypeInfo {
        FUNCTIONDELEGATEREF_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FUNCTIONDELEGATEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FunctionDelegateRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FunctionDelegateRef"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EventDelay {
}

pub trait EventDelayTrait: TypeObject {
}

impl EventDelayTrait for EventDelay {
}

pub static EVENTDELAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventDelay",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EventDelay as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EVENTDELAY_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EventDelay {
    fn type_info(&self) -> &'static TypeInfo {
        EVENTDELAY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EVENTDELAY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventDelay-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("EventDelay"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TimingViewSettings {
    pub _glacier_base: DataContainer,
    pub enable: bool,
    pub frame_count: u32,
    pub frame_delay_count: u32,
    pub frame_count_to_report: u32,
    pub time_range: f32,
    pub time_offset: f32,
    pub log_threshold: f32,
    pub autolock_threshold: f32,
    pub autolock_name_filter: String,
    pub legend_screen_offset: i32,
    pub legend_column_width: i32,
    pub legend_display_mode: i32,
    pub average_frame_count: i32,
    pub draw_enable: bool,
    pub draw_bars_enable: bool,
    pub draw_legend_enable: bool,
    pub draw_cpu_legend_enable: bool,
    pub draw_gpu_legend_enable: bool,
    pub draw_spu_legend_enable: bool,
    pub snoop_enable: bool,
    pub snoop_only: bool,
    pub snoop_event_name: String,
    pub sort_by_time: bool,
    pub filter_enable: bool,
    pub filter_event_name: String,
    pub bar_min_time: f64,
    pub bar_height: u32,
    pub bar_pad: u32,
    pub bar_sync_processor: i32,
    pub max_cpu_legend_column_count: i32,
    pub max_gpu_legend_column_count: i32,
    pub max_spu_legend_column_count: i32,
    pub max_frame_event_count: u32,
    pub collection_enable: bool,
}

pub trait TimingViewSettingsTrait: DataContainerTrait {
    fn enable(&self) -> &bool;
    fn frame_count(&self) -> &u32;
    fn frame_delay_count(&self) -> &u32;
    fn frame_count_to_report(&self) -> &u32;
    fn time_range(&self) -> &f32;
    fn time_offset(&self) -> &f32;
    fn log_threshold(&self) -> &f32;
    fn autolock_threshold(&self) -> &f32;
    fn autolock_name_filter(&self) -> &String;
    fn legend_screen_offset(&self) -> &i32;
    fn legend_column_width(&self) -> &i32;
    fn legend_display_mode(&self) -> &i32;
    fn average_frame_count(&self) -> &i32;
    fn draw_enable(&self) -> &bool;
    fn draw_bars_enable(&self) -> &bool;
    fn draw_legend_enable(&self) -> &bool;
    fn draw_cpu_legend_enable(&self) -> &bool;
    fn draw_gpu_legend_enable(&self) -> &bool;
    fn draw_spu_legend_enable(&self) -> &bool;
    fn snoop_enable(&self) -> &bool;
    fn snoop_only(&self) -> &bool;
    fn snoop_event_name(&self) -> &String;
    fn sort_by_time(&self) -> &bool;
    fn filter_enable(&self) -> &bool;
    fn filter_event_name(&self) -> &String;
    fn bar_min_time(&self) -> &f64;
    fn bar_height(&self) -> &u32;
    fn bar_pad(&self) -> &u32;
    fn bar_sync_processor(&self) -> &i32;
    fn max_cpu_legend_column_count(&self) -> &i32;
    fn max_gpu_legend_column_count(&self) -> &i32;
    fn max_spu_legend_column_count(&self) -> &i32;
    fn max_frame_event_count(&self) -> &u32;
    fn collection_enable(&self) -> &bool;
}

impl TimingViewSettingsTrait for TimingViewSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn frame_count(&self) -> &u32 {
        &self.frame_count
    }
    fn frame_delay_count(&self) -> &u32 {
        &self.frame_delay_count
    }
    fn frame_count_to_report(&self) -> &u32 {
        &self.frame_count_to_report
    }
    fn time_range(&self) -> &f32 {
        &self.time_range
    }
    fn time_offset(&self) -> &f32 {
        &self.time_offset
    }
    fn log_threshold(&self) -> &f32 {
        &self.log_threshold
    }
    fn autolock_threshold(&self) -> &f32 {
        &self.autolock_threshold
    }
    fn autolock_name_filter(&self) -> &String {
        &self.autolock_name_filter
    }
    fn legend_screen_offset(&self) -> &i32 {
        &self.legend_screen_offset
    }
    fn legend_column_width(&self) -> &i32 {
        &self.legend_column_width
    }
    fn legend_display_mode(&self) -> &i32 {
        &self.legend_display_mode
    }
    fn average_frame_count(&self) -> &i32 {
        &self.average_frame_count
    }
    fn draw_enable(&self) -> &bool {
        &self.draw_enable
    }
    fn draw_bars_enable(&self) -> &bool {
        &self.draw_bars_enable
    }
    fn draw_legend_enable(&self) -> &bool {
        &self.draw_legend_enable
    }
    fn draw_cpu_legend_enable(&self) -> &bool {
        &self.draw_cpu_legend_enable
    }
    fn draw_gpu_legend_enable(&self) -> &bool {
        &self.draw_gpu_legend_enable
    }
    fn draw_spu_legend_enable(&self) -> &bool {
        &self.draw_spu_legend_enable
    }
    fn snoop_enable(&self) -> &bool {
        &self.snoop_enable
    }
    fn snoop_only(&self) -> &bool {
        &self.snoop_only
    }
    fn snoop_event_name(&self) -> &String {
        &self.snoop_event_name
    }
    fn sort_by_time(&self) -> &bool {
        &self.sort_by_time
    }
    fn filter_enable(&self) -> &bool {
        &self.filter_enable
    }
    fn filter_event_name(&self) -> &String {
        &self.filter_event_name
    }
    fn bar_min_time(&self) -> &f64 {
        &self.bar_min_time
    }
    fn bar_height(&self) -> &u32 {
        &self.bar_height
    }
    fn bar_pad(&self) -> &u32 {
        &self.bar_pad
    }
    fn bar_sync_processor(&self) -> &i32 {
        &self.bar_sync_processor
    }
    fn max_cpu_legend_column_count(&self) -> &i32 {
        &self.max_cpu_legend_column_count
    }
    fn max_gpu_legend_column_count(&self) -> &i32 {
        &self.max_gpu_legend_column_count
    }
    fn max_spu_legend_column_count(&self) -> &i32 {
        &self.max_spu_legend_column_count
    }
    fn max_frame_event_count(&self) -> &u32 {
        &self.max_frame_event_count
    }
    fn collection_enable(&self) -> &bool {
        &self.collection_enable
    }
}

impl DataContainerTrait for TimingViewSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TIMINGVIEWSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimingViewSettings",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TimingViewSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, enable),
            },
            FieldInfoData {
                name: "FrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TimingViewSettings, frame_count),
            },
            FieldInfoData {
                name: "FrameDelayCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TimingViewSettings, frame_delay_count),
            },
            FieldInfoData {
                name: "FrameCountToReport",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TimingViewSettings, frame_count_to_report),
            },
            FieldInfoData {
                name: "TimeRange",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TimingViewSettings, time_range),
            },
            FieldInfoData {
                name: "TimeOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TimingViewSettings, time_offset),
            },
            FieldInfoData {
                name: "LogThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TimingViewSettings, log_threshold),
            },
            FieldInfoData {
                name: "AutolockThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TimingViewSettings, autolock_threshold),
            },
            FieldInfoData {
                name: "AutolockNameFilter",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TimingViewSettings, autolock_name_filter),
            },
            FieldInfoData {
                name: "LegendScreenOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TimingViewSettings, legend_screen_offset),
            },
            FieldInfoData {
                name: "LegendColumnWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TimingViewSettings, legend_column_width),
            },
            FieldInfoData {
                name: "LegendDisplayMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TimingViewSettings, legend_display_mode),
            },
            FieldInfoData {
                name: "AverageFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TimingViewSettings, average_frame_count),
            },
            FieldInfoData {
                name: "DrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, draw_enable),
            },
            FieldInfoData {
                name: "DrawBarsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, draw_bars_enable),
            },
            FieldInfoData {
                name: "DrawLegendEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, draw_legend_enable),
            },
            FieldInfoData {
                name: "DrawCpuLegendEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, draw_cpu_legend_enable),
            },
            FieldInfoData {
                name: "DrawGpuLegendEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, draw_gpu_legend_enable),
            },
            FieldInfoData {
                name: "DrawSpuLegendEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, draw_spu_legend_enable),
            },
            FieldInfoData {
                name: "SnoopEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, snoop_enable),
            },
            FieldInfoData {
                name: "SnoopOnly",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, snoop_only),
            },
            FieldInfoData {
                name: "SnoopEventName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TimingViewSettings, snoop_event_name),
            },
            FieldInfoData {
                name: "SortByTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, sort_by_time),
            },
            FieldInfoData {
                name: "FilterEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, filter_enable),
            },
            FieldInfoData {
                name: "FilterEventName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TimingViewSettings, filter_event_name),
            },
            FieldInfoData {
                name: "BarMinTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float64",
                rust_offset: offset_of!(TimingViewSettings, bar_min_time),
            },
            FieldInfoData {
                name: "BarHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TimingViewSettings, bar_height),
            },
            FieldInfoData {
                name: "BarPad",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TimingViewSettings, bar_pad),
            },
            FieldInfoData {
                name: "BarSyncProcessor",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TimingViewSettings, bar_sync_processor),
            },
            FieldInfoData {
                name: "MaxCpuLegendColumnCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TimingViewSettings, max_cpu_legend_column_count),
            },
            FieldInfoData {
                name: "MaxGpuLegendColumnCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TimingViewSettings, max_gpu_legend_column_count),
            },
            FieldInfoData {
                name: "MaxSpuLegendColumnCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TimingViewSettings, max_spu_legend_column_count),
            },
            FieldInfoData {
                name: "MaxFrameEventCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TimingViewSettings, max_frame_event_count),
            },
            FieldInfoData {
                name: "CollectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, collection_enable),
            },
        ],
    }),
    array_type: Some(TIMINGVIEWSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimingViewSettings {
    fn type_info(&self) -> &'static TypeInfo {
        TIMINGVIEWSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TIMINGVIEWSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimingViewSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TimingViewSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PerfJournalSettings {
    pub _glacier_base: DataContainer,
    pub enable: bool,
    pub journal_expensive_stats: bool,
    pub journal_all_s_p_u: bool,
    pub journal_sample_interval: i32,
    pub journal_report_average: bool,
    pub journal_frame_count: i32,
    pub auto_journal_enable: bool,
    pub auto_journal_screenshot: bool,
    pub auto_journal_threshold_ms: f32,
    pub auto_journal_min_frames: i32,
    pub trace_enable: bool,
    pub auto_journal_continuous_capture: bool,
    pub floats_always_have_decimal: bool,
}

pub trait PerfJournalSettingsTrait: DataContainerTrait {
    fn enable(&self) -> &bool;
    fn journal_expensive_stats(&self) -> &bool;
    fn journal_all_s_p_u(&self) -> &bool;
    fn journal_sample_interval(&self) -> &i32;
    fn journal_report_average(&self) -> &bool;
    fn journal_frame_count(&self) -> &i32;
    fn auto_journal_enable(&self) -> &bool;
    fn auto_journal_screenshot(&self) -> &bool;
    fn auto_journal_threshold_ms(&self) -> &f32;
    fn auto_journal_min_frames(&self) -> &i32;
    fn trace_enable(&self) -> &bool;
    fn auto_journal_continuous_capture(&self) -> &bool;
    fn floats_always_have_decimal(&self) -> &bool;
}

impl PerfJournalSettingsTrait for PerfJournalSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn journal_expensive_stats(&self) -> &bool {
        &self.journal_expensive_stats
    }
    fn journal_all_s_p_u(&self) -> &bool {
        &self.journal_all_s_p_u
    }
    fn journal_sample_interval(&self) -> &i32 {
        &self.journal_sample_interval
    }
    fn journal_report_average(&self) -> &bool {
        &self.journal_report_average
    }
    fn journal_frame_count(&self) -> &i32 {
        &self.journal_frame_count
    }
    fn auto_journal_enable(&self) -> &bool {
        &self.auto_journal_enable
    }
    fn auto_journal_screenshot(&self) -> &bool {
        &self.auto_journal_screenshot
    }
    fn auto_journal_threshold_ms(&self) -> &f32 {
        &self.auto_journal_threshold_ms
    }
    fn auto_journal_min_frames(&self) -> &i32 {
        &self.auto_journal_min_frames
    }
    fn trace_enable(&self) -> &bool {
        &self.trace_enable
    }
    fn auto_journal_continuous_capture(&self) -> &bool {
        &self.auto_journal_continuous_capture
    }
    fn floats_always_have_decimal(&self) -> &bool {
        &self.floats_always_have_decimal
    }
}

impl DataContainerTrait for PerfJournalSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PERFJOURNALSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfJournalSettings",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PerfJournalSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalSettings, enable),
            },
            FieldInfoData {
                name: "JournalExpensiveStats",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalSettings, journal_expensive_stats),
            },
            FieldInfoData {
                name: "JournalAllSPU",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalSettings, journal_all_s_p_u),
            },
            FieldInfoData {
                name: "JournalSampleInterval",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PerfJournalSettings, journal_sample_interval),
            },
            FieldInfoData {
                name: "JournalReportAverage",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalSettings, journal_report_average),
            },
            FieldInfoData {
                name: "JournalFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PerfJournalSettings, journal_frame_count),
            },
            FieldInfoData {
                name: "AutoJournalEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalSettings, auto_journal_enable),
            },
            FieldInfoData {
                name: "AutoJournalScreenshot",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalSettings, auto_journal_screenshot),
            },
            FieldInfoData {
                name: "AutoJournalThresholdMs",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PerfJournalSettings, auto_journal_threshold_ms),
            },
            FieldInfoData {
                name: "AutoJournalMinFrames",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PerfJournalSettings, auto_journal_min_frames),
            },
            FieldInfoData {
                name: "TraceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalSettings, trace_enable),
            },
            FieldInfoData {
                name: "AutoJournalContinuousCapture",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalSettings, auto_journal_continuous_capture),
            },
            FieldInfoData {
                name: "FloatsAlwaysHaveDecimal",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalSettings, floats_always_have_decimal),
            },
        ],
    }),
    array_type: Some(PERFJOURNALSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PerfJournalSettings {
    fn type_info(&self) -> &'static TypeInfo {
        PERFJOURNALSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PERFJOURNALSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfJournalSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PerfJournalSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PerfHudSettings {
    pub _glacier_base: DataContainer,
    pub enable: bool,
    pub draw_cpu_enable: bool,
    pub draw_gpu_enable: bool,
    pub draw_spu_enable: bool,
    pub simple_summary_mode: bool,
    pub timer_category_set_name: String,
    pub timer_category_sets_enable: bool,
    pub draw_ungrouped_timings: bool,
    pub draw_config_file: bool,
    pub hud_right_margin: i32,
    pub hud_top_margin: i32,
    pub hud_alpha: u8,
    pub hud_compact: bool,
}

pub trait PerfHudSettingsTrait: DataContainerTrait {
    fn enable(&self) -> &bool;
    fn draw_cpu_enable(&self) -> &bool;
    fn draw_gpu_enable(&self) -> &bool;
    fn draw_spu_enable(&self) -> &bool;
    fn simple_summary_mode(&self) -> &bool;
    fn timer_category_set_name(&self) -> &String;
    fn timer_category_sets_enable(&self) -> &bool;
    fn draw_ungrouped_timings(&self) -> &bool;
    fn draw_config_file(&self) -> &bool;
    fn hud_right_margin(&self) -> &i32;
    fn hud_top_margin(&self) -> &i32;
    fn hud_alpha(&self) -> &u8;
    fn hud_compact(&self) -> &bool;
}

impl PerfHudSettingsTrait for PerfHudSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn draw_cpu_enable(&self) -> &bool {
        &self.draw_cpu_enable
    }
    fn draw_gpu_enable(&self) -> &bool {
        &self.draw_gpu_enable
    }
    fn draw_spu_enable(&self) -> &bool {
        &self.draw_spu_enable
    }
    fn simple_summary_mode(&self) -> &bool {
        &self.simple_summary_mode
    }
    fn timer_category_set_name(&self) -> &String {
        &self.timer_category_set_name
    }
    fn timer_category_sets_enable(&self) -> &bool {
        &self.timer_category_sets_enable
    }
    fn draw_ungrouped_timings(&self) -> &bool {
        &self.draw_ungrouped_timings
    }
    fn draw_config_file(&self) -> &bool {
        &self.draw_config_file
    }
    fn hud_right_margin(&self) -> &i32 {
        &self.hud_right_margin
    }
    fn hud_top_margin(&self) -> &i32 {
        &self.hud_top_margin
    }
    fn hud_alpha(&self) -> &u8 {
        &self.hud_alpha
    }
    fn hud_compact(&self) -> &bool {
        &self.hud_compact
    }
}

impl DataContainerTrait for PerfHudSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PERFHUDSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfHudSettings",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PerfHudSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfHudSettings, enable),
            },
            FieldInfoData {
                name: "DrawCpuEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfHudSettings, draw_cpu_enable),
            },
            FieldInfoData {
                name: "DrawGpuEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfHudSettings, draw_gpu_enable),
            },
            FieldInfoData {
                name: "DrawSpuEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfHudSettings, draw_spu_enable),
            },
            FieldInfoData {
                name: "SimpleSummaryMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfHudSettings, simple_summary_mode),
            },
            FieldInfoData {
                name: "TimerCategorySetName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(PerfHudSettings, timer_category_set_name),
            },
            FieldInfoData {
                name: "TimerCategorySetsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfHudSettings, timer_category_sets_enable),
            },
            FieldInfoData {
                name: "DrawUngroupedTimings",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfHudSettings, draw_ungrouped_timings),
            },
            FieldInfoData {
                name: "DrawConfigFile",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfHudSettings, draw_config_file),
            },
            FieldInfoData {
                name: "HudRightMargin",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PerfHudSettings, hud_right_margin),
            },
            FieldInfoData {
                name: "HudTopMargin",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PerfHudSettings, hud_top_margin),
            },
            FieldInfoData {
                name: "HudAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(PerfHudSettings, hud_alpha),
            },
            FieldInfoData {
                name: "HudCompact",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfHudSettings, hud_compact),
            },
        ],
    }),
    array_type: Some(PERFHUDSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PerfHudSettings {
    fn type_info(&self) -> &'static TypeInfo {
        PERFHUDSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PERFHUDSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfHudSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PerfHudSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CoreLogUpdateSettingsMessage {
}

pub trait CoreLogUpdateSettingsMessageTrait: TypeObject {
}

impl CoreLogUpdateSettingsMessageTrait for CoreLogUpdateSettingsMessage {
}

pub static CORELOGUPDATESETTINGSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreLogUpdateSettingsMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreLogUpdateSettingsMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreLogUpdateSettingsMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CORELOGUPDATESETTINGSMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct SplitScreenSetPrimaryLocalPlayerIdMessage {
}

pub trait SplitScreenSetPrimaryLocalPlayerIdMessageTrait: TypeObject {
}

impl SplitScreenSetPrimaryLocalPlayerIdMessageTrait for SplitScreenSetPrimaryLocalPlayerIdMessage {
}

pub static SPLITSCREENSETPRIMARYLOCALPLAYERIDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SplitScreenSetPrimaryLocalPlayerIdMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SplitScreenSetPrimaryLocalPlayerIdMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SplitScreenSetPrimaryLocalPlayerIdMessage {
    fn type_info(&self) -> &'static TypeInfo {
        SPLITSCREENSETPRIMARYLOCALPLAYERIDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct StreamInstallRequestResumeMessage {
}

pub trait StreamInstallRequestResumeMessageTrait: TypeObject {
}

impl StreamInstallRequestResumeMessageTrait for StreamInstallRequestResumeMessage {
}

pub static STREAMINSTALLREQUESTRESUMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallRequestResumeMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamInstallRequestResumeMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for StreamInstallRequestResumeMessage {
    fn type_info(&self) -> &'static TypeInfo {
        STREAMINSTALLREQUESTRESUMEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct StreamInstallRequestSuspendMessage {
}

pub trait StreamInstallRequestSuspendMessageTrait: TypeObject {
}

impl StreamInstallRequestSuspendMessageTrait for StreamInstallRequestSuspendMessage {
}

pub static STREAMINSTALLREQUESTSUSPENDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallRequestSuspendMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamInstallRequestSuspendMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for StreamInstallRequestSuspendMessage {
    fn type_info(&self) -> &'static TypeInfo {
        STREAMINSTALLREQUESTSUSPENDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct StreamInstallGroupInstallationProgressMessage {
}

pub trait StreamInstallGroupInstallationProgressMessageTrait: TypeObject {
}

impl StreamInstallGroupInstallationProgressMessageTrait for StreamInstallGroupInstallationProgressMessage {
}

pub static STREAMINSTALLGROUPINSTALLATIONPROGRESSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallGroupInstallationProgressMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamInstallGroupInstallationProgressMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for StreamInstallGroupInstallationProgressMessage {
    fn type_info(&self) -> &'static TypeInfo {
        STREAMINSTALLGROUPINSTALLATIONPROGRESSMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct StreamInstallGroupInstalledMessage {
}

pub trait StreamInstallGroupInstalledMessageTrait: TypeObject {
}

impl StreamInstallGroupInstalledMessageTrait for StreamInstallGroupInstalledMessage {
}

pub static STREAMINSTALLGROUPINSTALLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallGroupInstalledMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamInstallGroupInstalledMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for StreamInstallGroupInstalledMessage {
    fn type_info(&self) -> &'static TypeInfo {
        STREAMINSTALLGROUPINSTALLEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct StreamInstallInstallationProgressMessage {
}

pub trait StreamInstallInstallationProgressMessageTrait: TypeObject {
}

impl StreamInstallInstallationProgressMessageTrait for StreamInstallInstallationProgressMessage {
}

pub static STREAMINSTALLINSTALLATIONPROGRESSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallInstallationProgressMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamInstallInstallationProgressMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for StreamInstallInstallationProgressMessage {
    fn type_info(&self) -> &'static TypeInfo {
        STREAMINSTALLINSTALLATIONPROGRESSMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct StreamInstallInstallationFailedMessage {
}

pub trait StreamInstallInstallationFailedMessageTrait: TypeObject {
}

impl StreamInstallInstallationFailedMessageTrait for StreamInstallInstallationFailedMessage {
}

pub static STREAMINSTALLINSTALLATIONFAILEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallInstallationFailedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamInstallInstallationFailedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for StreamInstallInstallationFailedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        STREAMINSTALLINSTALLATIONFAILEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct StreamInstallInstallationCompletedMessage {
}

pub trait StreamInstallInstallationCompletedMessageTrait: TypeObject {
}

impl StreamInstallInstallationCompletedMessageTrait for StreamInstallInstallationCompletedMessage {
}

pub static STREAMINSTALLINSTALLATIONCOMPLETEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallInstallationCompletedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamInstallInstallationCompletedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for StreamInstallInstallationCompletedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        STREAMINSTALLINSTALLATIONCOMPLETEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct CoreApplicationActivationMessage {
}

pub trait CoreApplicationActivationMessageTrait: TypeObject {
}

impl CoreApplicationActivationMessageTrait for CoreApplicationActivationMessage {
}

pub static COREAPPLICATIONACTIVATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreApplicationActivationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreApplicationActivationMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreApplicationActivationMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COREAPPLICATIONACTIVATIONMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ApplicationActivationState {
    #[default]
    ApplicationActivationState_CodeActivated = 0,
    ApplicationActivationState_Deactivated = 1,
    ApplicationActivationState_PointerActivated = 2,
    ApplicationActivationState_Invalid = 255,
}

pub static APPLICATIONACTIVATIONSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ApplicationActivationState",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(APPLICATIONACTIVATIONSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ApplicationActivationState {
    fn type_info(&self) -> &'static TypeInfo {
        APPLICATIONACTIVATIONSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static APPLICATIONACTIVATIONSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ApplicationActivationState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ApplicationActivationState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CoreCleanupMessage {
}

pub trait CoreCleanupMessageTrait: TypeObject {
}

impl CoreCleanupMessageTrait for CoreCleanupMessage {
}

pub static CORECLEANUPMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreCleanupMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreCleanupMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreCleanupMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CORECLEANUPMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct CoreSublevelStartStreamOutMessage {
}

pub trait CoreSublevelStartStreamOutMessageTrait: TypeObject {
}

impl CoreSublevelStartStreamOutMessageTrait for CoreSublevelStartStreamOutMessage {
}

pub static CORESUBLEVELSTARTSTREAMOUTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreSublevelStartStreamOutMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreSublevelStartStreamOutMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreSublevelStartStreamOutMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CORESUBLEVELSTARTSTREAMOUTMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct CorePanicMessage {
}

pub trait CorePanicMessageTrait: TypeObject {
}

impl CorePanicMessageTrait for CorePanicMessage {
}

pub static COREPANICMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CorePanicMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CorePanicMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CorePanicMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COREPANICMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct CoreQuitMessage {
}

pub trait CoreQuitMessageTrait: TypeObject {
}

impl CoreQuitMessageTrait for CoreQuitMessage {
}

pub static COREQUITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreQuitMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreQuitMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreQuitMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COREQUITMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct CoreUpdateClipboardMessage {
}

pub trait CoreUpdateClipboardMessageTrait: TypeObject {
}

impl CoreUpdateClipboardMessageTrait for CoreUpdateClipboardMessage {
}

pub static COREUPDATECLIPBOARDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreUpdateClipboardMessage",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreUpdateClipboardMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreUpdateClipboardMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COREUPDATECLIPBOARDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct CoreMainThreadInitMessage {
}

pub trait CoreMainThreadInitMessageTrait: TypeObject {
}

impl CoreMainThreadInitMessageTrait for CoreMainThreadInitMessage {
}

pub static COREMAINTHREADINITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreMainThreadInitMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreMainThreadInitMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreMainThreadInitMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COREMAINTHREADINITMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct CoreHibernateMessage {
}

pub trait CoreHibernateMessageTrait: TypeObject {
}

impl CoreHibernateMessageTrait for CoreHibernateMessage {
}

pub static COREHIBERNATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreHibernateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreHibernateMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreHibernateMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COREHIBERNATEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct CoreQuittingInitiatedMessage {
}

pub trait CoreQuittingInitiatedMessageTrait: TypeObject {
}

impl CoreQuittingInitiatedMessageTrait for CoreQuittingInitiatedMessage {
}

pub static COREQUITTINGINITIATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreQuittingInitiatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreQuittingInitiatedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreQuittingInitiatedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        COREQUITTINGINITIATEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct SparseTransformArray {
    pub indices: Vec<u16>,
    pub transforms: Vec<LinearTransform>,
    pub count: u32,
}

pub trait SparseTransformArrayTrait: TypeObject {
    fn indices(&self) -> &Vec<u16>;
    fn transforms(&self) -> &Vec<LinearTransform>;
    fn count(&self) -> &u32;
}

impl SparseTransformArrayTrait for SparseTransformArray {
    fn indices(&self) -> &Vec<u16> {
        &self.indices
    }
    fn transforms(&self) -> &Vec<LinearTransform> {
        &self.transforms
    }
    fn count(&self) -> &u32 {
        &self.count
    }
}

pub static SPARSETRANSFORMARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SparseTransformArray",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SparseTransformArray as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Indices",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint16-Array",
                rust_offset: offset_of!(SparseTransformArray, indices),
            },
            FieldInfoData {
                name: "Transforms",
                flags: MemberInfoFlags::new(144),
                field_type: "LinearTransform-Array",
                rust_offset: offset_of!(SparseTransformArray, transforms),
            },
            FieldInfoData {
                name: "Count",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SparseTransformArray, count),
            },
        ],
    }),
    array_type: Some(SPARSETRANSFORMARRAY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SparseTransformArray {
    fn type_info(&self) -> &'static TypeInfo {
        SPARSETRANSFORMARRAY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SPARSETRANSFORMARRAY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SparseTransformArray-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SparseTransformArray"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SplineCurve {
    pub x_values0: Vec4,
    pub x_values1: Vec4,
    pub x_values2: Vec4,
    pub y_values0: Vec4,
    pub y_values1: Vec4,
    pub y_values2: Vec4,
    pub y_values3: Vec4,
    pub g_values0: Vec4,
    pub g_values1: Vec4,
    pub g_values2: Vec4,
    pub g_values3: Vec4,
    pub g_values4: Vec4,
    pub g_values5: Vec4,
    pub spline_type: SplineType,
}

pub trait SplineCurveTrait: TypeObject {
    fn x_values0(&self) -> &Vec4;
    fn x_values1(&self) -> &Vec4;
    fn x_values2(&self) -> &Vec4;
    fn y_values0(&self) -> &Vec4;
    fn y_values1(&self) -> &Vec4;
    fn y_values2(&self) -> &Vec4;
    fn y_values3(&self) -> &Vec4;
    fn g_values0(&self) -> &Vec4;
    fn g_values1(&self) -> &Vec4;
    fn g_values2(&self) -> &Vec4;
    fn g_values3(&self) -> &Vec4;
    fn g_values4(&self) -> &Vec4;
    fn g_values5(&self) -> &Vec4;
    fn spline_type(&self) -> &SplineType;
}

impl SplineCurveTrait for SplineCurve {
    fn x_values0(&self) -> &Vec4 {
        &self.x_values0
    }
    fn x_values1(&self) -> &Vec4 {
        &self.x_values1
    }
    fn x_values2(&self) -> &Vec4 {
        &self.x_values2
    }
    fn y_values0(&self) -> &Vec4 {
        &self.y_values0
    }
    fn y_values1(&self) -> &Vec4 {
        &self.y_values1
    }
    fn y_values2(&self) -> &Vec4 {
        &self.y_values2
    }
    fn y_values3(&self) -> &Vec4 {
        &self.y_values3
    }
    fn g_values0(&self) -> &Vec4 {
        &self.g_values0
    }
    fn g_values1(&self) -> &Vec4 {
        &self.g_values1
    }
    fn g_values2(&self) -> &Vec4 {
        &self.g_values2
    }
    fn g_values3(&self) -> &Vec4 {
        &self.g_values3
    }
    fn g_values4(&self) -> &Vec4 {
        &self.g_values4
    }
    fn g_values5(&self) -> &Vec4 {
        &self.g_values5
    }
    fn spline_type(&self) -> &SplineType {
        &self.spline_type
    }
}

pub static SPLINECURVE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SplineCurve",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SplineCurve as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "XValues0",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, x_values0),
            },
            FieldInfoData {
                name: "XValues1",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, x_values1),
            },
            FieldInfoData {
                name: "XValues2",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, x_values2),
            },
            FieldInfoData {
                name: "YValues0",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, y_values0),
            },
            FieldInfoData {
                name: "YValues1",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, y_values1),
            },
            FieldInfoData {
                name: "YValues2",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, y_values2),
            },
            FieldInfoData {
                name: "YValues3",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, y_values3),
            },
            FieldInfoData {
                name: "GValues0",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, g_values0),
            },
            FieldInfoData {
                name: "GValues1",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, g_values1),
            },
            FieldInfoData {
                name: "GValues2",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, g_values2),
            },
            FieldInfoData {
                name: "GValues3",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, g_values3),
            },
            FieldInfoData {
                name: "GValues4",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, g_values4),
            },
            FieldInfoData {
                name: "GValues5",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, g_values5),
            },
            FieldInfoData {
                name: "SplineType",
                flags: MemberInfoFlags::new(0),
                field_type: "SplineType",
                rust_offset: offset_of!(SplineCurve, spline_type),
            },
        ],
    }),
    array_type: Some(SPLINECURVE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SplineCurve {
    fn type_info(&self) -> &'static TypeInfo {
        SPLINECURVE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SPLINECURVE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SplineCurve-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SplineCurve"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum SplineType {
    #[default]
    SplineType_5ControlPoints = 5,
    SplineType_9ControlPoints = 9,
    SplineType_13ControlPoints = 13,
}

pub static SPLINETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SplineType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(SPLINETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SplineType {
    fn type_info(&self) -> &'static TypeInfo {
        SPLINETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SPLINETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SplineType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SplineType"),
    array_type: None,
    alignment: 8,
};



pub static EVALUATEFLOATCURVE_FLOAT32_FLOAT32_FLOATCURVE__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EvaluateFloatCurve(Float32,Float32,FloatCurve)",
    flags: MemberInfoFlags::new(793),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub static EVALUATEAUDIOCURVE_FLOAT32_FLOAT32_AUDIOCURVE__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EvaluateAudioCurve(Float32,Float32,AudioCurve)",
    flags: MemberInfoFlags::new(793),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FloatCurveCollectionAsset {
    pub _glacier_base: Asset,
    pub curves: Vec<Option<Arc<Mutex<dyn FloatCurveTrait>>>>,
}

pub trait FloatCurveCollectionAssetTrait: AssetTrait {
    fn curves(&self) -> &Vec<Option<Arc<Mutex<dyn FloatCurveTrait>>>>;
}

impl FloatCurveCollectionAssetTrait for FloatCurveCollectionAsset {
    fn curves(&self) -> &Vec<Option<Arc<Mutex<dyn FloatCurveTrait>>>> {
        &self.curves
    }
}

impl AssetTrait for FloatCurveCollectionAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl DataContainerTrait for FloatCurveCollectionAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FLOATCURVECOLLECTIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurveCollectionAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatCurveCollectionAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Curves",
                flags: MemberInfoFlags::new(144),
                field_type: "FloatCurve-Array",
                rust_offset: offset_of!(FloatCurveCollectionAsset, curves),
            },
        ],
    }),
    array_type: Some(FLOATCURVECOLLECTIONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatCurveCollectionAsset {
    fn type_info(&self) -> &'static TypeInfo {
        FLOATCURVECOLLECTIONASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FLOATCURVECOLLECTIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurveCollectionAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FloatCurveCollectionAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FloatCurve {
    pub _glacier_base: DataContainer,
    pub points: Vec<FloatCurvePoint>,
    pub min_x: f32,
    pub max_x: f32,
}

pub trait FloatCurveTrait: DataContainerTrait {
    fn points(&self) -> &Vec<FloatCurvePoint>;
    fn min_x(&self) -> &f32;
    fn max_x(&self) -> &f32;
}

impl FloatCurveTrait for FloatCurve {
    fn points(&self) -> &Vec<FloatCurvePoint> {
        &self.points
    }
    fn min_x(&self) -> &f32 {
        &self.min_x
    }
    fn max_x(&self) -> &f32 {
        &self.max_x
    }
}

impl DataContainerTrait for FloatCurve {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FLOATCURVE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurve",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatCurve as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Points",
                flags: MemberInfoFlags::new(144),
                field_type: "FloatCurvePoint-Array",
                rust_offset: offset_of!(FloatCurve, points),
            },
            FieldInfoData {
                name: "MinX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatCurve, min_x),
            },
            FieldInfoData {
                name: "MaxX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatCurve, max_x),
            },
        ],
    }),
    array_type: Some(FLOATCURVE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatCurve {
    fn type_info(&self) -> &'static TypeInfo {
        FLOATCURVE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FLOATCURVE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurve-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FloatCurve"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum FloatCurveType {
    #[default]
    FloatCurveType_Spline = 0,
    FloatCurveType_Smooth = 1,
    FloatCurveType_Linear = 2,
    FloatCurveType_NearestValue = 3,
}

pub static FLOATCURVETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurveType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(FLOATCURVETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FloatCurveType {
    fn type_info(&self) -> &'static TypeInfo {
        FLOATCURVETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FLOATCURVETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurveType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FloatCurveType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FloatCurvePoint {
    pub x: f32,
    pub y: f32,
    pub in_tangent_offset_x: f32,
    pub in_tangent_offset_y: f32,
    pub out_tangent_offset_x: f32,
    pub out_tangent_offset_y: f32,
    pub curve_type: FloatCurveType,
}

pub trait FloatCurvePointTrait: TypeObject {
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
    fn in_tangent_offset_x(&self) -> &f32;
    fn in_tangent_offset_y(&self) -> &f32;
    fn out_tangent_offset_x(&self) -> &f32;
    fn out_tangent_offset_y(&self) -> &f32;
    fn curve_type(&self) -> &FloatCurveType;
}

impl FloatCurvePointTrait for FloatCurvePoint {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn in_tangent_offset_x(&self) -> &f32 {
        &self.in_tangent_offset_x
    }
    fn in_tangent_offset_y(&self) -> &f32 {
        &self.in_tangent_offset_y
    }
    fn out_tangent_offset_x(&self) -> &f32 {
        &self.out_tangent_offset_x
    }
    fn out_tangent_offset_y(&self) -> &f32 {
        &self.out_tangent_offset_y
    }
    fn curve_type(&self) -> &FloatCurveType {
        &self.curve_type
    }
}

pub static FLOATCURVEPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurvePoint",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatCurvePoint as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatCurvePoint, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatCurvePoint, y),
            },
            FieldInfoData {
                name: "InTangentOffsetX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatCurvePoint, in_tangent_offset_x),
            },
            FieldInfoData {
                name: "InTangentOffsetY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatCurvePoint, in_tangent_offset_y),
            },
            FieldInfoData {
                name: "OutTangentOffsetX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatCurvePoint, out_tangent_offset_x),
            },
            FieldInfoData {
                name: "OutTangentOffsetY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatCurvePoint, out_tangent_offset_y),
            },
            FieldInfoData {
                name: "CurveType",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatCurveType",
                rust_offset: offset_of!(FloatCurvePoint, curve_type),
            },
        ],
    }),
    array_type: Some(FLOATCURVEPOINT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for FloatCurvePoint {
    fn type_info(&self) -> &'static TypeInfo {
        FLOATCURVEPOINT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FLOATCURVEPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurvePoint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FloatCurvePoint"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EffectCurvePoint {
    pub x: f32,
    pub y: f32,
    pub k: f32,
}

pub trait EffectCurvePointTrait: TypeObject {
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
    fn k(&self) -> &f32;
}

impl EffectCurvePointTrait for EffectCurvePoint {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn k(&self) -> &f32 {
        &self.k
    }
}

pub static EFFECTCURVEPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectCurvePoint",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectCurvePoint as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EffectCurvePoint, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EffectCurvePoint, y),
            },
            FieldInfoData {
                name: "K",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EffectCurvePoint, k),
            },
        ],
    }),
    array_type: Some(EFFECTCURVEPOINT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EffectCurvePoint {
    fn type_info(&self) -> &'static TypeInfo {
        EFFECTCURVEPOINT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EFFECTCURVEPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectCurvePoint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("EffectCurvePoint"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AudioCurve {
    pub points: Vec<AudioCurvePoint>,
    pub curve_type: AudioCurveType,
}

pub trait AudioCurveTrait: TypeObject {
    fn points(&self) -> &Vec<AudioCurvePoint>;
    fn curve_type(&self) -> &AudioCurveType;
}

impl AudioCurveTrait for AudioCurve {
    fn points(&self) -> &Vec<AudioCurvePoint> {
        &self.points
    }
    fn curve_type(&self) -> &AudioCurveType {
        &self.curve_type
    }
}

pub static AUDIOCURVE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurve",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioCurve as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Points",
                flags: MemberInfoFlags::new(144),
                field_type: "AudioCurvePoint-Array",
                rust_offset: offset_of!(AudioCurve, points),
            },
            FieldInfoData {
                name: "CurveType",
                flags: MemberInfoFlags::new(0),
                field_type: "AudioCurveType",
                rust_offset: offset_of!(AudioCurve, curve_type),
            },
        ],
    }),
    array_type: Some(AUDIOCURVE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioCurve {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOCURVE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AUDIOCURVE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurve-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("AudioCurve"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AudioCurveType {
    #[default]
    AudioCurveType_Spline = 0,
    AudioCurveType_Smooth = 1,
    AudioCurveType_Linear = 2,
    AudioCurveType_NearestValue = 3,
}

pub static AUDIOCURVETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurveType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(AUDIOCURVETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AudioCurveType {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOCURVETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AUDIOCURVETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurveType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("AudioCurveType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AudioCurvePoint {
    pub x: f32,
    pub y: f32,
    pub k: f32,
}

pub trait AudioCurvePointTrait: TypeObject {
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
    fn k(&self) -> &f32;
}

impl AudioCurvePointTrait for AudioCurvePoint {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn k(&self) -> &f32 {
        &self.k
    }
}

pub static AUDIOCURVEPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurvePoint",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioCurvePoint as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioCurvePoint, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioCurvePoint, y),
            },
            FieldInfoData {
                name: "K",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioCurvePoint, k),
            },
        ],
    }),
    array_type: Some(AUDIOCURVEPOINT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AudioCurvePoint {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOCURVEPOINT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AUDIOCURVEPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurvePoint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("AudioCurvePoint"),
    array_type: None,
    alignment: 8,
};



pub static FBASSERT_BOOLEAN__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FbAssert(Boolean)",
    flags: MemberInfoFlags::new(793),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DebugRenderSettings {
    pub _glacier_base: DataContainer,
    pub enable: bool,
    pub draw_stats_enable: bool,
    pub text_view_distance: f32,
    pub text_queue_max_line_count: u32,
    pub text_queue_time_visible: f32,
    pub text_queue_location_top: bool,
    pub process_job_count: i32,
    pub dx_max_vertex_count: u32,
    pub dx_line2d_antialiasing_enable: bool,
    pub dx_line3d_antialiasing_enable: bool,
}

pub trait DebugRenderSettingsTrait: DataContainerTrait {
    fn enable(&self) -> &bool;
    fn draw_stats_enable(&self) -> &bool;
    fn text_view_distance(&self) -> &f32;
    fn text_queue_max_line_count(&self) -> &u32;
    fn text_queue_time_visible(&self) -> &f32;
    fn text_queue_location_top(&self) -> &bool;
    fn process_job_count(&self) -> &i32;
    fn dx_max_vertex_count(&self) -> &u32;
    fn dx_line2d_antialiasing_enable(&self) -> &bool;
    fn dx_line3d_antialiasing_enable(&self) -> &bool;
}

impl DebugRenderSettingsTrait for DebugRenderSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn draw_stats_enable(&self) -> &bool {
        &self.draw_stats_enable
    }
    fn text_view_distance(&self) -> &f32 {
        &self.text_view_distance
    }
    fn text_queue_max_line_count(&self) -> &u32 {
        &self.text_queue_max_line_count
    }
    fn text_queue_time_visible(&self) -> &f32 {
        &self.text_queue_time_visible
    }
    fn text_queue_location_top(&self) -> &bool {
        &self.text_queue_location_top
    }
    fn process_job_count(&self) -> &i32 {
        &self.process_job_count
    }
    fn dx_max_vertex_count(&self) -> &u32 {
        &self.dx_max_vertex_count
    }
    fn dx_line2d_antialiasing_enable(&self) -> &bool {
        &self.dx_line2d_antialiasing_enable
    }
    fn dx_line3d_antialiasing_enable(&self) -> &bool {
        &self.dx_line3d_antialiasing_enable
    }
}

impl DataContainerTrait for DebugRenderSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DEBUGRENDERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugRenderSettings",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebugRenderSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebugRenderSettings, enable),
            },
            FieldInfoData {
                name: "DrawStatsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebugRenderSettings, draw_stats_enable),
            },
            FieldInfoData {
                name: "TextViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebugRenderSettings, text_view_distance),
            },
            FieldInfoData {
                name: "TextQueueMaxLineCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebugRenderSettings, text_queue_max_line_count),
            },
            FieldInfoData {
                name: "TextQueueTimeVisible",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebugRenderSettings, text_queue_time_visible),
            },
            FieldInfoData {
                name: "TextQueueLocationTop",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebugRenderSettings, text_queue_location_top),
            },
            FieldInfoData {
                name: "ProcessJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DebugRenderSettings, process_job_count),
            },
            FieldInfoData {
                name: "DxMaxVertexCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebugRenderSettings, dx_max_vertex_count),
            },
            FieldInfoData {
                name: "DxLine2dAntialiasingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebugRenderSettings, dx_line2d_antialiasing_enable),
            },
            FieldInfoData {
                name: "DxLine3dAntialiasingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebugRenderSettings, dx_line3d_antialiasing_enable),
            },
        ],
    }),
    array_type: Some(DEBUGRENDERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DebugRenderSettings {
    fn type_info(&self) -> &'static TypeInfo {
        DEBUGRENDERSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEBUGRENDERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugRenderSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DebugRenderSettings"),
    array_type: None,
    alignment: 8,
};



pub static GETSCREENHEIGHT_UINT32__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetScreenHeight(Uint32)",
    flags: MemberInfoFlags::new(793),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub static GETSCREENWIDTH_UINT32__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetScreenWidth(Uint32)",
    flags: MemberInfoFlags::new(793),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub static DRAWTEXT2D_INT32_INT32_FLOAT32_CSTRING__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DrawText2d(Int32,Int32,Float32,CString)",
    flags: MemberInfoFlags::new(793),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RuntimeModule {
}

pub trait RuntimeModuleTrait: TypeObject {
}

impl RuntimeModuleTrait for RuntimeModule {
}

pub static RUNTIMEMODULE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeModule",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RuntimeModule as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RUNTIMEMODULE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RuntimeModule {
    fn type_info(&self) -> &'static TypeInfo {
        RUNTIMEMODULE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RUNTIMEMODULE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeModule-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RuntimeModule"),
    array_type: None,
    alignment: 8,
};



pub static BOXEDVALUEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxedValueRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("BoxedValueRef"),
    array_type: None,
    alignment: 8,
};



pub static TYPEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeRef"),
    array_type: None,
    alignment: 8,
};



pub static DBOBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DbObject-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DbObject"),
    array_type: None,
    alignment: 8,
};



pub static DBOBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DbObject",
    flags: MemberInfoFlags::new(45),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(DBOBJECT_ARRAY_TYPE_INFO),
    alignment: 8,
};



pub static FILEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FileRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FileRef"),
    array_type: None,
    alignment: 8,
};



pub static STRING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "String-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("String"),
    array_type: None,
    alignment: 8,
};



pub static STRING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "String",
    flags: MemberInfoFlags::new(16589),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(STRING_ARRAY_TYPE_INFO),
    alignment: 8,
};



pub static CSTRING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CString-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("CString"),
    array_type: None,
    alignment: 8,
};



pub static VOID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Void-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Void"),
    array_type: None,
    alignment: 8,
};



pub static VOID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Void",
    flags: MemberInfoFlags::new(13),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(VOID_ARRAY_TYPE_INFO),
    alignment: 0,
};



pub static RESOURCEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ResourceRef"),
    array_type: None,
    alignment: 8,
};



pub static SHA1_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SHA1-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SHA1"),
    array_type: None,
    alignment: 8,
};



pub static GUID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Guid-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Guid"),
    array_type: None,
    alignment: 8,
};



pub static FLOAT64_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Float64-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Float64"),
    array_type: None,
    alignment: 8,
};



pub static FLOAT32_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Float32-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Float32"),
    array_type: None,
    alignment: 8,
};



pub static INT64_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int64-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Int64"),
    array_type: None,
    alignment: 8,
};



pub static UINT64_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Uint64-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Uint64"),
    array_type: None,
    alignment: 8,
};



pub static INT32_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int32-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Int32"),
    array_type: None,
    alignment: 8,
};



pub static UINT32_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Uint32-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Uint32"),
    array_type: None,
    alignment: 8,
};



pub static INT16_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int16-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Int16"),
    array_type: None,
    alignment: 8,
};



pub static UINT16_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Uint16-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Uint16"),
    array_type: None,
    alignment: 8,
};



pub static INT8_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int8-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Int8"),
    array_type: None,
    alignment: 8,
};



pub static UINT8_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Uint8-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Uint8"),
    array_type: None,
    alignment: 8,
};



pub static BOOLEAN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Boolean-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Boolean"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AxisAlignedBox {
    pub min: Vec3,
    pub max: Vec3,
}

pub trait AxisAlignedBoxTrait: TypeObject {
    fn min(&self) -> &Vec3;
    fn max(&self) -> &Vec3;
}

impl AxisAlignedBoxTrait for AxisAlignedBox {
    fn min(&self) -> &Vec3 {
        &self.min
    }
    fn max(&self) -> &Vec3 {
        &self.max
    }
}

pub static AXISALIGNEDBOX_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AxisAlignedBox",
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AxisAlignedBox as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "min",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AxisAlignedBox, min),
            },
            FieldInfoData {
                name: "max",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AxisAlignedBox, max),
            },
        ],
    }),
    array_type: Some(AXISALIGNEDBOX_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AxisAlignedBox {
    fn type_info(&self) -> &'static TypeInfo {
        AXISALIGNEDBOX_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AXISALIGNEDBOX_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AxisAlignedBox-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("AxisAlignedBox"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Mat4 {
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m14: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m24: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
    pub m34: f32,
    pub m41: f32,
    pub m42: f32,
    pub m43: f32,
    pub m44: f32,
}

pub trait Mat4Trait: TypeObject {
    fn m11(&self) -> &f32;
    fn m12(&self) -> &f32;
    fn m13(&self) -> &f32;
    fn m14(&self) -> &f32;
    fn m21(&self) -> &f32;
    fn m22(&self) -> &f32;
    fn m23(&self) -> &f32;
    fn m24(&self) -> &f32;
    fn m31(&self) -> &f32;
    fn m32(&self) -> &f32;
    fn m33(&self) -> &f32;
    fn m34(&self) -> &f32;
    fn m41(&self) -> &f32;
    fn m42(&self) -> &f32;
    fn m43(&self) -> &f32;
    fn m44(&self) -> &f32;
}

impl Mat4Trait for Mat4 {
    fn m11(&self) -> &f32 {
        &self.m11
    }
    fn m12(&self) -> &f32 {
        &self.m12
    }
    fn m13(&self) -> &f32 {
        &self.m13
    }
    fn m14(&self) -> &f32 {
        &self.m14
    }
    fn m21(&self) -> &f32 {
        &self.m21
    }
    fn m22(&self) -> &f32 {
        &self.m22
    }
    fn m23(&self) -> &f32 {
        &self.m23
    }
    fn m24(&self) -> &f32 {
        &self.m24
    }
    fn m31(&self) -> &f32 {
        &self.m31
    }
    fn m32(&self) -> &f32 {
        &self.m32
    }
    fn m33(&self) -> &f32 {
        &self.m33
    }
    fn m34(&self) -> &f32 {
        &self.m34
    }
    fn m41(&self) -> &f32 {
        &self.m41
    }
    fn m42(&self) -> &f32 {
        &self.m42
    }
    fn m43(&self) -> &f32 {
        &self.m43
    }
    fn m44(&self) -> &f32 {
        &self.m44
    }
}

pub static MAT4_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Mat4",
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Mat4 as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "m11",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m11),
            },
            FieldInfoData {
                name: "m12",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m12),
            },
            FieldInfoData {
                name: "m13",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m13),
            },
            FieldInfoData {
                name: "m14",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m14),
            },
            FieldInfoData {
                name: "m21",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m21),
            },
            FieldInfoData {
                name: "m22",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m22),
            },
            FieldInfoData {
                name: "m23",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m23),
            },
            FieldInfoData {
                name: "m24",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m24),
            },
            FieldInfoData {
                name: "m31",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m31),
            },
            FieldInfoData {
                name: "m32",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m32),
            },
            FieldInfoData {
                name: "m33",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m33),
            },
            FieldInfoData {
                name: "m34",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m34),
            },
            FieldInfoData {
                name: "m41",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m41),
            },
            FieldInfoData {
                name: "m42",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m42),
            },
            FieldInfoData {
                name: "m43",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m43),
            },
            FieldInfoData {
                name: "m44",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m44),
            },
        ],
    }),
    array_type: Some(MAT4_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Mat4 {
    fn type_info(&self) -> &'static TypeInfo {
        MAT4_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MAT4_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Mat4-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Mat4"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinearTransform {
    pub right: Vec3,
    pub up: Vec3,
    pub forward: Vec3,
    pub trans: Vec3,
}

pub trait LinearTransformTrait: TypeObject {
    fn right(&self) -> &Vec3;
    fn up(&self) -> &Vec3;
    fn forward(&self) -> &Vec3;
    fn trans(&self) -> &Vec3;
}

impl LinearTransformTrait for LinearTransform {
    fn right(&self) -> &Vec3 {
        &self.right
    }
    fn up(&self) -> &Vec3 {
        &self.up
    }
    fn forward(&self) -> &Vec3 {
        &self.forward
    }
    fn trans(&self) -> &Vec3 {
        &self.trans
    }
}

pub static LINEARTRANSFORM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearTransform",
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinearTransform as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "right",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LinearTransform, right),
            },
            FieldInfoData {
                name: "up",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LinearTransform, up),
            },
            FieldInfoData {
                name: "forward",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LinearTransform, forward),
            },
            FieldInfoData {
                name: "trans",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LinearTransform, trans),
            },
        ],
    }),
    array_type: Some(LINEARTRANSFORM_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LinearTransform {
    fn type_info(&self) -> &'static TypeInfo {
        LINEARTRANSFORM_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LINEARTRANSFORM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearTransform-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("LinearTransform"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Plane {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub trait PlaneTrait: TypeObject {
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
    fn z(&self) -> &f32;
    fn w(&self) -> &f32;
}

impl PlaneTrait for Plane {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn z(&self) -> &f32 {
        &self.z
    }
    fn w(&self) -> &f32 {
        &self.w
    }
}

pub static PLANE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Plane",
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Plane as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Plane, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Plane, y),
            },
            FieldInfoData {
                name: "z",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Plane, z),
            },
            FieldInfoData {
                name: "w",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Plane, w),
            },
        ],
    }),
    array_type: Some(PLANE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Plane {
    fn type_info(&self) -> &'static TypeInfo {
        PLANE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static PLANE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Plane-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Plane"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub trait QuatTrait: TypeObject {
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
    fn z(&self) -> &f32;
    fn w(&self) -> &f32;
}

impl QuatTrait for Quat {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn z(&self) -> &f32 {
        &self.z
    }
    fn w(&self) -> &f32 {
        &self.w
    }
}

pub static QUAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Quat",
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Quat as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Quat, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Quat, y),
            },
            FieldInfoData {
                name: "z",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Quat, z),
            },
            FieldInfoData {
                name: "w",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Quat, w),
            },
        ],
    }),
    array_type: Some(QUAT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Quat {
    fn type_info(&self) -> &'static TypeInfo {
        QUAT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static QUAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Quat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Quat"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub trait Vec4Trait: TypeObject {
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
    fn z(&self) -> &f32;
    fn w(&self) -> &f32;
}

impl Vec4Trait for Vec4 {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn z(&self) -> &f32 {
        &self.z
    }
    fn w(&self) -> &f32 {
        &self.w
    }
}

pub static VEC4_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4",
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec4 as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Vec4, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Vec4, y),
            },
            FieldInfoData {
                name: "z",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Vec4, z),
            },
            FieldInfoData {
                name: "w",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Vec4, w),
            },
        ],
    }),
    array_type: Some(VEC4_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Vec4 {
    fn type_info(&self) -> &'static TypeInfo {
        VEC4_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEC4_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Vec4"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub trait Vec3Trait: TypeObject {
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
    fn z(&self) -> &f32;
}

impl Vec3Trait for Vec3 {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn z(&self) -> &f32 {
        &self.z
    }
}

pub static VEC3_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3",
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec3 as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Vec3, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Vec3, y),
            },
            FieldInfoData {
                name: "z",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Vec3, z),
            },
        ],
    }),
    array_type: Some(VEC3_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Vec3 {
    fn type_info(&self) -> &'static TypeInfo {
        VEC3_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEC3_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Vec3"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub trait Vec2Trait: TypeObject {
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
}

impl Vec2Trait for Vec2 {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
}

pub static VEC2_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2",
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec2 as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Vec2, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Vec2, y),
            },
        ],
    }),
    array_type: Some(VEC2_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for Vec2 {
    fn type_info(&self) -> &'static TypeInfo {
        VEC2_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEC2_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Vec2"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FbVec {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub trait FbVecTrait: TypeObject {
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
    fn z(&self) -> &f32;
    fn w(&self) -> &f32;
}

impl FbVecTrait for FbVec {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn z(&self) -> &f32 {
        &self.z
    }
    fn w(&self) -> &f32 {
        &self.w
    }
}

pub static VEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec",
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FbVec as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FbVec, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FbVec, y),
            },
            FieldInfoData {
                name: "z",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FbVec, z),
            },
            FieldInfoData {
                name: "w",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FbVec, w),
            },
        ],
    }),
    array_type: Some(VEC_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FbVec {
    fn type_info(&self) -> &'static TypeInfo {
        VEC_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Vec"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EventDispatcher {
}

pub trait EventDispatcherTrait: TypeObject {
}

impl EventDispatcherTrait for EventDispatcher {
}

pub static EVENTDISPATCHER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventDispatcher",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EventDispatcher as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EVENTDISPATCHER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EventDispatcher {
    fn type_info(&self) -> &'static TypeInfo {
        EVENTDISPATCHER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EVENTDISPATCHER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventDispatcher-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("EventDispatcher"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DataContainer {
    pub _glacier_dc_core: glacier_reflect::data_container::DataContainerCore,
}

pub trait DataContainerTrait: TypeObject {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore;
}

impl DataContainerTrait for DataContainer {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        &self._glacier_dc_core
    }
}

pub static DATACONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainer",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DataContainer as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DATACONTAINER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DataContainer {
    fn type_info(&self) -> &'static TypeInfo {
        DATACONTAINER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DATACONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DataContainer"),
    array_type: None,
    alignment: 8,
};


