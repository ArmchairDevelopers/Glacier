use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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
    name_hash: 2648306668,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcComputePsoInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGDISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcCsSkinningDispatchInstructionData-Array",
    name_hash: 1151252516,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcCsSkinningDispatchInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGBUFFERINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcCsSkinningBufferInstructionData-Array",
    name_hash: 323541940,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcCsSkinningBufferInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGPARAMSINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcCsSkinningParamsInstructionData-Array",
    name_hash: 2706013992,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcCsSkinningParamsInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCVERTEXBUFFERINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcVertexBufferInstructionData-Array",
    name_hash: 3100728573,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcVertexBufferInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCSAMPLERPOINTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcSamplerPointer-Array",
    name_hash: 2089383730,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcSamplerPointer"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCSAMPLERTABLEWRITERINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcSamplerTableWriterInstructionData-Array",
    name_hash: 1065783728,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcSamplerTableWriterInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcRvmDescriptorTableAssemblyInstructionData-Array",
    name_hash: 2154525487,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcRvmDescriptorTableAssemblyInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCSHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcShaderDispatchDrawInstructionData-Array",
    name_hash: 3737147916,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcShaderDispatchDrawInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCDISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcDispatchInstructionData-Array",
    name_hash: 131210917,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcDispatchInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCPSOPRELOADOP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcPSOPreloadOp-Array",
    name_hash: 146041647,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcPSOPreloadOp"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCSAMPLER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcSampler-Array",
    name_hash: 1209302601,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcSampler"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12PCROOTSIGNATURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcRootSignature-Array",
    name_hash: 2254539583,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcRootSignature"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12VERTEXBUFFERVIEWINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12VertexBufferViewInstructionData-Array",
    name_hash: 1425794819,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12VertexBufferViewInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12ViewStateInstructionData-Array",
    name_hash: 1224866076,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12ViewStateInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static D3D12_GPU_DESCRIPTOR_HANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_GPU_DESCRIPTOR_HANDLE-Array",
    name_hash: 2619108485,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_GPU_DESCRIPTOR_HANDLE"),
    array_type: None,
    alignment: 8,
};



pub static D3D12_VERTEX_BUFFER_VIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_VERTEX_BUFFER_VIEW-Array",
    name_hash: 706748123,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_VERTEX_BUFFER_VIEW"),
    array_type: None,
    alignment: 8,
};



pub static DX12RVMINPUTELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmInputElement-Array",
    name_hash: 4110538479,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmInputElement"),
    array_type: None,
    alignment: 8,
};



pub static DX12RVMSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmShader-Array",
    name_hash: 1210346926,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmShader"),
    array_type: None,
    alignment: 8,
};



pub static DX12RVMROOTSIGNATURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmRootSignature-Array",
    name_hash: 2080483941,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmRootSignature"),
    array_type: None,
    alignment: 8,
};



pub static D3D12_VIEWPORT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_VIEWPORT-Array",
    name_hash: 695137770,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_VIEWPORT"),
    array_type: None,
    alignment: 8,
};



pub static D3D12_PIPELINE_STATE_FLAGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_PIPELINE_STATE_FLAGS-Array",
    name_hash: 2116967284,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_PIPELINE_STATE_FLAGS"),
    array_type: None,
    alignment: 8,
};



pub static D3D12_CACHED_PIPELINE_STATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_CACHED_PIPELINE_STATE-Array",
    name_hash: 689138435,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_CACHED_PIPELINE_STATE"),
    array_type: None,
    alignment: 8,
};



pub static DXGI_SAMPLE_DESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DXGI_SAMPLE_DESC-Array",
    name_hash: 762048020,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DXGI_SAMPLE_DESC"),
    array_type: None,
    alignment: 8,
};



pub static D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_INDEX_BUFFER_STRIP_CUT_VALUE-Array",
    name_hash: 4072153509,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_INDEX_BUFFER_STRIP_CUT_VALUE"),
    array_type: None,
    alignment: 8,
};



pub static DXGI_FORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DXGI_FORMAT-Array",
    name_hash: 59435807,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DXGI_FORMAT"),
    array_type: None,
    alignment: 8,
};



pub static D3D12_PRIMITIVE_TOPOLOGY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_PRIMITIVE_TOPOLOGY-Array",
    name_hash: 938740921,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_PRIMITIVE_TOPOLOGY"),
    array_type: None,
    alignment: 8,
};



pub static D3D12_PRIMITIVE_TOPOLOGY_TYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_PRIMITIVE_TOPOLOGY_TYPE-Array",
    name_hash: 1007518238,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_PRIMITIVE_TOPOLOGY_TYPE"),
    array_type: None,
    alignment: 8,
};



pub static DX12RVMSODECLARATIONENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmSoDeclarationEntry-Array",
    name_hash: 2421225327,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmSoDeclarationEntry"),
    array_type: None,
    alignment: 8,
};



pub static DX12RVMDEPTHSTENCILDESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmDepthStencilDesc-Array",
    name_hash: 4128548753,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmDepthStencilDesc"),
    array_type: None,
    alignment: 8,
};



pub static DX12RVMRASTERIZERDESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmRasterizerDesc-Array",
    name_hash: 2057370321,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmRasterizerDesc"),
    array_type: None,
    alignment: 8,
};



pub static DX12RVMBLENDDESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmBlendDesc-Array",
    name_hash: 906295255,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmBlendDesc"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyDebugInfo-Array",
    name_hash: 1230639812,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyDebugInfo"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12BLENDSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12BlendStateData-Array",
    name_hash: 1707407176,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12BlendStateData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyInstructionData-Array",
    name_hash: 294310579,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12ROOTWRITEOP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootWriteOp-Array",
    name_hash: 3205643210,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RootWriteOp"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionBatchData-Array",
    name_hash: 794019065,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionBatchData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionData-Array",
    name_hash: 1301437125,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12SHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12ShaderDispatchDrawInstructionData-Array",
    name_hash: 1303219999,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12ShaderDispatchDrawInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12ROOTSIGNATUREDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootSignatureDebugInfo-Array",
    name_hash: 3014091379,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RootSignatureDebugInfo"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12TABLEDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12TableDebugInfo-Array",
    name_hash: 268283503,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12TableDebugInfo"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12TABLEDEBUGENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12TableDebugEntry-Array",
    name_hash: 2585461717,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12TableDebugEntry"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12INPUTELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12InputElement-Array",
    name_hash: 3477658118,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12InputElement"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12SHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12Shader-Array",
    name_hash: 3670147527,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12Shader"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12BINARYBLOB_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12BinaryBlob-Array",
    name_hash: 1622697378,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12BinaryBlob"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12SHADERSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12ShaderState-Array",
    name_hash: 1842938160,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12ShaderState"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ShaderDispatchDrawInstructionData-Array",
    name_hash: 4057563516,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11ShaderDispatchDrawInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11LegacyVertexBufferConversionInstructionData-Array",
    name_hash: 1629023776,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11LegacyVertexBufferConversionInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ViewStateInstructionData-Array",
    name_hash: 1370564063,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11ViewStateInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static D3D11_CONSERVATIVE_RASTERIZATION_MODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D11_CONSERVATIVE_RASTERIZATION_MODE-Array",
    name_hash: 4270242220,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D11_CONSERVATIVE_RASTERIZATION_MODE"),
    array_type: None,
    alignment: 8,
};



pub static DX11RVMVIEWPORTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmViewports-Array",
    name_hash: 3386238819,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmViewports"),
    array_type: None,
    alignment: 8,
};



pub static DX11RVMSCISSORRECTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmScissorRects-Array",
    name_hash: 354943635,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmScissorRects"),
    array_type: None,
    alignment: 8,
};



pub static DX11RVMDEPTHSTENCILSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmDepthStencilState-Array",
    name_hash: 807735124,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmDepthStencilState"),
    array_type: None,
    alignment: 8,
};



pub static DX11RVMBLENDSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBlendState-Array",
    name_hash: 702759218,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmBlendState"),
    array_type: None,
    alignment: 8,
};



pub static D3D11_CULL_MODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D11_CULL_MODE-Array",
    name_hash: 2125933015,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D11_CULL_MODE"),
    array_type: None,
    alignment: 8,
};



pub static D3D11_FILL_MODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D11_FILL_MODE-Array",
    name_hash: 3082601294,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D11_FILL_MODE"),
    array_type: None,
    alignment: 8,
};



pub static DX11RVMSAMPLER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmSampler-Array",
    name_hash: 2353680912,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmSampler"),
    array_type: None,
    alignment: 8,
};



pub static DX11RVMVSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmVsShader-Array",
    name_hash: 166327080,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmVsShader"),
    array_type: None,
    alignment: 8,
};



pub static DX11RVMPSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmPsShader-Array",
    name_hash: 4014088942,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmPsShader"),
    array_type: None,
    alignment: 8,
};



pub static DX11RVMHSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmHsShader-Array",
    name_hash: 3760367606,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmHsShader"),
    array_type: None,
    alignment: 8,
};



pub static DX11RVMDSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmDsShader-Array",
    name_hash: 3474744698,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmDsShader"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11BYTECODEELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ByteCodeElement-Array",
    name_hash: 833050388,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11ByteCodeElement"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11DSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11DsShader-Array",
    name_hash: 1555616243,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11DsShader"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11HSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11HsShader-Array",
    name_hash: 447549055,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11HsShader"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11PSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11PsShader-Array",
    name_hash: 2094960487,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11PsShader"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11VSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11VsShader-Array",
    name_hash: 757702305,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11VsShader"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11INPUTELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11InputElement-Array",
    name_hash: 4013942725,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11InputElement"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11SAMPLER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11Sampler-Array",
    name_hash: 1310887609,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11Sampler"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11BLENDSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11BlendStateData-Array",
    name_hash: 416352907,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11BlendStateData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11SERIALIZEDBLENDSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11SerializedBlendState-Array",
    name_hash: 1310644041,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11SerializedBlendState"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11TEXTURECONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11TextureConversionInstructionData-Array",
    name_hash: 2921850242,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11TextureConversionInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11BUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11BufferConversionInstructionData-Array",
    name_hash: 1861636029,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11BufferConversionInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11LEGACYDRAWSTATEBUILDERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11LegacyDrawStateBuilderData-Array",
    name_hash: 1756477310,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11LegacyDrawStateBuilderData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHLEGACYDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ShaderDispatchLegacyDrawInstructionData-Array",
    name_hash: 1078710185,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11ShaderDispatchLegacyDrawInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ApplyParametersInstructionData-Array",
    name_hash: 2668919595,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11ApplyParametersInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSBLOCK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ApplyParametersBlock-Array",
    name_hash: 1887442474,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11ApplyParametersBlock"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11DISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11DispatchInstructionData-Array",
    name_hash: 855862421,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11DispatchInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_LODFADEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_LodFadeInstructionData-Array",
    name_hash: 3303674712,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_LodFadeInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionData-Array",
    name_hash: 3038227991,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SLICECOUNTINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SliceCountInstructionData-Array",
    name_hash: 279236650,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_SliceCountInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_TESSELLATIONPARAMETERSINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_TessellationParametersInstructionData-Array",
    name_hash: 962014954,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_TessellationParametersInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_VECTORSUBTRACTINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_VectorSubtractInstructionData-Array",
    name_hash: 2096013684,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_VectorSubtractInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_OFFSETTRANSLATIONINMATRIXINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionData-Array",
    name_hash: 3685109539,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_CPUTOGPUMATRIXINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_CpuToGpuMatrixInstructionData-Array",
    name_hash: 155409149,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_CpuToGpuMatrixInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_FLOATTOVECINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_FloatToVecInstructionData-Array",
    name_hash: 3936998498,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_FloatToVecInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_LegacyVertexBufferConversionInstructionData-Array",
    name_hash: 3873153788,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_LegacyVertexBufferConversionInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_VERTEXSTREAMPROCESSINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_VertexStreamProcessInstructionData-Array",
    name_hash: 324517862,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_VertexStreamProcessInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ViewStateInstructionData-Array",
    name_hash: 3058223747,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ViewStateInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMPOINTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmPointer-Array",
    name_hash: 3206713091,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmPointer"),
    array_type: None,
    alignment: 8,
};



pub static GPUMAT4X3_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GpuMat4x3-Array",
    name_hash: 212515220,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("GpuMat4x3"),
    array_type: None,
    alignment: 8,
};



pub static IVEC4_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVec4-Array",
    name_hash: 2797400732,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("IVec4"),
    array_type: None,
    alignment: 8,
};



pub static IVEC3_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVec3-Array",
    name_hash: 923557531,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("IVec3"),
    array_type: None,
    alignment: 8,
};



pub static IVEC2_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVec2-Array",
    name_hash: 1647647130,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("IVec2"),
    array_type: None,
    alignment: 8,
};



pub static HALF4_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half4-Array",
    name_hash: 1272735014,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Half4"),
    array_type: None,
    alignment: 8,
};



pub static HALF3_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half3-Array",
    name_hash: 2402987425,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Half3"),
    array_type: None,
    alignment: 8,
};



pub static HALF2_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half2-Array",
    name_hash: 3547322912,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Half2"),
    array_type: None,
    alignment: 8,
};



pub static HALF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half-Array",
    name_hash: 1325657682,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Half"),
    array_type: None,
    alignment: 8,
};



pub static RVMLEGACYINSTRUCTIONS_NS_LEGACYINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyInstructions_ns_LegacyInstructionData-Array",
    name_hash: 2674853638,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmLegacyInstructions_ns_LegacyInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchData-Array",
    name_hash: 2333680876,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstanceTableAssemblyData-Array",
    name_hash: 2014540584,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_InstanceTableAssemblyData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DEPTHBIASGROUPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DepthBiasGroupData-Array",
    name_hash: 407209002,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DepthBiasGroupData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_TABLEASSEMBLYINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_TableAssemblyInstructionBatchData-Array",
    name_hash: 3493548069,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_TableAssemblyInstructionBatchData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_TABLEASSEMBLYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_TableAssemblyData-Array",
    name_hash: 1858486177,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_TableAssemblyData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_WRITEOPGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_WriteOpGroup-Array",
    name_hash: 2466878764,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_WriteOpGroup"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_WRITEOP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_WriteOp-Array",
    name_hash: 3604053939,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_WriteOp"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DrawStateBuilderInstructionData-Array",
    name_hash: 1302708591,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DrawStateBuilderInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DispatchInstructionData-Array",
    name_hash: 3284416841,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DispatchInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DIRECTINPUTINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DirectInputInstructionData-Array",
    name_hash: 4065433602,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DirectInputInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_CONSTANTVALUEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ConstantValueInstructionData-Array",
    name_hash: 501365260,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ConstantValueInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static CHAR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "char-Array",
    name_hash: 3920249577,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("char"),
    array_type: None,
    alignment: 8,
};



pub static PARAMDBHASH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParamDbHash-Array",
    name_hash: 1724247018,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ParamDbHash"),
    array_type: None,
    alignment: 8,
};



pub static RVMENCODEDTABLEENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmEncodedTableEntry-Array",
    name_hash: 1895629360,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmEncodedTableEntry"),
    array_type: None,
    alignment: 8,
};



pub static RVMCONTEXTSORTKEYINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmContextSortKeyInfo-Array",
    name_hash: 1608051556,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmContextSortKeyInfo"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PREPAREDVERTEXELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_PreparedVertexElement-Array",
    name_hash: 3818491234,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_PreparedVertexElement"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PREPAREDVERTEXSTREAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_PreparedVertexStream-Array",
    name_hash: 727141792,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_PreparedVertexStream"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SERIALIZEDPARAMDBKEY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SerializedParamDbKey-Array",
    name_hash: 2785839709,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_SerializedParamDbKey"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_BASESHADERSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_BaseShaderState-Array",
    name_hash: 2733637818,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_BaseShaderState"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RUNTIMEINSTANTIATEDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RuntimeInstantiatedType-Array",
    name_hash: 2741816915,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RuntimeInstantiatedType"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDMULTIHASHVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedMultiHashView-Array",
    name_hash: 702316316,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbSerializedMultiHashView"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDREADVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedReadView-Array",
    name_hash: 2874628661,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbSerializedReadView"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEWREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedHashViewRef-Array",
    name_hash: 1223871908,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbSerializedHashViewRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedHashView-Array",
    name_hash: 774325301,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbSerializedHashView"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDFILTERVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedFilterView-Array",
    name_hash: 1083508519,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbSerializedFilterView"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_COMBINEDSERIALIZEDPARAMETERBLOCK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_CombinedSerializedParameterBlock-Array",
    name_hash: 1120465414,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_CombinedSerializedParameterBlock"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCKREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SerializedParameterBlockRef-Array",
    name_hash: 2918914866,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_SerializedParameterBlockRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SerializedParameterBlock-Array",
    name_hash: 1370268515,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_SerializedParameterBlock"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTUREREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableExternalTextureRef-Array",
    name_hash: 2691943709,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ShaderStreamableExternalTextureRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTUREREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableTextureRef-Array",
    name_hash: 3763369952,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ShaderStreamableTextureRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SURFACESHADERDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SurfaceShaderDebugInfo-Array",
    name_hash: 707627986,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_SurfaceShaderDebugInfo"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SURFACESHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SurfaceShader-Array",
    name_hash: 2102099117,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_SurfaceShader"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableTexture-Array",
    name_hash: 2835350897,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ShaderStreamableTexture"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableExternalTexture-Array",
    name_hash: 1693287980,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ShaderStreamableExternalTexture"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionInstanceRef-Array",
    name_hash: 1746913410,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmFunctionInstanceRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionInstance-Array",
    name_hash: 3191782611,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmFunctionInstance"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_SETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Settings-Array",
    name_hash: 1100612180,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Settings"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMPERMUTATIONSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationSet-Array",
    name_hash: 3331162924,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmPermutationSet"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMPERMUTATIONLOOKUPTABLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationLookupTable-Array",
    name_hash: 3797074258,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmPermutationLookupTable"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMPERMUTATIONTREE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationTree-Array",
    name_hash: 2285614600,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmPermutationTree"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMPERMUTATIONREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationRef-Array",
    name_hash: 2792979455,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmPermutationRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMPERMUTATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutation-Array",
    name_hash: 378487246,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmPermutation"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMFUNCTIONINPUTTABLEINDICES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionInputTableIndices-Array",
    name_hash: 2443551725,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmFunctionInputTableIndices"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMDISPATCHDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmDispatchDebugInfo-Array",
    name_hash: 4173554871,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmDispatchDebugInfo"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMFUNCTIONDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionDebugInfo-Array",
    name_hash: 3838974245,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmFunctionDebugInfo"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMDISPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmDispatch-Array",
    name_hash: 2436704680,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmDispatch"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RVMFUNCTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunction-Array",
    name_hash: 1900107258,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmFunction"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_INSTRUCTIONBATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstructionBatch-Array",
    name_hash: 4121637781,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_InstructionBatch"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_RTTITYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RttiType-Array",
    name_hash: 4126664722,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RttiType"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DEFAULTVALUEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueRef-Array",
    name_hash: 4040066464,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DefaultValueRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_VALUEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ValueRef-Array",
    name_hash: 2012878411,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ValueRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DEFAULTVALUEZEROMEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueZeroMem-Array",
    name_hash: 2303578070,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DefaultValueZeroMem"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DEFAULTVALUESTRUCTLEGACYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueStructLegacyData-Array",
    name_hash: 2929086147,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DefaultValueStructLegacyData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLETEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueSimpleTexture-Array",
    name_hash: 4053121600,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DefaultValueSimpleTexture"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLEBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueSimpleBuffer-Array",
    name_hash: 2765366495,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DefaultValueSimpleBuffer"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_INSTRUCTIONBATCHREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstructionBatchRef-Array",
    name_hash: 1796194052,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_InstructionBatchRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PARAMDBKEYREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbKeyRef-Array",
    name_hash: 3675870078,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbKeyRef"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_PARAMDBKEYDESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbKeyDesc-Array",
    name_hash: 3035982014,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbKeyDesc"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTTLASNULLDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasNullData-Array",
    name_hash: 2362981417,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtTlasNullData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTTLASDYNAMICDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasDynamicData-Array",
    name_hash: 182961639,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtTlasDynamicData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTTLASSTATICDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasStaticData-Array",
    name_hash: 4018024522,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtTlasStaticData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTTLASBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasBaseData-Array",
    name_hash: 1292595591,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtTlasBaseData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTSHADERTABLERECORDWRITERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtShaderTableRecordWriterData-Array",
    name_hash: 2658628941,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtShaderTableRecordWriterData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTNULLHITSHADERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtNullHitShaderData-Array",
    name_hash: 788921855,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtNullHitShaderData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTHITCOLLECTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtHitCollectionData-Array",
    name_hash: 2382758715,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtHitCollectionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTSTATICBLASBUILDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtStaticBlasBuildData-Array",
    name_hash: 2132700202,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtStaticBlasBuildData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTDYNAMICBLASBUILDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtDynamicBlasBuildData-Array",
    name_hash: 1575953447,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtDynamicBlasBuildData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTBLASBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtBlasBaseData-Array",
    name_hash: 3333393233,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtBlasBaseData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTHITSHADERCONSTANTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtHitShaderConstantData-Array",
    name_hash: 1653923130,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtHitShaderConstantData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTINDEXBUFFERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtIndexBufferData-Array",
    name_hash: 2367695398,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtIndexBufferData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTSORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtSortData-Array",
    name_hash: 1516598146,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtSortData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTDISPATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtDispatchData-Array",
    name_hash: 1341272648,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtDispatchData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12RTCOLLECTIONPRELOADOP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtCollectionPreloadOp-Array",
    name_hash: 2017918336,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtCollectionPreloadOp"),
    array_type: None,
    alignment: 8,
};



pub static RVMLEGACYLIGHTMAPINSTANCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyLightMapInstance-Array",
    name_hash: 2296388742,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmLegacyLightMapInstance"),
    array_type: None,
    alignment: 8,
};



pub static RVMLEGACYLIGHTPROBES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyLightProbes-Array",
    name_hash: 3215410250,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmLegacyLightProbes"),
    array_type: None,
    alignment: 8,
};



pub static RVMLEGACYPERMUTATIONDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyPermutationDebugInfo-Array",
    name_hash: 4185369924,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmLegacyPermutationDebugInfo"),
    array_type: None,
    alignment: 8,
};



pub static RVMLEGACYFORWARDLIGHTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyForwardLightState-Array",
    name_hash: 1258533439,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmLegacyForwardLightState"),
    array_type: None,
    alignment: 8,
};



pub static FRUSTUMSOA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FrustumSoA-Array",
    name_hash: 3230565810,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FrustumSoA"),
    array_type: None,
    alignment: 8,
};



pub static LODVIEWSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LodViewState-Array",
    name_hash: 2995987404,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("LodViewState"),
    array_type: None,
    alignment: 8,
};



pub static TESSELLATIONVIEWSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TessellationViewState-Array",
    name_hash: 2468496386,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TessellationViewState"),
    array_type: None,
    alignment: 8,
};



pub static STENCILSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StencilState-Array",
    name_hash: 467696652,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("StencilState"),
    array_type: None,
    alignment: 8,
};



pub static REFLECTIONSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReflectionState-Array",
    name_hash: 3255179393,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ReflectionState"),
    array_type: None,
    alignment: 8,
};



pub static FOGSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogState-Array",
    name_hash: 3307693896,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FogState"),
    array_type: None,
    alignment: 8,
};



pub static PROJECTIONSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProjectionState-Array",
    name_hash: 1602905979,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ProjectionState"),
    array_type: None,
    alignment: 8,
};



pub static VIEWSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ViewState-Array",
    name_hash: 2666841067,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ViewState"),
    array_type: None,
    alignment: 8,
};



pub static OUTDOORLIGHTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutdoorLightState-Array",
    name_hash: 2642892320,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("OutdoorLightState"),
    array_type: None,
    alignment: 8,
};



pub static DX12NVRVMROOTSIGNATURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmRootSignature-Array",
    name_hash: 4206626909,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12NvRvmRootSignature"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvDescriptorTableAssemblyInstructionData-Array",
    name_hash: 1238883149,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12NvDescriptorTableAssemblyInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionBatchData-Array",
    name_hash: 2981559681,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionBatchData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionData-Array",
    name_hash: 2812518269,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12NVCONSTANTBUFFERASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvConstantBufferAssemblyInstructionData-Array",
    name_hash: 1698528798,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12NvConstantBufferAssemblyInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvDescriptorTable-Array",
    name_hash: 3185479771,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12NvDescriptorTable"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX12NVRVMROOTSIGNATURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvRvmRootSignature-Array",
    name_hash: 910734589,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12NvRvmRootSignature"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEDEPTHINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11NvViewStateDepthInstructionData-Array",
    name_hash: 3886566602,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11NvViewStateDepthInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11NVDRAWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11NvDrawStateInstructionData-Array",
    name_hash: 3762818730,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11NvDrawStateInstructionData"),
    array_type: None,
    alignment: 8,
};



pub static RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11NvViewStateInstructionData-Array",
    name_hash: 1080772583,
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
    name_hash: 1603143183,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static RADIOSITYTYPEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityTypeOverride-Array",
    name_hash: 1306231483,
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
    name_hash: 1029133718,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static LOCALPLAYERID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlayerId-Array",
    name_hash: 620677794,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("LocalPlayerId"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl DataContainerTrait for NodeGraph {
}

pub static NODEGRAPH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NodeGraph",
    name_hash: 4122580041,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        super_class_offset: offset_of!(NodeGraph, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NodeGraph as Default>::default())),
            create_boxed: || Box::new(<NodeGraph as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static NODEGRAPH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NodeGraph-Array",
    name_hash: 3763116669,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("NodeGraph"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Node {
    pub _glacier_base: DataContainer,
}

pub trait NodeTrait: DataContainerTrait {
}

impl NodeTrait for Node {
}

impl DataContainerTrait for Node {
}

pub static NODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Node",
    name_hash: 2088947621,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(Node, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Node as Default>::default())),
            create_boxed: || Box::new(<Node as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static NODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Node-Array",
    name_hash: 2543151121,
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
    name_hash: 3998479156,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static GAMEPLATFORM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GamePlatform-Array",
    name_hash: 273591936,
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
    name_hash: 4023029371,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static DESIGNERENUMERATIONSORTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DesignerEnumerationSortType-Array",
    name_hash: 749741647,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DesignerEnumerationSortType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn live_editing_enable_mut(&mut self) -> &mut bool;
    fn disable_safe_state_for_refresh(&self) -> &bool;
    fn disable_safe_state_for_refresh_mut(&mut self) -> &mut bool;
    fn host(&self) -> &String;
    fn host_mut(&mut self) -> &mut String;
    fn host_user(&self) -> &String;
    fn host_user_mut(&mut self) -> &mut String;
    fn host_user_domain(&self) -> &String;
    fn host_user_domain_mut(&mut self) -> &mut String;
    fn init_seed(&self) -> &String;
    fn init_seed_mut(&mut self) -> &mut String;
    fn trace_host(&self) -> &String;
    fn trace_host_mut(&mut self) -> &mut String;
    fn forward_trace(&self) -> &bool;
    fn forward_trace_mut(&mut self) -> &mut bool;
    fn forward_trace_host(&self) -> &String;
    fn forward_trace_host_mut(&mut self) -> &mut String;
    fn forward_trace_port(&self) -> &i32;
    fn forward_trace_port_mut(&mut self) -> &mut i32;
    fn perf_track_enabled(&self) -> &bool;
    fn perf_track_enabled_mut(&mut self) -> &mut bool;
    fn log_level(&self) -> &CoreLogLevel;
    fn log_level_mut(&mut self) -> &mut CoreLogLevel;
    fn dialog_level(&self) -> &CoreLogLevel;
    fn dialog_level_mut(&mut self) -> &mut CoreLogLevel;
    fn user_log_enabled(&self) -> &bool;
    fn user_log_enabled_mut(&mut self) -> &mut bool;
    fn log_settings(&self) -> &LogSettings;
    fn log_settings_mut(&mut self) -> &mut LogSettings;
    fn allow_assign_vars_from_lua(&self) -> &bool;
    fn allow_assign_vars_from_lua_mut(&mut self) -> &mut bool;
    fn display_asserts(&self) -> &bool;
    fn display_asserts_mut(&mut self) -> &mut bool;
    fn debug_output_mode(&self) -> &u32;
    fn debug_output_mode_mut(&mut self) -> &mut u32;
    fn crash_on_fatal_errors(&self) -> &bool;
    fn crash_on_fatal_errors_mut(&mut self) -> &mut bool;
    fn enable_memory_snapshots(&self) -> &bool;
    fn enable_memory_snapshots_mut(&mut self) -> &mut bool;
    fn breakpad_mode(&self) -> &u32;
    fn breakpad_mode_mut(&mut self) -> &mut u32;
    fn random_time_seed(&self) -> &i32;
    fn random_time_seed_mut(&mut self) -> &mut i32;
    fn random_tick_seed(&self) -> &i32;
    fn random_tick_seed_mut(&mut self) -> &mut i32;
    fn random_session_id(&self) -> &i32;
    fn random_session_id_mut(&mut self) -> &mut i32;
    fn local_player_id_to_command(&self) -> &LocalPlayerId;
    fn local_player_id_to_command_mut(&mut self) -> &mut LocalPlayerId;
    fn game_configuration_name(&self) -> &String;
    fn game_configuration_name_mut(&mut self) -> &mut String;
    fn use_storage_server(&self) -> &bool;
    fn use_storage_server_mut(&mut self) -> &mut bool;
    fn hardware_profile(&self) -> &HardwareProfile;
    fn hardware_profile_mut(&mut self) -> &mut HardwareProfile;
    fn hardware_cpu_bias(&self) -> &f32;
    fn hardware_cpu_bias_mut(&mut self) -> &mut f32;
    fn hardware_gpu_bias(&self) -> &f32;
    fn hardware_gpu_bias_mut(&mut self) -> &mut f32;
    fn profile_directory_name(&self) -> &String;
    fn profile_directory_name_mut(&mut self) -> &mut String;
    fn resource_auto_job_thread(&self) -> &bool;
    fn resource_auto_job_thread_mut(&mut self) -> &mut bool;
    fn resource_auto_job_thread_affinity_mask(&self) -> &u32;
    fn resource_auto_job_thread_affinity_mask_mut(&mut self) -> &mut u32;
    fn enable_localization(&self) -> &bool;
    fn enable_localization_mut(&mut self) -> &mut bool;
    fn available_languages(&self) -> &String;
    fn available_languages_mut(&mut self) -> &mut String;
    fn enable_spike_packet_buffering(&self) -> &bool;
    fn enable_spike_packet_buffering_mut(&mut self) -> &mut bool;
    fn world_size(&self) -> &f32;
    fn world_size_mut(&mut self) -> &mut f32;
    fn stream_install_allow_play_from_bluray(&self) -> &StreamInstallFromBlurayType;
    fn stream_install_allow_play_from_bluray_mut(&mut self) -> &mut StreamInstallFromBlurayType;
    fn stream_install_enable_delayed_superbundle_mount(&self) -> &bool;
    fn stream_install_enable_delayed_superbundle_mount_mut(&mut self) -> &mut bool;
    fn stream_install_all_languages(&self) -> &bool;
    fn stream_install_all_languages_mut(&mut self) -> &mut bool;
    fn operational_context_data_keys(&self) -> &Vec<String>;
    fn operational_context_data_keys_mut(&mut self) -> &mut Vec<String>;
}

impl CoreSettingsTrait for CoreSettings {
    fn live_editing_enable(&self) -> &bool {
        &self.live_editing_enable
    }
    fn live_editing_enable_mut(&mut self) -> &mut bool {
        &mut self.live_editing_enable
    }
    fn disable_safe_state_for_refresh(&self) -> &bool {
        &self.disable_safe_state_for_refresh
    }
    fn disable_safe_state_for_refresh_mut(&mut self) -> &mut bool {
        &mut self.disable_safe_state_for_refresh
    }
    fn host(&self) -> &String {
        &self.host
    }
    fn host_mut(&mut self) -> &mut String {
        &mut self.host
    }
    fn host_user(&self) -> &String {
        &self.host_user
    }
    fn host_user_mut(&mut self) -> &mut String {
        &mut self.host_user
    }
    fn host_user_domain(&self) -> &String {
        &self.host_user_domain
    }
    fn host_user_domain_mut(&mut self) -> &mut String {
        &mut self.host_user_domain
    }
    fn init_seed(&self) -> &String {
        &self.init_seed
    }
    fn init_seed_mut(&mut self) -> &mut String {
        &mut self.init_seed
    }
    fn trace_host(&self) -> &String {
        &self.trace_host
    }
    fn trace_host_mut(&mut self) -> &mut String {
        &mut self.trace_host
    }
    fn forward_trace(&self) -> &bool {
        &self.forward_trace
    }
    fn forward_trace_mut(&mut self) -> &mut bool {
        &mut self.forward_trace
    }
    fn forward_trace_host(&self) -> &String {
        &self.forward_trace_host
    }
    fn forward_trace_host_mut(&mut self) -> &mut String {
        &mut self.forward_trace_host
    }
    fn forward_trace_port(&self) -> &i32 {
        &self.forward_trace_port
    }
    fn forward_trace_port_mut(&mut self) -> &mut i32 {
        &mut self.forward_trace_port
    }
    fn perf_track_enabled(&self) -> &bool {
        &self.perf_track_enabled
    }
    fn perf_track_enabled_mut(&mut self) -> &mut bool {
        &mut self.perf_track_enabled
    }
    fn log_level(&self) -> &CoreLogLevel {
        &self.log_level
    }
    fn log_level_mut(&mut self) -> &mut CoreLogLevel {
        &mut self.log_level
    }
    fn dialog_level(&self) -> &CoreLogLevel {
        &self.dialog_level
    }
    fn dialog_level_mut(&mut self) -> &mut CoreLogLevel {
        &mut self.dialog_level
    }
    fn user_log_enabled(&self) -> &bool {
        &self.user_log_enabled
    }
    fn user_log_enabled_mut(&mut self) -> &mut bool {
        &mut self.user_log_enabled
    }
    fn log_settings(&self) -> &LogSettings {
        &self.log_settings
    }
    fn log_settings_mut(&mut self) -> &mut LogSettings {
        &mut self.log_settings
    }
    fn allow_assign_vars_from_lua(&self) -> &bool {
        &self.allow_assign_vars_from_lua
    }
    fn allow_assign_vars_from_lua_mut(&mut self) -> &mut bool {
        &mut self.allow_assign_vars_from_lua
    }
    fn display_asserts(&self) -> &bool {
        &self.display_asserts
    }
    fn display_asserts_mut(&mut self) -> &mut bool {
        &mut self.display_asserts
    }
    fn debug_output_mode(&self) -> &u32 {
        &self.debug_output_mode
    }
    fn debug_output_mode_mut(&mut self) -> &mut u32 {
        &mut self.debug_output_mode
    }
    fn crash_on_fatal_errors(&self) -> &bool {
        &self.crash_on_fatal_errors
    }
    fn crash_on_fatal_errors_mut(&mut self) -> &mut bool {
        &mut self.crash_on_fatal_errors
    }
    fn enable_memory_snapshots(&self) -> &bool {
        &self.enable_memory_snapshots
    }
    fn enable_memory_snapshots_mut(&mut self) -> &mut bool {
        &mut self.enable_memory_snapshots
    }
    fn breakpad_mode(&self) -> &u32 {
        &self.breakpad_mode
    }
    fn breakpad_mode_mut(&mut self) -> &mut u32 {
        &mut self.breakpad_mode
    }
    fn random_time_seed(&self) -> &i32 {
        &self.random_time_seed
    }
    fn random_time_seed_mut(&mut self) -> &mut i32 {
        &mut self.random_time_seed
    }
    fn random_tick_seed(&self) -> &i32 {
        &self.random_tick_seed
    }
    fn random_tick_seed_mut(&mut self) -> &mut i32 {
        &mut self.random_tick_seed
    }
    fn random_session_id(&self) -> &i32 {
        &self.random_session_id
    }
    fn random_session_id_mut(&mut self) -> &mut i32 {
        &mut self.random_session_id
    }
    fn local_player_id_to_command(&self) -> &LocalPlayerId {
        &self.local_player_id_to_command
    }
    fn local_player_id_to_command_mut(&mut self) -> &mut LocalPlayerId {
        &mut self.local_player_id_to_command
    }
    fn game_configuration_name(&self) -> &String {
        &self.game_configuration_name
    }
    fn game_configuration_name_mut(&mut self) -> &mut String {
        &mut self.game_configuration_name
    }
    fn use_storage_server(&self) -> &bool {
        &self.use_storage_server
    }
    fn use_storage_server_mut(&mut self) -> &mut bool {
        &mut self.use_storage_server
    }
    fn hardware_profile(&self) -> &HardwareProfile {
        &self.hardware_profile
    }
    fn hardware_profile_mut(&mut self) -> &mut HardwareProfile {
        &mut self.hardware_profile
    }
    fn hardware_cpu_bias(&self) -> &f32 {
        &self.hardware_cpu_bias
    }
    fn hardware_cpu_bias_mut(&mut self) -> &mut f32 {
        &mut self.hardware_cpu_bias
    }
    fn hardware_gpu_bias(&self) -> &f32 {
        &self.hardware_gpu_bias
    }
    fn hardware_gpu_bias_mut(&mut self) -> &mut f32 {
        &mut self.hardware_gpu_bias
    }
    fn profile_directory_name(&self) -> &String {
        &self.profile_directory_name
    }
    fn profile_directory_name_mut(&mut self) -> &mut String {
        &mut self.profile_directory_name
    }
    fn resource_auto_job_thread(&self) -> &bool {
        &self.resource_auto_job_thread
    }
    fn resource_auto_job_thread_mut(&mut self) -> &mut bool {
        &mut self.resource_auto_job_thread
    }
    fn resource_auto_job_thread_affinity_mask(&self) -> &u32 {
        &self.resource_auto_job_thread_affinity_mask
    }
    fn resource_auto_job_thread_affinity_mask_mut(&mut self) -> &mut u32 {
        &mut self.resource_auto_job_thread_affinity_mask
    }
    fn enable_localization(&self) -> &bool {
        &self.enable_localization
    }
    fn enable_localization_mut(&mut self) -> &mut bool {
        &mut self.enable_localization
    }
    fn available_languages(&self) -> &String {
        &self.available_languages
    }
    fn available_languages_mut(&mut self) -> &mut String {
        &mut self.available_languages
    }
    fn enable_spike_packet_buffering(&self) -> &bool {
        &self.enable_spike_packet_buffering
    }
    fn enable_spike_packet_buffering_mut(&mut self) -> &mut bool {
        &mut self.enable_spike_packet_buffering
    }
    fn world_size(&self) -> &f32 {
        &self.world_size
    }
    fn world_size_mut(&mut self) -> &mut f32 {
        &mut self.world_size
    }
    fn stream_install_allow_play_from_bluray(&self) -> &StreamInstallFromBlurayType {
        &self.stream_install_allow_play_from_bluray
    }
    fn stream_install_allow_play_from_bluray_mut(&mut self) -> &mut StreamInstallFromBlurayType {
        &mut self.stream_install_allow_play_from_bluray
    }
    fn stream_install_enable_delayed_superbundle_mount(&self) -> &bool {
        &self.stream_install_enable_delayed_superbundle_mount
    }
    fn stream_install_enable_delayed_superbundle_mount_mut(&mut self) -> &mut bool {
        &mut self.stream_install_enable_delayed_superbundle_mount
    }
    fn stream_install_all_languages(&self) -> &bool {
        &self.stream_install_all_languages
    }
    fn stream_install_all_languages_mut(&mut self) -> &mut bool {
        &mut self.stream_install_all_languages
    }
    fn operational_context_data_keys(&self) -> &Vec<String> {
        &self.operational_context_data_keys
    }
    fn operational_context_data_keys_mut(&mut self) -> &mut Vec<String> {
        &mut self.operational_context_data_keys
    }
}

pub static CORESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreSettings",
    name_hash: 3087180827,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreSettings as Default>::default())),
            create_boxed: || Box::new(<CoreSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "LiveEditingEnable",
                name_hash: 3660493806,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, live_editing_enable),
            },
            FieldInfoData {
                name: "DisableSafeStateForRefresh",
                name_hash: 2983743153,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, disable_safe_state_for_refresh),
            },
            FieldInfoData {
                name: "Host",
                name_hash: 2089155077,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoreSettings, host),
            },
            FieldInfoData {
                name: "HostUser",
                name_hash: 3816077620,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoreSettings, host_user),
            },
            FieldInfoData {
                name: "HostUserDomain",
                name_hash: 2838452564,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoreSettings, host_user_domain),
            },
            FieldInfoData {
                name: "InitSeed",
                name_hash: 4105745672,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoreSettings, init_seed),
            },
            FieldInfoData {
                name: "TraceHost",
                name_hash: 1744445764,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoreSettings, trace_host),
            },
            FieldInfoData {
                name: "ForwardTrace",
                name_hash: 327696351,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, forward_trace),
            },
            FieldInfoData {
                name: "ForwardTraceHost",
                name_hash: 460409695,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoreSettings, forward_trace_host),
            },
            FieldInfoData {
                name: "ForwardTracePort",
                name_hash: 460105830,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CoreSettings, forward_trace_port),
            },
            FieldInfoData {
                name: "PerfTrackEnabled",
                name_hash: 560195790,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, perf_track_enabled),
            },
            FieldInfoData {
                name: "LogLevel",
                name_hash: 1867701815,
                flags: MemberInfoFlags::new(0),
                field_type: "CoreLogLevel",
                rust_offset: offset_of!(CoreSettings, log_level),
            },
            FieldInfoData {
                name: "DialogLevel",
                name_hash: 3297009691,
                flags: MemberInfoFlags::new(0),
                field_type: "CoreLogLevel",
                rust_offset: offset_of!(CoreSettings, dialog_level),
            },
            FieldInfoData {
                name: "UserLogEnabled",
                name_hash: 999413301,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, user_log_enabled),
            },
            FieldInfoData {
                name: "LogSettings",
                name_hash: 1204791236,
                flags: MemberInfoFlags::new(0),
                field_type: "LogSettings",
                rust_offset: offset_of!(CoreSettings, log_settings),
            },
            FieldInfoData {
                name: "AllowAssignVarsFromLua",
                name_hash: 1741206117,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, allow_assign_vars_from_lua),
            },
            FieldInfoData {
                name: "DisplayAsserts",
                name_hash: 1371733582,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, display_asserts),
            },
            FieldInfoData {
                name: "DebugOutputMode",
                name_hash: 2365706184,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(CoreSettings, debug_output_mode),
            },
            FieldInfoData {
                name: "CrashOnFatalErrors",
                name_hash: 3503810106,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, crash_on_fatal_errors),
            },
            FieldInfoData {
                name: "EnableMemorySnapshots",
                name_hash: 1830691450,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, enable_memory_snapshots),
            },
            FieldInfoData {
                name: "BreakpadMode",
                name_hash: 1741582860,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(CoreSettings, breakpad_mode),
            },
            FieldInfoData {
                name: "RandomTimeSeed",
                name_hash: 1358958012,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CoreSettings, random_time_seed),
            },
            FieldInfoData {
                name: "RandomTickSeed",
                name_hash: 671402876,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CoreSettings, random_tick_seed),
            },
            FieldInfoData {
                name: "RandomSessionId",
                name_hash: 95247661,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(CoreSettings, random_session_id),
            },
            FieldInfoData {
                name: "LocalPlayerIdToCommand",
                name_hash: 425545418,
                flags: MemberInfoFlags::new(0),
                field_type: "LocalPlayerId",
                rust_offset: offset_of!(CoreSettings, local_player_id_to_command),
            },
            FieldInfoData {
                name: "GameConfigurationName",
                name_hash: 2876236092,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoreSettings, game_configuration_name),
            },
            FieldInfoData {
                name: "UseStorageServer",
                name_hash: 3367165722,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, use_storage_server),
            },
            FieldInfoData {
                name: "HardwareProfile",
                name_hash: 1651739824,
                flags: MemberInfoFlags::new(0),
                field_type: "HardwareProfile",
                rust_offset: offset_of!(CoreSettings, hardware_profile),
            },
            FieldInfoData {
                name: "HardwareCpuBias",
                name_hash: 1750607332,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CoreSettings, hardware_cpu_bias),
            },
            FieldInfoData {
                name: "HardwareGpuBias",
                name_hash: 2612024544,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CoreSettings, hardware_gpu_bias),
            },
            FieldInfoData {
                name: "ProfileDirectoryName",
                name_hash: 615982176,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoreSettings, profile_directory_name),
            },
            FieldInfoData {
                name: "ResourceAutoJobThread",
                name_hash: 2182021673,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, resource_auto_job_thread),
            },
            FieldInfoData {
                name: "ResourceAutoJobThreadAffinityMask",
                name_hash: 3712905759,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(CoreSettings, resource_auto_job_thread_affinity_mask),
            },
            FieldInfoData {
                name: "EnableLocalization",
                name_hash: 984529671,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, enable_localization),
            },
            FieldInfoData {
                name: "AvailableLanguages",
                name_hash: 788680445,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CoreSettings, available_languages),
            },
            FieldInfoData {
                name: "EnableSpikePacketBuffering",
                name_hash: 183801544,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, enable_spike_packet_buffering),
            },
            FieldInfoData {
                name: "WorldSize",
                name_hash: 2492064770,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CoreSettings, world_size),
            },
            FieldInfoData {
                name: "StreamInstallAllowPlayFromBluray",
                name_hash: 963139010,
                flags: MemberInfoFlags::new(0),
                field_type: "StreamInstallFromBlurayType",
                rust_offset: offset_of!(CoreSettings, stream_install_allow_play_from_bluray),
            },
            FieldInfoData {
                name: "StreamInstallEnableDelayedSuperbundleMount",
                name_hash: 2200849781,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, stream_install_enable_delayed_superbundle_mount),
            },
            FieldInfoData {
                name: "StreamInstallAllLanguages",
                name_hash: 1116256248,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CoreSettings, stream_install_all_languages),
            },
            FieldInfoData {
                name: "OperationalContextDataKeys",
                name_hash: 4004052726,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static CORESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreSettings-Array",
    name_hash: 1294056111,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("CoreSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LogSettings {
    pub channels: Vec<BoxedTypeObject /* ChannelSettings */>,
    pub writers: Vec<BoxedTypeObject /* WriterSettings */>,
}

pub trait LogSettingsTrait: TypeObject {
    fn channels(&self) -> &Vec<BoxedTypeObject /* ChannelSettings */>;
    fn channels_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ChannelSettings */>;
    fn writers(&self) -> &Vec<BoxedTypeObject /* WriterSettings */>;
    fn writers_mut(&mut self) -> &mut Vec<BoxedTypeObject /* WriterSettings */>;
}

impl LogSettingsTrait for LogSettings {
    fn channels(&self) -> &Vec<BoxedTypeObject /* ChannelSettings */> {
        &self.channels
    }
    fn channels_mut(&mut self) -> &mut Vec<BoxedTypeObject /* ChannelSettings */> {
        &mut self.channels
    }
    fn writers(&self) -> &Vec<BoxedTypeObject /* WriterSettings */> {
        &self.writers
    }
    fn writers_mut(&mut self) -> &mut Vec<BoxedTypeObject /* WriterSettings */> {
        &mut self.writers
    }
}

pub static LOGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogSettings",
    name_hash: 1204791236,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LogSettings as Default>::default())),
            create_boxed: || Box::new(<LogSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Channels",
                name_hash: 1585412981,
                flags: MemberInfoFlags::new(144),
                field_type: "ChannelSettings-Array",
                rust_offset: offset_of!(LogSettings, channels),
            },
            FieldInfoData {
                name: "Writers",
                name_hash: 1241823609,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static LOGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogSettings-Array",
    name_hash: 788581488,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("LogSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WriterSettings {
    pub id: String,
    pub class: String,
    pub params: String,
}

pub trait WriterSettingsTrait: TypeObject {
    fn id(&self) -> &String;
    fn id_mut(&mut self) -> &mut String;
    fn class(&self) -> &String;
    fn class_mut(&mut self) -> &mut String;
    fn params(&self) -> &String;
    fn params_mut(&mut self) -> &mut String;
}

impl WriterSettingsTrait for WriterSettings {
    fn id(&self) -> &String {
        &self.id
    }
    fn id_mut(&mut self) -> &mut String {
        &mut self.id
    }
    fn class(&self) -> &String {
        &self.class
    }
    fn class_mut(&mut self) -> &mut String {
        &mut self.class
    }
    fn params(&self) -> &String {
        &self.params
    }
    fn params_mut(&mut self) -> &mut String {
        &mut self.params
    }
}

pub static WRITERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WriterSettings",
    name_hash: 3362836047,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WriterSettings as Default>::default())),
            create_boxed: || Box::new(<WriterSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Id",
                name_hash: 5862152,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WriterSettings, id),
            },
            FieldInfoData {
                name: "Class",
                name_hash: 212635851,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WriterSettings, class),
            },
            FieldInfoData {
                name: "Params",
                name_hash: 3371566681,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static WRITERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WriterSettings-Array",
    name_hash: 217267067,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("WriterSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ChannelSettings {
    pub id: String,
    pub inherits_level: bool,
    pub inherits_writers: bool,
    pub level: CoreLogLevel,
    pub writers: Vec<String>,
}

pub trait ChannelSettingsTrait: TypeObject {
    fn id(&self) -> &String;
    fn id_mut(&mut self) -> &mut String;
    fn inherits_level(&self) -> &bool;
    fn inherits_level_mut(&mut self) -> &mut bool;
    fn inherits_writers(&self) -> &bool;
    fn inherits_writers_mut(&mut self) -> &mut bool;
    fn level(&self) -> &CoreLogLevel;
    fn level_mut(&mut self) -> &mut CoreLogLevel;
    fn writers(&self) -> &Vec<String>;
    fn writers_mut(&mut self) -> &mut Vec<String>;
}

impl ChannelSettingsTrait for ChannelSettings {
    fn id(&self) -> &String {
        &self.id
    }
    fn id_mut(&mut self) -> &mut String {
        &mut self.id
    }
    fn inherits_level(&self) -> &bool {
        &self.inherits_level
    }
    fn inherits_level_mut(&mut self) -> &mut bool {
        &mut self.inherits_level
    }
    fn inherits_writers(&self) -> &bool {
        &self.inherits_writers
    }
    fn inherits_writers_mut(&mut self) -> &mut bool {
        &mut self.inherits_writers
    }
    fn level(&self) -> &CoreLogLevel {
        &self.level
    }
    fn level_mut(&mut self) -> &mut CoreLogLevel {
        &mut self.level
    }
    fn writers(&self) -> &Vec<String> {
        &self.writers
    }
    fn writers_mut(&mut self) -> &mut Vec<String> {
        &mut self.writers
    }
}

pub static CHANNELSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChannelSettings",
    name_hash: 2890218179,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ChannelSettings as Default>::default())),
            create_boxed: || Box::new(<ChannelSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Id",
                name_hash: 5862152,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ChannelSettings, id),
            },
            FieldInfoData {
                name: "InheritsLevel",
                name_hash: 4271479909,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ChannelSettings, inherits_level),
            },
            FieldInfoData {
                name: "InheritsWriters",
                name_hash: 2474988527,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ChannelSettings, inherits_writers),
            },
            FieldInfoData {
                name: "Level",
                name_hash: 218262515,
                flags: MemberInfoFlags::new(0),
                field_type: "CoreLogLevel",
                rust_offset: offset_of!(ChannelSettings, level),
            },
            FieldInfoData {
                name: "Writers",
                name_hash: 1241823609,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static CHANNELSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChannelSettings-Array",
    name_hash: 717749751,
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
    name_hash: 3964114759,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static STREAMINSTALLFROMBLURAYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallFromBlurayType-Array",
    name_hash: 1887266931,
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
    name_hash: 1651739824,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static HARDWAREPROFILE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HardwareProfile-Array",
    name_hash: 3386620420,
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
    name_hash: 2867831724,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static CORELOGLEVEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreLogLevel-Array",
    name_hash: 2398704408,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("CoreLogLevel"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ContentPreset {
    pub _glacier_base: DataContainer,
    pub content_name: String,
    pub content_type: String,
    pub content_parameters: Option<LockedTypeObject /* DataContainer */>,
}

pub trait ContentPresetTrait: DataContainerTrait {
    fn content_name(&self) -> &String;
    fn content_name_mut(&mut self) -> &mut String;
    fn content_type(&self) -> &String;
    fn content_type_mut(&mut self) -> &mut String;
    fn content_parameters(&self) -> &Option<LockedTypeObject /* DataContainer */>;
    fn content_parameters_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */>;
}

impl ContentPresetTrait for ContentPreset {
    fn content_name(&self) -> &String {
        &self.content_name
    }
    fn content_name_mut(&mut self) -> &mut String {
        &mut self.content_name
    }
    fn content_type(&self) -> &String {
        &self.content_type
    }
    fn content_type_mut(&mut self) -> &mut String {
        &mut self.content_type
    }
    fn content_parameters(&self) -> &Option<LockedTypeObject /* DataContainer */> {
        &self.content_parameters
    }
    fn content_parameters_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */> {
        &mut self.content_parameters
    }
}

impl DataContainerTrait for ContentPreset {
}

pub static CONTENTPRESET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ContentPreset",
    name_hash: 2096307881,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(ContentPreset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ContentPreset as Default>::default())),
            create_boxed: || Box::new(<ContentPreset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ContentName",
                name_hash: 4236678091,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ContentPreset, content_name),
            },
            FieldInfoData {
                name: "ContentType",
                name_hash: 4237582196,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ContentPreset, content_type),
            },
            FieldInfoData {
                name: "ContentParameters",
                name_hash: 1952600534,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONTENTPRESET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ContentPreset-Array",
    name_hash: 2248451357,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ContentPreset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ContentPresetContainer {
    pub _glacier_base: Asset,
    pub presets: Vec<Option<LockedTypeObject /* ContentPreset */>>,
}

pub trait ContentPresetContainerTrait: AssetTrait {
    fn presets(&self) -> &Vec<Option<LockedTypeObject /* ContentPreset */>>;
    fn presets_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ContentPreset */>>;
}

impl ContentPresetContainerTrait for ContentPresetContainer {
    fn presets(&self) -> &Vec<Option<LockedTypeObject /* ContentPreset */>> {
        &self.presets
    }
    fn presets_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ContentPreset */>> {
        &mut self.presets
    }
}

impl AssetTrait for ContentPresetContainer {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl DataContainerTrait for ContentPresetContainer {
}

pub static CONTENTPRESETCONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ContentPresetContainer",
    name_hash: 281552686,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        super_class_offset: offset_of!(ContentPresetContainer, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ContentPresetContainer as Default>::default())),
            create_boxed: || Box::new(<ContentPresetContainer as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Presets",
                name_hash: 3463460435,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONTENTPRESETCONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ContentPresetContainer-Array",
    name_hash: 457109402,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ContentPresetContainer"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TimeSpan {
    pub internal: i64,
}

pub trait TimeSpanTrait: TypeObject {
    fn internal(&self) -> &i64;
    fn internal_mut(&mut self) -> &mut i64;
}

impl TimeSpanTrait for TimeSpan {
    fn internal(&self) -> &i64 {
        &self.internal
    }
    fn internal_mut(&mut self) -> &mut i64 {
        &mut self.internal
    }
}

pub static TIMESPAN_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimeSpan",
    name_hash: 2998612924,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TimeSpan as Default>::default())),
            create_boxed: || Box::new(<TimeSpan as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Internal",
                name_hash: 4280094690,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static TIMESPAN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimeSpan-Array",
    name_hash: 2495851272,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TimeSpan"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ThreadSettings {
    pub _glacier_base: DataContainer,
    pub processor_count: i32,
    pub max_live_edit_processor_count: i32,
    pub job_thread_priority: i32,
    pub free_processor_count: i32,
}

pub trait ThreadSettingsTrait: DataContainerTrait {
    fn processor_count(&self) -> &i32;
    fn processor_count_mut(&mut self) -> &mut i32;
    fn max_live_edit_processor_count(&self) -> &i32;
    fn max_live_edit_processor_count_mut(&mut self) -> &mut i32;
    fn job_thread_priority(&self) -> &i32;
    fn job_thread_priority_mut(&mut self) -> &mut i32;
    fn free_processor_count(&self) -> &i32;
    fn free_processor_count_mut(&mut self) -> &mut i32;
}

impl ThreadSettingsTrait for ThreadSettings {
    fn processor_count(&self) -> &i32 {
        &self.processor_count
    }
    fn processor_count_mut(&mut self) -> &mut i32 {
        &mut self.processor_count
    }
    fn max_live_edit_processor_count(&self) -> &i32 {
        &self.max_live_edit_processor_count
    }
    fn max_live_edit_processor_count_mut(&mut self) -> &mut i32 {
        &mut self.max_live_edit_processor_count
    }
    fn job_thread_priority(&self) -> &i32 {
        &self.job_thread_priority
    }
    fn job_thread_priority_mut(&mut self) -> &mut i32 {
        &mut self.job_thread_priority
    }
    fn free_processor_count(&self) -> &i32 {
        &self.free_processor_count
    }
    fn free_processor_count_mut(&mut self) -> &mut i32 {
        &mut self.free_processor_count
    }
}

impl DataContainerTrait for ThreadSettings {
}

pub static THREADSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ThreadSettings",
    name_hash: 2011850126,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(ThreadSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ThreadSettings as Default>::default())),
            create_boxed: || Box::new(<ThreadSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ProcessorCount",
                name_hash: 391539216,
                flags: MemberInfoFlags::new(8192),
                field_type: "Int32",
                rust_offset: offset_of!(ThreadSettings, processor_count),
            },
            FieldInfoData {
                name: "MaxLiveEditProcessorCount",
                name_hash: 1778801038,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ThreadSettings, max_live_edit_processor_count),
            },
            FieldInfoData {
                name: "JobThreadPriority",
                name_hash: 4057060638,
                flags: MemberInfoFlags::new(8192),
                field_type: "Int32",
                rust_offset: offset_of!(ThreadSettings, job_thread_priority),
            },
            FieldInfoData {
                name: "FreeProcessorCount",
                name_hash: 2871723972,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static THREADSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ThreadSettings-Array",
    name_hash: 3632950970,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ThreadSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TemplateDescriptorRegistryAsset {
    pub _glacier_base: Asset,
    pub descriptors: Vec<Option<LockedTypeObject /* TemplateDescriptorData */>>,
}

pub trait TemplateDescriptorRegistryAssetTrait: AssetTrait {
    fn descriptors(&self) -> &Vec<Option<LockedTypeObject /* TemplateDescriptorData */>>;
    fn descriptors_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TemplateDescriptorData */>>;
}

impl TemplateDescriptorRegistryAssetTrait for TemplateDescriptorRegistryAsset {
    fn descriptors(&self) -> &Vec<Option<LockedTypeObject /* TemplateDescriptorData */>> {
        &self.descriptors
    }
    fn descriptors_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TemplateDescriptorData */>> {
        &mut self.descriptors
    }
}

impl AssetTrait for TemplateDescriptorRegistryAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl DataContainerTrait for TemplateDescriptorRegistryAsset {
}

pub static TEMPLATEDESCRIPTORREGISTRYASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateDescriptorRegistryAsset",
    name_hash: 3962082979,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        super_class_offset: offset_of!(TemplateDescriptorRegistryAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TemplateDescriptorRegistryAsset as Default>::default())),
            create_boxed: || Box::new(<TemplateDescriptorRegistryAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Descriptors",
                name_hash: 1636671237,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TEMPLATEDESCRIPTORREGISTRYASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateDescriptorRegistryAsset-Array",
    name_hash: 1831680535,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TemplateDescriptorRegistryAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TemplateDescriptorData {
    pub _glacier_base: DataContainer,
    pub target_asset: Option<LockedTypeObject /* Asset */>,
    pub exposed_objects: Vec<BoxedTypeObject /* TemplateExposedObject */>,
}

pub trait TemplateDescriptorDataTrait: DataContainerTrait {
    fn target_asset(&self) -> &Option<LockedTypeObject /* Asset */>;
    fn target_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* Asset */>;
    fn exposed_objects(&self) -> &Vec<BoxedTypeObject /* TemplateExposedObject */>;
    fn exposed_objects_mut(&mut self) -> &mut Vec<BoxedTypeObject /* TemplateExposedObject */>;
}

impl TemplateDescriptorDataTrait for TemplateDescriptorData {
    fn target_asset(&self) -> &Option<LockedTypeObject /* Asset */> {
        &self.target_asset
    }
    fn target_asset_mut(&mut self) -> &mut Option<LockedTypeObject /* Asset */> {
        &mut self.target_asset
    }
    fn exposed_objects(&self) -> &Vec<BoxedTypeObject /* TemplateExposedObject */> {
        &self.exposed_objects
    }
    fn exposed_objects_mut(&mut self) -> &mut Vec<BoxedTypeObject /* TemplateExposedObject */> {
        &mut self.exposed_objects
    }
}

impl DataContainerTrait for TemplateDescriptorData {
}

pub static TEMPLATEDESCRIPTORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateDescriptorData",
    name_hash: 1165915222,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(TemplateDescriptorData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TemplateDescriptorData as Default>::default())),
            create_boxed: || Box::new(<TemplateDescriptorData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TargetAsset",
                name_hash: 2259053956,
                flags: MemberInfoFlags::new(0),
                field_type: "Asset",
                rust_offset: offset_of!(TemplateDescriptorData, target_asset),
            },
            FieldInfoData {
                name: "ExposedObjects",
                name_hash: 72905843,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TEMPLATEDESCRIPTORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateDescriptorData-Array",
    name_hash: 2256681442,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TemplateDescriptorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TemplateExposedObject {
    pub object: Option<LockedTypeObject /* DataContainer */>,
    pub object_ref: glacier_util::guid::Guid,
}

pub trait TemplateExposedObjectTrait: TypeObject {
    fn object(&self) -> &Option<LockedTypeObject /* DataContainer */>;
    fn object_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */>;
    fn object_ref(&self) -> &glacier_util::guid::Guid;
    fn object_ref_mut(&mut self) -> &mut glacier_util::guid::Guid;
}

impl TemplateExposedObjectTrait for TemplateExposedObject {
    fn object(&self) -> &Option<LockedTypeObject /* DataContainer */> {
        &self.object
    }
    fn object_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */> {
        &mut self.object
    }
    fn object_ref(&self) -> &glacier_util::guid::Guid {
        &self.object_ref
    }
    fn object_ref_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.object_ref
    }
}

pub static TEMPLATEEXPOSEDOBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateExposedObject",
    name_hash: 1374653008,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TemplateExposedObject as Default>::default())),
            create_boxed: || Box::new(<TemplateExposedObject as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Object",
                name_hash: 2866508144,
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(TemplateExposedObject, object),
            },
            FieldInfoData {
                name: "ObjectRef",
                name_hash: 3207455617,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static TEMPLATEEXPOSEDOBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateExposedObject-Array",
    name_hash: 695445732,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TemplateExposedObject"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TemplateAsset {
    pub _glacier_base: Asset,
    pub objects: Vec<BoxedTypeObject /* TemplateTarget */>,
}

pub trait TemplateAssetTrait: AssetTrait {
    fn objects(&self) -> &Vec<BoxedTypeObject /* TemplateTarget */>;
    fn objects_mut(&mut self) -> &mut Vec<BoxedTypeObject /* TemplateTarget */>;
}

impl TemplateAssetTrait for TemplateAsset {
    fn objects(&self) -> &Vec<BoxedTypeObject /* TemplateTarget */> {
        &self.objects
    }
    fn objects_mut(&mut self) -> &mut Vec<BoxedTypeObject /* TemplateTarget */> {
        &mut self.objects
    }
}

impl AssetTrait for TemplateAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl DataContainerTrait for TemplateAsset {
}

pub static TEMPLATEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateAsset",
    name_hash: 764778533,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        super_class_offset: offset_of!(TemplateAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TemplateAsset as Default>::default())),
            create_boxed: || Box::new(<TemplateAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Objects",
                name_hash: 105488131,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TEMPLATEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateAsset-Array",
    name_hash: 1253871761,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TemplateAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TemplateTarget {
    pub target_ref: glacier_util::guid::Guid,
    pub template: Option<LockedTypeObject /* Template */>,
}

pub trait TemplateTargetTrait: TypeObject {
    fn target_ref(&self) -> &glacier_util::guid::Guid;
    fn target_ref_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn template(&self) -> &Option<LockedTypeObject /* Template */>;
    fn template_mut(&mut self) -> &mut Option<LockedTypeObject /* Template */>;
}

impl TemplateTargetTrait for TemplateTarget {
    fn target_ref(&self) -> &glacier_util::guid::Guid {
        &self.target_ref
    }
    fn target_ref_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.target_ref
    }
    fn template(&self) -> &Option<LockedTypeObject /* Template */> {
        &self.template
    }
    fn template_mut(&mut self) -> &mut Option<LockedTypeObject /* Template */> {
        &mut self.template
    }
}

pub static TEMPLATETARGET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateTarget",
    name_hash: 2957944708,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TemplateTarget as Default>::default())),
            create_boxed: || Box::new(<TemplateTarget as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TargetRef",
                name_hash: 3654158949,
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(TemplateTarget, target_ref),
            },
            FieldInfoData {
                name: "Template",
                name_hash: 2427043285,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static TEMPLATETARGET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateTarget-Array",
    name_hash: 1936497584,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TemplateTarget"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Template {
    pub _glacier_base: DataContainer,
}

pub trait TemplateTrait: DataContainerTrait {
}

impl TemplateTrait for Template {
}

impl DataContainerTrait for Template {
}

pub static TEMPLATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Template",
    name_hash: 2427043285,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(Template, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Template as Default>::default())),
            create_boxed: || Box::new(<Template as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TEMPLATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Template-Array",
    name_hash: 778632673,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Template"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn platform_mut(&mut self) -> &mut GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl DataContainerTrait for SuperbundleLayoutSettings {
}

pub static SUPERBUNDLELAYOUTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SuperbundleLayoutSettings",
    name_hash: 3897452687,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(SuperbundleLayoutSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SuperbundleLayoutSettings as Default>::default())),
            create_boxed: || Box::new(<SuperbundleLayoutSettings as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SUPERBUNDLELAYOUTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SuperbundleLayoutSettings-Array",
    name_hash: 2935735611,
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
    name_hash: 336125989,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static INSTALLPACKAGELAYERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InstallPackageLayerType-Array",
    name_hash: 1552275601,
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
    name_hash: 1141667238,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static INSTALLPACKAGETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InstallPackageType-Array",
    name_hash: 715440658,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("InstallPackageType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn default_mut(&mut self) -> &mut bool;
    fn xenon(&self) -> &bool;
    fn xenon_mut(&mut self) -> &mut bool;
    fn ps3(&self) -> &bool;
    fn ps3_mut(&mut self) -> &mut bool;
    fn gen4a(&self) -> &bool;
    fn gen4a_mut(&mut self) -> &mut bool;
    fn gen4b(&self) -> &bool;
    fn gen4b_mut(&mut self) -> &mut bool;
    fn android(&self) -> &bool;
    fn android_mut(&mut self) -> &mut bool;
    fn i_o_s(&self) -> &bool;
    fn i_o_s_mut(&mut self) -> &mut bool;
    fn o_s_x(&self) -> &bool;
    fn o_s_x_mut(&mut self) -> &mut bool;
    fn linux(&self) -> &bool;
    fn linux_mut(&mut self) -> &mut bool;
}

impl PlatformScalableBoolTrait for PlatformScalableBool {
    fn default(&self) -> &bool {
        &self.default
    }
    fn default_mut(&mut self) -> &mut bool {
        &mut self.default
    }
    fn xenon(&self) -> &bool {
        &self.xenon
    }
    fn xenon_mut(&mut self) -> &mut bool {
        &mut self.xenon
    }
    fn ps3(&self) -> &bool {
        &self.ps3
    }
    fn ps3_mut(&mut self) -> &mut bool {
        &mut self.ps3
    }
    fn gen4a(&self) -> &bool {
        &self.gen4a
    }
    fn gen4a_mut(&mut self) -> &mut bool {
        &mut self.gen4a
    }
    fn gen4b(&self) -> &bool {
        &self.gen4b
    }
    fn gen4b_mut(&mut self) -> &mut bool {
        &mut self.gen4b
    }
    fn android(&self) -> &bool {
        &self.android
    }
    fn android_mut(&mut self) -> &mut bool {
        &mut self.android
    }
    fn i_o_s(&self) -> &bool {
        &self.i_o_s
    }
    fn i_o_s_mut(&mut self) -> &mut bool {
        &mut self.i_o_s
    }
    fn o_s_x(&self) -> &bool {
        &self.o_s_x
    }
    fn o_s_x_mut(&mut self) -> &mut bool {
        &mut self.o_s_x
    }
    fn linux(&self) -> &bool {
        &self.linux
    }
    fn linux_mut(&mut self) -> &mut bool {
        &mut self.linux
    }
}

pub static PLATFORMSCALABLEBOOL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformScalableBool",
    name_hash: 201139331,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlatformScalableBool as Default>::default())),
            create_boxed: || Box::new(<PlatformScalableBool as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Default",
                name_hash: 3998752238,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlatformScalableBool, default),
            },
            FieldInfoData {
                name: "Xenon",
                name_hash: 241675831,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlatformScalableBool, xenon),
            },
            FieldInfoData {
                name: "Ps3",
                name_hash: 193466773,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlatformScalableBool, ps3),
            },
            FieldInfoData {
                name: "Gen4a",
                name_hash: 207859612,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlatformScalableBool, gen4a),
            },
            FieldInfoData {
                name: "Gen4b",
                name_hash: 207859615,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlatformScalableBool, gen4b),
            },
            FieldInfoData {
                name: "Android",
                name_hash: 557715870,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlatformScalableBool, android),
            },
            FieldInfoData {
                name: "iOS",
                name_hash: 193417008,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlatformScalableBool, i_o_s),
            },
            FieldInfoData {
                name: "OSX",
                name_hash: 193454785,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlatformScalableBool, o_s_x),
            },
            FieldInfoData {
                name: "Linux",
                name_hash: 217831523,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static PLATFORMSCALABLEBOOL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformScalableBool-Array",
    name_hash: 3188225847,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PlatformScalableBool"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn default_mut(&mut self) -> &mut f32;
    fn xenon(&self) -> &f32;
    fn xenon_mut(&mut self) -> &mut f32;
    fn ps3(&self) -> &f32;
    fn ps3_mut(&mut self) -> &mut f32;
    fn gen4a(&self) -> &f32;
    fn gen4a_mut(&mut self) -> &mut f32;
    fn gen4b(&self) -> &f32;
    fn gen4b_mut(&mut self) -> &mut f32;
    fn android(&self) -> &f32;
    fn android_mut(&mut self) -> &mut f32;
    fn i_o_s(&self) -> &f32;
    fn i_o_s_mut(&mut self) -> &mut f32;
    fn o_s_x(&self) -> &f32;
    fn o_s_x_mut(&mut self) -> &mut f32;
    fn linux(&self) -> &f32;
    fn linux_mut(&mut self) -> &mut f32;
}

impl PlatformScalableFloatTrait for PlatformScalableFloat {
    fn default(&self) -> &f32 {
        &self.default
    }
    fn default_mut(&mut self) -> &mut f32 {
        &mut self.default
    }
    fn xenon(&self) -> &f32 {
        &self.xenon
    }
    fn xenon_mut(&mut self) -> &mut f32 {
        &mut self.xenon
    }
    fn ps3(&self) -> &f32 {
        &self.ps3
    }
    fn ps3_mut(&mut self) -> &mut f32 {
        &mut self.ps3
    }
    fn gen4a(&self) -> &f32 {
        &self.gen4a
    }
    fn gen4a_mut(&mut self) -> &mut f32 {
        &mut self.gen4a
    }
    fn gen4b(&self) -> &f32 {
        &self.gen4b
    }
    fn gen4b_mut(&mut self) -> &mut f32 {
        &mut self.gen4b
    }
    fn android(&self) -> &f32 {
        &self.android
    }
    fn android_mut(&mut self) -> &mut f32 {
        &mut self.android
    }
    fn i_o_s(&self) -> &f32 {
        &self.i_o_s
    }
    fn i_o_s_mut(&mut self) -> &mut f32 {
        &mut self.i_o_s
    }
    fn o_s_x(&self) -> &f32 {
        &self.o_s_x
    }
    fn o_s_x_mut(&mut self) -> &mut f32 {
        &mut self.o_s_x
    }
    fn linux(&self) -> &f32 {
        &self.linux
    }
    fn linux_mut(&mut self) -> &mut f32 {
        &mut self.linux
    }
}

pub static PLATFORMSCALABLEFLOAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformScalableFloat",
    name_hash: 2338341277,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlatformScalableFloat as Default>::default())),
            create_boxed: || Box::new(<PlatformScalableFloat as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Default",
                name_hash: 3998752238,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlatformScalableFloat, default),
            },
            FieldInfoData {
                name: "Xenon",
                name_hash: 241675831,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlatformScalableFloat, xenon),
            },
            FieldInfoData {
                name: "Ps3",
                name_hash: 193466773,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlatformScalableFloat, ps3),
            },
            FieldInfoData {
                name: "Gen4a",
                name_hash: 207859612,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlatformScalableFloat, gen4a),
            },
            FieldInfoData {
                name: "Gen4b",
                name_hash: 207859615,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlatformScalableFloat, gen4b),
            },
            FieldInfoData {
                name: "Android",
                name_hash: 557715870,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlatformScalableFloat, android),
            },
            FieldInfoData {
                name: "iOS",
                name_hash: 193417008,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlatformScalableFloat, i_o_s),
            },
            FieldInfoData {
                name: "OSX",
                name_hash: 193454785,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PlatformScalableFloat, o_s_x),
            },
            FieldInfoData {
                name: "Linux",
                name_hash: 217831523,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static PLATFORMSCALABLEFLOAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformScalableFloat-Array",
    name_hash: 3987123497,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PlatformScalableFloat"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn default_mut(&mut self) -> &mut i32;
    fn xenon(&self) -> &i32;
    fn xenon_mut(&mut self) -> &mut i32;
    fn ps3(&self) -> &i32;
    fn ps3_mut(&mut self) -> &mut i32;
    fn gen4a(&self) -> &i32;
    fn gen4a_mut(&mut self) -> &mut i32;
    fn gen4b(&self) -> &i32;
    fn gen4b_mut(&mut self) -> &mut i32;
    fn android(&self) -> &i32;
    fn android_mut(&mut self) -> &mut i32;
    fn i_o_s(&self) -> &i32;
    fn i_o_s_mut(&mut self) -> &mut i32;
    fn o_s_x(&self) -> &i32;
    fn o_s_x_mut(&mut self) -> &mut i32;
    fn linux(&self) -> &i32;
    fn linux_mut(&mut self) -> &mut i32;
}

impl PlatformScalableIntTrait for PlatformScalableInt {
    fn default(&self) -> &i32 {
        &self.default
    }
    fn default_mut(&mut self) -> &mut i32 {
        &mut self.default
    }
    fn xenon(&self) -> &i32 {
        &self.xenon
    }
    fn xenon_mut(&mut self) -> &mut i32 {
        &mut self.xenon
    }
    fn ps3(&self) -> &i32 {
        &self.ps3
    }
    fn ps3_mut(&mut self) -> &mut i32 {
        &mut self.ps3
    }
    fn gen4a(&self) -> &i32 {
        &self.gen4a
    }
    fn gen4a_mut(&mut self) -> &mut i32 {
        &mut self.gen4a
    }
    fn gen4b(&self) -> &i32 {
        &self.gen4b
    }
    fn gen4b_mut(&mut self) -> &mut i32 {
        &mut self.gen4b
    }
    fn android(&self) -> &i32 {
        &self.android
    }
    fn android_mut(&mut self) -> &mut i32 {
        &mut self.android
    }
    fn i_o_s(&self) -> &i32 {
        &self.i_o_s
    }
    fn i_o_s_mut(&mut self) -> &mut i32 {
        &mut self.i_o_s
    }
    fn o_s_x(&self) -> &i32 {
        &self.o_s_x
    }
    fn o_s_x_mut(&mut self) -> &mut i32 {
        &mut self.o_s_x
    }
    fn linux(&self) -> &i32 {
        &self.linux
    }
    fn linux_mut(&mut self) -> &mut i32 {
        &mut self.linux
    }
}

pub static PLATFORMSCALABLEINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformScalableInt",
    name_hash: 526684030,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlatformScalableInt as Default>::default())),
            create_boxed: || Box::new(<PlatformScalableInt as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Default",
                name_hash: 3998752238,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PlatformScalableInt, default),
            },
            FieldInfoData {
                name: "Xenon",
                name_hash: 241675831,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PlatformScalableInt, xenon),
            },
            FieldInfoData {
                name: "Ps3",
                name_hash: 193466773,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PlatformScalableInt, ps3),
            },
            FieldInfoData {
                name: "Gen4a",
                name_hash: 207859612,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PlatformScalableInt, gen4a),
            },
            FieldInfoData {
                name: "Gen4b",
                name_hash: 207859615,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PlatformScalableInt, gen4b),
            },
            FieldInfoData {
                name: "Android",
                name_hash: 557715870,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PlatformScalableInt, android),
            },
            FieldInfoData {
                name: "iOS",
                name_hash: 193417008,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PlatformScalableInt, i_o_s),
            },
            FieldInfoData {
                name: "OSX",
                name_hash: 193454785,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PlatformScalableInt, o_s_x),
            },
            FieldInfoData {
                name: "Linux",
                name_hash: 217831523,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static PLATFORMSCALABLEINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformScalableInt-Array",
    name_hash: 1549471818,
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
    name_hash: 164574298,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static QUALITYSCALABLEENABLED_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableEnabled-Array",
    name_hash: 362981358,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("QualityScalableEnabled"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct QualityScalableBool {
    pub low: bool,
    pub medium: bool,
    pub high: bool,
    pub ultra: bool,
    pub cinematic: bool,
}

pub trait QualityScalableBoolTrait: TypeObject {
    fn low(&self) -> &bool;
    fn low_mut(&mut self) -> &mut bool;
    fn medium(&self) -> &bool;
    fn medium_mut(&mut self) -> &mut bool;
    fn high(&self) -> &bool;
    fn high_mut(&mut self) -> &mut bool;
    fn ultra(&self) -> &bool;
    fn ultra_mut(&mut self) -> &mut bool;
    fn cinematic(&self) -> &bool;
    fn cinematic_mut(&mut self) -> &mut bool;
}

impl QualityScalableBoolTrait for QualityScalableBool {
    fn low(&self) -> &bool {
        &self.low
    }
    fn low_mut(&mut self) -> &mut bool {
        &mut self.low
    }
    fn medium(&self) -> &bool {
        &self.medium
    }
    fn medium_mut(&mut self) -> &mut bool {
        &mut self.medium
    }
    fn high(&self) -> &bool {
        &self.high
    }
    fn high_mut(&mut self) -> &mut bool {
        &mut self.high
    }
    fn ultra(&self) -> &bool {
        &self.ultra
    }
    fn ultra_mut(&mut self) -> &mut bool {
        &mut self.ultra
    }
    fn cinematic(&self) -> &bool {
        &self.cinematic
    }
    fn cinematic_mut(&mut self) -> &mut bool {
        &mut self.cinematic
    }
}

pub static QUALITYSCALABLEBOOL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableBool",
    name_hash: 2708592209,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QualityScalableBool as Default>::default())),
            create_boxed: || Box::new(<QualityScalableBool as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Low",
                name_hash: 193454161,
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(QualityScalableBool, low),
            },
            FieldInfoData {
                name: "Medium",
                name_hash: 2647104632,
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(QualityScalableBool, medium),
            },
            FieldInfoData {
                name: "High",
                name_hash: 2089152523,
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(QualityScalableBool, high),
            },
            FieldInfoData {
                name: "Ultra",
                name_hash: 220157755,
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(QualityScalableBool, ultra),
            },
            FieldInfoData {
                name: "Cinematic",
                name_hash: 4097794678,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static QUALITYSCALABLEBOOL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableBool-Array",
    name_hash: 2260846693,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("QualityScalableBool"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct QualityScalableFloat {
    pub low: f32,
    pub medium: f32,
    pub high: f32,
    pub ultra: f32,
    pub cinematic: f32,
}

pub trait QualityScalableFloatTrait: TypeObject {
    fn low(&self) -> &f32;
    fn low_mut(&mut self) -> &mut f32;
    fn medium(&self) -> &f32;
    fn medium_mut(&mut self) -> &mut f32;
    fn high(&self) -> &f32;
    fn high_mut(&mut self) -> &mut f32;
    fn ultra(&self) -> &f32;
    fn ultra_mut(&mut self) -> &mut f32;
    fn cinematic(&self) -> &f32;
    fn cinematic_mut(&mut self) -> &mut f32;
}

impl QualityScalableFloatTrait for QualityScalableFloat {
    fn low(&self) -> &f32 {
        &self.low
    }
    fn low_mut(&mut self) -> &mut f32 {
        &mut self.low
    }
    fn medium(&self) -> &f32 {
        &self.medium
    }
    fn medium_mut(&mut self) -> &mut f32 {
        &mut self.medium
    }
    fn high(&self) -> &f32 {
        &self.high
    }
    fn high_mut(&mut self) -> &mut f32 {
        &mut self.high
    }
    fn ultra(&self) -> &f32 {
        &self.ultra
    }
    fn ultra_mut(&mut self) -> &mut f32 {
        &mut self.ultra
    }
    fn cinematic(&self) -> &f32 {
        &self.cinematic
    }
    fn cinematic_mut(&mut self) -> &mut f32 {
        &mut self.cinematic
    }
}

pub static QUALITYSCALABLEFLOAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableFloat",
    name_hash: 3479764111,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QualityScalableFloat as Default>::default())),
            create_boxed: || Box::new(<QualityScalableFloat as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Low",
                name_hash: 193454161,
                flags: MemberInfoFlags::new(8192),
                field_type: "Float32",
                rust_offset: offset_of!(QualityScalableFloat, low),
            },
            FieldInfoData {
                name: "Medium",
                name_hash: 2647104632,
                flags: MemberInfoFlags::new(8192),
                field_type: "Float32",
                rust_offset: offset_of!(QualityScalableFloat, medium),
            },
            FieldInfoData {
                name: "High",
                name_hash: 2089152523,
                flags: MemberInfoFlags::new(8192),
                field_type: "Float32",
                rust_offset: offset_of!(QualityScalableFloat, high),
            },
            FieldInfoData {
                name: "Ultra",
                name_hash: 220157755,
                flags: MemberInfoFlags::new(8192),
                field_type: "Float32",
                rust_offset: offset_of!(QualityScalableFloat, ultra),
            },
            FieldInfoData {
                name: "Cinematic",
                name_hash: 4097794678,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static QUALITYSCALABLEFLOAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableFloat-Array",
    name_hash: 2897566011,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("QualityScalableFloat"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct QualityScalableInt {
    pub low: i32,
    pub medium: i32,
    pub high: i32,
    pub ultra: i32,
    pub cinematic: i32,
}

pub trait QualityScalableIntTrait: TypeObject {
    fn low(&self) -> &i32;
    fn low_mut(&mut self) -> &mut i32;
    fn medium(&self) -> &i32;
    fn medium_mut(&mut self) -> &mut i32;
    fn high(&self) -> &i32;
    fn high_mut(&mut self) -> &mut i32;
    fn ultra(&self) -> &i32;
    fn ultra_mut(&mut self) -> &mut i32;
    fn cinematic(&self) -> &i32;
    fn cinematic_mut(&mut self) -> &mut i32;
}

impl QualityScalableIntTrait for QualityScalableInt {
    fn low(&self) -> &i32 {
        &self.low
    }
    fn low_mut(&mut self) -> &mut i32 {
        &mut self.low
    }
    fn medium(&self) -> &i32 {
        &self.medium
    }
    fn medium_mut(&mut self) -> &mut i32 {
        &mut self.medium
    }
    fn high(&self) -> &i32 {
        &self.high
    }
    fn high_mut(&mut self) -> &mut i32 {
        &mut self.high
    }
    fn ultra(&self) -> &i32 {
        &self.ultra
    }
    fn ultra_mut(&mut self) -> &mut i32 {
        &mut self.ultra
    }
    fn cinematic(&self) -> &i32 {
        &self.cinematic
    }
    fn cinematic_mut(&mut self) -> &mut i32 {
        &mut self.cinematic
    }
}

pub static QUALITYSCALABLEINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableInt",
    name_hash: 2034327276,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QualityScalableInt as Default>::default())),
            create_boxed: || Box::new(<QualityScalableInt as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Low",
                name_hash: 193454161,
                flags: MemberInfoFlags::new(8192),
                field_type: "Int32",
                rust_offset: offset_of!(QualityScalableInt, low),
            },
            FieldInfoData {
                name: "Medium",
                name_hash: 2647104632,
                flags: MemberInfoFlags::new(8192),
                field_type: "Int32",
                rust_offset: offset_of!(QualityScalableInt, medium),
            },
            FieldInfoData {
                name: "High",
                name_hash: 2089152523,
                flags: MemberInfoFlags::new(8192),
                field_type: "Int32",
                rust_offset: offset_of!(QualityScalableInt, high),
            },
            FieldInfoData {
                name: "Ultra",
                name_hash: 220157755,
                flags: MemberInfoFlags::new(8192),
                field_type: "Int32",
                rust_offset: offset_of!(QualityScalableInt, ultra),
            },
            FieldInfoData {
                name: "Cinematic",
                name_hash: 4097794678,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static QUALITYSCALABLEINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableInt-Array",
    name_hash: 372670168,
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
    name_hash: 4026849054,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static QUALITYLEVEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityLevel-Array",
    name_hash: 1034340394,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("QualityLevel"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ResourceManagerSettings {
    pub _glacier_base: SystemSettings,
    pub cas_bundle_read_buffer_size_kb: i32,
    pub cas_bundle_decompress_buffer_size_kb: i32,
    pub cas_bundle_decompress_buffer_count: i32,
    pub bundle_profiling_enable: bool,
}

pub trait ResourceManagerSettingsTrait: SystemSettingsTrait {
    fn cas_bundle_read_buffer_size_kb(&self) -> &i32;
    fn cas_bundle_read_buffer_size_kb_mut(&mut self) -> &mut i32;
    fn cas_bundle_decompress_buffer_size_kb(&self) -> &i32;
    fn cas_bundle_decompress_buffer_size_kb_mut(&mut self) -> &mut i32;
    fn cas_bundle_decompress_buffer_count(&self) -> &i32;
    fn cas_bundle_decompress_buffer_count_mut(&mut self) -> &mut i32;
    fn bundle_profiling_enable(&self) -> &bool;
    fn bundle_profiling_enable_mut(&mut self) -> &mut bool;
}

impl ResourceManagerSettingsTrait for ResourceManagerSettings {
    fn cas_bundle_read_buffer_size_kb(&self) -> &i32 {
        &self.cas_bundle_read_buffer_size_kb
    }
    fn cas_bundle_read_buffer_size_kb_mut(&mut self) -> &mut i32 {
        &mut self.cas_bundle_read_buffer_size_kb
    }
    fn cas_bundle_decompress_buffer_size_kb(&self) -> &i32 {
        &self.cas_bundle_decompress_buffer_size_kb
    }
    fn cas_bundle_decompress_buffer_size_kb_mut(&mut self) -> &mut i32 {
        &mut self.cas_bundle_decompress_buffer_size_kb
    }
    fn cas_bundle_decompress_buffer_count(&self) -> &i32 {
        &self.cas_bundle_decompress_buffer_count
    }
    fn cas_bundle_decompress_buffer_count_mut(&mut self) -> &mut i32 {
        &mut self.cas_bundle_decompress_buffer_count
    }
    fn bundle_profiling_enable(&self) -> &bool {
        &self.bundle_profiling_enable
    }
    fn bundle_profiling_enable_mut(&mut self) -> &mut bool {
        &mut self.bundle_profiling_enable
    }
}

impl SystemSettingsTrait for ResourceManagerSettings {
    fn platform(&self) -> &GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl DataContainerTrait for ResourceManagerSettings {
}

pub static RESOURCEMANAGERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceManagerSettings",
    name_hash: 1846520025,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(ResourceManagerSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ResourceManagerSettings as Default>::default())),
            create_boxed: || Box::new(<ResourceManagerSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "CasBundleReadBufferSizeKb",
                name_hash: 1627350526,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ResourceManagerSettings, cas_bundle_read_buffer_size_kb),
            },
            FieldInfoData {
                name: "CasBundleDecompressBufferSizeKb",
                name_hash: 4137626923,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ResourceManagerSettings, cas_bundle_decompress_buffer_size_kb),
            },
            FieldInfoData {
                name: "CasBundleDecompressBufferCount",
                name_hash: 2318797924,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ResourceManagerSettings, cas_bundle_decompress_buffer_count),
            },
            FieldInfoData {
                name: "BundleProfilingEnable",
                name_hash: 2207788158,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static RESOURCEMANAGERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceManagerSettings-Array",
    name_hash: 2593314541,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ResourceManagerSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn shadow_enable_mut(&mut self) -> &mut BoolOverride;
    fn sun_shadow_enable(&self) -> &BoolOverride;
    fn sun_shadow_enable_mut(&mut self) -> &mut BoolOverride;
    fn local_shadow_enable(&self) -> &BoolOverride;
    fn local_shadow_enable_mut(&mut self) -> &mut BoolOverride;
    fn dynamic_reflection_enable(&self) -> &BoolOverride;
    fn dynamic_reflection_enable_mut(&mut self) -> &mut BoolOverride;
    fn static_reflection_enable(&self) -> &BoolOverride;
    fn static_reflection_enable_mut(&mut self) -> &mut BoolOverride;
    fn planar_shadow_enable(&self) -> &BoolOverride;
    fn planar_shadow_enable_mut(&mut self) -> &mut BoolOverride;
    fn hologram_enable(&self) -> &BoolOverride;
    fn hologram_enable_mut(&mut self) -> &mut BoolOverride;
    fn hologram_projector_index(&self) -> &u32;
    fn hologram_projector_index_mut(&mut self) -> &mut u32;
    fn distant_shadow_cache_enable(&self) -> &BoolOverride;
    fn distant_shadow_cache_enable_mut(&mut self) -> &mut BoolOverride;
    fn dynamic_distant_shadow_cache_enable(&self) -> &BoolOverride;
    fn dynamic_distant_shadow_cache_enable_mut(&mut self) -> &mut BoolOverride;
    fn local_shadow_cache_enable(&self) -> &BoolOverride;
    fn local_shadow_cache_enable_mut(&mut self) -> &mut BoolOverride;
    fn root_shader_effect(&self) -> &BoolOverride;
    fn root_shader_effect_mut(&mut self) -> &mut BoolOverride;
}

impl RenderingOverridesTrait for RenderingOverrides {
    fn shadow_enable(&self) -> &BoolOverride {
        &self.shadow_enable
    }
    fn shadow_enable_mut(&mut self) -> &mut BoolOverride {
        &mut self.shadow_enable
    }
    fn sun_shadow_enable(&self) -> &BoolOverride {
        &self.sun_shadow_enable
    }
    fn sun_shadow_enable_mut(&mut self) -> &mut BoolOverride {
        &mut self.sun_shadow_enable
    }
    fn local_shadow_enable(&self) -> &BoolOverride {
        &self.local_shadow_enable
    }
    fn local_shadow_enable_mut(&mut self) -> &mut BoolOverride {
        &mut self.local_shadow_enable
    }
    fn dynamic_reflection_enable(&self) -> &BoolOverride {
        &self.dynamic_reflection_enable
    }
    fn dynamic_reflection_enable_mut(&mut self) -> &mut BoolOverride {
        &mut self.dynamic_reflection_enable
    }
    fn static_reflection_enable(&self) -> &BoolOverride {
        &self.static_reflection_enable
    }
    fn static_reflection_enable_mut(&mut self) -> &mut BoolOverride {
        &mut self.static_reflection_enable
    }
    fn planar_shadow_enable(&self) -> &BoolOverride {
        &self.planar_shadow_enable
    }
    fn planar_shadow_enable_mut(&mut self) -> &mut BoolOverride {
        &mut self.planar_shadow_enable
    }
    fn hologram_enable(&self) -> &BoolOverride {
        &self.hologram_enable
    }
    fn hologram_enable_mut(&mut self) -> &mut BoolOverride {
        &mut self.hologram_enable
    }
    fn hologram_projector_index(&self) -> &u32 {
        &self.hologram_projector_index
    }
    fn hologram_projector_index_mut(&mut self) -> &mut u32 {
        &mut self.hologram_projector_index
    }
    fn distant_shadow_cache_enable(&self) -> &BoolOverride {
        &self.distant_shadow_cache_enable
    }
    fn distant_shadow_cache_enable_mut(&mut self) -> &mut BoolOverride {
        &mut self.distant_shadow_cache_enable
    }
    fn dynamic_distant_shadow_cache_enable(&self) -> &BoolOverride {
        &self.dynamic_distant_shadow_cache_enable
    }
    fn dynamic_distant_shadow_cache_enable_mut(&mut self) -> &mut BoolOverride {
        &mut self.dynamic_distant_shadow_cache_enable
    }
    fn local_shadow_cache_enable(&self) -> &BoolOverride {
        &self.local_shadow_cache_enable
    }
    fn local_shadow_cache_enable_mut(&mut self) -> &mut BoolOverride {
        &mut self.local_shadow_cache_enable
    }
    fn root_shader_effect(&self) -> &BoolOverride {
        &self.root_shader_effect
    }
    fn root_shader_effect_mut(&mut self) -> &mut BoolOverride {
        &mut self.root_shader_effect
    }
}

pub static RENDERINGOVERRIDES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderingOverrides",
    name_hash: 4053255464,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RenderingOverrides as Default>::default())),
            create_boxed: || Box::new(<RenderingOverrides as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ShadowEnable",
                name_hash: 1716915298,
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, shadow_enable),
            },
            FieldInfoData {
                name: "SunShadowEnable",
                name_hash: 1710228394,
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, sun_shadow_enable),
            },
            FieldInfoData {
                name: "LocalShadowEnable",
                name_hash: 3483637135,
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, local_shadow_enable),
            },
            FieldInfoData {
                name: "DynamicReflectionEnable",
                name_hash: 3781264470,
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, dynamic_reflection_enable),
            },
            FieldInfoData {
                name: "StaticReflectionEnable",
                name_hash: 2113718235,
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, static_reflection_enable),
            },
            FieldInfoData {
                name: "PlanarShadowEnable",
                name_hash: 309077186,
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, planar_shadow_enable),
            },
            FieldInfoData {
                name: "HologramEnable",
                name_hash: 2433847705,
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, hologram_enable),
            },
            FieldInfoData {
                name: "HologramProjectorIndex",
                name_hash: 3471155086,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(RenderingOverrides, hologram_projector_index),
            },
            FieldInfoData {
                name: "DistantShadowCacheEnable",
                name_hash: 740536415,
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, distant_shadow_cache_enable),
            },
            FieldInfoData {
                name: "DynamicDistantShadowCacheEnable",
                name_hash: 3783922698,
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, dynamic_distant_shadow_cache_enable),
            },
            FieldInfoData {
                name: "LocalShadowCacheEnable",
                name_hash: 873357667,
                flags: MemberInfoFlags::new(0),
                field_type: "BoolOverride",
                rust_offset: offset_of!(RenderingOverrides, local_shadow_cache_enable),
            },
            FieldInfoData {
                name: "RootShaderEffect",
                name_hash: 125334397,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static RENDERINGOVERRIDES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderingOverrides-Array",
    name_hash: 3512002204,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RenderingOverrides"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ChunkOnlyResourceType {
}

pub trait ChunkOnlyResourceTypeTrait: TypeObject {
}

impl ChunkOnlyResourceTypeTrait for ChunkOnlyResourceType {
}

pub static CHUNKONLYRESOURCETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChunkOnlyResourceType",
    name_hash: 4235960280,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ChunkOnlyResourceType as Default>::default())),
            create_boxed: || Box::new(<ChunkOnlyResourceType as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static CHUNKONLYRESOURCETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChunkOnlyResourceType-Array",
    name_hash: 3550034540,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ChunkOnlyResourceType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct IResourceObject {
}

pub trait IResourceObjectTrait: TypeObject {
}

impl IResourceObjectTrait for IResourceObject {
}

pub static IRESOURCEOBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IResourceObject",
    name_hash: 3948500627,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IResourceObject as Default>::default())),
            create_boxed: || Box::new(<IResourceObject as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static IRESOURCEOBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IResourceObject-Array",
    name_hash: 1641345319,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("IResourceObject"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ProxyResourceObject {
}

pub trait ProxyResourceObjectTrait: TypeObject {
}

impl ProxyResourceObjectTrait for ProxyResourceObject {
}

pub static PROXYRESOURCEOBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProxyResourceObject",
    name_hash: 2655600150,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProxyResourceObject as Default>::default())),
            create_boxed: || Box::new(<ProxyResourceObject as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static PROXYRESOURCEOBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProxyResourceObject-Array",
    name_hash: 4208792354,
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
    name_hash: 1765115455,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static BOOLOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolOverride-Array",
    name_hash: 3529612171,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("BoolOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DataContainerPolicyPipelineResultBase {
    pub _glacier_base: SetDataResultListener,
    pub secondary_result: Option<LockedTypeObject /* DataContainer */>,
}

pub trait DataContainerPolicyPipelineResultBaseTrait: SetDataResultListenerTrait {
    fn secondary_result(&self) -> &Option<LockedTypeObject /* DataContainer */>;
    fn secondary_result_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */>;
}

impl DataContainerPolicyPipelineResultBaseTrait for DataContainerPolicyPipelineResultBase {
    fn secondary_result(&self) -> &Option<LockedTypeObject /* DataContainer */> {
        &self.secondary_result
    }
    fn secondary_result_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */> {
        &mut self.secondary_result
    }
}

impl SetDataResultListenerTrait for DataContainerPolicyPipelineResultBase {
}

impl DataContainerTrait for DataContainerPolicyPipelineResultBase {
}

pub static DATACONTAINERPOLICYPIPELINERESULTBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainerPolicyPipelineResultBase",
    name_hash: 3071586828,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SETDATARESULTLISTENER_TYPE_INFO),
        super_class_offset: offset_of!(DataContainerPolicyPipelineResultBase, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DataContainerPolicyPipelineResultBase as Default>::default())),
            create_boxed: || Box::new(<DataContainerPolicyPipelineResultBase as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "SecondaryResult",
                name_hash: 31966518,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DATACONTAINERPOLICYPIPELINERESULTBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainerPolicyPipelineResultBase-Array",
    name_hash: 2091901496,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DataContainerPolicyPipelineResultBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SetDataResultListener {
    pub _glacier_base: DataContainer,
}

pub trait SetDataResultListenerTrait: DataContainerTrait {
}

impl SetDataResultListenerTrait for SetDataResultListener {
}

impl DataContainerTrait for SetDataResultListener {
}

pub static SETDATARESULTLISTENER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetDataResultListener",
    name_hash: 375126112,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(SetDataResultListener, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SetDataResultListener as Default>::default())),
            create_boxed: || Box::new(<SetDataResultListener as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SETDATARESULTLISTENER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetDataResultListener-Array",
    name_hash: 1182740052,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SetDataResultListener"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DataBusData {
    pub _glacier_base: Asset,
    pub flags: u16,
    pub property_connections: Vec<BoxedTypeObject /* PropertyConnection */>,
    pub link_connections: Vec<BoxedTypeObject /* LinkConnection */>,
    pub interface: Option<LockedTypeObject /* DynamicDataContainer */>,
}

pub trait DataBusDataTrait: AssetTrait {
    fn flags(&self) -> &u16;
    fn flags_mut(&mut self) -> &mut u16;
    fn property_connections(&self) -> &Vec<BoxedTypeObject /* PropertyConnection */>;
    fn property_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* PropertyConnection */>;
    fn link_connections(&self) -> &Vec<BoxedTypeObject /* LinkConnection */>;
    fn link_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* LinkConnection */>;
    fn interface(&self) -> &Option<LockedTypeObject /* DynamicDataContainer */>;
    fn interface_mut(&mut self) -> &mut Option<LockedTypeObject /* DynamicDataContainer */>;
}

impl DataBusDataTrait for DataBusData {
    fn flags(&self) -> &u16 {
        &self.flags
    }
    fn flags_mut(&mut self) -> &mut u16 {
        &mut self.flags
    }
    fn property_connections(&self) -> &Vec<BoxedTypeObject /* PropertyConnection */> {
        &self.property_connections
    }
    fn property_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* PropertyConnection */> {
        &mut self.property_connections
    }
    fn link_connections(&self) -> &Vec<BoxedTypeObject /* LinkConnection */> {
        &self.link_connections
    }
    fn link_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* LinkConnection */> {
        &mut self.link_connections
    }
    fn interface(&self) -> &Option<LockedTypeObject /* DynamicDataContainer */> {
        &self.interface
    }
    fn interface_mut(&mut self) -> &mut Option<LockedTypeObject /* DynamicDataContainer */> {
        &mut self.interface
    }
}

impl AssetTrait for DataBusData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl DataContainerTrait for DataBusData {
}

pub static DATABUSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataBusData",
    name_hash: 2537093761,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        super_class_offset: offset_of!(DataBusData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DataBusData as Default>::default())),
            create_boxed: || Box::new(<DataBusData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Flags",
                name_hash: 207064858,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(DataBusData, flags),
            },
            FieldInfoData {
                name: "PropertyConnections",
                name_hash: 1023567943,
                flags: MemberInfoFlags::new(144),
                field_type: "PropertyConnection-Array",
                rust_offset: offset_of!(DataBusData, property_connections),
            },
            FieldInfoData {
                name: "LinkConnections",
                name_hash: 1859919712,
                flags: MemberInfoFlags::new(144),
                field_type: "LinkConnection-Array",
                rust_offset: offset_of!(DataBusData, link_connections),
            },
            FieldInfoData {
                name: "Interface",
                name_hash: 3803884256,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DATABUSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataBusData-Array",
    name_hash: 504648757,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DataBusData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DynamicDataContainer {
    pub _glacier_base: DataContainer,
    pub fields: Vec<BoxedTypeObject /* DataField */>,
}

pub trait DynamicDataContainerTrait: DataContainerTrait {
    fn fields(&self) -> &Vec<BoxedTypeObject /* DataField */>;
    fn fields_mut(&mut self) -> &mut Vec<BoxedTypeObject /* DataField */>;
}

impl DynamicDataContainerTrait for DynamicDataContainer {
    fn fields(&self) -> &Vec<BoxedTypeObject /* DataField */> {
        &self.fields
    }
    fn fields_mut(&mut self) -> &mut Vec<BoxedTypeObject /* DataField */> {
        &mut self.fields
    }
}

impl DataContainerTrait for DynamicDataContainer {
}

pub static DYNAMICDATACONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicDataContainer",
    name_hash: 1420547975,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(DynamicDataContainer, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DynamicDataContainer as Default>::default())),
            create_boxed: || Box::new(<DynamicDataContainer as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Fields",
                name_hash: 2525411604,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DYNAMICDATACONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicDataContainer-Array",
    name_hash: 1794185267,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DynamicDataContainer"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DataField {
    pub value: String,
    pub value_ref: Option<LockedTypeObject /* DataContainer */>,
    pub id: i32,
    pub access_type: FieldAccessType,
}

pub trait DataFieldTrait: TypeObject {
    fn value(&self) -> &String;
    fn value_mut(&mut self) -> &mut String;
    fn value_ref(&self) -> &Option<LockedTypeObject /* DataContainer */>;
    fn value_ref_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */>;
    fn id(&self) -> &i32;
    fn id_mut(&mut self) -> &mut i32;
    fn access_type(&self) -> &FieldAccessType;
    fn access_type_mut(&mut self) -> &mut FieldAccessType;
}

impl DataFieldTrait for DataField {
    fn value(&self) -> &String {
        &self.value
    }
    fn value_mut(&mut self) -> &mut String {
        &mut self.value
    }
    fn value_ref(&self) -> &Option<LockedTypeObject /* DataContainer */> {
        &self.value_ref
    }
    fn value_ref_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */> {
        &mut self.value_ref
    }
    fn id(&self) -> &i32 {
        &self.id
    }
    fn id_mut(&mut self) -> &mut i32 {
        &mut self.id
    }
    fn access_type(&self) -> &FieldAccessType {
        &self.access_type
    }
    fn access_type_mut(&mut self) -> &mut FieldAccessType {
        &mut self.access_type
    }
}

pub static DATAFIELD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataField",
    name_hash: 4163394679,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DataField as Default>::default())),
            create_boxed: || Box::new(<DataField as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(DataField, value),
            },
            FieldInfoData {
                name: "ValueRef",
                name_hash: 3291194335,
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(DataField, value_ref),
            },
            FieldInfoData {
                name: "Id",
                name_hash: 5862152,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DataField, id),
            },
            FieldInfoData {
                name: "AccessType",
                name_hash: 2232606137,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static DATAFIELD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataField-Array",
    name_hash: 2188428867,
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
    name_hash: 2804496955,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static FIELDACCESSTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FieldAccessType-Array",
    name_hash: 1275896207,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FieldAccessType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LinkConnection {
    pub source: Option<LockedTypeObject /* DataContainer */>,
    pub target: Option<LockedTypeObject /* DataContainer */>,
    pub source_field_id: i32,
    pub target_field_id: i32,
}

pub trait LinkConnectionTrait: TypeObject {
    fn source(&self) -> &Option<LockedTypeObject /* DataContainer */>;
    fn source_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */>;
    fn target(&self) -> &Option<LockedTypeObject /* DataContainer */>;
    fn target_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */>;
    fn source_field_id(&self) -> &i32;
    fn source_field_id_mut(&mut self) -> &mut i32;
    fn target_field_id(&self) -> &i32;
    fn target_field_id_mut(&mut self) -> &mut i32;
}

impl LinkConnectionTrait for LinkConnection {
    fn source(&self) -> &Option<LockedTypeObject /* DataContainer */> {
        &self.source
    }
    fn source_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */> {
        &mut self.source
    }
    fn target(&self) -> &Option<LockedTypeObject /* DataContainer */> {
        &self.target
    }
    fn target_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */> {
        &mut self.target
    }
    fn source_field_id(&self) -> &i32 {
        &self.source_field_id
    }
    fn source_field_id_mut(&mut self) -> &mut i32 {
        &mut self.source_field_id
    }
    fn target_field_id(&self) -> &i32 {
        &self.target_field_id
    }
    fn target_field_id_mut(&mut self) -> &mut i32 {
        &mut self.target_field_id
    }
}

pub static LINKCONNECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkConnection",
    name_hash: 2138769587,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinkConnection as Default>::default())),
            create_boxed: || Box::new(<LinkConnection as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Source",
                name_hash: 3339738264,
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(LinkConnection, source),
            },
            FieldInfoData {
                name: "Target",
                name_hash: 3215022804,
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(LinkConnection, target),
            },
            FieldInfoData {
                name: "SourceFieldId",
                name_hash: 4259496439,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(LinkConnection, source_field_id),
            },
            FieldInfoData {
                name: "TargetFieldId",
                name_hash: 2227058747,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static LINKCONNECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkConnection-Array",
    name_hash: 4104123399,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("LinkConnection"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PropertyConnection {
    pub source: Option<LockedTypeObject /* DataContainer */>,
    pub target: Option<LockedTypeObject /* DataContainer */>,
    pub source_field_id: i32,
    pub target_field_id: i32,
    pub flags: u32,
}

pub trait PropertyConnectionTrait: TypeObject {
    fn source(&self) -> &Option<LockedTypeObject /* DataContainer */>;
    fn source_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */>;
    fn target(&self) -> &Option<LockedTypeObject /* DataContainer */>;
    fn target_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */>;
    fn source_field_id(&self) -> &i32;
    fn source_field_id_mut(&mut self) -> &mut i32;
    fn target_field_id(&self) -> &i32;
    fn target_field_id_mut(&mut self) -> &mut i32;
    fn flags(&self) -> &u32;
    fn flags_mut(&mut self) -> &mut u32;
}

impl PropertyConnectionTrait for PropertyConnection {
    fn source(&self) -> &Option<LockedTypeObject /* DataContainer */> {
        &self.source
    }
    fn source_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */> {
        &mut self.source
    }
    fn target(&self) -> &Option<LockedTypeObject /* DataContainer */> {
        &self.target
    }
    fn target_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */> {
        &mut self.target
    }
    fn source_field_id(&self) -> &i32 {
        &self.source_field_id
    }
    fn source_field_id_mut(&mut self) -> &mut i32 {
        &mut self.source_field_id
    }
    fn target_field_id(&self) -> &i32 {
        &self.target_field_id
    }
    fn target_field_id_mut(&mut self) -> &mut i32 {
        &mut self.target_field_id
    }
    fn flags(&self) -> &u32 {
        &self.flags
    }
    fn flags_mut(&mut self) -> &mut u32 {
        &mut self.flags
    }
}

pub static PROPERTYCONNECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyConnection",
    name_hash: 2373726644,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PropertyConnection as Default>::default())),
            create_boxed: || Box::new(<PropertyConnection as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Source",
                name_hash: 3339738264,
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(PropertyConnection, source),
            },
            FieldInfoData {
                name: "Target",
                name_hash: 3215022804,
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(PropertyConnection, target),
            },
            FieldInfoData {
                name: "SourceFieldId",
                name_hash: 4259496439,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PropertyConnection, source_field_id),
            },
            FieldInfoData {
                name: "TargetFieldId",
                name_hash: 2227058747,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PropertyConnection, target_field_id),
            },
            FieldInfoData {
                name: "Flags",
                name_hash: 207064858,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static PROPERTYCONNECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyConnection-Array",
    name_hash: 2236337408,
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
    name_hash: 607813900,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static INPUTPROPERTYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputPropertyType-Array",
    name_hash: 3367301432,
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
    name_hash: 1346397661,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static PROPERTYCONNECTIONTARGETTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyConnectionTargetType-Array",
    name_hash: 1088225769,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PropertyConnectionTargetType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SettingsBundleAsset {
    pub _glacier_base: Asset,
    pub is_dedicated_server_bundle: bool,
    pub settings: Vec<Option<LockedTypeObject /* SystemSettings */>>,
}

pub trait SettingsBundleAssetTrait: AssetTrait {
    fn is_dedicated_server_bundle(&self) -> &bool;
    fn is_dedicated_server_bundle_mut(&mut self) -> &mut bool;
    fn settings(&self) -> &Vec<Option<LockedTypeObject /* SystemSettings */>>;
    fn settings_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* SystemSettings */>>;
}

impl SettingsBundleAssetTrait for SettingsBundleAsset {
    fn is_dedicated_server_bundle(&self) -> &bool {
        &self.is_dedicated_server_bundle
    }
    fn is_dedicated_server_bundle_mut(&mut self) -> &mut bool {
        &mut self.is_dedicated_server_bundle
    }
    fn settings(&self) -> &Vec<Option<LockedTypeObject /* SystemSettings */>> {
        &self.settings
    }
    fn settings_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* SystemSettings */>> {
        &mut self.settings
    }
}

impl AssetTrait for SettingsBundleAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl DataContainerTrait for SettingsBundleAsset {
}

pub static SETTINGSBUNDLEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SettingsBundleAsset",
    name_hash: 2592508388,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        super_class_offset: offset_of!(SettingsBundleAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SettingsBundleAsset as Default>::default())),
            create_boxed: || Box::new(<SettingsBundleAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "IsDedicatedServerBundle",
                name_hash: 3755269045,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SettingsBundleAsset, is_dedicated_server_bundle),
            },
            FieldInfoData {
                name: "Settings",
                name_hash: 649772672,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SETTINGSBUNDLEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SettingsBundleAsset-Array",
    name_hash: 485526992,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SettingsBundleAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SystemSettings {
    pub _glacier_base: DataContainer,
    pub platform: GamePlatform,
}

pub trait SystemSettingsTrait: DataContainerTrait {
    fn platform(&self) -> &GamePlatform;
    fn platform_mut(&mut self) -> &mut GamePlatform;
}

impl SystemSettingsTrait for SystemSettings {
    fn platform(&self) -> &GamePlatform {
        &self.platform
    }
    fn platform_mut(&mut self) -> &mut GamePlatform {
        &mut self.platform
    }
}

impl DataContainerTrait for SystemSettings {
}

pub static SYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SystemSettings",
    name_hash: 3778658821,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(SystemSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SystemSettings as Default>::default())),
            create_boxed: || Box::new(<SystemSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Platform",
                name_hash: 942751002,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SystemSettings-Array",
    name_hash: 195848625,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SystemSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DataBusPeer {
    pub _glacier_base: GameDataContainer,
    pub flags: u32,
}

pub trait DataBusPeerTrait: GameDataContainerTrait {
    fn flags(&self) -> &u32;
    fn flags_mut(&mut self) -> &mut u32;
}

impl DataBusPeerTrait for DataBusPeer {
    fn flags(&self) -> &u32 {
        &self.flags
    }
    fn flags_mut(&mut self) -> &mut u32 {
        &mut self.flags
    }
}

impl GameDataContainerTrait for DataBusPeer {
}

impl DataContainerTrait for DataBusPeer {
}

pub static DATABUSPEER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataBusPeer",
    name_hash: 2536380147,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEDATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(DataBusPeer, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DataBusPeer as Default>::default())),
            create_boxed: || Box::new(<DataBusPeer as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Flags",
                name_hash: 207064858,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DATABUSPEER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataBusPeer-Array",
    name_hash: 3174609607,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DataBusPeer"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct GameDataContainer {
    pub _glacier_base: DataContainer,
}

pub trait GameDataContainerTrait: DataContainerTrait {
}

impl GameDataContainerTrait for GameDataContainer {
}

impl DataContainerTrait for GameDataContainer {
}

pub static GAMEDATACONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameDataContainer",
    name_hash: 1216294748,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(GameDataContainer, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GameDataContainer as Default>::default())),
            create_boxed: || Box::new(<GameDataContainer as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static GAMEDATACONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameDataContainer-Array",
    name_hash: 1009391592,
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
    name_hash: 229961746,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static REALM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Realm-Array",
    name_hash: 3951466278,
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
    name_hash: 253983220,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static LANGUAGEFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanguageFormat-Array",
    name_hash: 4279518656,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("LanguageFormat"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl DataContainerTrait for DataContainerPolicyAsset {
}

pub static DATACONTAINERPOLICYASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainerPolicyAsset",
    name_hash: 2526688738,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        super_class_offset: offset_of!(DataContainerPolicyAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DataContainerPolicyAsset as Default>::default())),
            create_boxed: || Box::new(<DataContainerPolicyAsset as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DATACONTAINERPOLICYASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainerPolicyAsset-Array",
    name_hash: 880919766,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DataContainerPolicyAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Asset {
    pub _glacier_base: DataContainer,
    pub name: String,
}

pub trait AssetTrait: DataContainerTrait {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
}

impl AssetTrait for Asset {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
}

impl DataContainerTrait for Asset {
}

pub static ASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Asset",
    name_hash: 205976053,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(Asset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Asset as Default>::default())),
            create_boxed: || Box::new(<Asset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Asset-Array",
    name_hash: 1037910721,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Asset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct NativeFunctionTypeInfoAsset {
    pub _glacier_base: FunctionTypeInfoAsset,
    pub function: glacier_reflect::builtin::TypeRef,
}

pub trait NativeFunctionTypeInfoAssetTrait: FunctionTypeInfoAssetTrait {
    fn function(&self) -> &glacier_reflect::builtin::TypeRef;
    fn function_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
}

impl NativeFunctionTypeInfoAssetTrait for NativeFunctionTypeInfoAsset {
    fn function(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.function
    }
    fn function_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.function
    }
}

impl FunctionTypeInfoAssetTrait for NativeFunctionTypeInfoAsset {
    fn parameters(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoParameterDataContainer */>> {
        self._glacier_base.parameters()
    }
    fn parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoParameterDataContainer */>> {
        self._glacier_base.parameters_mut()
    }
    fn owner(&self) -> &Option<LockedTypeObject /* ClassInfoAsset */> {
        self._glacier_base.owner()
    }
    fn owner_mut(&mut self) -> &mut Option<LockedTypeObject /* ClassInfoAsset */> {
        self._glacier_base.owner_mut()
    }
}

impl TypeInfoAssetTrait for NativeFunctionTypeInfoAsset {
    fn module_name(&self) -> &String {
        self._glacier_base.module_name()
    }
    fn module_name_mut(&mut self) -> &mut String {
        self._glacier_base.module_name_mut()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn type_name_mut(&mut self) -> &mut String {
        self._glacier_base.type_name_mut()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn is_meta_mut(&mut self) -> &mut bool {
        self._glacier_base.is_meta_mut()
    }
    fn attributes(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        self._glacier_base.attributes()
    }
    fn attributes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        self._glacier_base.attributes_mut()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
    fn is_native_mut(&mut self) -> &mut bool {
        self._glacier_base.is_native_mut()
    }
}

impl AssetTrait for NativeFunctionTypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl DataContainerTrait for NativeFunctionTypeInfoAsset {
}

pub static NATIVEFUNCTIONTYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NativeFunctionTypeInfoAsset",
    name_hash: 1507313024,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FUNCTIONTYPEINFOASSET_TYPE_INFO),
        super_class_offset: offset_of!(NativeFunctionTypeInfoAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NativeFunctionTypeInfoAsset as Default>::default())),
            create_boxed: || Box::new(<NativeFunctionTypeInfoAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Function",
                name_hash: 4136871687,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static NATIVEFUNCTIONTYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NativeFunctionTypeInfoAsset-Array",
    name_hash: 2442431412,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("NativeFunctionTypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldStructValue {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: glacier_reflect::builtin::BoxedValueRef,
}

pub trait TypeInfoFieldStructValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &glacier_reflect::builtin::BoxedValueRef;
    fn value_mut(&mut self) -> &mut glacier_reflect::builtin::BoxedValueRef;
}

impl TypeInfoFieldStructValueTrait for TypeInfoFieldStructValue {
    fn value(&self) -> &glacier_reflect::builtin::BoxedValueRef {
        &self.value
    }
    fn value_mut(&mut self) -> &mut glacier_reflect::builtin::BoxedValueRef {
        &mut self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldStructValue {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
    fn field_mut(&mut self) -> &mut String {
        self._glacier_base.field_mut()
    }
}

impl DataContainerTrait for TypeInfoFieldStructValue {
}

pub static TYPEINFOFIELDSTRUCTVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldStructValue",
    name_hash: 3725132493,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldStructValue, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldStructValue as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldStructValue as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDSTRUCTVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldStructValue-Array",
    name_hash: 2891224825,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldStructValue"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldRefValue {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: Option<LockedTypeObject /* DataContainer */>,
}

pub trait TypeInfoFieldRefValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &Option<LockedTypeObject /* DataContainer */>;
    fn value_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */>;
}

impl TypeInfoFieldRefValueTrait for TypeInfoFieldRefValue {
    fn value(&self) -> &Option<LockedTypeObject /* DataContainer */> {
        &self.value
    }
    fn value_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */> {
        &mut self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldRefValue {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
    fn field_mut(&mut self) -> &mut String {
        self._glacier_base.field_mut()
    }
}

impl DataContainerTrait for TypeInfoFieldRefValue {
}

pub static TYPEINFOFIELDREFVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldRefValue",
    name_hash: 3239583083,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldRefValue, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldRefValue as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldRefValue as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDREFVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldRefValue-Array",
    name_hash: 4229573983,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldRefValue"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldTypeRefValue {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: glacier_reflect::builtin::TypeRef,
}

pub trait TypeInfoFieldTypeRefValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &glacier_reflect::builtin::TypeRef;
    fn value_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
}

impl TypeInfoFieldTypeRefValueTrait for TypeInfoFieldTypeRefValue {
    fn value(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.value
    }
    fn value_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldTypeRefValue {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
    fn field_mut(&mut self) -> &mut String {
        self._glacier_base.field_mut()
    }
}

impl DataContainerTrait for TypeInfoFieldTypeRefValue {
}

pub static TYPEINFOFIELDTYPEREFVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldTypeRefValue",
    name_hash: 311054483,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldTypeRefValue, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldTypeRefValue as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldTypeRefValue as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDTYPEREFVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldTypeRefValue-Array",
    name_hash: 2354670375,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldTypeRefValue"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldSHA1Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: glacier_reflect::builtin::SHA1,
}

pub trait TypeInfoFieldSHA1ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &glacier_reflect::builtin::SHA1;
    fn value_mut(&mut self) -> &mut glacier_reflect::builtin::SHA1;
}

impl TypeInfoFieldSHA1ValueTrait for TypeInfoFieldSHA1Value {
    fn value(&self) -> &glacier_reflect::builtin::SHA1 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut glacier_reflect::builtin::SHA1 {
        &mut self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldSHA1Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
    fn field_mut(&mut self) -> &mut String {
        self._glacier_base.field_mut()
    }
}

impl DataContainerTrait for TypeInfoFieldSHA1Value {
}

pub static TYPEINFOFIELDSHA1VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldSHA1Value",
    name_hash: 561881169,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldSHA1Value, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldSHA1Value as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldSHA1Value as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDSHA1VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldSHA1Value-Array",
    name_hash: 385684069,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldSHA1Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldGuidValue {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: glacier_util::guid::Guid,
}

pub trait TypeInfoFieldGuidValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &glacier_util::guid::Guid;
    fn value_mut(&mut self) -> &mut glacier_util::guid::Guid;
}

impl TypeInfoFieldGuidValueTrait for TypeInfoFieldGuidValue {
    fn value(&self) -> &glacier_util::guid::Guid {
        &self.value
    }
    fn value_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldGuidValue {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
    fn field_mut(&mut self) -> &mut String {
        self._glacier_base.field_mut()
    }
}

impl DataContainerTrait for TypeInfoFieldGuidValue {
}

pub static TYPEINFOFIELDGUIDVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldGuidValue",
    name_hash: 1075237957,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldGuidValue, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldGuidValue as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldGuidValue as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDGUIDVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldGuidValue-Array",
    name_hash: 710552689,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldGuidValue"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldEnumValue {
    pub _glacier_base: TypeInfoFieldValue,
    pub enum_info: glacier_reflect::builtin::TypeRef,
    pub value: u32,
}

pub trait TypeInfoFieldEnumValueTrait: TypeInfoFieldValueTrait {
    fn enum_info(&self) -> &glacier_reflect::builtin::TypeRef;
    fn enum_info_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
    fn value(&self) -> &u32;
    fn value_mut(&mut self) -> &mut u32;
}

impl TypeInfoFieldEnumValueTrait for TypeInfoFieldEnumValue {
    fn enum_info(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.enum_info
    }
    fn enum_info_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.enum_info
    }
    fn value(&self) -> &u32 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut u32 {
        &mut self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldEnumValue {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
    fn field_mut(&mut self) -> &mut String {
        self._glacier_base.field_mut()
    }
}

impl DataContainerTrait for TypeInfoFieldEnumValue {
}

pub static TYPEINFOFIELDENUMVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldEnumValue",
    name_hash: 3141004105,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldEnumValue, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldEnumValue as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldEnumValue as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "EnumInfo",
                name_hash: 553375480,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeRef",
                rust_offset: offset_of!(TypeInfoFieldEnumValue, enum_info),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDENUMVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldEnumValue-Array",
    name_hash: 1458581885,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldEnumValue"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldStringValue {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: String,
}

pub trait TypeInfoFieldStringValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &String;
    fn value_mut(&mut self) -> &mut String;
}

impl TypeInfoFieldStringValueTrait for TypeInfoFieldStringValue {
    fn value(&self) -> &String {
        &self.value
    }
    fn value_mut(&mut self) -> &mut String {
        &mut self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldStringValue {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
    fn field_mut(&mut self) -> &mut String {
        self._glacier_base.field_mut()
    }
}

impl DataContainerTrait for TypeInfoFieldStringValue {
}

pub static TYPEINFOFIELDSTRINGVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldStringValue",
    name_hash: 1740017135,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldStringValue, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldStringValue as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldStringValue as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDSTRINGVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldStringValue-Array",
    name_hash: 3123358683,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldStringValue"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldFloat64Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: f64,
}

pub trait TypeInfoFieldFloat64ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &f64;
    fn value_mut(&mut self) -> &mut f64;
}

impl TypeInfoFieldFloat64ValueTrait for TypeInfoFieldFloat64Value {
    fn value(&self) -> &f64 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut f64 {
        &mut self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldFloat64Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
    fn field_mut(&mut self) -> &mut String {
        self._glacier_base.field_mut()
    }
}

impl DataContainerTrait for TypeInfoFieldFloat64Value {
}

pub static TYPEINFOFIELDFLOAT64VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldFloat64Value",
    name_hash: 4224787912,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldFloat64Value, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldFloat64Value as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldFloat64Value as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDFLOAT64VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldFloat64Value-Array",
    name_hash: 1793620092,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldFloat64Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldFloat32Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: f32,
}

pub trait TypeInfoFieldFloat32ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &f32;
    fn value_mut(&mut self) -> &mut f32;
}

impl TypeInfoFieldFloat32ValueTrait for TypeInfoFieldFloat32Value {
    fn value(&self) -> &f32 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut f32 {
        &mut self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldFloat32Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
    fn field_mut(&mut self) -> &mut String {
        self._glacier_base.field_mut()
    }
}

impl DataContainerTrait for TypeInfoFieldFloat32Value {
}

pub static TYPEINFOFIELDFLOAT32VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldFloat32Value",
    name_hash: 3786307851,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldFloat32Value, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldFloat32Value as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldFloat32Value as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDFLOAT32VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldFloat32Value-Array",
    name_hash: 805734335,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldFloat32Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldUint64Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: u64,
}

pub trait TypeInfoFieldUint64ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &u64;
    fn value_mut(&mut self) -> &mut u64;
}

impl TypeInfoFieldUint64ValueTrait for TypeInfoFieldUint64Value {
    fn value(&self) -> &u64 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut u64 {
        &mut self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldUint64Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
    fn field_mut(&mut self) -> &mut String {
        self._glacier_base.field_mut()
    }
}

impl DataContainerTrait for TypeInfoFieldUint64Value {
}

pub static TYPEINFOFIELDUINT64VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint64Value",
    name_hash: 930437342,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldUint64Value, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldUint64Value as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldUint64Value as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDUINT64VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint64Value-Array",
    name_hash: 917625450,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldUint64Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldUint32Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: u32,
}

pub trait TypeInfoFieldUint32ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &u32;
    fn value_mut(&mut self) -> &mut u32;
}

impl TypeInfoFieldUint32ValueTrait for TypeInfoFieldUint32Value {
    fn value(&self) -> &u32 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut u32 {
        &mut self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldUint32Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
    fn field_mut(&mut self) -> &mut String {
        self._glacier_base.field_mut()
    }
}

impl DataContainerTrait for TypeInfoFieldUint32Value {
}

pub static TYPEINFOFIELDUINT32VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint32Value",
    name_hash: 2067737501,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldUint32Value, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldUint32Value as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldUint32Value as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDUINT32VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint32Value-Array",
    name_hash: 2274170665,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldUint32Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldUint16Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: u16,
}

pub trait TypeInfoFieldUint16ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &u16;
    fn value_mut(&mut self) -> &mut u16;
}

impl TypeInfoFieldUint16ValueTrait for TypeInfoFieldUint16Value {
    fn value(&self) -> &u16 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut u16 {
        &mut self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldUint16Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
    fn field_mut(&mut self) -> &mut String {
        self._glacier_base.field_mut()
    }
}

impl DataContainerTrait for TypeInfoFieldUint16Value {
}

pub static TYPEINFOFIELDUINT16VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint16Value",
    name_hash: 52241435,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldUint16Value, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldUint16Value as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldUint16Value as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDUINT16VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint16Value-Array",
    name_hash: 572275375,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldUint16Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldUint8Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: u8,
}

pub trait TypeInfoFieldUint8ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &u8;
    fn value_mut(&mut self) -> &mut u8;
}

impl TypeInfoFieldUint8ValueTrait for TypeInfoFieldUint8Value {
    fn value(&self) -> &u8 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut u8 {
        &mut self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldUint8Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
    fn field_mut(&mut self) -> &mut String {
        self._glacier_base.field_mut()
    }
}

impl DataContainerTrait for TypeInfoFieldUint8Value {
}

pub static TYPEINFOFIELDUINT8VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint8Value",
    name_hash: 1768455620,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldUint8Value, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldUint8Value as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldUint8Value as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDUINT8VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint8Value-Array",
    name_hash: 1365385840,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldUint8Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldInt64Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: i64,
}

pub trait TypeInfoFieldInt64ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &i64;
    fn value_mut(&mut self) -> &mut i64;
}

impl TypeInfoFieldInt64ValueTrait for TypeInfoFieldInt64Value {
    fn value(&self) -> &i64 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut i64 {
        &mut self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldInt64Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
    fn field_mut(&mut self) -> &mut String {
        self._glacier_base.field_mut()
    }
}

impl DataContainerTrait for TypeInfoFieldInt64Value {
}

pub static TYPEINFOFIELDINT64VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt64Value",
    name_hash: 1651489227,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldInt64Value, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldInt64Value as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldInt64Value as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDINT64VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt64Value-Array",
    name_hash: 3205830911,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldInt64Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldInt32Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: i32,
}

pub trait TypeInfoFieldInt32ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &i32;
    fn value_mut(&mut self) -> &mut i32;
}

impl TypeInfoFieldInt32ValueTrait for TypeInfoFieldInt32Value {
    fn value(&self) -> &i32 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut i32 {
        &mut self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldInt32Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
    fn field_mut(&mut self) -> &mut String {
        self._glacier_base.field_mut()
    }
}

impl DataContainerTrait for TypeInfoFieldInt32Value {
}

pub static TYPEINFOFIELDINT32VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt32Value",
    name_hash: 357647240,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldInt32Value, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldInt32Value as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldInt32Value as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDINT32VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt32Value-Array",
    name_hash: 2956191164,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldInt32Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldInt16Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: i16,
}

pub trait TypeInfoFieldInt16ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &i16;
    fn value_mut(&mut self) -> &mut i16;
}

impl TypeInfoFieldInt16ValueTrait for TypeInfoFieldInt16Value {
    fn value(&self) -> &i16 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut i16 {
        &mut self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldInt16Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
    fn field_mut(&mut self) -> &mut String {
        self._glacier_base.field_mut()
    }
}

impl DataContainerTrait for TypeInfoFieldInt16Value {
}

pub static TYPEINFOFIELDINT16VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt16Value",
    name_hash: 2931243790,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldInt16Value, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldInt16Value as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldInt16Value as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDINT16VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt16Value-Array",
    name_hash: 2852217914,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldInt16Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldInt8Value {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: i8,
}

pub trait TypeInfoFieldInt8ValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &i8;
    fn value_mut(&mut self) -> &mut i8;
}

impl TypeInfoFieldInt8ValueTrait for TypeInfoFieldInt8Value {
    fn value(&self) -> &i8 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut i8 {
        &mut self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldInt8Value {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
    fn field_mut(&mut self) -> &mut String {
        self._glacier_base.field_mut()
    }
}

impl DataContainerTrait for TypeInfoFieldInt8Value {
}

pub static TYPEINFOFIELDINT8VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt8Value",
    name_hash: 3245360177,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldInt8Value, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldInt8Value as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldInt8Value as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDINT8VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt8Value-Array",
    name_hash: 2839238277,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldInt8Value"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldBooleanValue {
    pub _glacier_base: TypeInfoFieldValue,
    pub value: bool,
}

pub trait TypeInfoFieldBooleanValueTrait: TypeInfoFieldValueTrait {
    fn value(&self) -> &bool;
    fn value_mut(&mut self) -> &mut bool;
}

impl TypeInfoFieldBooleanValueTrait for TypeInfoFieldBooleanValue {
    fn value(&self) -> &bool {
        &self.value
    }
    fn value_mut(&mut self) -> &mut bool {
        &mut self.value
    }
}

impl TypeInfoFieldValueTrait for TypeInfoFieldBooleanValue {
    fn field(&self) -> &String {
        self._glacier_base.field()
    }
    fn field_mut(&mut self) -> &mut String {
        self._glacier_base.field_mut()
    }
}

impl DataContainerTrait for TypeInfoFieldBooleanValue {
}

pub static TYPEINFOFIELDBOOLEANVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldBooleanValue",
    name_hash: 848872766,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldBooleanValue, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldBooleanValue as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldBooleanValue as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDBOOLEANVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldBooleanValue-Array",
    name_hash: 2585843082,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldBooleanValue"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn module_name_mut(&mut self) -> &mut String {
        self._glacier_base.module_name_mut()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn type_name_mut(&mut self) -> &mut String {
        self._glacier_base.type_name_mut()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn is_meta_mut(&mut self) -> &mut bool {
        self._glacier_base.is_meta_mut()
    }
    fn attributes(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        self._glacier_base.attributes()
    }
    fn attributes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        self._glacier_base.attributes_mut()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
    fn is_native_mut(&mut self) -> &mut bool {
        self._glacier_base.is_native_mut()
    }
}

impl AssetTrait for PrimitiveTypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl DataContainerTrait for PrimitiveTypeInfoAsset {
}

pub static PRIMITIVETYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrimitiveTypeInfoAsset",
    name_hash: 1221897698,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOASSET_TYPE_INFO),
        super_class_offset: offset_of!(PrimitiveTypeInfoAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PrimitiveTypeInfoAsset as Default>::default())),
            create_boxed: || Box::new(<PrimitiveTypeInfoAsset as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PRIMITIVETYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrimitiveTypeInfoAsset-Array",
    name_hash: 1991654614,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PrimitiveTypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EnumTypeInfoAsset {
    pub _glacier_base: TypeInfoAsset,
    pub enumerators: Vec<BoxedTypeObject /* EnumTypeEnumeratorData */>,
}

pub trait EnumTypeInfoAssetTrait: TypeInfoAssetTrait {
    fn enumerators(&self) -> &Vec<BoxedTypeObject /* EnumTypeEnumeratorData */>;
    fn enumerators_mut(&mut self) -> &mut Vec<BoxedTypeObject /* EnumTypeEnumeratorData */>;
}

impl EnumTypeInfoAssetTrait for EnumTypeInfoAsset {
    fn enumerators(&self) -> &Vec<BoxedTypeObject /* EnumTypeEnumeratorData */> {
        &self.enumerators
    }
    fn enumerators_mut(&mut self) -> &mut Vec<BoxedTypeObject /* EnumTypeEnumeratorData */> {
        &mut self.enumerators
    }
}

impl TypeInfoAssetTrait for EnumTypeInfoAsset {
    fn module_name(&self) -> &String {
        self._glacier_base.module_name()
    }
    fn module_name_mut(&mut self) -> &mut String {
        self._glacier_base.module_name_mut()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn type_name_mut(&mut self) -> &mut String {
        self._glacier_base.type_name_mut()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn is_meta_mut(&mut self) -> &mut bool {
        self._glacier_base.is_meta_mut()
    }
    fn attributes(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        self._glacier_base.attributes()
    }
    fn attributes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        self._glacier_base.attributes_mut()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
    fn is_native_mut(&mut self) -> &mut bool {
        self._glacier_base.is_native_mut()
    }
}

impl AssetTrait for EnumTypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl DataContainerTrait for EnumTypeInfoAsset {
}

pub static ENUMTYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumTypeInfoAsset",
    name_hash: 128961360,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOASSET_TYPE_INFO),
        super_class_offset: offset_of!(EnumTypeInfoAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnumTypeInfoAsset as Default>::default())),
            create_boxed: || Box::new(<EnumTypeInfoAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enumerators",
                name_hash: 2230533850,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ENUMTYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumTypeInfoAsset-Array",
    name_hash: 3097536996,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("EnumTypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EnumTypeEnumeratorData {
    pub name: String,
    pub value: i32,
    pub attributes: Vec<Option<LockedTypeObject /* TypeInfoAttribute */>>,
}

pub trait EnumTypeEnumeratorDataTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn value(&self) -> &i32;
    fn value_mut(&mut self) -> &mut i32;
    fn attributes(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoAttribute */>>;
    fn attributes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoAttribute */>>;
}

impl EnumTypeEnumeratorDataTrait for EnumTypeEnumeratorData {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn value(&self) -> &i32 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut i32 {
        &mut self.value
    }
    fn attributes(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        &mut self.attributes
    }
}

pub static ENUMTYPEENUMERATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumTypeEnumeratorData",
    name_hash: 584985074,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnumTypeEnumeratorData as Default>::default())),
            create_boxed: || Box::new(<EnumTypeEnumeratorData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EnumTypeEnumeratorData, name),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EnumTypeEnumeratorData, value),
            },
            FieldInfoData {
                name: "Attributes",
                name_hash: 3723762538,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static ENUMTYPEENUMERATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumTypeEnumeratorData-Array",
    name_hash: 4169105606,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("EnumTypeEnumeratorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FunctionTypeInfoAsset {
    pub _glacier_base: TypeInfoAsset,
    pub parameters: Vec<Option<LockedTypeObject /* TypeInfoParameterDataContainer */>>,
    pub owner: Option<LockedTypeObject /* ClassInfoAsset */>,
}

pub trait FunctionTypeInfoAssetTrait: TypeInfoAssetTrait {
    fn parameters(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoParameterDataContainer */>>;
    fn parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoParameterDataContainer */>>;
    fn owner(&self) -> &Option<LockedTypeObject /* ClassInfoAsset */>;
    fn owner_mut(&mut self) -> &mut Option<LockedTypeObject /* ClassInfoAsset */>;
}

impl FunctionTypeInfoAssetTrait for FunctionTypeInfoAsset {
    fn parameters(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoParameterDataContainer */>> {
        &self.parameters
    }
    fn parameters_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoParameterDataContainer */>> {
        &mut self.parameters
    }
    fn owner(&self) -> &Option<LockedTypeObject /* ClassInfoAsset */> {
        &self.owner
    }
    fn owner_mut(&mut self) -> &mut Option<LockedTypeObject /* ClassInfoAsset */> {
        &mut self.owner
    }
}

impl TypeInfoAssetTrait for FunctionTypeInfoAsset {
    fn module_name(&self) -> &String {
        self._glacier_base.module_name()
    }
    fn module_name_mut(&mut self) -> &mut String {
        self._glacier_base.module_name_mut()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn type_name_mut(&mut self) -> &mut String {
        self._glacier_base.type_name_mut()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn is_meta_mut(&mut self) -> &mut bool {
        self._glacier_base.is_meta_mut()
    }
    fn attributes(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        self._glacier_base.attributes()
    }
    fn attributes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        self._glacier_base.attributes_mut()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
    fn is_native_mut(&mut self) -> &mut bool {
        self._glacier_base.is_native_mut()
    }
}

impl AssetTrait for FunctionTypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl DataContainerTrait for FunctionTypeInfoAsset {
}

pub static FUNCTIONTYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FunctionTypeInfoAsset",
    name_hash: 3131820609,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOASSET_TYPE_INFO),
        super_class_offset: offset_of!(FunctionTypeInfoAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FunctionTypeInfoAsset as Default>::default())),
            create_boxed: || Box::new(<FunctionTypeInfoAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Parameters",
                name_hash: 3325515039,
                flags: MemberInfoFlags::new(144),
                field_type: "TypeInfoParameterDataContainer-Array",
                rust_offset: offset_of!(FunctionTypeInfoAsset, parameters),
            },
            FieldInfoData {
                name: "Owner",
                name_hash: 217695012,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static FUNCTIONTYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FunctionTypeInfoAsset-Array",
    name_hash: 2567959669,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FunctionTypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoParameterDataContainer {
    pub _glacier_base: DataContainer,
    pub name: String,
    pub type_ref: TypeInfoRef,
    pub is_array: bool,
    pub qualifier: TypeInfoParameterQualifier,
    pub default_value: Option<LockedTypeObject /* DataContainer */>,
}

pub trait TypeInfoParameterDataContainerTrait: DataContainerTrait {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn type_ref(&self) -> &TypeInfoRef;
    fn type_ref_mut(&mut self) -> &mut TypeInfoRef;
    fn is_array(&self) -> &bool;
    fn is_array_mut(&mut self) -> &mut bool;
    fn qualifier(&self) -> &TypeInfoParameterQualifier;
    fn qualifier_mut(&mut self) -> &mut TypeInfoParameterQualifier;
    fn default_value(&self) -> &Option<LockedTypeObject /* DataContainer */>;
    fn default_value_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */>;
}

impl TypeInfoParameterDataContainerTrait for TypeInfoParameterDataContainer {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn type_ref(&self) -> &TypeInfoRef {
        &self.type_ref
    }
    fn type_ref_mut(&mut self) -> &mut TypeInfoRef {
        &mut self.type_ref
    }
    fn is_array(&self) -> &bool {
        &self.is_array
    }
    fn is_array_mut(&mut self) -> &mut bool {
        &mut self.is_array
    }
    fn qualifier(&self) -> &TypeInfoParameterQualifier {
        &self.qualifier
    }
    fn qualifier_mut(&mut self) -> &mut TypeInfoParameterQualifier {
        &mut self.qualifier
    }
    fn default_value(&self) -> &Option<LockedTypeObject /* DataContainer */> {
        &self.default_value
    }
    fn default_value_mut(&mut self) -> &mut Option<LockedTypeObject /* DataContainer */> {
        &mut self.default_value
    }
}

impl DataContainerTrait for TypeInfoParameterDataContainer {
}

pub static TYPEINFOPARAMETERDATACONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoParameterDataContainer",
    name_hash: 3019381005,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoParameterDataContainer, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoParameterDataContainer as Default>::default())),
            create_boxed: || Box::new(<TypeInfoParameterDataContainer as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TypeInfoParameterDataContainer, name),
            },
            FieldInfoData {
                name: "TypeRef",
                name_hash: 2743797740,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeInfoRef",
                rust_offset: offset_of!(TypeInfoParameterDataContainer, type_ref),
            },
            FieldInfoData {
                name: "IsArray",
                name_hash: 2763287494,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TypeInfoParameterDataContainer, is_array),
            },
            FieldInfoData {
                name: "Qualifier",
                name_hash: 3894681981,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeInfoParameterQualifier",
                rust_offset: offset_of!(TypeInfoParameterDataContainer, qualifier),
            },
            FieldInfoData {
                name: "DefaultValue",
                name_hash: 2066049125,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOPARAMETERDATACONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoParameterDataContainer-Array",
    name_hash: 3255786681,
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
    name_hash: 2731866882,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static TYPEINFOPARAMETERQUALIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoParameterQualifier-Array",
    name_hash: 3383761462,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoParameterQualifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FunctionTypeInfoRef {
    pub asset: Option<LockedTypeObject /* FunctionTypeInfoAsset */>,
    pub type_info: glacier_reflect::builtin::TypeRef,
}

pub trait FunctionTypeInfoRefTrait: TypeObject {
    fn asset(&self) -> &Option<LockedTypeObject /* FunctionTypeInfoAsset */>;
    fn asset_mut(&mut self) -> &mut Option<LockedTypeObject /* FunctionTypeInfoAsset */>;
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef;
    fn type_info_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
}

impl FunctionTypeInfoRefTrait for FunctionTypeInfoRef {
    fn asset(&self) -> &Option<LockedTypeObject /* FunctionTypeInfoAsset */> {
        &self.asset
    }
    fn asset_mut(&mut self) -> &mut Option<LockedTypeObject /* FunctionTypeInfoAsset */> {
        &mut self.asset
    }
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.type_info
    }
    fn type_info_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.type_info
    }
}

pub static FUNCTIONTYPEINFOREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FunctionTypeInfoRef",
    name_hash: 215837920,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FunctionTypeInfoRef as Default>::default())),
            create_boxed: || Box::new(<FunctionTypeInfoRef as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Asset",
                name_hash: 205976053,
                flags: MemberInfoFlags::new(0),
                field_type: "FunctionTypeInfoAsset",
                rust_offset: offset_of!(FunctionTypeInfoRef, asset),
            },
            FieldInfoData {
                name: "TypeInfo",
                name_hash: 351127923,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static FUNCTIONTYPEINFOREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FunctionTypeInfoRef-Array",
    name_hash: 979763924,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FunctionTypeInfoRef"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ValueTypeInfoAsset {
    pub _glacier_base: ComplexTypeInfoAsset,
}

pub trait ValueTypeInfoAssetTrait: ComplexTypeInfoAssetTrait {
}

impl ValueTypeInfoAssetTrait for ValueTypeInfoAsset {
}

impl ComplexTypeInfoAssetTrait for ValueTypeInfoAsset {
    fn field_collections(&self) -> &Vec<BoxedTypeObject /* TypeInfoFieldCollectionRef */> {
        self._glacier_base.field_collections()
    }
    fn field_collections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* TypeInfoFieldCollectionRef */> {
        self._glacier_base.field_collections_mut()
    }
    fn alignment(&self) -> &u32 {
        self._glacier_base.alignment()
    }
    fn alignment_mut(&mut self) -> &mut u32 {
        self._glacier_base.alignment_mut()
    }
}

impl TypeInfoAssetTrait for ValueTypeInfoAsset {
    fn module_name(&self) -> &String {
        self._glacier_base.module_name()
    }
    fn module_name_mut(&mut self) -> &mut String {
        self._glacier_base.module_name_mut()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn type_name_mut(&mut self) -> &mut String {
        self._glacier_base.type_name_mut()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn is_meta_mut(&mut self) -> &mut bool {
        self._glacier_base.is_meta_mut()
    }
    fn attributes(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        self._glacier_base.attributes()
    }
    fn attributes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        self._glacier_base.attributes_mut()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
    fn is_native_mut(&mut self) -> &mut bool {
        self._glacier_base.is_native_mut()
    }
}

impl AssetTrait for ValueTypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl DataContainerTrait for ValueTypeInfoAsset {
}

pub static VALUETYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValueTypeInfoAsset",
    name_hash: 519591112,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPLEXTYPEINFOASSET_TYPE_INFO),
        super_class_offset: offset_of!(ValueTypeInfoAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ValueTypeInfoAsset as Default>::default())),
            create_boxed: || Box::new(<ValueTypeInfoAsset as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static VALUETYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValueTypeInfoAsset-Array",
    name_hash: 7792508,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ValueTypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClassInfoAsset {
    pub _glacier_base: ComplexTypeInfoAsset,
    pub super_class_ref: ClassInfoRef,
    pub is_abstract: bool,
    pub is_sealed: bool,
}

pub trait ClassInfoAssetTrait: ComplexTypeInfoAssetTrait {
    fn super_class_ref(&self) -> &ClassInfoRef;
    fn super_class_ref_mut(&mut self) -> &mut ClassInfoRef;
    fn is_abstract(&self) -> &bool;
    fn is_abstract_mut(&mut self) -> &mut bool;
    fn is_sealed(&self) -> &bool;
    fn is_sealed_mut(&mut self) -> &mut bool;
}

impl ClassInfoAssetTrait for ClassInfoAsset {
    fn super_class_ref(&self) -> &ClassInfoRef {
        &self.super_class_ref
    }
    fn super_class_ref_mut(&mut self) -> &mut ClassInfoRef {
        &mut self.super_class_ref
    }
    fn is_abstract(&self) -> &bool {
        &self.is_abstract
    }
    fn is_abstract_mut(&mut self) -> &mut bool {
        &mut self.is_abstract
    }
    fn is_sealed(&self) -> &bool {
        &self.is_sealed
    }
    fn is_sealed_mut(&mut self) -> &mut bool {
        &mut self.is_sealed
    }
}

impl ComplexTypeInfoAssetTrait for ClassInfoAsset {
    fn field_collections(&self) -> &Vec<BoxedTypeObject /* TypeInfoFieldCollectionRef */> {
        self._glacier_base.field_collections()
    }
    fn field_collections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* TypeInfoFieldCollectionRef */> {
        self._glacier_base.field_collections_mut()
    }
    fn alignment(&self) -> &u32 {
        self._glacier_base.alignment()
    }
    fn alignment_mut(&mut self) -> &mut u32 {
        self._glacier_base.alignment_mut()
    }
}

impl TypeInfoAssetTrait for ClassInfoAsset {
    fn module_name(&self) -> &String {
        self._glacier_base.module_name()
    }
    fn module_name_mut(&mut self) -> &mut String {
        self._glacier_base.module_name_mut()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn type_name_mut(&mut self) -> &mut String {
        self._glacier_base.type_name_mut()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn is_meta_mut(&mut self) -> &mut bool {
        self._glacier_base.is_meta_mut()
    }
    fn attributes(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        self._glacier_base.attributes()
    }
    fn attributes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        self._glacier_base.attributes_mut()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
    fn is_native_mut(&mut self) -> &mut bool {
        self._glacier_base.is_native_mut()
    }
}

impl AssetTrait for ClassInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl DataContainerTrait for ClassInfoAsset {
}

pub static CLASSINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClassInfoAsset",
    name_hash: 3710628341,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPLEXTYPEINFOASSET_TYPE_INFO),
        super_class_offset: offset_of!(ClassInfoAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClassInfoAsset as Default>::default())),
            create_boxed: || Box::new(<ClassInfoAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "SuperClassRef",
                name_hash: 2257450427,
                flags: MemberInfoFlags::new(0),
                field_type: "ClassInfoRef",
                rust_offset: offset_of!(ClassInfoAsset, super_class_ref),
            },
            FieldInfoData {
                name: "IsAbstract",
                name_hash: 2534040223,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClassInfoAsset, is_abstract),
            },
            FieldInfoData {
                name: "IsSealed",
                name_hash: 451801797,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CLASSINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClassInfoAsset-Array",
    name_hash: 3612476097,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ClassInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClassInfoRef {
    pub asset: Option<LockedTypeObject /* ClassInfoAsset */>,
    pub type_info: glacier_reflect::builtin::TypeRef,
}

pub trait ClassInfoRefTrait: TypeObject {
    fn asset(&self) -> &Option<LockedTypeObject /* ClassInfoAsset */>;
    fn asset_mut(&mut self) -> &mut Option<LockedTypeObject /* ClassInfoAsset */>;
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef;
    fn type_info_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
}

impl ClassInfoRefTrait for ClassInfoRef {
    fn asset(&self) -> &Option<LockedTypeObject /* ClassInfoAsset */> {
        &self.asset
    }
    fn asset_mut(&mut self) -> &mut Option<LockedTypeObject /* ClassInfoAsset */> {
        &mut self.asset
    }
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.type_info
    }
    fn type_info_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.type_info
    }
}

pub static CLASSINFOREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClassInfoRef",
    name_hash: 3694971540,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClassInfoRef as Default>::default())),
            create_boxed: || Box::new(<ClassInfoRef as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Asset",
                name_hash: 205976053,
                flags: MemberInfoFlags::new(0),
                field_type: "ClassInfoAsset",
                rust_offset: offset_of!(ClassInfoRef, asset),
            },
            FieldInfoData {
                name: "TypeInfo",
                name_hash: 351127923,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static CLASSINFOREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClassInfoRef-Array",
    name_hash: 1606141600,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ClassInfoRef"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ComplexTypeInfoAsset {
    pub _glacier_base: TypeInfoAsset,
    pub field_collections: Vec<BoxedTypeObject /* TypeInfoFieldCollectionRef */>,
    pub alignment: u32,
}

pub trait ComplexTypeInfoAssetTrait: TypeInfoAssetTrait {
    fn field_collections(&self) -> &Vec<BoxedTypeObject /* TypeInfoFieldCollectionRef */>;
    fn field_collections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* TypeInfoFieldCollectionRef */>;
    fn alignment(&self) -> &u32;
    fn alignment_mut(&mut self) -> &mut u32;
}

impl ComplexTypeInfoAssetTrait for ComplexTypeInfoAsset {
    fn field_collections(&self) -> &Vec<BoxedTypeObject /* TypeInfoFieldCollectionRef */> {
        &self.field_collections
    }
    fn field_collections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* TypeInfoFieldCollectionRef */> {
        &mut self.field_collections
    }
    fn alignment(&self) -> &u32 {
        &self.alignment
    }
    fn alignment_mut(&mut self) -> &mut u32 {
        &mut self.alignment
    }
}

impl TypeInfoAssetTrait for ComplexTypeInfoAsset {
    fn module_name(&self) -> &String {
        self._glacier_base.module_name()
    }
    fn module_name_mut(&mut self) -> &mut String {
        self._glacier_base.module_name_mut()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn type_name_mut(&mut self) -> &mut String {
        self._glacier_base.type_name_mut()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn is_meta_mut(&mut self) -> &mut bool {
        self._glacier_base.is_meta_mut()
    }
    fn attributes(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        self._glacier_base.attributes()
    }
    fn attributes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        self._glacier_base.attributes_mut()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
    fn is_native_mut(&mut self) -> &mut bool {
        self._glacier_base.is_native_mut()
    }
}

impl AssetTrait for ComplexTypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl DataContainerTrait for ComplexTypeInfoAsset {
}

pub static COMPLEXTYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComplexTypeInfoAsset",
    name_hash: 1039248227,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOASSET_TYPE_INFO),
        super_class_offset: offset_of!(ComplexTypeInfoAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ComplexTypeInfoAsset as Default>::default())),
            create_boxed: || Box::new(<ComplexTypeInfoAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "FieldCollections",
                name_hash: 3056002242,
                flags: MemberInfoFlags::new(144),
                field_type: "TypeInfoFieldCollectionRef-Array",
                rust_offset: offset_of!(ComplexTypeInfoAsset, field_collections),
            },
            FieldInfoData {
                name: "Alignment",
                name_hash: 1711241338,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static COMPLEXTYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComplexTypeInfoAsset-Array",
    name_hash: 2196168535,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ComplexTypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldCollectionRef {
    pub collection: Option<LockedTypeObject /* TypeInfoFieldCollection */>,
}

pub trait TypeInfoFieldCollectionRefTrait: TypeObject {
    fn collection(&self) -> &Option<LockedTypeObject /* TypeInfoFieldCollection */>;
    fn collection_mut(&mut self) -> &mut Option<LockedTypeObject /* TypeInfoFieldCollection */>;
}

impl TypeInfoFieldCollectionRefTrait for TypeInfoFieldCollectionRef {
    fn collection(&self) -> &Option<LockedTypeObject /* TypeInfoFieldCollection */> {
        &self.collection
    }
    fn collection_mut(&mut self) -> &mut Option<LockedTypeObject /* TypeInfoFieldCollection */> {
        &mut self.collection
    }
}

pub static TYPEINFOFIELDCOLLECTIONREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldCollectionRef",
    name_hash: 1482443958,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldCollectionRef as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldCollectionRef as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Collection",
                name_hash: 3772761619,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static TYPEINFOFIELDCOLLECTIONREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldCollectionRef-Array",
    name_hash: 4140605698,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldCollectionRef"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldCollection {
    pub _glacier_base: DataContainer,
    pub fields: Vec<Option<LockedTypeObject /* TypeInfoFieldData */>>,
    pub default_values: Vec<Option<LockedTypeObject /* TypeInfoFieldValue */>>,
}

pub trait TypeInfoFieldCollectionTrait: DataContainerTrait {
    fn fields(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoFieldData */>>;
    fn fields_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoFieldData */>>;
    fn default_values(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoFieldValue */>>;
    fn default_values_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoFieldValue */>>;
}

impl TypeInfoFieldCollectionTrait for TypeInfoFieldCollection {
    fn fields(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoFieldData */>> {
        &self.fields
    }
    fn fields_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoFieldData */>> {
        &mut self.fields
    }
    fn default_values(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoFieldValue */>> {
        &self.default_values
    }
    fn default_values_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoFieldValue */>> {
        &mut self.default_values
    }
}

impl DataContainerTrait for TypeInfoFieldCollection {
}

pub static TYPEINFOFIELDCOLLECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldCollection",
    name_hash: 1521451879,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldCollection, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldCollection as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldCollection as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Fields",
                name_hash: 2525411604,
                flags: MemberInfoFlags::new(144),
                field_type: "TypeInfoFieldData-Array",
                rust_offset: offset_of!(TypeInfoFieldCollection, fields),
            },
            FieldInfoData {
                name: "DefaultValues",
                name_hash: 3755111798,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDCOLLECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldCollection-Array",
    name_hash: 3185993043,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldCollection"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoFieldValue {
    pub _glacier_base: DataContainer,
    pub field: String,
}

pub trait TypeInfoFieldValueTrait: DataContainerTrait {
    fn field(&self) -> &String;
    fn field_mut(&mut self) -> &mut String;
}

impl TypeInfoFieldValueTrait for TypeInfoFieldValue {
    fn field(&self) -> &String {
        &self.field
    }
    fn field_mut(&mut self) -> &mut String {
        &mut self.field
    }
}

impl DataContainerTrait for TypeInfoFieldValue {
}

pub static TYPEINFOFIELDVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldValue",
    name_hash: 3740099226,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldValue, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldValue as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldValue as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Field",
                name_hash: 206678151,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldValue-Array",
    name_hash: 4117382574,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldValue"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    pub attributes: Vec<Option<LockedTypeObject /* TypeInfoAttribute */>>,
}

pub trait TypeInfoFieldDataTrait: DataContainerTrait {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn type_ref(&self) -> &TypeInfoRef;
    fn type_ref_mut(&mut self) -> &mut TypeInfoRef;
    fn is_array(&self) -> &bool;
    fn is_array_mut(&mut self) -> &mut bool;
    fn protection_level(&self) -> &ProtectionLevel;
    fn protection_level_mut(&mut self) -> &mut ProtectionLevel;
    fn memory_sort_index(&self) -> &u32;
    fn memory_sort_index_mut(&mut self) -> &mut u32;
    fn is_meta(&self) -> &bool;
    fn is_meta_mut(&mut self) -> &mut bool;
    fn is_exposed(&self) -> &bool;
    fn is_exposed_mut(&mut self) -> &mut bool;
    fn access_type(&self) -> &AccessType;
    fn access_type_mut(&mut self) -> &mut AccessType;
    fn always_persist(&self) -> &bool;
    fn always_persist_mut(&mut self) -> &mut bool;
    fn attributes(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoAttribute */>>;
    fn attributes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoAttribute */>>;
}

impl TypeInfoFieldDataTrait for TypeInfoFieldData {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn type_ref(&self) -> &TypeInfoRef {
        &self.type_ref
    }
    fn type_ref_mut(&mut self) -> &mut TypeInfoRef {
        &mut self.type_ref
    }
    fn is_array(&self) -> &bool {
        &self.is_array
    }
    fn is_array_mut(&mut self) -> &mut bool {
        &mut self.is_array
    }
    fn protection_level(&self) -> &ProtectionLevel {
        &self.protection_level
    }
    fn protection_level_mut(&mut self) -> &mut ProtectionLevel {
        &mut self.protection_level
    }
    fn memory_sort_index(&self) -> &u32 {
        &self.memory_sort_index
    }
    fn memory_sort_index_mut(&mut self) -> &mut u32 {
        &mut self.memory_sort_index
    }
    fn is_meta(&self) -> &bool {
        &self.is_meta
    }
    fn is_meta_mut(&mut self) -> &mut bool {
        &mut self.is_meta
    }
    fn is_exposed(&self) -> &bool {
        &self.is_exposed
    }
    fn is_exposed_mut(&mut self) -> &mut bool {
        &mut self.is_exposed
    }
    fn access_type(&self) -> &AccessType {
        &self.access_type
    }
    fn access_type_mut(&mut self) -> &mut AccessType {
        &mut self.access_type
    }
    fn always_persist(&self) -> &bool {
        &self.always_persist
    }
    fn always_persist_mut(&mut self) -> &mut bool {
        &mut self.always_persist
    }
    fn attributes(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        &mut self.attributes
    }
}

impl DataContainerTrait for TypeInfoFieldData {
}

pub static TYPEINFOFIELDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldData",
    name_hash: 4148644417,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoFieldData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoFieldData as Default>::default())),
            create_boxed: || Box::new(<TypeInfoFieldData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TypeInfoFieldData, name),
            },
            FieldInfoData {
                name: "TypeRef",
                name_hash: 2743797740,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeInfoRef",
                rust_offset: offset_of!(TypeInfoFieldData, type_ref),
            },
            FieldInfoData {
                name: "IsArray",
                name_hash: 2763287494,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TypeInfoFieldData, is_array),
            },
            FieldInfoData {
                name: "ProtectionLevel",
                name_hash: 1432881680,
                flags: MemberInfoFlags::new(0),
                field_type: "ProtectionLevel",
                rust_offset: offset_of!(TypeInfoFieldData, protection_level),
            },
            FieldInfoData {
                name: "MemorySortIndex",
                name_hash: 745243264,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TypeInfoFieldData, memory_sort_index),
            },
            FieldInfoData {
                name: "IsMeta",
                name_hash: 2816483074,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TypeInfoFieldData, is_meta),
            },
            FieldInfoData {
                name: "IsExposed",
                name_hash: 1478860463,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TypeInfoFieldData, is_exposed),
            },
            FieldInfoData {
                name: "AccessType",
                name_hash: 2232606137,
                flags: MemberInfoFlags::new(0),
                field_type: "AccessType",
                rust_offset: offset_of!(TypeInfoFieldData, access_type),
            },
            FieldInfoData {
                name: "AlwaysPersist",
                name_hash: 870738286,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TypeInfoFieldData, always_persist),
            },
            FieldInfoData {
                name: "Attributes",
                name_hash: 3723762538,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOFIELDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldData-Array",
    name_hash: 3159061621,
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
    name_hash: 2232606137,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static ACCESSTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AccessType-Array",
    name_hash: 1816816653,
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
    name_hash: 1432881680,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static PROTECTIONLEVEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProtectionLevel-Array",
    name_hash: 3187210276,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ProtectionLevel"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoRef {
    pub asset: Option<LockedTypeObject /* TypeInfoAsset */>,
    pub type_info: glacier_reflect::builtin::TypeRef,
}

pub trait TypeInfoRefTrait: TypeObject {
    fn asset(&self) -> &Option<LockedTypeObject /* TypeInfoAsset */>;
    fn asset_mut(&mut self) -> &mut Option<LockedTypeObject /* TypeInfoAsset */>;
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef;
    fn type_info_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
}

impl TypeInfoRefTrait for TypeInfoRef {
    fn asset(&self) -> &Option<LockedTypeObject /* TypeInfoAsset */> {
        &self.asset
    }
    fn asset_mut(&mut self) -> &mut Option<LockedTypeObject /* TypeInfoAsset */> {
        &mut self.asset
    }
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.type_info
    }
    fn type_info_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.type_info
    }
}

pub static TYPEINFOREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoRef",
    name_hash: 4165132322,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoRef as Default>::default())),
            create_boxed: || Box::new(<TypeInfoRef as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Asset",
                name_hash: 205976053,
                flags: MemberInfoFlags::new(0),
                field_type: "TypeInfoAsset",
                rust_offset: offset_of!(TypeInfoRef, asset),
            },
            FieldInfoData {
                name: "TypeInfo",
                name_hash: 351127923,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static TYPEINFOREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoRef-Array",
    name_hash: 2824953494,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoRef"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoAsset {
    pub _glacier_base: Asset,
    pub module_name: String,
    pub type_name: String,
    pub is_meta: bool,
    pub attributes: Vec<Option<LockedTypeObject /* TypeInfoAttribute */>>,
    pub is_native: bool,
}

pub trait TypeInfoAssetTrait: AssetTrait {
    fn module_name(&self) -> &String;
    fn module_name_mut(&mut self) -> &mut String;
    fn type_name(&self) -> &String;
    fn type_name_mut(&mut self) -> &mut String;
    fn is_meta(&self) -> &bool;
    fn is_meta_mut(&mut self) -> &mut bool;
    fn attributes(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoAttribute */>>;
    fn attributes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoAttribute */>>;
    fn is_native(&self) -> &bool;
    fn is_native_mut(&mut self) -> &mut bool;
}

impl TypeInfoAssetTrait for TypeInfoAsset {
    fn module_name(&self) -> &String {
        &self.module_name
    }
    fn module_name_mut(&mut self) -> &mut String {
        &mut self.module_name
    }
    fn type_name(&self) -> &String {
        &self.type_name
    }
    fn type_name_mut(&mut self) -> &mut String {
        &mut self.type_name
    }
    fn is_meta(&self) -> &bool {
        &self.is_meta
    }
    fn is_meta_mut(&mut self) -> &mut bool {
        &mut self.is_meta
    }
    fn attributes(&self) -> &Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* TypeInfoAttribute */>> {
        &mut self.attributes
    }
    fn is_native(&self) -> &bool {
        &self.is_native
    }
    fn is_native_mut(&mut self) -> &mut bool {
        &mut self.is_native
    }
}

impl AssetTrait for TypeInfoAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl DataContainerTrait for TypeInfoAsset {
}

pub static TYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoAsset",
    name_hash: 360793603,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoAsset as Default>::default())),
            create_boxed: || Box::new(<TypeInfoAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ModuleName",
                name_hash: 1751459736,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TypeInfoAsset, module_name),
            },
            FieldInfoData {
                name: "TypeName",
                name_hash: 351160634,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TypeInfoAsset, type_name),
            },
            FieldInfoData {
                name: "IsMeta",
                name_hash: 2816483074,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TypeInfoAsset, is_meta),
            },
            FieldInfoData {
                name: "Attributes",
                name_hash: 3723762538,
                flags: MemberInfoFlags::new(144),
                field_type: "TypeInfoAttribute-Array",
                rust_offset: offset_of!(TypeInfoAsset, attributes),
            },
            FieldInfoData {
                name: "IsNative",
                name_hash: 578033790,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoAsset-Array",
    name_hash: 2575284407,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoAttribute {
    pub _glacier_base: DataContainer,
    pub name: String,
    pub arguments: Vec<BoxedTypeObject /* TypeInfoAttributeArgument */>,
    pub is_native: bool,
}

pub trait TypeInfoAttributeTrait: DataContainerTrait {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn arguments(&self) -> &Vec<BoxedTypeObject /* TypeInfoAttributeArgument */>;
    fn arguments_mut(&mut self) -> &mut Vec<BoxedTypeObject /* TypeInfoAttributeArgument */>;
    fn is_native(&self) -> &bool;
    fn is_native_mut(&mut self) -> &mut bool;
}

impl TypeInfoAttributeTrait for TypeInfoAttribute {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn arguments(&self) -> &Vec<BoxedTypeObject /* TypeInfoAttributeArgument */> {
        &self.arguments
    }
    fn arguments_mut(&mut self) -> &mut Vec<BoxedTypeObject /* TypeInfoAttributeArgument */> {
        &mut self.arguments
    }
    fn is_native(&self) -> &bool {
        &self.is_native
    }
    fn is_native_mut(&mut self) -> &mut bool {
        &mut self.is_native
    }
}

impl DataContainerTrait for TypeInfoAttribute {
}

pub static TYPEINFOATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoAttribute",
    name_hash: 692651663,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(TypeInfoAttribute, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoAttribute as Default>::default())),
            create_boxed: || Box::new(<TypeInfoAttribute as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TypeInfoAttribute, name),
            },
            FieldInfoData {
                name: "Arguments",
                name_hash: 435589477,
                flags: MemberInfoFlags::new(144),
                field_type: "TypeInfoAttributeArgument-Array",
                rust_offset: offset_of!(TypeInfoAttribute, arguments),
            },
            FieldInfoData {
                name: "IsNative",
                name_hash: 578033790,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TYPEINFOATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoAttribute-Array",
    name_hash: 3035947835,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoAttribute"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TypeInfoAttributeArgument {
    pub name: String,
    pub value: String,
}

pub trait TypeInfoAttributeArgumentTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn value(&self) -> &String;
    fn value_mut(&mut self) -> &mut String;
}

impl TypeInfoAttributeArgumentTrait for TypeInfoAttributeArgument {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn value(&self) -> &String {
        &self.value
    }
    fn value_mut(&mut self) -> &mut String {
        &mut self.value
    }
}

pub static TYPEINFOATTRIBUTEARGUMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoAttributeArgument",
    name_hash: 2479041500,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TypeInfoAttributeArgument as Default>::default())),
            create_boxed: || Box::new(<TypeInfoAttributeArgument as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TypeInfoAttributeArgument, name),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static TYPEINFOATTRIBUTEARGUMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoAttributeArgument-Array",
    name_hash: 2123986024,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoAttributeArgument"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FunctionDelegateRef {
    pub type_info: glacier_reflect::builtin::TypeRef,
}

pub trait FunctionDelegateRefTrait: TypeObject {
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef;
    fn type_info_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef;
}

impl FunctionDelegateRefTrait for FunctionDelegateRef {
    fn type_info(&self) -> &glacier_reflect::builtin::TypeRef {
        &self.type_info
    }
    fn type_info_mut(&mut self) -> &mut glacier_reflect::builtin::TypeRef {
        &mut self.type_info
    }
}

pub static FUNCTIONDELEGATEREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FunctionDelegateRef",
    name_hash: 901571689,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FunctionDelegateRef as Default>::default())),
            create_boxed: || Box::new(<FunctionDelegateRef as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TypeInfo",
                name_hash: 351127923,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static FUNCTIONDELEGATEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FunctionDelegateRef-Array",
    name_hash: 1196161885,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FunctionDelegateRef"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EventDelay {
}

pub trait EventDelayTrait: TypeObject {
}

impl EventDelayTrait for EventDelay {
}

pub static EVENTDELAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventDelay",
    name_hash: 3231866844,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EventDelay as Default>::default())),
            create_boxed: || Box::new(<EventDelay as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static EVENTDELAY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventDelay-Array",
    name_hash: 1796188264,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("EventDelay"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn enable_mut(&mut self) -> &mut bool;
    fn frame_count(&self) -> &u32;
    fn frame_count_mut(&mut self) -> &mut u32;
    fn frame_delay_count(&self) -> &u32;
    fn frame_delay_count_mut(&mut self) -> &mut u32;
    fn frame_count_to_report(&self) -> &u32;
    fn frame_count_to_report_mut(&mut self) -> &mut u32;
    fn time_range(&self) -> &f32;
    fn time_range_mut(&mut self) -> &mut f32;
    fn time_offset(&self) -> &f32;
    fn time_offset_mut(&mut self) -> &mut f32;
    fn log_threshold(&self) -> &f32;
    fn log_threshold_mut(&mut self) -> &mut f32;
    fn autolock_threshold(&self) -> &f32;
    fn autolock_threshold_mut(&mut self) -> &mut f32;
    fn autolock_name_filter(&self) -> &String;
    fn autolock_name_filter_mut(&mut self) -> &mut String;
    fn legend_screen_offset(&self) -> &i32;
    fn legend_screen_offset_mut(&mut self) -> &mut i32;
    fn legend_column_width(&self) -> &i32;
    fn legend_column_width_mut(&mut self) -> &mut i32;
    fn legend_display_mode(&self) -> &i32;
    fn legend_display_mode_mut(&mut self) -> &mut i32;
    fn average_frame_count(&self) -> &i32;
    fn average_frame_count_mut(&mut self) -> &mut i32;
    fn draw_enable(&self) -> &bool;
    fn draw_enable_mut(&mut self) -> &mut bool;
    fn draw_bars_enable(&self) -> &bool;
    fn draw_bars_enable_mut(&mut self) -> &mut bool;
    fn draw_legend_enable(&self) -> &bool;
    fn draw_legend_enable_mut(&mut self) -> &mut bool;
    fn draw_cpu_legend_enable(&self) -> &bool;
    fn draw_cpu_legend_enable_mut(&mut self) -> &mut bool;
    fn draw_gpu_legend_enable(&self) -> &bool;
    fn draw_gpu_legend_enable_mut(&mut self) -> &mut bool;
    fn draw_spu_legend_enable(&self) -> &bool;
    fn draw_spu_legend_enable_mut(&mut self) -> &mut bool;
    fn snoop_enable(&self) -> &bool;
    fn snoop_enable_mut(&mut self) -> &mut bool;
    fn snoop_only(&self) -> &bool;
    fn snoop_only_mut(&mut self) -> &mut bool;
    fn snoop_event_name(&self) -> &String;
    fn snoop_event_name_mut(&mut self) -> &mut String;
    fn sort_by_time(&self) -> &bool;
    fn sort_by_time_mut(&mut self) -> &mut bool;
    fn filter_enable(&self) -> &bool;
    fn filter_enable_mut(&mut self) -> &mut bool;
    fn filter_event_name(&self) -> &String;
    fn filter_event_name_mut(&mut self) -> &mut String;
    fn bar_min_time(&self) -> &f64;
    fn bar_min_time_mut(&mut self) -> &mut f64;
    fn bar_height(&self) -> &u32;
    fn bar_height_mut(&mut self) -> &mut u32;
    fn bar_pad(&self) -> &u32;
    fn bar_pad_mut(&mut self) -> &mut u32;
    fn bar_sync_processor(&self) -> &i32;
    fn bar_sync_processor_mut(&mut self) -> &mut i32;
    fn max_cpu_legend_column_count(&self) -> &i32;
    fn max_cpu_legend_column_count_mut(&mut self) -> &mut i32;
    fn max_gpu_legend_column_count(&self) -> &i32;
    fn max_gpu_legend_column_count_mut(&mut self) -> &mut i32;
    fn max_spu_legend_column_count(&self) -> &i32;
    fn max_spu_legend_column_count_mut(&mut self) -> &mut i32;
    fn max_frame_event_count(&self) -> &u32;
    fn max_frame_event_count_mut(&mut self) -> &mut u32;
    fn collection_enable(&self) -> &bool;
    fn collection_enable_mut(&mut self) -> &mut bool;
}

impl TimingViewSettingsTrait for TimingViewSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn frame_count(&self) -> &u32 {
        &self.frame_count
    }
    fn frame_count_mut(&mut self) -> &mut u32 {
        &mut self.frame_count
    }
    fn frame_delay_count(&self) -> &u32 {
        &self.frame_delay_count
    }
    fn frame_delay_count_mut(&mut self) -> &mut u32 {
        &mut self.frame_delay_count
    }
    fn frame_count_to_report(&self) -> &u32 {
        &self.frame_count_to_report
    }
    fn frame_count_to_report_mut(&mut self) -> &mut u32 {
        &mut self.frame_count_to_report
    }
    fn time_range(&self) -> &f32 {
        &self.time_range
    }
    fn time_range_mut(&mut self) -> &mut f32 {
        &mut self.time_range
    }
    fn time_offset(&self) -> &f32 {
        &self.time_offset
    }
    fn time_offset_mut(&mut self) -> &mut f32 {
        &mut self.time_offset
    }
    fn log_threshold(&self) -> &f32 {
        &self.log_threshold
    }
    fn log_threshold_mut(&mut self) -> &mut f32 {
        &mut self.log_threshold
    }
    fn autolock_threshold(&self) -> &f32 {
        &self.autolock_threshold
    }
    fn autolock_threshold_mut(&mut self) -> &mut f32 {
        &mut self.autolock_threshold
    }
    fn autolock_name_filter(&self) -> &String {
        &self.autolock_name_filter
    }
    fn autolock_name_filter_mut(&mut self) -> &mut String {
        &mut self.autolock_name_filter
    }
    fn legend_screen_offset(&self) -> &i32 {
        &self.legend_screen_offset
    }
    fn legend_screen_offset_mut(&mut self) -> &mut i32 {
        &mut self.legend_screen_offset
    }
    fn legend_column_width(&self) -> &i32 {
        &self.legend_column_width
    }
    fn legend_column_width_mut(&mut self) -> &mut i32 {
        &mut self.legend_column_width
    }
    fn legend_display_mode(&self) -> &i32 {
        &self.legend_display_mode
    }
    fn legend_display_mode_mut(&mut self) -> &mut i32 {
        &mut self.legend_display_mode
    }
    fn average_frame_count(&self) -> &i32 {
        &self.average_frame_count
    }
    fn average_frame_count_mut(&mut self) -> &mut i32 {
        &mut self.average_frame_count
    }
    fn draw_enable(&self) -> &bool {
        &self.draw_enable
    }
    fn draw_enable_mut(&mut self) -> &mut bool {
        &mut self.draw_enable
    }
    fn draw_bars_enable(&self) -> &bool {
        &self.draw_bars_enable
    }
    fn draw_bars_enable_mut(&mut self) -> &mut bool {
        &mut self.draw_bars_enable
    }
    fn draw_legend_enable(&self) -> &bool {
        &self.draw_legend_enable
    }
    fn draw_legend_enable_mut(&mut self) -> &mut bool {
        &mut self.draw_legend_enable
    }
    fn draw_cpu_legend_enable(&self) -> &bool {
        &self.draw_cpu_legend_enable
    }
    fn draw_cpu_legend_enable_mut(&mut self) -> &mut bool {
        &mut self.draw_cpu_legend_enable
    }
    fn draw_gpu_legend_enable(&self) -> &bool {
        &self.draw_gpu_legend_enable
    }
    fn draw_gpu_legend_enable_mut(&mut self) -> &mut bool {
        &mut self.draw_gpu_legend_enable
    }
    fn draw_spu_legend_enable(&self) -> &bool {
        &self.draw_spu_legend_enable
    }
    fn draw_spu_legend_enable_mut(&mut self) -> &mut bool {
        &mut self.draw_spu_legend_enable
    }
    fn snoop_enable(&self) -> &bool {
        &self.snoop_enable
    }
    fn snoop_enable_mut(&mut self) -> &mut bool {
        &mut self.snoop_enable
    }
    fn snoop_only(&self) -> &bool {
        &self.snoop_only
    }
    fn snoop_only_mut(&mut self) -> &mut bool {
        &mut self.snoop_only
    }
    fn snoop_event_name(&self) -> &String {
        &self.snoop_event_name
    }
    fn snoop_event_name_mut(&mut self) -> &mut String {
        &mut self.snoop_event_name
    }
    fn sort_by_time(&self) -> &bool {
        &self.sort_by_time
    }
    fn sort_by_time_mut(&mut self) -> &mut bool {
        &mut self.sort_by_time
    }
    fn filter_enable(&self) -> &bool {
        &self.filter_enable
    }
    fn filter_enable_mut(&mut self) -> &mut bool {
        &mut self.filter_enable
    }
    fn filter_event_name(&self) -> &String {
        &self.filter_event_name
    }
    fn filter_event_name_mut(&mut self) -> &mut String {
        &mut self.filter_event_name
    }
    fn bar_min_time(&self) -> &f64 {
        &self.bar_min_time
    }
    fn bar_min_time_mut(&mut self) -> &mut f64 {
        &mut self.bar_min_time
    }
    fn bar_height(&self) -> &u32 {
        &self.bar_height
    }
    fn bar_height_mut(&mut self) -> &mut u32 {
        &mut self.bar_height
    }
    fn bar_pad(&self) -> &u32 {
        &self.bar_pad
    }
    fn bar_pad_mut(&mut self) -> &mut u32 {
        &mut self.bar_pad
    }
    fn bar_sync_processor(&self) -> &i32 {
        &self.bar_sync_processor
    }
    fn bar_sync_processor_mut(&mut self) -> &mut i32 {
        &mut self.bar_sync_processor
    }
    fn max_cpu_legend_column_count(&self) -> &i32 {
        &self.max_cpu_legend_column_count
    }
    fn max_cpu_legend_column_count_mut(&mut self) -> &mut i32 {
        &mut self.max_cpu_legend_column_count
    }
    fn max_gpu_legend_column_count(&self) -> &i32 {
        &self.max_gpu_legend_column_count
    }
    fn max_gpu_legend_column_count_mut(&mut self) -> &mut i32 {
        &mut self.max_gpu_legend_column_count
    }
    fn max_spu_legend_column_count(&self) -> &i32 {
        &self.max_spu_legend_column_count
    }
    fn max_spu_legend_column_count_mut(&mut self) -> &mut i32 {
        &mut self.max_spu_legend_column_count
    }
    fn max_frame_event_count(&self) -> &u32 {
        &self.max_frame_event_count
    }
    fn max_frame_event_count_mut(&mut self) -> &mut u32 {
        &mut self.max_frame_event_count
    }
    fn collection_enable(&self) -> &bool {
        &self.collection_enable
    }
    fn collection_enable_mut(&mut self) -> &mut bool {
        &mut self.collection_enable
    }
}

impl DataContainerTrait for TimingViewSettings {
}

pub static TIMINGVIEWSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimingViewSettings",
    name_hash: 279163933,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(TimingViewSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TimingViewSettings as Default>::default())),
            create_boxed: || Box::new(<TimingViewSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                name_hash: 2342790116,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, enable),
            },
            FieldInfoData {
                name: "FrameCount",
                name_hash: 741182715,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TimingViewSettings, frame_count),
            },
            FieldInfoData {
                name: "FrameDelayCount",
                name_hash: 4056595598,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TimingViewSettings, frame_delay_count),
            },
            FieldInfoData {
                name: "FrameCountToReport",
                name_hash: 2403515438,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TimingViewSettings, frame_count_to_report),
            },
            FieldInfoData {
                name: "TimeRange",
                name_hash: 170778575,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TimingViewSettings, time_range),
            },
            FieldInfoData {
                name: "TimeOffset",
                name_hash: 2388918461,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TimingViewSettings, time_offset),
            },
            FieldInfoData {
                name: "LogThreshold",
                name_hash: 267133270,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TimingViewSettings, log_threshold),
            },
            FieldInfoData {
                name: "AutolockThreshold",
                name_hash: 2834654070,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TimingViewSettings, autolock_threshold),
            },
            FieldInfoData {
                name: "AutolockNameFilter",
                name_hash: 700307558,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TimingViewSettings, autolock_name_filter),
            },
            FieldInfoData {
                name: "LegendScreenOffset",
                name_hash: 2099355397,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TimingViewSettings, legend_screen_offset),
            },
            FieldInfoData {
                name: "LegendColumnWidth",
                name_hash: 4015483860,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TimingViewSettings, legend_column_width),
            },
            FieldInfoData {
                name: "LegendDisplayMode",
                name_hash: 3524433245,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TimingViewSettings, legend_display_mode),
            },
            FieldInfoData {
                name: "AverageFrameCount",
                name_hash: 4155437432,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TimingViewSettings, average_frame_count),
            },
            FieldInfoData {
                name: "DrawEnable",
                name_hash: 1347356004,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, draw_enable),
            },
            FieldInfoData {
                name: "DrawBarsEnable",
                name_hash: 3694545030,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, draw_bars_enable),
            },
            FieldInfoData {
                name: "DrawLegendEnable",
                name_hash: 1644114661,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, draw_legend_enable),
            },
            FieldInfoData {
                name: "DrawCpuLegendEnable",
                name_hash: 450915267,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, draw_cpu_legend_enable),
            },
            FieldInfoData {
                name: "DrawGpuLegendEnable",
                name_hash: 591363015,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, draw_gpu_legend_enable),
            },
            FieldInfoData {
                name: "DrawSpuLegendEnable",
                name_hash: 3543221203,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, draw_spu_legend_enable),
            },
            FieldInfoData {
                name: "SnoopEnable",
                name_hash: 332594601,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, snoop_enable),
            },
            FieldInfoData {
                name: "SnoopOnly",
                name_hash: 477460828,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, snoop_only),
            },
            FieldInfoData {
                name: "SnoopEventName",
                name_hash: 570566851,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TimingViewSettings, snoop_event_name),
            },
            FieldInfoData {
                name: "SortByTime",
                name_hash: 1839336369,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, sort_by_time),
            },
            FieldInfoData {
                name: "FilterEnable",
                name_hash: 3231420132,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimingViewSettings, filter_enable),
            },
            FieldInfoData {
                name: "FilterEventName",
                name_hash: 1452602030,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TimingViewSettings, filter_event_name),
            },
            FieldInfoData {
                name: "BarMinTime",
                name_hash: 1638553323,
                flags: MemberInfoFlags::new(0),
                field_type: "Float64",
                rust_offset: offset_of!(TimingViewSettings, bar_min_time),
            },
            FieldInfoData {
                name: "BarHeight",
                name_hash: 2196900171,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TimingViewSettings, bar_height),
            },
            FieldInfoData {
                name: "BarPad",
                name_hash: 2672961185,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TimingViewSettings, bar_pad),
            },
            FieldInfoData {
                name: "BarSyncProcessor",
                name_hash: 2518052805,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TimingViewSettings, bar_sync_processor),
            },
            FieldInfoData {
                name: "MaxCpuLegendColumnCount",
                name_hash: 3144102947,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TimingViewSettings, max_cpu_legend_column_count),
            },
            FieldInfoData {
                name: "MaxGpuLegendColumnCount",
                name_hash: 1835173031,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TimingViewSettings, max_gpu_legend_column_count),
            },
            FieldInfoData {
                name: "MaxSpuLegendColumnCount",
                name_hash: 3652642355,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TimingViewSettings, max_spu_legend_column_count),
            },
            FieldInfoData {
                name: "MaxFrameEventCount",
                name_hash: 3847876579,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TimingViewSettings, max_frame_event_count),
            },
            FieldInfoData {
                name: "CollectionEnable",
                name_hash: 2029381618,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TIMINGVIEWSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimingViewSettings-Array",
    name_hash: 89522089,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TimingViewSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn enable_mut(&mut self) -> &mut bool;
    fn journal_expensive_stats(&self) -> &bool;
    fn journal_expensive_stats_mut(&mut self) -> &mut bool;
    fn journal_all_s_p_u(&self) -> &bool;
    fn journal_all_s_p_u_mut(&mut self) -> &mut bool;
    fn journal_sample_interval(&self) -> &i32;
    fn journal_sample_interval_mut(&mut self) -> &mut i32;
    fn journal_report_average(&self) -> &bool;
    fn journal_report_average_mut(&mut self) -> &mut bool;
    fn journal_frame_count(&self) -> &i32;
    fn journal_frame_count_mut(&mut self) -> &mut i32;
    fn auto_journal_enable(&self) -> &bool;
    fn auto_journal_enable_mut(&mut self) -> &mut bool;
    fn auto_journal_screenshot(&self) -> &bool;
    fn auto_journal_screenshot_mut(&mut self) -> &mut bool;
    fn auto_journal_threshold_ms(&self) -> &f32;
    fn auto_journal_threshold_ms_mut(&mut self) -> &mut f32;
    fn auto_journal_min_frames(&self) -> &i32;
    fn auto_journal_min_frames_mut(&mut self) -> &mut i32;
    fn trace_enable(&self) -> &bool;
    fn trace_enable_mut(&mut self) -> &mut bool;
    fn auto_journal_continuous_capture(&self) -> &bool;
    fn auto_journal_continuous_capture_mut(&mut self) -> &mut bool;
    fn floats_always_have_decimal(&self) -> &bool;
    fn floats_always_have_decimal_mut(&mut self) -> &mut bool;
}

impl PerfJournalSettingsTrait for PerfJournalSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn journal_expensive_stats(&self) -> &bool {
        &self.journal_expensive_stats
    }
    fn journal_expensive_stats_mut(&mut self) -> &mut bool {
        &mut self.journal_expensive_stats
    }
    fn journal_all_s_p_u(&self) -> &bool {
        &self.journal_all_s_p_u
    }
    fn journal_all_s_p_u_mut(&mut self) -> &mut bool {
        &mut self.journal_all_s_p_u
    }
    fn journal_sample_interval(&self) -> &i32 {
        &self.journal_sample_interval
    }
    fn journal_sample_interval_mut(&mut self) -> &mut i32 {
        &mut self.journal_sample_interval
    }
    fn journal_report_average(&self) -> &bool {
        &self.journal_report_average
    }
    fn journal_report_average_mut(&mut self) -> &mut bool {
        &mut self.journal_report_average
    }
    fn journal_frame_count(&self) -> &i32 {
        &self.journal_frame_count
    }
    fn journal_frame_count_mut(&mut self) -> &mut i32 {
        &mut self.journal_frame_count
    }
    fn auto_journal_enable(&self) -> &bool {
        &self.auto_journal_enable
    }
    fn auto_journal_enable_mut(&mut self) -> &mut bool {
        &mut self.auto_journal_enable
    }
    fn auto_journal_screenshot(&self) -> &bool {
        &self.auto_journal_screenshot
    }
    fn auto_journal_screenshot_mut(&mut self) -> &mut bool {
        &mut self.auto_journal_screenshot
    }
    fn auto_journal_threshold_ms(&self) -> &f32 {
        &self.auto_journal_threshold_ms
    }
    fn auto_journal_threshold_ms_mut(&mut self) -> &mut f32 {
        &mut self.auto_journal_threshold_ms
    }
    fn auto_journal_min_frames(&self) -> &i32 {
        &self.auto_journal_min_frames
    }
    fn auto_journal_min_frames_mut(&mut self) -> &mut i32 {
        &mut self.auto_journal_min_frames
    }
    fn trace_enable(&self) -> &bool {
        &self.trace_enable
    }
    fn trace_enable_mut(&mut self) -> &mut bool {
        &mut self.trace_enable
    }
    fn auto_journal_continuous_capture(&self) -> &bool {
        &self.auto_journal_continuous_capture
    }
    fn auto_journal_continuous_capture_mut(&mut self) -> &mut bool {
        &mut self.auto_journal_continuous_capture
    }
    fn floats_always_have_decimal(&self) -> &bool {
        &self.floats_always_have_decimal
    }
    fn floats_always_have_decimal_mut(&mut self) -> &mut bool {
        &mut self.floats_always_have_decimal
    }
}

impl DataContainerTrait for PerfJournalSettings {
}

pub static PERFJOURNALSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfJournalSettings",
    name_hash: 2895838304,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(PerfJournalSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PerfJournalSettings as Default>::default())),
            create_boxed: || Box::new(<PerfJournalSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                name_hash: 2342790116,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalSettings, enable),
            },
            FieldInfoData {
                name: "JournalExpensiveStats",
                name_hash: 1674963338,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalSettings, journal_expensive_stats),
            },
            FieldInfoData {
                name: "JournalAllSPU",
                name_hash: 3815279827,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalSettings, journal_all_s_p_u),
            },
            FieldInfoData {
                name: "JournalSampleInterval",
                name_hash: 3738154557,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PerfJournalSettings, journal_sample_interval),
            },
            FieldInfoData {
                name: "JournalReportAverage",
                name_hash: 943410281,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalSettings, journal_report_average),
            },
            FieldInfoData {
                name: "JournalFrameCount",
                name_hash: 1200953114,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PerfJournalSettings, journal_frame_count),
            },
            FieldInfoData {
                name: "AutoJournalEnable",
                name_hash: 2365091754,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalSettings, auto_journal_enable),
            },
            FieldInfoData {
                name: "AutoJournalScreenshot",
                name_hash: 2032220711,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalSettings, auto_journal_screenshot),
            },
            FieldInfoData {
                name: "AutoJournalThresholdMs",
                name_hash: 4146079490,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PerfJournalSettings, auto_journal_threshold_ms),
            },
            FieldInfoData {
                name: "AutoJournalMinFrames",
                name_hash: 1480205103,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PerfJournalSettings, auto_journal_min_frames),
            },
            FieldInfoData {
                name: "TraceEnable",
                name_hash: 848654053,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalSettings, trace_enable),
            },
            FieldInfoData {
                name: "AutoJournalContinuousCapture",
                name_hash: 4039757442,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfJournalSettings, auto_journal_continuous_capture),
            },
            FieldInfoData {
                name: "FloatsAlwaysHaveDecimal",
                name_hash: 4073052774,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PERFJOURNALSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfJournalSettings-Array",
    name_hash: 955331156,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PerfJournalSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn enable_mut(&mut self) -> &mut bool;
    fn draw_cpu_enable(&self) -> &bool;
    fn draw_cpu_enable_mut(&mut self) -> &mut bool;
    fn draw_gpu_enable(&self) -> &bool;
    fn draw_gpu_enable_mut(&mut self) -> &mut bool;
    fn draw_spu_enable(&self) -> &bool;
    fn draw_spu_enable_mut(&mut self) -> &mut bool;
    fn simple_summary_mode(&self) -> &bool;
    fn simple_summary_mode_mut(&mut self) -> &mut bool;
    fn timer_category_set_name(&self) -> &String;
    fn timer_category_set_name_mut(&mut self) -> &mut String;
    fn timer_category_sets_enable(&self) -> &bool;
    fn timer_category_sets_enable_mut(&mut self) -> &mut bool;
    fn draw_ungrouped_timings(&self) -> &bool;
    fn draw_ungrouped_timings_mut(&mut self) -> &mut bool;
    fn draw_config_file(&self) -> &bool;
    fn draw_config_file_mut(&mut self) -> &mut bool;
    fn hud_right_margin(&self) -> &i32;
    fn hud_right_margin_mut(&mut self) -> &mut i32;
    fn hud_top_margin(&self) -> &i32;
    fn hud_top_margin_mut(&mut self) -> &mut i32;
    fn hud_alpha(&self) -> &u8;
    fn hud_alpha_mut(&mut self) -> &mut u8;
    fn hud_compact(&self) -> &bool;
    fn hud_compact_mut(&mut self) -> &mut bool;
}

impl PerfHudSettingsTrait for PerfHudSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn draw_cpu_enable(&self) -> &bool {
        &self.draw_cpu_enable
    }
    fn draw_cpu_enable_mut(&mut self) -> &mut bool {
        &mut self.draw_cpu_enable
    }
    fn draw_gpu_enable(&self) -> &bool {
        &self.draw_gpu_enable
    }
    fn draw_gpu_enable_mut(&mut self) -> &mut bool {
        &mut self.draw_gpu_enable
    }
    fn draw_spu_enable(&self) -> &bool {
        &self.draw_spu_enable
    }
    fn draw_spu_enable_mut(&mut self) -> &mut bool {
        &mut self.draw_spu_enable
    }
    fn simple_summary_mode(&self) -> &bool {
        &self.simple_summary_mode
    }
    fn simple_summary_mode_mut(&mut self) -> &mut bool {
        &mut self.simple_summary_mode
    }
    fn timer_category_set_name(&self) -> &String {
        &self.timer_category_set_name
    }
    fn timer_category_set_name_mut(&mut self) -> &mut String {
        &mut self.timer_category_set_name
    }
    fn timer_category_sets_enable(&self) -> &bool {
        &self.timer_category_sets_enable
    }
    fn timer_category_sets_enable_mut(&mut self) -> &mut bool {
        &mut self.timer_category_sets_enable
    }
    fn draw_ungrouped_timings(&self) -> &bool {
        &self.draw_ungrouped_timings
    }
    fn draw_ungrouped_timings_mut(&mut self) -> &mut bool {
        &mut self.draw_ungrouped_timings
    }
    fn draw_config_file(&self) -> &bool {
        &self.draw_config_file
    }
    fn draw_config_file_mut(&mut self) -> &mut bool {
        &mut self.draw_config_file
    }
    fn hud_right_margin(&self) -> &i32 {
        &self.hud_right_margin
    }
    fn hud_right_margin_mut(&mut self) -> &mut i32 {
        &mut self.hud_right_margin
    }
    fn hud_top_margin(&self) -> &i32 {
        &self.hud_top_margin
    }
    fn hud_top_margin_mut(&mut self) -> &mut i32 {
        &mut self.hud_top_margin
    }
    fn hud_alpha(&self) -> &u8 {
        &self.hud_alpha
    }
    fn hud_alpha_mut(&mut self) -> &mut u8 {
        &mut self.hud_alpha
    }
    fn hud_compact(&self) -> &bool {
        &self.hud_compact
    }
    fn hud_compact_mut(&mut self) -> &mut bool {
        &mut self.hud_compact
    }
}

impl DataContainerTrait for PerfHudSettings {
}

pub static PERFHUDSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfHudSettings",
    name_hash: 88833368,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(PerfHudSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PerfHudSettings as Default>::default())),
            create_boxed: || Box::new(<PerfHudSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                name_hash: 2342790116,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfHudSettings, enable),
            },
            FieldInfoData {
                name: "DrawCpuEnable",
                name_hash: 1628031874,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfHudSettings, draw_cpu_enable),
            },
            FieldInfoData {
                name: "DrawGpuEnable",
                name_hash: 3838929798,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfHudSettings, draw_gpu_enable),
            },
            FieldInfoData {
                name: "DrawSpuEnable",
                name_hash: 3218986898,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfHudSettings, draw_spu_enable),
            },
            FieldInfoData {
                name: "SimpleSummaryMode",
                name_hash: 200277124,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfHudSettings, simple_summary_mode),
            },
            FieldInfoData {
                name: "TimerCategorySetName",
                name_hash: 4001058103,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(PerfHudSettings, timer_category_set_name),
            },
            FieldInfoData {
                name: "TimerCategorySetsEnable",
                name_hash: 1575667170,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfHudSettings, timer_category_sets_enable),
            },
            FieldInfoData {
                name: "DrawUngroupedTimings",
                name_hash: 935600323,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfHudSettings, draw_ungrouped_timings),
            },
            FieldInfoData {
                name: "DrawConfigFile",
                name_hash: 1611334409,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PerfHudSettings, draw_config_file),
            },
            FieldInfoData {
                name: "HudRightMargin",
                name_hash: 3509999682,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PerfHudSettings, hud_right_margin),
            },
            FieldInfoData {
                name: "HudTopMargin",
                name_hash: 354091593,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PerfHudSettings, hud_top_margin),
            },
            FieldInfoData {
                name: "HudAlpha",
                name_hash: 1217473736,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(PerfHudSettings, hud_alpha),
            },
            FieldInfoData {
                name: "HudCompact",
                name_hash: 1823758555,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PERFHUDSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfHudSettings-Array",
    name_hash: 2134456300,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PerfHudSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreLogUpdateSettingsMessage {
}

pub trait CoreLogUpdateSettingsMessageTrait: TypeObject {
}

impl CoreLogUpdateSettingsMessageTrait for CoreLogUpdateSettingsMessage {
}

pub static CORELOGUPDATESETTINGSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreLogUpdateSettingsMessage",
    name_hash: 577721221,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreLogUpdateSettingsMessage as Default>::default())),
            create_boxed: || Box::new(<CoreLogUpdateSettingsMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct SplitScreenSetPrimaryLocalPlayerIdMessage {
}

pub trait SplitScreenSetPrimaryLocalPlayerIdMessageTrait: TypeObject {
}

impl SplitScreenSetPrimaryLocalPlayerIdMessageTrait for SplitScreenSetPrimaryLocalPlayerIdMessage {
}

pub static SPLITSCREENSETPRIMARYLOCALPLAYERIDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SplitScreenSetPrimaryLocalPlayerIdMessage",
    name_hash: 3491929933,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SplitScreenSetPrimaryLocalPlayerIdMessage as Default>::default())),
            create_boxed: || Box::new(<SplitScreenSetPrimaryLocalPlayerIdMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct StreamInstallRequestResumeMessage {
}

pub trait StreamInstallRequestResumeMessageTrait: TypeObject {
}

impl StreamInstallRequestResumeMessageTrait for StreamInstallRequestResumeMessage {
}

pub static STREAMINSTALLREQUESTRESUMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallRequestResumeMessage",
    name_hash: 3533591931,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamInstallRequestResumeMessage as Default>::default())),
            create_boxed: || Box::new(<StreamInstallRequestResumeMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct StreamInstallRequestSuspendMessage {
}

pub trait StreamInstallRequestSuspendMessageTrait: TypeObject {
}

impl StreamInstallRequestSuspendMessageTrait for StreamInstallRequestSuspendMessage {
}

pub static STREAMINSTALLREQUESTSUSPENDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallRequestSuspendMessage",
    name_hash: 2306105512,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamInstallRequestSuspendMessage as Default>::default())),
            create_boxed: || Box::new(<StreamInstallRequestSuspendMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct StreamInstallGroupInstallationProgressMessage {
}

pub trait StreamInstallGroupInstallationProgressMessageTrait: TypeObject {
}

impl StreamInstallGroupInstallationProgressMessageTrait for StreamInstallGroupInstallationProgressMessage {
}

pub static STREAMINSTALLGROUPINSTALLATIONPROGRESSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallGroupInstallationProgressMessage",
    name_hash: 3222622413,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamInstallGroupInstallationProgressMessage as Default>::default())),
            create_boxed: || Box::new(<StreamInstallGroupInstallationProgressMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct StreamInstallGroupInstalledMessage {
}

pub trait StreamInstallGroupInstalledMessageTrait: TypeObject {
}

impl StreamInstallGroupInstalledMessageTrait for StreamInstallGroupInstalledMessage {
}

pub static STREAMINSTALLGROUPINSTALLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallGroupInstalledMessage",
    name_hash: 4236237356,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamInstallGroupInstalledMessage as Default>::default())),
            create_boxed: || Box::new(<StreamInstallGroupInstalledMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct StreamInstallInstallationProgressMessage {
}

pub trait StreamInstallInstallationProgressMessageTrait: TypeObject {
}

impl StreamInstallInstallationProgressMessageTrait for StreamInstallInstallationProgressMessage {
}

pub static STREAMINSTALLINSTALLATIONPROGRESSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallInstallationProgressMessage",
    name_hash: 1057555858,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamInstallInstallationProgressMessage as Default>::default())),
            create_boxed: || Box::new(<StreamInstallInstallationProgressMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct StreamInstallInstallationFailedMessage {
}

pub trait StreamInstallInstallationFailedMessageTrait: TypeObject {
}

impl StreamInstallInstallationFailedMessageTrait for StreamInstallInstallationFailedMessage {
}

pub static STREAMINSTALLINSTALLATIONFAILEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallInstallationFailedMessage",
    name_hash: 3289399500,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamInstallInstallationFailedMessage as Default>::default())),
            create_boxed: || Box::new(<StreamInstallInstallationFailedMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct StreamInstallInstallationCompletedMessage {
}

pub trait StreamInstallInstallationCompletedMessageTrait: TypeObject {
}

impl StreamInstallInstallationCompletedMessageTrait for StreamInstallInstallationCompletedMessage {
}

pub static STREAMINSTALLINSTALLATIONCOMPLETEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallInstallationCompletedMessage",
    name_hash: 101241602,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamInstallInstallationCompletedMessage as Default>::default())),
            create_boxed: || Box::new(<StreamInstallInstallationCompletedMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreApplicationActivationMessage {
}

pub trait CoreApplicationActivationMessageTrait: TypeObject {
}

impl CoreApplicationActivationMessageTrait for CoreApplicationActivationMessage {
}

pub static COREAPPLICATIONACTIVATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreApplicationActivationMessage",
    name_hash: 2815310683,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreApplicationActivationMessage as Default>::default())),
            create_boxed: || Box::new(<CoreApplicationActivationMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
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
    name_hash: 317771708,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static APPLICATIONACTIVATIONSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ApplicationActivationState-Array",
    name_hash: 96356104,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ApplicationActivationState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreCleanupMessage {
}

pub trait CoreCleanupMessageTrait: TypeObject {
}

impl CoreCleanupMessageTrait for CoreCleanupMessage {
}

pub static CORECLEANUPMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreCleanupMessage",
    name_hash: 2831461877,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreCleanupMessage as Default>::default())),
            create_boxed: || Box::new(<CoreCleanupMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreSublevelStartStreamOutMessage {
}

pub trait CoreSublevelStartStreamOutMessageTrait: TypeObject {
}

impl CoreSublevelStartStreamOutMessageTrait for CoreSublevelStartStreamOutMessage {
}

pub static CORESUBLEVELSTARTSTREAMOUTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreSublevelStartStreamOutMessage",
    name_hash: 1419526837,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreSublevelStartStreamOutMessage as Default>::default())),
            create_boxed: || Box::new(<CoreSublevelStartStreamOutMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct CorePanicMessage {
}

pub trait CorePanicMessageTrait: TypeObject {
}

impl CorePanicMessageTrait for CorePanicMessage {
}

pub static COREPANICMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CorePanicMessage",
    name_hash: 2116451264,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CorePanicMessage as Default>::default())),
            create_boxed: || Box::new(<CorePanicMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreQuitMessage {
}

pub trait CoreQuitMessageTrait: TypeObject {
}

impl CoreQuitMessageTrait for CoreQuitMessage {
}

pub static COREQUITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreQuitMessage",
    name_hash: 3910396076,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreQuitMessage as Default>::default())),
            create_boxed: || Box::new(<CoreQuitMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreUpdateClipboardMessage {
}

pub trait CoreUpdateClipboardMessageTrait: TypeObject {
}

impl CoreUpdateClipboardMessageTrait for CoreUpdateClipboardMessage {
}

pub static COREUPDATECLIPBOARDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreUpdateClipboardMessage",
    name_hash: 3654911496,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreUpdateClipboardMessage as Default>::default())),
            create_boxed: || Box::new(<CoreUpdateClipboardMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreMainThreadInitMessage {
}

pub trait CoreMainThreadInitMessageTrait: TypeObject {
}

impl CoreMainThreadInitMessageTrait for CoreMainThreadInitMessage {
}

pub static COREMAINTHREADINITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreMainThreadInitMessage",
    name_hash: 3728023242,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreMainThreadInitMessage as Default>::default())),
            create_boxed: || Box::new(<CoreMainThreadInitMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreHibernateMessage {
}

pub trait CoreHibernateMessageTrait: TypeObject {
}

impl CoreHibernateMessageTrait for CoreHibernateMessage {
}

pub static COREHIBERNATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreHibernateMessage",
    name_hash: 4193118591,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreHibernateMessage as Default>::default())),
            create_boxed: || Box::new(<CoreHibernateMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct CoreQuittingInitiatedMessage {
}

pub trait CoreQuittingInitiatedMessageTrait: TypeObject {
}

impl CoreQuittingInitiatedMessageTrait for CoreQuittingInitiatedMessage {
}

pub static COREQUITTINGINITIATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreQuittingInitiatedMessage",
    name_hash: 3242141247,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CoreQuittingInitiatedMessage as Default>::default())),
            create_boxed: || Box::new(<CoreQuittingInitiatedMessage as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct SparseTransformArray {
    pub indices: Vec<u16>,
    pub transforms: Vec<BoxedTypeObject /* LinearTransform */>,
    pub count: u32,
}

pub trait SparseTransformArrayTrait: TypeObject {
    fn indices(&self) -> &Vec<u16>;
    fn indices_mut(&mut self) -> &mut Vec<u16>;
    fn transforms(&self) -> &Vec<BoxedTypeObject /* LinearTransform */>;
    fn transforms_mut(&mut self) -> &mut Vec<BoxedTypeObject /* LinearTransform */>;
    fn count(&self) -> &u32;
    fn count_mut(&mut self) -> &mut u32;
}

impl SparseTransformArrayTrait for SparseTransformArray {
    fn indices(&self) -> &Vec<u16> {
        &self.indices
    }
    fn indices_mut(&mut self) -> &mut Vec<u16> {
        &mut self.indices
    }
    fn transforms(&self) -> &Vec<BoxedTypeObject /* LinearTransform */> {
        &self.transforms
    }
    fn transforms_mut(&mut self) -> &mut Vec<BoxedTypeObject /* LinearTransform */> {
        &mut self.transforms
    }
    fn count(&self) -> &u32 {
        &self.count
    }
    fn count_mut(&mut self) -> &mut u32 {
        &mut self.count
    }
}

pub static SPARSETRANSFORMARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SparseTransformArray",
    name_hash: 2155854710,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SparseTransformArray as Default>::default())),
            create_boxed: || Box::new(<SparseTransformArray as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Indices",
                name_hash: 1672980602,
                flags: MemberInfoFlags::new(144),
                field_type: "Uint16-Array",
                rust_offset: offset_of!(SparseTransformArray, indices),
            },
            FieldInfoData {
                name: "Transforms",
                name_hash: 1906106874,
                flags: MemberInfoFlags::new(144),
                field_type: "LinearTransform-Array",
                rust_offset: offset_of!(SparseTransformArray, transforms),
            },
            FieldInfoData {
                name: "Count",
                name_hash: 212413894,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static SPARSETRANSFORMARRAY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SparseTransformArray-Array",
    name_hash: 2779242562,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SparseTransformArray"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn x_values0_mut(&mut self) -> &mut Vec4;
    fn x_values1(&self) -> &Vec4;
    fn x_values1_mut(&mut self) -> &mut Vec4;
    fn x_values2(&self) -> &Vec4;
    fn x_values2_mut(&mut self) -> &mut Vec4;
    fn y_values0(&self) -> &Vec4;
    fn y_values0_mut(&mut self) -> &mut Vec4;
    fn y_values1(&self) -> &Vec4;
    fn y_values1_mut(&mut self) -> &mut Vec4;
    fn y_values2(&self) -> &Vec4;
    fn y_values2_mut(&mut self) -> &mut Vec4;
    fn y_values3(&self) -> &Vec4;
    fn y_values3_mut(&mut self) -> &mut Vec4;
    fn g_values0(&self) -> &Vec4;
    fn g_values0_mut(&mut self) -> &mut Vec4;
    fn g_values1(&self) -> &Vec4;
    fn g_values1_mut(&mut self) -> &mut Vec4;
    fn g_values2(&self) -> &Vec4;
    fn g_values2_mut(&mut self) -> &mut Vec4;
    fn g_values3(&self) -> &Vec4;
    fn g_values3_mut(&mut self) -> &mut Vec4;
    fn g_values4(&self) -> &Vec4;
    fn g_values4_mut(&mut self) -> &mut Vec4;
    fn g_values5(&self) -> &Vec4;
    fn g_values5_mut(&mut self) -> &mut Vec4;
    fn spline_type(&self) -> &SplineType;
    fn spline_type_mut(&mut self) -> &mut SplineType;
}

impl SplineCurveTrait for SplineCurve {
    fn x_values0(&self) -> &Vec4 {
        &self.x_values0
    }
    fn x_values0_mut(&mut self) -> &mut Vec4 {
        &mut self.x_values0
    }
    fn x_values1(&self) -> &Vec4 {
        &self.x_values1
    }
    fn x_values1_mut(&mut self) -> &mut Vec4 {
        &mut self.x_values1
    }
    fn x_values2(&self) -> &Vec4 {
        &self.x_values2
    }
    fn x_values2_mut(&mut self) -> &mut Vec4 {
        &mut self.x_values2
    }
    fn y_values0(&self) -> &Vec4 {
        &self.y_values0
    }
    fn y_values0_mut(&mut self) -> &mut Vec4 {
        &mut self.y_values0
    }
    fn y_values1(&self) -> &Vec4 {
        &self.y_values1
    }
    fn y_values1_mut(&mut self) -> &mut Vec4 {
        &mut self.y_values1
    }
    fn y_values2(&self) -> &Vec4 {
        &self.y_values2
    }
    fn y_values2_mut(&mut self) -> &mut Vec4 {
        &mut self.y_values2
    }
    fn y_values3(&self) -> &Vec4 {
        &self.y_values3
    }
    fn y_values3_mut(&mut self) -> &mut Vec4 {
        &mut self.y_values3
    }
    fn g_values0(&self) -> &Vec4 {
        &self.g_values0
    }
    fn g_values0_mut(&mut self) -> &mut Vec4 {
        &mut self.g_values0
    }
    fn g_values1(&self) -> &Vec4 {
        &self.g_values1
    }
    fn g_values1_mut(&mut self) -> &mut Vec4 {
        &mut self.g_values1
    }
    fn g_values2(&self) -> &Vec4 {
        &self.g_values2
    }
    fn g_values2_mut(&mut self) -> &mut Vec4 {
        &mut self.g_values2
    }
    fn g_values3(&self) -> &Vec4 {
        &self.g_values3
    }
    fn g_values3_mut(&mut self) -> &mut Vec4 {
        &mut self.g_values3
    }
    fn g_values4(&self) -> &Vec4 {
        &self.g_values4
    }
    fn g_values4_mut(&mut self) -> &mut Vec4 {
        &mut self.g_values4
    }
    fn g_values5(&self) -> &Vec4 {
        &self.g_values5
    }
    fn g_values5_mut(&mut self) -> &mut Vec4 {
        &mut self.g_values5
    }
    fn spline_type(&self) -> &SplineType {
        &self.spline_type
    }
    fn spline_type_mut(&mut self) -> &mut SplineType {
        &mut self.spline_type
    }
}

pub static SPLINECURVE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SplineCurve",
    name_hash: 2881532991,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SplineCurve as Default>::default())),
            create_boxed: || Box::new(<SplineCurve as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "XValues0",
                name_hash: 3593824309,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, x_values0),
            },
            FieldInfoData {
                name: "XValues1",
                name_hash: 3593824308,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, x_values1),
            },
            FieldInfoData {
                name: "XValues2",
                name_hash: 3593824311,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, x_values2),
            },
            FieldInfoData {
                name: "YValues0",
                name_hash: 957382996,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, y_values0),
            },
            FieldInfoData {
                name: "YValues1",
                name_hash: 957382997,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, y_values1),
            },
            FieldInfoData {
                name: "YValues2",
                name_hash: 957382998,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, y_values2),
            },
            FieldInfoData {
                name: "YValues3",
                name_hash: 957382999,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, y_values3),
            },
            FieldInfoData {
                name: "GValues0",
                name_hash: 2487331978,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, g_values0),
            },
            FieldInfoData {
                name: "GValues1",
                name_hash: 2487331979,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, g_values1),
            },
            FieldInfoData {
                name: "GValues2",
                name_hash: 2487331976,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, g_values2),
            },
            FieldInfoData {
                name: "GValues3",
                name_hash: 2487331977,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, g_values3),
            },
            FieldInfoData {
                name: "GValues4",
                name_hash: 2487331982,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, g_values4),
            },
            FieldInfoData {
                name: "GValues5",
                name_hash: 2487331983,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(SplineCurve, g_values5),
            },
            FieldInfoData {
                name: "SplineType",
                name_hash: 3992327344,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static SPLINECURVE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SplineCurve-Array",
    name_hash: 2283227531,
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
    name_hash: 3992327344,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static SPLINETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SplineType-Array",
    name_hash: 1596211716,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SplineType"),
    array_type: None,
    alignment: 8,
};



pub static EVALUATEFLOATCURVE_FLOAT32_FLOAT32_FLOATCURVE__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EvaluateFloatCurve(Float32,Float32,FloatCurve)",
    name_hash: 13200063,
    flags: MemberInfoFlags::new(793),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub static EVALUATEAUDIOCURVE_FLOAT32_FLOAT32_AUDIOCURVE__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EvaluateAudioCurve(Float32,Float32,AudioCurve)",
    name_hash: 2308503487,
    flags: MemberInfoFlags::new(793),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FloatCurveCollectionAsset {
    pub _glacier_base: Asset,
    pub curves: Vec<Option<LockedTypeObject /* FloatCurve */>>,
}

pub trait FloatCurveCollectionAssetTrait: AssetTrait {
    fn curves(&self) -> &Vec<Option<LockedTypeObject /* FloatCurve */>>;
    fn curves_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* FloatCurve */>>;
}

impl FloatCurveCollectionAssetTrait for FloatCurveCollectionAsset {
    fn curves(&self) -> &Vec<Option<LockedTypeObject /* FloatCurve */>> {
        &self.curves
    }
    fn curves_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* FloatCurve */>> {
        &mut self.curves
    }
}

impl AssetTrait for FloatCurveCollectionAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl DataContainerTrait for FloatCurveCollectionAsset {
}

pub static FLOATCURVECOLLECTIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurveCollectionAsset",
    name_hash: 617697028,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        super_class_offset: offset_of!(FloatCurveCollectionAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatCurveCollectionAsset as Default>::default())),
            create_boxed: || Box::new(<FloatCurveCollectionAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Curves",
                name_hash: 2729642401,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static FLOATCURVECOLLECTIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurveCollectionAsset-Array",
    name_hash: 2083447600,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FloatCurveCollectionAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FloatCurve {
    pub _glacier_base: DataContainer,
    pub points: Vec<BoxedTypeObject /* FloatCurvePoint */>,
    pub min_x: f32,
    pub max_x: f32,
}

pub trait FloatCurveTrait: DataContainerTrait {
    fn points(&self) -> &Vec<BoxedTypeObject /* FloatCurvePoint */>;
    fn points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* FloatCurvePoint */>;
    fn min_x(&self) -> &f32;
    fn min_x_mut(&mut self) -> &mut f32;
    fn max_x(&self) -> &f32;
    fn max_x_mut(&mut self) -> &mut f32;
}

impl FloatCurveTrait for FloatCurve {
    fn points(&self) -> &Vec<BoxedTypeObject /* FloatCurvePoint */> {
        &self.points
    }
    fn points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* FloatCurvePoint */> {
        &mut self.points
    }
    fn min_x(&self) -> &f32 {
        &self.min_x
    }
    fn min_x_mut(&mut self) -> &mut f32 {
        &mut self.min_x
    }
    fn max_x(&self) -> &f32 {
        &self.max_x
    }
    fn max_x_mut(&mut self) -> &mut f32 {
        &mut self.max_x
    }
}

impl DataContainerTrait for FloatCurve {
}

pub static FLOATCURVE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurve",
    name_hash: 3138637890,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(FloatCurve, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatCurve as Default>::default())),
            create_boxed: || Box::new(<FloatCurve as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Points",
                name_hash: 3383606106,
                flags: MemberInfoFlags::new(144),
                field_type: "FloatCurvePoint-Array",
                rust_offset: offset_of!(FloatCurve, points),
            },
            FieldInfoData {
                name: "MinX",
                name_hash: 2088770807,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatCurve, min_x),
            },
            FieldInfoData {
                name: "MaxX",
                name_hash: 2088779177,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static FLOATCURVE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurve-Array",
    name_hash: 349839350,
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
    name_hash: 3310725626,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static FLOATCURVETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurveType-Array",
    name_hash: 3105250510,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FloatCurveType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn x_mut(&mut self) -> &mut f32;
    fn y(&self) -> &f32;
    fn y_mut(&mut self) -> &mut f32;
    fn in_tangent_offset_x(&self) -> &f32;
    fn in_tangent_offset_x_mut(&mut self) -> &mut f32;
    fn in_tangent_offset_y(&self) -> &f32;
    fn in_tangent_offset_y_mut(&mut self) -> &mut f32;
    fn out_tangent_offset_x(&self) -> &f32;
    fn out_tangent_offset_x_mut(&mut self) -> &mut f32;
    fn out_tangent_offset_y(&self) -> &f32;
    fn out_tangent_offset_y_mut(&mut self) -> &mut f32;
    fn curve_type(&self) -> &FloatCurveType;
    fn curve_type_mut(&mut self) -> &mut FloatCurveType;
}

impl FloatCurvePointTrait for FloatCurvePoint {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
    fn in_tangent_offset_x(&self) -> &f32 {
        &self.in_tangent_offset_x
    }
    fn in_tangent_offset_x_mut(&mut self) -> &mut f32 {
        &mut self.in_tangent_offset_x
    }
    fn in_tangent_offset_y(&self) -> &f32 {
        &self.in_tangent_offset_y
    }
    fn in_tangent_offset_y_mut(&mut self) -> &mut f32 {
        &mut self.in_tangent_offset_y
    }
    fn out_tangent_offset_x(&self) -> &f32 {
        &self.out_tangent_offset_x
    }
    fn out_tangent_offset_x_mut(&mut self) -> &mut f32 {
        &mut self.out_tangent_offset_x
    }
    fn out_tangent_offset_y(&self) -> &f32 {
        &self.out_tangent_offset_y
    }
    fn out_tangent_offset_y_mut(&mut self) -> &mut f32 {
        &mut self.out_tangent_offset_y
    }
    fn curve_type(&self) -> &FloatCurveType {
        &self.curve_type
    }
    fn curve_type_mut(&mut self) -> &mut FloatCurveType {
        &mut self.curve_type
    }
}

pub static FLOATCURVEPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurvePoint",
    name_hash: 1875772590,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatCurvePoint as Default>::default())),
            create_boxed: || Box::new(<FloatCurvePoint as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                name_hash: 177661,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatCurvePoint, x),
            },
            FieldInfoData {
                name: "Y",
                name_hash: 177660,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatCurvePoint, y),
            },
            FieldInfoData {
                name: "InTangentOffsetX",
                name_hash: 2987401972,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatCurvePoint, in_tangent_offset_x),
            },
            FieldInfoData {
                name: "InTangentOffsetY",
                name_hash: 2987401973,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatCurvePoint, in_tangent_offset_y),
            },
            FieldInfoData {
                name: "OutTangentOffsetX",
                name_hash: 3383076733,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatCurvePoint, out_tangent_offset_x),
            },
            FieldInfoData {
                name: "OutTangentOffsetY",
                name_hash: 3383076732,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatCurvePoint, out_tangent_offset_y),
            },
            FieldInfoData {
                name: "CurveType",
                name_hash: 2399916074,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static FLOATCURVEPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurvePoint-Array",
    name_hash: 1838476570,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FloatCurvePoint"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EffectCurvePoint {
    pub x: f32,
    pub y: f32,
    pub k: f32,
}

pub trait EffectCurvePointTrait: TypeObject {
    fn x(&self) -> &f32;
    fn x_mut(&mut self) -> &mut f32;
    fn y(&self) -> &f32;
    fn y_mut(&mut self) -> &mut f32;
    fn k(&self) -> &f32;
    fn k_mut(&mut self) -> &mut f32;
}

impl EffectCurvePointTrait for EffectCurvePoint {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
    fn k(&self) -> &f32 {
        &self.k
    }
    fn k_mut(&mut self) -> &mut f32 {
        &mut self.k
    }
}

pub static EFFECTCURVEPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectCurvePoint",
    name_hash: 1422990217,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EffectCurvePoint as Default>::default())),
            create_boxed: || Box::new(<EffectCurvePoint as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                name_hash: 177661,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EffectCurvePoint, x),
            },
            FieldInfoData {
                name: "Y",
                name_hash: 177660,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EffectCurvePoint, y),
            },
            FieldInfoData {
                name: "K",
                name_hash: 177646,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static EFFECTCURVEPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectCurvePoint-Array",
    name_hash: 838897981,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("EffectCurvePoint"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioCurve {
    pub points: Vec<BoxedTypeObject /* AudioCurvePoint */>,
    pub curve_type: AudioCurveType,
}

pub trait AudioCurveTrait: TypeObject {
    fn points(&self) -> &Vec<BoxedTypeObject /* AudioCurvePoint */>;
    fn points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* AudioCurvePoint */>;
    fn curve_type(&self) -> &AudioCurveType;
    fn curve_type_mut(&mut self) -> &mut AudioCurveType;
}

impl AudioCurveTrait for AudioCurve {
    fn points(&self) -> &Vec<BoxedTypeObject /* AudioCurvePoint */> {
        &self.points
    }
    fn points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* AudioCurvePoint */> {
        &mut self.points
    }
    fn curve_type(&self) -> &AudioCurveType {
        &self.curve_type
    }
    fn curve_type_mut(&mut self) -> &mut AudioCurveType {
        &mut self.curve_type
    }
}

pub static AUDIOCURVE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurve",
    name_hash: 2895553188,
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioCurve as Default>::default())),
            create_boxed: || Box::new(<AudioCurve as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Points",
                name_hash: 3383606106,
                flags: MemberInfoFlags::new(144),
                field_type: "AudioCurvePoint-Array",
                rust_offset: offset_of!(AudioCurve, points),
            },
            FieldInfoData {
                name: "CurveType",
                name_hash: 2399916074,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static AUDIOCURVE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurve-Array",
    name_hash: 3039049232,
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
    name_hash: 2262257948,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static AUDIOCURVETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurveType-Array",
    name_hash: 3510667560,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("AudioCurveType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioCurvePoint {
    pub x: f32,
    pub y: f32,
    pub k: f32,
}

pub trait AudioCurvePointTrait: TypeObject {
    fn x(&self) -> &f32;
    fn x_mut(&mut self) -> &mut f32;
    fn y(&self) -> &f32;
    fn y_mut(&mut self) -> &mut f32;
    fn k(&self) -> &f32;
    fn k_mut(&mut self) -> &mut f32;
}

impl AudioCurvePointTrait for AudioCurvePoint {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
    fn k(&self) -> &f32 {
        &self.k
    }
    fn k_mut(&mut self) -> &mut f32 {
        &mut self.k
    }
}

pub static AUDIOCURVEPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurvePoint",
    name_hash: 1645150792,
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioCurvePoint as Default>::default())),
            create_boxed: || Box::new(<AudioCurvePoint as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                name_hash: 177661,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioCurvePoint, x),
            },
            FieldInfoData {
                name: "Y",
                name_hash: 177660,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioCurvePoint, y),
            },
            FieldInfoData {
                name: "K",
                name_hash: 177646,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static AUDIOCURVEPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurvePoint-Array",
    name_hash: 1413068028,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("AudioCurvePoint"),
    array_type: None,
    alignment: 8,
};



pub static FBASSERT_BOOLEAN__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FbAssert(Boolean)",
    name_hash: 3953984294,
    flags: MemberInfoFlags::new(793),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn enable_mut(&mut self) -> &mut bool;
    fn draw_stats_enable(&self) -> &bool;
    fn draw_stats_enable_mut(&mut self) -> &mut bool;
    fn text_view_distance(&self) -> &f32;
    fn text_view_distance_mut(&mut self) -> &mut f32;
    fn text_queue_max_line_count(&self) -> &u32;
    fn text_queue_max_line_count_mut(&mut self) -> &mut u32;
    fn text_queue_time_visible(&self) -> &f32;
    fn text_queue_time_visible_mut(&mut self) -> &mut f32;
    fn text_queue_location_top(&self) -> &bool;
    fn text_queue_location_top_mut(&mut self) -> &mut bool;
    fn process_job_count(&self) -> &i32;
    fn process_job_count_mut(&mut self) -> &mut i32;
    fn dx_max_vertex_count(&self) -> &u32;
    fn dx_max_vertex_count_mut(&mut self) -> &mut u32;
    fn dx_line2d_antialiasing_enable(&self) -> &bool;
    fn dx_line2d_antialiasing_enable_mut(&mut self) -> &mut bool;
    fn dx_line3d_antialiasing_enable(&self) -> &bool;
    fn dx_line3d_antialiasing_enable_mut(&mut self) -> &mut bool;
}

impl DebugRenderSettingsTrait for DebugRenderSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    fn draw_stats_enable(&self) -> &bool {
        &self.draw_stats_enable
    }
    fn draw_stats_enable_mut(&mut self) -> &mut bool {
        &mut self.draw_stats_enable
    }
    fn text_view_distance(&self) -> &f32 {
        &self.text_view_distance
    }
    fn text_view_distance_mut(&mut self) -> &mut f32 {
        &mut self.text_view_distance
    }
    fn text_queue_max_line_count(&self) -> &u32 {
        &self.text_queue_max_line_count
    }
    fn text_queue_max_line_count_mut(&mut self) -> &mut u32 {
        &mut self.text_queue_max_line_count
    }
    fn text_queue_time_visible(&self) -> &f32 {
        &self.text_queue_time_visible
    }
    fn text_queue_time_visible_mut(&mut self) -> &mut f32 {
        &mut self.text_queue_time_visible
    }
    fn text_queue_location_top(&self) -> &bool {
        &self.text_queue_location_top
    }
    fn text_queue_location_top_mut(&mut self) -> &mut bool {
        &mut self.text_queue_location_top
    }
    fn process_job_count(&self) -> &i32 {
        &self.process_job_count
    }
    fn process_job_count_mut(&mut self) -> &mut i32 {
        &mut self.process_job_count
    }
    fn dx_max_vertex_count(&self) -> &u32 {
        &self.dx_max_vertex_count
    }
    fn dx_max_vertex_count_mut(&mut self) -> &mut u32 {
        &mut self.dx_max_vertex_count
    }
    fn dx_line2d_antialiasing_enable(&self) -> &bool {
        &self.dx_line2d_antialiasing_enable
    }
    fn dx_line2d_antialiasing_enable_mut(&mut self) -> &mut bool {
        &mut self.dx_line2d_antialiasing_enable
    }
    fn dx_line3d_antialiasing_enable(&self) -> &bool {
        &self.dx_line3d_antialiasing_enable
    }
    fn dx_line3d_antialiasing_enable_mut(&mut self) -> &mut bool {
        &mut self.dx_line3d_antialiasing_enable
    }
}

impl DataContainerTrait for DebugRenderSettings {
}

pub static DEBUGRENDERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugRenderSettings",
    name_hash: 2700858171,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(DebugRenderSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebugRenderSettings as Default>::default())),
            create_boxed: || Box::new(<DebugRenderSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                name_hash: 2342790116,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebugRenderSettings, enable),
            },
            FieldInfoData {
                name: "DrawStatsEnable",
                name_hash: 711726149,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebugRenderSettings, draw_stats_enable),
            },
            FieldInfoData {
                name: "TextViewDistance",
                name_hash: 853125686,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebugRenderSettings, text_view_distance),
            },
            FieldInfoData {
                name: "TextQueueMaxLineCount",
                name_hash: 1799437200,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebugRenderSettings, text_queue_max_line_count),
            },
            FieldInfoData {
                name: "TextQueueTimeVisible",
                name_hash: 1402582002,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebugRenderSettings, text_queue_time_visible),
            },
            FieldInfoData {
                name: "TextQueueLocationTop",
                name_hash: 607168255,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebugRenderSettings, text_queue_location_top),
            },
            FieldInfoData {
                name: "ProcessJobCount",
                name_hash: 3110006474,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DebugRenderSettings, process_job_count),
            },
            FieldInfoData {
                name: "DxMaxVertexCount",
                name_hash: 981947110,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DebugRenderSettings, dx_max_vertex_count),
            },
            FieldInfoData {
                name: "DxLine2dAntialiasingEnable",
                name_hash: 944294596,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebugRenderSettings, dx_line2d_antialiasing_enable),
            },
            FieldInfoData {
                name: "DxLine3dAntialiasingEnable",
                name_hash: 542123237,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DEBUGRENDERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugRenderSettings-Array",
    name_hash: 1773009551,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DebugRenderSettings"),
    array_type: None,
    alignment: 8,
};



pub static GETSCREENHEIGHT_UINT32__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetScreenHeight(Uint32)",
    name_hash: 718026086,
    flags: MemberInfoFlags::new(793),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub static GETSCREENWIDTH_UINT32__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetScreenWidth(Uint32)",
    name_hash: 3732009535,
    flags: MemberInfoFlags::new(793),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub static DRAWTEXT2D_INT32_INT32_FLOAT32_CSTRING__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DrawText2d(Int32,Int32,Float32,CString)",
    name_hash: 3864832740,
    flags: MemberInfoFlags::new(793),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct RuntimeModule {
}

pub trait RuntimeModuleTrait: TypeObject {
}

impl RuntimeModuleTrait for RuntimeModule {
}

pub static RUNTIMEMODULE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeModule",
    name_hash: 1332162147,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RuntimeModule as Default>::default())),
            create_boxed: || Box::new(<RuntimeModule as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static RUNTIMEMODULE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeModule-Array",
    name_hash: 624075351,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RuntimeModule"),
    array_type: None,
    alignment: 8,
};



pub static BOXEDVALUEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxedValueRef-Array",
    name_hash: 3734579679,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("BoxedValueRef"),
    array_type: None,
    alignment: 8,
};



pub static TYPEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeRef-Array",
    name_hash: 1163585496,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeRef"),
    array_type: None,
    alignment: 8,
};



pub static DBOBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DbObject-Array",
    name_hash: 575128482,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DbObject"),
    array_type: None,
    alignment: 8,
};



pub static DBOBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DbObject",
    name_hash: 3650719382,
    flags: MemberInfoFlags::new(45),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(DBOBJECT_ARRAY_TYPE_INFO),
    alignment: 8,
};



pub static FILEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FileRef-Array",
    name_hash: 4115558342,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FileRef"),
    array_type: None,
    alignment: 8,
};



pub static STRING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "String-Array",
    name_hash: 3719579620,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("String"),
    array_type: None,
    alignment: 8,
};



pub static STRING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "String",
    name_hash: 3320113488,
    flags: MemberInfoFlags::new(16589),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(STRING_ARRAY_TYPE_INFO),
    alignment: 8,
};



pub static CSTRING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CString-Array",
    name_hash: 2327192135,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("CString"),
    array_type: None,
    alignment: 8,
};



pub static VOID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Void-Array",
    name_hash: 3452188581,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Void"),
    array_type: None,
    alignment: 8,
};



pub static VOID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Void",
    name_hash: 2089252369,
    flags: MemberInfoFlags::new(13),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(VOID_ARRAY_TYPE_INFO),
    alignment: 0,
};



pub static RESOURCEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceRef-Array",
    name_hash: 2012889066,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ResourceRef"),
    array_type: None,
    alignment: 8,
};



pub static SHA1_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SHA1-Array",
    name_hash: 1448381914,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SHA1"),
    array_type: None,
    alignment: 8,
};



pub static GUID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Guid-Array",
    name_hash: 2607922254,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Guid"),
    array_type: None,
    alignment: 8,
};



pub static FLOAT64_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Float64-Array",
    name_hash: 789532579,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Float64"),
    array_type: None,
    alignment: 8,
};



pub static FLOAT32_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Float32-Array",
    name_hash: 1758053120,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Float32"),
    array_type: None,
    alignment: 8,
};



pub static INT64_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int64-Array",
    name_hash: 1330920896,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Int64"),
    array_type: None,
    alignment: 8,
};



pub static UINT64_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Uint64-Array",
    name_hash: 2441156149,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Uint64"),
    array_type: None,
    alignment: 8,
};



pub static INT32_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int32-Array",
    name_hash: 256527203,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Int32"),
    array_type: None,
    alignment: 8,
};



pub static UINT32_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Uint32-Array",
    name_hash: 91208342,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Uint32"),
    array_type: None,
    alignment: 8,
};



pub static INT16_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int16-Array",
    name_hash: 2612106277,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Int16"),
    array_type: None,
    alignment: 8,
};



pub static UINT16_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Uint16-Array",
    name_hash: 3259113552,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Uint16"),
    array_type: None,
    alignment: 8,
};



pub static INT8_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int8-Array",
    name_hash: 881898362,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Int8"),
    array_type: None,
    alignment: 8,
};



pub static UINT8_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Uint8-Array",
    name_hash: 2653196463,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Uint8"),
    array_type: None,
    alignment: 8,
};



pub static BOOLEAN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Boolean-Array",
    name_hash: 2861268117,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Boolean"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct AxisAlignedBox {
    pub min: Vec3,
    pub max: Vec3,
}

pub trait AxisAlignedBoxTrait: TypeObject {
    fn min(&self) -> &Vec3;
    fn min_mut(&mut self) -> &mut Vec3;
    fn max(&self) -> &Vec3;
    fn max_mut(&mut self) -> &mut Vec3;
}

impl AxisAlignedBoxTrait for AxisAlignedBox {
    fn min(&self) -> &Vec3 {
        &self.min
    }
    fn min_mut(&mut self) -> &mut Vec3 {
        &mut self.min
    }
    fn max(&self) -> &Vec3 {
        &self.max
    }
    fn max_mut(&mut self) -> &mut Vec3 {
        &mut self.max
    }
}

pub static AXISALIGNEDBOX_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AxisAlignedBox",
    name_hash: 3329912543,
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AxisAlignedBox as Default>::default())),
            create_boxed: || Box::new(<AxisAlignedBox as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "min",
                name_hash: 193413807,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(AxisAlignedBox, min),
            },
            FieldInfoData {
                name: "max",
                name_hash: 193414065,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static AXISALIGNEDBOX_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AxisAlignedBox-Array",
    name_hash: 1617298411,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("AxisAlignedBox"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn m11_mut(&mut self) -> &mut f32;
    fn m12(&self) -> &f32;
    fn m12_mut(&mut self) -> &mut f32;
    fn m13(&self) -> &f32;
    fn m13_mut(&mut self) -> &mut f32;
    fn m14(&self) -> &f32;
    fn m14_mut(&mut self) -> &mut f32;
    fn m21(&self) -> &f32;
    fn m21_mut(&mut self) -> &mut f32;
    fn m22(&self) -> &f32;
    fn m22_mut(&mut self) -> &mut f32;
    fn m23(&self) -> &f32;
    fn m23_mut(&mut self) -> &mut f32;
    fn m24(&self) -> &f32;
    fn m24_mut(&mut self) -> &mut f32;
    fn m31(&self) -> &f32;
    fn m31_mut(&mut self) -> &mut f32;
    fn m32(&self) -> &f32;
    fn m32_mut(&mut self) -> &mut f32;
    fn m33(&self) -> &f32;
    fn m33_mut(&mut self) -> &mut f32;
    fn m34(&self) -> &f32;
    fn m34_mut(&mut self) -> &mut f32;
    fn m41(&self) -> &f32;
    fn m41_mut(&mut self) -> &mut f32;
    fn m42(&self) -> &f32;
    fn m42_mut(&mut self) -> &mut f32;
    fn m43(&self) -> &f32;
    fn m43_mut(&mut self) -> &mut f32;
    fn m44(&self) -> &f32;
    fn m44_mut(&mut self) -> &mut f32;
}

impl Mat4Trait for Mat4 {
    fn m11(&self) -> &f32 {
        &self.m11
    }
    fn m11_mut(&mut self) -> &mut f32 {
        &mut self.m11
    }
    fn m12(&self) -> &f32 {
        &self.m12
    }
    fn m12_mut(&mut self) -> &mut f32 {
        &mut self.m12
    }
    fn m13(&self) -> &f32 {
        &self.m13
    }
    fn m13_mut(&mut self) -> &mut f32 {
        &mut self.m13
    }
    fn m14(&self) -> &f32 {
        &self.m14
    }
    fn m14_mut(&mut self) -> &mut f32 {
        &mut self.m14
    }
    fn m21(&self) -> &f32 {
        &self.m21
    }
    fn m21_mut(&mut self) -> &mut f32 {
        &mut self.m21
    }
    fn m22(&self) -> &f32 {
        &self.m22
    }
    fn m22_mut(&mut self) -> &mut f32 {
        &mut self.m22
    }
    fn m23(&self) -> &f32 {
        &self.m23
    }
    fn m23_mut(&mut self) -> &mut f32 {
        &mut self.m23
    }
    fn m24(&self) -> &f32 {
        &self.m24
    }
    fn m24_mut(&mut self) -> &mut f32 {
        &mut self.m24
    }
    fn m31(&self) -> &f32 {
        &self.m31
    }
    fn m31_mut(&mut self) -> &mut f32 {
        &mut self.m31
    }
    fn m32(&self) -> &f32 {
        &self.m32
    }
    fn m32_mut(&mut self) -> &mut f32 {
        &mut self.m32
    }
    fn m33(&self) -> &f32 {
        &self.m33
    }
    fn m33_mut(&mut self) -> &mut f32 {
        &mut self.m33
    }
    fn m34(&self) -> &f32 {
        &self.m34
    }
    fn m34_mut(&mut self) -> &mut f32 {
        &mut self.m34
    }
    fn m41(&self) -> &f32 {
        &self.m41
    }
    fn m41_mut(&mut self) -> &mut f32 {
        &mut self.m41
    }
    fn m42(&self) -> &f32 {
        &self.m42
    }
    fn m42_mut(&mut self) -> &mut f32 {
        &mut self.m42
    }
    fn m43(&self) -> &f32 {
        &self.m43
    }
    fn m43_mut(&mut self) -> &mut f32 {
        &mut self.m43
    }
    fn m44(&self) -> &f32 {
        &self.m44
    }
    fn m44_mut(&mut self) -> &mut f32 {
        &mut self.m44
    }
}

pub static MAT4_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Mat4",
    name_hash: 2088779593,
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Mat4 as Default>::default())),
            create_boxed: || Box::new(<Mat4 as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "m11",
                name_hash: 193416744,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m11),
            },
            FieldInfoData {
                name: "m12",
                name_hash: 193416747,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m12),
            },
            FieldInfoData {
                name: "m13",
                name_hash: 193416746,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m13),
            },
            FieldInfoData {
                name: "m14",
                name_hash: 193416749,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m14),
            },
            FieldInfoData {
                name: "m21",
                name_hash: 193416715,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m21),
            },
            FieldInfoData {
                name: "m22",
                name_hash: 193416712,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m22),
            },
            FieldInfoData {
                name: "m23",
                name_hash: 193416713,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m23),
            },
            FieldInfoData {
                name: "m24",
                name_hash: 193416718,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m24),
            },
            FieldInfoData {
                name: "m31",
                name_hash: 193416810,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m31),
            },
            FieldInfoData {
                name: "m32",
                name_hash: 193416809,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m32),
            },
            FieldInfoData {
                name: "m33",
                name_hash: 193416808,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m33),
            },
            FieldInfoData {
                name: "m34",
                name_hash: 193416815,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m34),
            },
            FieldInfoData {
                name: "m41",
                name_hash: 193416781,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m41),
            },
            FieldInfoData {
                name: "m42",
                name_hash: 193416782,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m42),
            },
            FieldInfoData {
                name: "m43",
                name_hash: 193416783,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Mat4, m43),
            },
            FieldInfoData {
                name: "m44",
                name_hash: 193416776,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static MAT4_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Mat4-Array",
    name_hash: 170362237,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Mat4"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LinearTransform {
    pub right: Vec3,
    pub up: Vec3,
    pub forward: Vec3,
    pub trans: Vec3,
}

pub trait LinearTransformTrait: TypeObject {
    fn right(&self) -> &Vec3;
    fn right_mut(&mut self) -> &mut Vec3;
    fn up(&self) -> &Vec3;
    fn up_mut(&mut self) -> &mut Vec3;
    fn forward(&self) -> &Vec3;
    fn forward_mut(&mut self) -> &mut Vec3;
    fn trans(&self) -> &Vec3;
    fn trans_mut(&mut self) -> &mut Vec3;
}

impl LinearTransformTrait for LinearTransform {
    fn right(&self) -> &Vec3 {
        &self.right
    }
    fn right_mut(&mut self) -> &mut Vec3 {
        &mut self.right
    }
    fn up(&self) -> &Vec3 {
        &self.up
    }
    fn up_mut(&mut self) -> &mut Vec3 {
        &mut self.up
    }
    fn forward(&self) -> &Vec3 {
        &self.forward
    }
    fn forward_mut(&mut self) -> &mut Vec3 {
        &mut self.forward
    }
    fn trans(&self) -> &Vec3 {
        &self.trans
    }
    fn trans_mut(&mut self) -> &mut Vec3 {
        &mut self.trans
    }
}

pub static LINEARTRANSFORM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearTransform",
    name_hash: 3812491028,
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinearTransform as Default>::default())),
            create_boxed: || Box::new(<LinearTransform as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "right",
                name_hash: 194951909,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LinearTransform, right),
            },
            FieldInfoData {
                name: "up",
                name_hash: 5861280,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LinearTransform, up),
            },
            FieldInfoData {
                name: "forward",
                name_hash: 1735999518,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(LinearTransform, forward),
            },
            FieldInfoData {
                name: "trans",
                name_hash: 182273375,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static LINEARTRANSFORM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearTransform-Array",
    name_hash: 888298272,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("LinearTransform"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Plane {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub trait PlaneTrait: TypeObject {
    fn x(&self) -> &f32;
    fn x_mut(&mut self) -> &mut f32;
    fn y(&self) -> &f32;
    fn y_mut(&mut self) -> &mut f32;
    fn z(&self) -> &f32;
    fn z_mut(&mut self) -> &mut f32;
    fn w(&self) -> &f32;
    fn w_mut(&mut self) -> &mut f32;
}

impl PlaneTrait for Plane {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
    fn z(&self) -> &f32 {
        &self.z
    }
    fn z_mut(&mut self) -> &mut f32 {
        &mut self.z
    }
    fn w(&self) -> &f32 {
        &self.w
    }
    fn w_mut(&mut self) -> &mut f32 {
        &mut self.w
    }
}

pub static PLANE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Plane",
    name_hash: 232719795,
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Plane as Default>::default())),
            create_boxed: || Box::new(<Plane as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                name_hash: 177629,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Plane, x),
            },
            FieldInfoData {
                name: "y",
                name_hash: 177628,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Plane, y),
            },
            FieldInfoData {
                name: "z",
                name_hash: 177631,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Plane, z),
            },
            FieldInfoData {
                name: "w",
                name_hash: 177618,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static PLANE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Plane-Array",
    name_hash: 3477975303,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Plane"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub trait QuatTrait: TypeObject {
    fn x(&self) -> &f32;
    fn x_mut(&mut self) -> &mut f32;
    fn y(&self) -> &f32;
    fn y_mut(&mut self) -> &mut f32;
    fn z(&self) -> &f32;
    fn z_mut(&mut self) -> &mut f32;
    fn w(&self) -> &f32;
    fn w_mut(&mut self) -> &mut f32;
}

impl QuatTrait for Quat {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
    fn z(&self) -> &f32 {
        &self.z
    }
    fn z_mut(&mut self) -> &mut f32 {
        &mut self.z
    }
    fn w(&self) -> &f32 {
        &self.w
    }
    fn w_mut(&mut self) -> &mut f32 {
        &mut self.w
    }
}

pub static QUAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Quat",
    name_hash: 2089188404,
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Quat as Default>::default())),
            create_boxed: || Box::new(<Quat as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                name_hash: 177629,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Quat, x),
            },
            FieldInfoData {
                name: "y",
                name_hash: 177628,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Quat, y),
            },
            FieldInfoData {
                name: "z",
                name_hash: 177631,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Quat, z),
            },
            FieldInfoData {
                name: "w",
                name_hash: 177618,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static QUAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Quat-Array",
    name_hash: 3867408256,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Quat"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub trait Vec4Trait: TypeObject {
    fn x(&self) -> &f32;
    fn x_mut(&mut self) -> &mut f32;
    fn y(&self) -> &f32;
    fn y_mut(&mut self) -> &mut f32;
    fn z(&self) -> &f32;
    fn z_mut(&mut self) -> &mut f32;
    fn w(&self) -> &f32;
    fn w_mut(&mut self) -> &mut f32;
}

impl Vec4Trait for Vec4 {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
    fn z(&self) -> &f32 {
        &self.z
    }
    fn z_mut(&mut self) -> &mut f32 {
        &mut self.z
    }
    fn w(&self) -> &f32 {
        &self.w
    }
    fn w_mut(&mut self) -> &mut f32 {
        &mut self.w
    }
}

pub static VEC4_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4",
    name_hash: 2089241857,
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec4 as Default>::default())),
            create_boxed: || Box::new(<Vec4 as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                name_hash: 177629,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Vec4, x),
            },
            FieldInfoData {
                name: "y",
                name_hash: 177628,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Vec4, y),
            },
            FieldInfoData {
                name: "z",
                name_hash: 177631,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Vec4, z),
            },
            FieldInfoData {
                name: "w",
                name_hash: 177618,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static VEC4_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4-Array",
    name_hash: 3933636277,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Vec4"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub trait Vec3Trait: TypeObject {
    fn x(&self) -> &f32;
    fn x_mut(&mut self) -> &mut f32;
    fn y(&self) -> &f32;
    fn y_mut(&mut self) -> &mut f32;
    fn z(&self) -> &f32;
    fn z_mut(&mut self) -> &mut f32;
}

impl Vec3Trait for Vec3 {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
    fn z(&self) -> &f32 {
        &self.z
    }
    fn z_mut(&mut self) -> &mut f32 {
        &mut self.z
    }
}

pub static VEC3_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3",
    name_hash: 2089241862,
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec3 as Default>::default())),
            create_boxed: || Box::new(<Vec3 as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                name_hash: 177629,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Vec3, x),
            },
            FieldInfoData {
                name: "y",
                name_hash: 177628,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Vec3, y),
            },
            FieldInfoData {
                name: "z",
                name_hash: 177631,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static VEC3_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3-Array",
    name_hash: 3919026738,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Vec3"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub trait Vec2Trait: TypeObject {
    fn x(&self) -> &f32;
    fn x_mut(&mut self) -> &mut f32;
    fn y(&self) -> &f32;
    fn y_mut(&mut self) -> &mut f32;
}

impl Vec2Trait for Vec2 {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
}

pub static VEC2_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2",
    name_hash: 2089241863,
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec2 as Default>::default())),
            create_boxed: || Box::new(<Vec2 as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                name_hash: 177629,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Vec2, x),
            },
            FieldInfoData {
                name: "y",
                name_hash: 177628,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static VEC2_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2-Array",
    name_hash: 2783882675,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Vec2"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct FbVec {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub trait FbVecTrait: TypeObject {
    fn x(&self) -> &f32;
    fn x_mut(&mut self) -> &mut f32;
    fn y(&self) -> &f32;
    fn y_mut(&mut self) -> &mut f32;
    fn z(&self) -> &f32;
    fn z_mut(&mut self) -> &mut f32;
    fn w(&self) -> &f32;
    fn w_mut(&mut self) -> &mut f32;
}

impl FbVecTrait for FbVec {
    fn x(&self) -> &f32 {
        &self.x
    }
    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
    fn z(&self) -> &f32 {
        &self.z
    }
    fn z_mut(&mut self) -> &mut f32 {
        &mut self.z
    }
    fn w(&self) -> &f32 {
        &self.w
    }
    fn w_mut(&mut self) -> &mut f32 {
        &mut self.w
    }
}

pub static VEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec",
    name_hash: 193460885,
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FbVec as Default>::default())),
            create_boxed: || Box::new(<FbVec as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "x",
                name_hash: 177629,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FbVec, x),
            },
            FieldInfoData {
                name: "y",
                name_hash: 177628,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FbVec, y),
            },
            FieldInfoData {
                name: "z",
                name_hash: 177631,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FbVec, z),
            },
            FieldInfoData {
                name: "w",
                name_hash: 177618,
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static VEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec-Array",
    name_hash: 1115015713,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Vec"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EventDispatcher {
}

pub trait EventDispatcherTrait: TypeObject {
}

impl EventDispatcherTrait for EventDispatcher {
}

pub static EVENTDISPATCHER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventDispatcher",
    name_hash: 4222524174,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EventDispatcher as Default>::default())),
            create_boxed: || Box::new(<EventDispatcher as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static EVENTDISPATCHER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventDispatcher-Array",
    name_hash: 1795212346,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("EventDispatcher"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DataContainer {
    pub _glacier_dc_core: glacier_reflect::data_container::DataContainerCore,
}

pub trait DataContainerTrait: TypeObject {
}

impl DataContainerTrait for DataContainer {
}

pub static DATACONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainer",
    name_hash: 3833490546,
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DataContainer as Default>::default())),
            create_boxed: || Box::new(<DataContainer as Default>::default()),
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        Some(&self._glacier_dc_core)
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        Some(&mut self._glacier_dc_core)
    }
}


pub static DATACONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainer-Array",
    name_hash: 1927783238,
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DataContainer"),
    array_type: None,
    alignment: 8,
};


