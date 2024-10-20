use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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
    registry.register_type(BOXEDVALUEREF_TYPE_INFO);
    registry.register_type(TYPEREF_ARRAY_TYPE_INFO);
    registry.register_type(TYPEREF_TYPE_INFO);
    registry.register_type(DBOBJECT_ARRAY_TYPE_INFO);
    registry.register_type(DBOBJECT_TYPE_INFO);
    registry.register_type(FILEREF_ARRAY_TYPE_INFO);
    registry.register_type(FILEREF_TYPE_INFO);
    registry.register_type(STRING_ARRAY_TYPE_INFO);
    registry.register_type(STRING_TYPE_INFO);
    registry.register_type(CSTRING_ARRAY_TYPE_INFO);
    registry.register_type(CSTRING_TYPE_INFO);
    registry.register_type(VOID_ARRAY_TYPE_INFO);
    registry.register_type(VOID_TYPE_INFO);
    registry.register_type(RESOURCEREF_ARRAY_TYPE_INFO);
    registry.register_type(RESOURCEREF_TYPE_INFO);
    registry.register_type(SHA1_ARRAY_TYPE_INFO);
    registry.register_type(SHA1_TYPE_INFO);
    registry.register_type(GUID_ARRAY_TYPE_INFO);
    registry.register_type(GUID_TYPE_INFO);
    registry.register_type(FLOAT64_ARRAY_TYPE_INFO);
    registry.register_type(FLOAT64_TYPE_INFO);
    registry.register_type(FLOAT32_ARRAY_TYPE_INFO);
    registry.register_type(FLOAT32_TYPE_INFO);
    registry.register_type(INT64_ARRAY_TYPE_INFO);
    registry.register_type(INT64_TYPE_INFO);
    registry.register_type(UINT64_ARRAY_TYPE_INFO);
    registry.register_type(UINT64_TYPE_INFO);
    registry.register_type(INT32_ARRAY_TYPE_INFO);
    registry.register_type(INT32_TYPE_INFO);
    registry.register_type(UINT32_ARRAY_TYPE_INFO);
    registry.register_type(UINT32_TYPE_INFO);
    registry.register_type(INT16_ARRAY_TYPE_INFO);
    registry.register_type(INT16_TYPE_INFO);
    registry.register_type(UINT16_ARRAY_TYPE_INFO);
    registry.register_type(UINT16_TYPE_INFO);
    registry.register_type(INT8_ARRAY_TYPE_INFO);
    registry.register_type(INT8_TYPE_INFO);
    registry.register_type(UINT8_ARRAY_TYPE_INFO);
    registry.register_type(UINT8_TYPE_INFO);
    registry.register_type(BOOLEAN_ARRAY_TYPE_INFO);
    registry.register_type(BOOLEAN_TYPE_INFO);
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


pub const RVMSERIALIZEDDB_NS_DX12PCCOMPUTEPSOINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcComputePsoInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcComputePsoInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGDISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcCsSkinningDispatchInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcCsSkinningDispatchInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGBUFFERINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcCsSkinningBufferInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcCsSkinningBufferInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12PCCSSKINNINGPARAMSINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcCsSkinningParamsInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcCsSkinningParamsInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12PCVERTEXBUFFERINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcVertexBufferInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcVertexBufferInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12PCSAMPLERPOINTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcSamplerPointer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcSamplerPointer-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12PCSAMPLERTABLEWRITERINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcSamplerTableWriterInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcSamplerTableWriterInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12PCRVMDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcRvmDescriptorTableAssemblyInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcRvmDescriptorTableAssemblyInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12PCSHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcShaderDispatchDrawInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcShaderDispatchDrawInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12PCDISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcDispatchInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcDispatchInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12PCPSOPRELOADOP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcPSOPreloadOp-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcPSOPreloadOp-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12PCSAMPLER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcSampler-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcSampler-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12PCROOTSIGNATURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12PcRootSignature-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12PcRootSignature-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12VERTEXBUFFERVIEWINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12VertexBufferViewInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12VertexBufferViewInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12ViewStateInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12ViewStateInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const D3D12_GPU_DESCRIPTOR_HANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_GPU_DESCRIPTOR_HANDLE-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_GPU_DESCRIPTOR_HANDLE-Array"),
    array_type: None,
    alignment: 8,
};



pub const D3D12_VERTEX_BUFFER_VIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_VERTEX_BUFFER_VIEW-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_VERTEX_BUFFER_VIEW-Array"),
    array_type: None,
    alignment: 8,
};



pub const DX12RVMINPUTELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmInputElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmInputElement-Array"),
    array_type: None,
    alignment: 8,
};



pub const DX12RVMSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmShader-Array"),
    array_type: None,
    alignment: 8,
};



pub const DX12RVMROOTSIGNATURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmRootSignature-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmRootSignature-Array"),
    array_type: None,
    alignment: 8,
};



pub const D3D12_VIEWPORT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_VIEWPORT-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_VIEWPORT-Array"),
    array_type: None,
    alignment: 8,
};



pub const D3D12_PIPELINE_STATE_FLAGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_PIPELINE_STATE_FLAGS-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_PIPELINE_STATE_FLAGS-Array"),
    array_type: None,
    alignment: 8,
};



pub const D3D12_CACHED_PIPELINE_STATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_CACHED_PIPELINE_STATE-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_CACHED_PIPELINE_STATE-Array"),
    array_type: None,
    alignment: 8,
};



pub const DXGI_SAMPLE_DESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DXGI_SAMPLE_DESC-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DXGI_SAMPLE_DESC-Array"),
    array_type: None,
    alignment: 8,
};



pub const D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_INDEX_BUFFER_STRIP_CUT_VALUE-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_INDEX_BUFFER_STRIP_CUT_VALUE-Array"),
    array_type: None,
    alignment: 8,
};



pub const DXGI_FORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DXGI_FORMAT-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DXGI_FORMAT-Array"),
    array_type: None,
    alignment: 8,
};



pub const D3D12_PRIMITIVE_TOPOLOGY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_PRIMITIVE_TOPOLOGY-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_PRIMITIVE_TOPOLOGY-Array"),
    array_type: None,
    alignment: 8,
};



pub const D3D12_PRIMITIVE_TOPOLOGY_TYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D12_PRIMITIVE_TOPOLOGY_TYPE-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D12_PRIMITIVE_TOPOLOGY_TYPE-Array"),
    array_type: None,
    alignment: 8,
};



pub const DX12RVMSODECLARATIONENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmSoDeclarationEntry-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmSoDeclarationEntry-Array"),
    array_type: None,
    alignment: 8,
};



pub const DX12RVMDEPTHSTENCILDESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmDepthStencilDesc-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmDepthStencilDesc-Array"),
    array_type: None,
    alignment: 8,
};



pub const DX12RVMRASTERIZERDESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmRasterizerDesc-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmRasterizerDesc-Array"),
    array_type: None,
    alignment: 8,
};



pub const DX12RVMBLENDDESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RvmBlendDesc-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12RvmBlendDesc-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyDebugInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyDebugInfo-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12BLENDSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12BlendStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12BlendStateData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12ROOTDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RootDescriptorTableAssemblyInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12ROOTWRITEOP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootWriteOp-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RootWriteOp-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionBatchData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionBatchData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12LEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12LegacyDrawStateBuilderInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12SHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12ShaderDispatchDrawInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12ShaderDispatchDrawInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12ROOTSIGNATUREDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RootSignatureDebugInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RootSignatureDebugInfo-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12TABLEDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12TableDebugInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12TableDebugInfo-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12TABLEDEBUGENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12TableDebugEntry-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12TableDebugEntry-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12INPUTELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12InputElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12InputElement-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12SHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12Shader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12Shader-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12BINARYBLOB_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12BinaryBlob-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12BinaryBlob-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12SHADERSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12ShaderState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12ShaderState-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ShaderDispatchDrawInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11ShaderDispatchDrawInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11LegacyVertexBufferConversionInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11LegacyVertexBufferConversionInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ViewStateInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11ViewStateInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const D3D11_CONSERVATIVE_RASTERIZATION_MODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D11_CONSERVATIVE_RASTERIZATION_MODE-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D11_CONSERVATIVE_RASTERIZATION_MODE-Array"),
    array_type: None,
    alignment: 8,
};



pub const DX11RVMVIEWPORTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmViewports-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmViewports-Array"),
    array_type: None,
    alignment: 8,
};



pub const DX11RVMSCISSORRECTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmScissorRects-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmScissorRects-Array"),
    array_type: None,
    alignment: 8,
};



pub const DX11RVMDEPTHSTENCILSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmDepthStencilState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmDepthStencilState-Array"),
    array_type: None,
    alignment: 8,
};



pub const DX11RVMBLENDSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmBlendState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmBlendState-Array"),
    array_type: None,
    alignment: 8,
};



pub const D3D11_CULL_MODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D11_CULL_MODE-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D11_CULL_MODE-Array"),
    array_type: None,
    alignment: 8,
};



pub const D3D11_FILL_MODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "D3D11_FILL_MODE-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("D3D11_FILL_MODE-Array"),
    array_type: None,
    alignment: 8,
};



pub const DX11RVMSAMPLER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmSampler-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmSampler-Array"),
    array_type: None,
    alignment: 8,
};



pub const DX11RVMVSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmVsShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmVsShader-Array"),
    array_type: None,
    alignment: 8,
};



pub const DX11RVMPSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmPsShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmPsShader-Array"),
    array_type: None,
    alignment: 8,
};



pub const DX11RVMHSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmHsShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmHsShader-Array"),
    array_type: None,
    alignment: 8,
};



pub const DX11RVMDSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RvmDsShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx11RvmDsShader-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11BYTECODEELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ByteCodeElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11ByteCodeElement-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11DSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11DsShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11DsShader-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11HSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11HsShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11HsShader-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11PSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11PsShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11PsShader-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11VSSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11VsShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11VsShader-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11INPUTELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11InputElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11InputElement-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11SAMPLER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11Sampler-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11Sampler-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11BLENDSTATEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11BlendStateData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11BlendStateData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11SERIALIZEDBLENDSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11SerializedBlendState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11SerializedBlendState-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11TEXTURECONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11TextureConversionInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11TextureConversionInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11BUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11BufferConversionInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11BufferConversionInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11LEGACYDRAWSTATEBUILDERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11LegacyDrawStateBuilderData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11LegacyDrawStateBuilderData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11SHADERDISPATCHLEGACYDRAWINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ShaderDispatchLegacyDrawInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11ShaderDispatchLegacyDrawInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ApplyParametersInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11ApplyParametersInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11APPLYPARAMETERSBLOCK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11ApplyParametersBlock-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11ApplyParametersBlock-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11DISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11DispatchInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11DispatchInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_LODFADEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_LodFadeInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_LodFadeInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_PACKLIGHTMAPWEIGHTINTOINSTANCEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_PackLightMapWeightIntoInstanceInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_SLICECOUNTINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SliceCountInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_SliceCountInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_TESSELLATIONPARAMETERSINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_TessellationParametersInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_TessellationParametersInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_VECTORSUBTRACTINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_VectorSubtractInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_VectorSubtractInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_OFFSETTRANSLATIONINMATRIXINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_OffsetTranslationInMatrixInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_CPUTOGPUMATRIXINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_CpuToGpuMatrixInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_CpuToGpuMatrixInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_FLOATTOVECINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_FloatToVecInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_FloatToVecInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_LEGACYVERTEXBUFFERCONVERSIONINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_LegacyVertexBufferConversionInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_LegacyVertexBufferConversionInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_VERTEXSTREAMPROCESSINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_VertexStreamProcessInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_VertexStreamProcessInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_VIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ViewStateInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ViewStateInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMPOINTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmPointer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmPointer-Array"),
    array_type: None,
    alignment: 8,
};



pub const GPUMAT4X3_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GpuMat4x3-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("GpuMat4x3-Array"),
    array_type: None,
    alignment: 8,
};



pub const IVEC4_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVec4-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("IVec4-Array"),
    array_type: None,
    alignment: 8,
};



pub const IVEC3_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVec3-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("IVec3-Array"),
    array_type: None,
    alignment: 8,
};



pub const IVEC2_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IVec2-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("IVec2-Array"),
    array_type: None,
    alignment: 8,
};



pub const HALF4_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half4-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Half4-Array"),
    array_type: None,
    alignment: 8,
};



pub const HALF3_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half3-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Half3-Array"),
    array_type: None,
    alignment: 8,
};



pub const HALF2_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half2-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Half2-Array"),
    array_type: None,
    alignment: 8,
};



pub const HALF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Half-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Half-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMLEGACYINSTRUCTIONS_NS_LEGACYINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyInstructions_ns_LegacyInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmLegacyInstructions_ns_LegacyInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_InstanceTableAssemblyInstructionBatchData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_INSTANCETABLEASSEMBLYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstanceTableAssemblyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_InstanceTableAssemblyData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DEPTHBIASGROUPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DepthBiasGroupData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DepthBiasGroupData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_TABLEASSEMBLYINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_TableAssemblyInstructionBatchData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_TableAssemblyInstructionBatchData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_TABLEASSEMBLYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_TableAssemblyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_TableAssemblyData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_WRITEOPGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_WriteOpGroup-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_WriteOpGroup-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_WRITEOP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_WriteOp-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_WriteOp-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DrawStateBuilderInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DrawStateBuilderInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DISPATCHINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DispatchInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DispatchInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DIRECTINPUTINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DirectInputInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DirectInputInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_CONSTANTVALUEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ConstantValueInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ConstantValueInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const CHAR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "char-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("char-Array"),
    array_type: None,
    alignment: 8,
};



pub const PARAMDBHASH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ParamDbHash-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ParamDbHash-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMENCODEDTABLEENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmEncodedTableEntry-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmEncodedTableEntry-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMCONTEXTSORTKEYINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmContextSortKeyInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmContextSortKeyInfo-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_PREPAREDVERTEXELEMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_PreparedVertexElement-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_PreparedVertexElement-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_PREPAREDVERTEXSTREAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_PreparedVertexStream-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_PreparedVertexStream-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_SERIALIZEDPARAMDBKEY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SerializedParamDbKey-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_SerializedParamDbKey-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_BASESHADERSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_BaseShaderState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_BaseShaderState-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_RUNTIMEINSTANTIATEDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RuntimeInstantiatedType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RuntimeInstantiatedType-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDMULTIHASHVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedMultiHashView-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbSerializedMultiHashView-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDREADVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedReadView-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbSerializedReadView-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEWREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedHashViewRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbSerializedHashViewRef-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDHASHVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedHashView-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbSerializedHashView-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_PARAMDBSERIALIZEDFILTERVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbSerializedFilterView-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbSerializedFilterView-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_COMBINEDSERIALIZEDPARAMETERBLOCK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_CombinedSerializedParameterBlock-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_CombinedSerializedParameterBlock-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCKREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SerializedParameterBlockRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_SerializedParameterBlockRef-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_SERIALIZEDPARAMETERBLOCK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SerializedParameterBlock-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_SerializedParameterBlock-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTUREREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableExternalTextureRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ShaderStreamableExternalTextureRef-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTUREREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableTextureRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ShaderStreamableTextureRef-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_SURFACESHADERDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SurfaceShaderDebugInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_SurfaceShaderDebugInfo-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_SURFACESHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_SurfaceShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_SurfaceShader-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_SHADERSTREAMABLETEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ShaderStreamableTexture-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_SHADERSTREAMABLEEXTERNALTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ShaderStreamableExternalTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ShaderStreamableExternalTexture-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionInstanceRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmFunctionInstanceRef-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_RVMFUNCTIONINSTANCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionInstance-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmFunctionInstance-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_SETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Settings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Settings-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_RVMPERMUTATIONSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationSet-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmPermutationSet-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_RVMPERMUTATIONLOOKUPTABLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationLookupTable-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmPermutationLookupTable-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_RVMPERMUTATIONTREE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationTree-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmPermutationTree-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_RVMPERMUTATIONREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutationRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmPermutationRef-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_RVMPERMUTATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmPermutation-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmPermutation-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_RVMFUNCTIONINPUTTABLEINDICES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionInputTableIndices-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmFunctionInputTableIndices-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_RVMDISPATCHDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmDispatchDebugInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmDispatchDebugInfo-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_RVMFUNCTIONDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunctionDebugInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmFunctionDebugInfo-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_RVMDISPATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmDispatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmDispatch-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_RVMFUNCTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RvmFunction-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RvmFunction-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_INSTRUCTIONBATCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstructionBatch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_InstructionBatch-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_RTTITYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_RttiType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_RttiType-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DEFAULTVALUEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DefaultValueRef-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_VALUEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ValueRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ValueRef-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DEFAULTVALUEZEROMEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueZeroMem-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DefaultValueZeroMem-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DEFAULTVALUESTRUCTLEGACYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueStructLegacyData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DefaultValueStructLegacyData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLETEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueSimpleTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DefaultValueSimpleTexture-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DEFAULTVALUESIMPLEBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_DefaultValueSimpleBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_DefaultValueSimpleBuffer-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_INSTRUCTIONBATCHREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_InstructionBatchRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_InstructionBatchRef-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_PARAMDBKEYREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbKeyRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbKeyRef-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_PARAMDBKEYDESC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_ParamDbKeyDesc-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_ParamDbKeyDesc-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12RTTLASNULLDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasNullData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtTlasNullData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12RTTLASDYNAMICDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasDynamicData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtTlasDynamicData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12RTTLASSTATICDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasStaticData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtTlasStaticData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12RTTLASBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtTlasBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtTlasBaseData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12RTSHADERTABLERECORDWRITERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtShaderTableRecordWriterData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtShaderTableRecordWriterData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12RTNULLHITSHADERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtNullHitShaderData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtNullHitShaderData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12RTHITCOLLECTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtHitCollectionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtHitCollectionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12RTSTATICBLASBUILDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtStaticBlasBuildData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtStaticBlasBuildData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12RTDYNAMICBLASBUILDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtDynamicBlasBuildData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtDynamicBlasBuildData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12RTBLASBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtBlasBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtBlasBaseData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12RTHITSHADERCONSTANTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtHitShaderConstantData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtHitShaderConstantData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12RTINDEXBUFFERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtIndexBufferData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtIndexBufferData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12RTSORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtSortData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtSortData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12RTDISPATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtDispatchData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtDispatchData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12RTCOLLECTIONPRELOADOP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12RtCollectionPreloadOp-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12RtCollectionPreloadOp-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMLEGACYLIGHTMAPINSTANCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyLightMapInstance-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmLegacyLightMapInstance-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMLEGACYLIGHTPROBES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyLightProbes-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmLegacyLightProbes-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMLEGACYPERMUTATIONDEBUGINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyPermutationDebugInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmLegacyPermutationDebugInfo-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMLEGACYFORWARDLIGHTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyForwardLightState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmLegacyForwardLightState-Array"),
    array_type: None,
    alignment: 8,
};



pub const FRUSTUMSOA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FrustumSoA-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FrustumSoA-Array"),
    array_type: None,
    alignment: 8,
};



pub const LODVIEWSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LodViewState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("LodViewState-Array"),
    array_type: None,
    alignment: 8,
};



pub const TESSELLATIONVIEWSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TessellationViewState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TessellationViewState-Array"),
    array_type: None,
    alignment: 8,
};



pub const STENCILSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StencilState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("StencilState-Array"),
    array_type: None,
    alignment: 8,
};



pub const REFLECTIONSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReflectionState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ReflectionState-Array"),
    array_type: None,
    alignment: 8,
};



pub const FOGSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FogState-Array"),
    array_type: None,
    alignment: 8,
};



pub const PROJECTIONSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProjectionState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ProjectionState-Array"),
    array_type: None,
    alignment: 8,
};



pub const VIEWSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ViewState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ViewState-Array"),
    array_type: None,
    alignment: 8,
};



pub const OUTDOORLIGHTSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutdoorLightState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("OutdoorLightState-Array"),
    array_type: None,
    alignment: 8,
};



pub const DX12NVRVMROOTSIGNATURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvRvmRootSignature-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Dx12NvRvmRootSignature-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLEASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvDescriptorTableAssemblyInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12NvDescriptorTableAssemblyInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONBATCHDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionBatchData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionBatchData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12NVLEGACYDRAWSTATEBUILDERINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12NvLegacyDrawStateBuilderInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12NVCONSTANTBUFFERASSEMBLYINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvConstantBufferAssemblyInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12NvConstantBufferAssemblyInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12NVDESCRIPTORTABLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvDescriptorTable-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12NvDescriptorTable-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX12NVRVMROOTSIGNATURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx12NvRvmRootSignature-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx12NvRvmRootSignature-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEDEPTHINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11NvViewStateDepthInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11NvViewStateDepthInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11NVDRAWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11NvDrawStateInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11NvDrawStateInstructionData-Array"),
    array_type: None,
    alignment: 8,
};



pub const RVMSERIALIZEDDB_NS_DX11NVVIEWSTATEINSTRUCTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSerializedDb_ns_Dx11NvViewStateInstructionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RvmSerializedDb_ns_Dx11NvViewStateInstructionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RadiosityTypeOverride {
    #[default]
    RadiosityTypeOverride_None = 0,
    RadiosityTypeOverride_Dynamic = 1,
    RadiosityTypeOverride_LightProbe = 2,
    RadiosityTypeOverride_Static = 3,
    RadiosityTypeOverride_TerrainProjected = 4,
    RadiosityTypeOverride_Proxy = 5,
}

pub const RADIOSITYTYPEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityTypeOverride",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(RADIOSITYTYPEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RadiosityTypeOverride {
    fn type_info() -> &'static TypeInfo {
        RADIOSITYTYPEOVERRIDE_TYPE_INFO
    }
}


pub const RADIOSITYTYPEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityTypeOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RadiosityTypeOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const LOCALPLAYERID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlayerId",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(LOCALPLAYERID_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LocalPlayerId {
    fn type_info() -> &'static TypeInfo {
        LOCALPLAYERID_TYPE_INFO
    }
}


pub const LOCALPLAYERID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalPlayerId-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("LocalPlayerId-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NodeGraph {
}

pub const NODEGRAPH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NodeGraph",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(NODEGRAPH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NodeGraph {
    fn type_info() -> &'static TypeInfo {
        NODEGRAPH_TYPE_INFO
    }
}


pub const NODEGRAPH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NodeGraph-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("NodeGraph-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Node {
}

pub const NODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Node",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(NODE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Node {
    fn type_info() -> &'static TypeInfo {
        NODE_TYPE_INFO
    }
}


pub const NODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Node-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Node-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const GAMEPLATFORM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GamePlatform",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(GAMEPLATFORM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GamePlatform {
    fn type_info() -> &'static TypeInfo {
        GAMEPLATFORM_TYPE_INFO
    }
}


pub const GAMEPLATFORM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GamePlatform-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("GamePlatform-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DesignerEnumerationSortType {
    #[default]
    DesignerEnumSort_None = 0,
    DesignerEnumSort_ByName = 1,
    DesignerEnumSort_ById = 2,
}

pub const DESIGNERENUMERATIONSORTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DesignerEnumerationSortType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(DESIGNERENUMERATIONSORTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DesignerEnumerationSortType {
    fn type_info() -> &'static TypeInfo {
        DESIGNERENUMERATIONSORTTYPE_TYPE_INFO
    }
}


pub const DESIGNERENUMERATIONSORTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DesignerEnumerationSortType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DesignerEnumerationSortType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const CORESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreSettings",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "LiveEditingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, live_editing_enable),
            },
            FieldInfoData {
                name: "DisableSafeStateForRefresh",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, disable_safe_state_for_refresh),
            },
            FieldInfoData {
                name: "Host",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, host),
            },
            FieldInfoData {
                name: "HostUser",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, host_user),
            },
            FieldInfoData {
                name: "HostUserDomain",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, host_user_domain),
            },
            FieldInfoData {
                name: "InitSeed",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, init_seed),
            },
            FieldInfoData {
                name: "TraceHost",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, trace_host),
            },
            FieldInfoData {
                name: "ForwardTrace",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, forward_trace),
            },
            FieldInfoData {
                name: "ForwardTraceHost",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, forward_trace_host),
            },
            FieldInfoData {
                name: "ForwardTracePort",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, forward_trace_port),
            },
            FieldInfoData {
                name: "PerfTrackEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, perf_track_enabled),
            },
            FieldInfoData {
                name: "LogLevel",
                flags: MemberInfoFlags::new(0),
                field_type: CORELOGLEVEL_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, log_level),
            },
            FieldInfoData {
                name: "DialogLevel",
                flags: MemberInfoFlags::new(0),
                field_type: CORELOGLEVEL_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, dialog_level),
            },
            FieldInfoData {
                name: "UserLogEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, user_log_enabled),
            },
            FieldInfoData {
                name: "LogSettings",
                flags: MemberInfoFlags::new(0),
                field_type: LOGSETTINGS_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, log_settings),
            },
            FieldInfoData {
                name: "AllowAssignVarsFromLua",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, allow_assign_vars_from_lua),
            },
            FieldInfoData {
                name: "DisplayAsserts",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, display_asserts),
            },
            FieldInfoData {
                name: "DebugOutputMode",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, debug_output_mode),
            },
            FieldInfoData {
                name: "CrashOnFatalErrors",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, crash_on_fatal_errors),
            },
            FieldInfoData {
                name: "EnableMemorySnapshots",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, enable_memory_snapshots),
            },
            FieldInfoData {
                name: "BreakpadMode",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, breakpad_mode),
            },
            FieldInfoData {
                name: "RandomTimeSeed",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, random_time_seed),
            },
            FieldInfoData {
                name: "RandomTickSeed",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, random_tick_seed),
            },
            FieldInfoData {
                name: "RandomSessionId",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, random_session_id),
            },
            FieldInfoData {
                name: "LocalPlayerIdToCommand",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALPLAYERID_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, local_player_id_to_command),
            },
            FieldInfoData {
                name: "GameConfigurationName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, game_configuration_name),
            },
            FieldInfoData {
                name: "UseStorageServer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, use_storage_server),
            },
            FieldInfoData {
                name: "HardwareProfile",
                flags: MemberInfoFlags::new(0),
                field_type: HARDWAREPROFILE_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, hardware_profile),
            },
            FieldInfoData {
                name: "HardwareCpuBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, hardware_cpu_bias),
            },
            FieldInfoData {
                name: "HardwareGpuBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, hardware_gpu_bias),
            },
            FieldInfoData {
                name: "ProfileDirectoryName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, profile_directory_name),
            },
            FieldInfoData {
                name: "ResourceAutoJobThread",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, resource_auto_job_thread),
            },
            FieldInfoData {
                name: "ResourceAutoJobThreadAffinityMask",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, resource_auto_job_thread_affinity_mask),
            },
            FieldInfoData {
                name: "EnableLocalization",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, enable_localization),
            },
            FieldInfoData {
                name: "AvailableLanguages",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, available_languages),
            },
            FieldInfoData {
                name: "EnableSpikePacketBuffering",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, enable_spike_packet_buffering),
            },
            FieldInfoData {
                name: "WorldSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, world_size),
            },
            FieldInfoData {
                name: "StreamInstallAllowPlayFromBluray",
                flags: MemberInfoFlags::new(0),
                field_type: STREAMINSTALLFROMBLURAYTYPE_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, stream_install_allow_play_from_bluray),
            },
            FieldInfoData {
                name: "StreamInstallEnableDelayedSuperbundleMount",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, stream_install_enable_delayed_superbundle_mount),
            },
            FieldInfoData {
                name: "StreamInstallAllLanguages",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, stream_install_all_languages),
            },
            FieldInfoData {
                name: "OperationalContextDataKeys",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CoreSettings, operational_context_data_keys),
            },
        ],
    }),
    array_type: Some(CORESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CoreSettings {
    fn type_info() -> &'static TypeInfo {
        CORESETTINGS_TYPE_INFO
    }
}


pub const CORESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("CoreSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LogSettings {
    pub channels: Vec<ChannelSettings>,
    pub writers: Vec<WriterSettings>,
}

pub const LOGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogSettings",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Channels",
                flags: MemberInfoFlags::new(144),
                field_type: CHANNELSETTINGS_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LogSettings, channels),
            },
            FieldInfoData {
                name: "Writers",
                flags: MemberInfoFlags::new(144),
                field_type: WRITERSETTINGS_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LogSettings, writers),
            },
        ],
    }),
    array_type: Some(LOGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LogSettings {
    fn type_info() -> &'static TypeInfo {
        LOGSETTINGS_TYPE_INFO
    }
}


pub const LOGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("LogSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WriterSettings {
    pub id: String,
    pub class: String,
    pub params: String,
}

pub const WRITERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WriterSettings",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(WriterSettings, id),
            },
            FieldInfoData {
                name: "Class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(WriterSettings, class),
            },
            FieldInfoData {
                name: "Params",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(WriterSettings, params),
            },
        ],
    }),
    array_type: Some(WRITERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WriterSettings {
    fn type_info() -> &'static TypeInfo {
        WRITERSETTINGS_TYPE_INFO
    }
}


pub const WRITERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WriterSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("WriterSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ChannelSettings {
    pub id: String,
    pub inherits_level: bool,
    pub inherits_writers: bool,
    pub level: CoreLogLevel,
    pub writers: Vec<String>,
}

pub const CHANNELSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChannelSettings",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ChannelSettings, id),
            },
            FieldInfoData {
                name: "InheritsLevel",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ChannelSettings, inherits_level),
            },
            FieldInfoData {
                name: "InheritsWriters",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ChannelSettings, inherits_writers),
            },
            FieldInfoData {
                name: "Level",
                flags: MemberInfoFlags::new(0),
                field_type: CORELOGLEVEL_TYPE_INFO,
                rust_offset: offset_of!(ChannelSettings, level),
            },
            FieldInfoData {
                name: "Writers",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ChannelSettings, writers),
            },
        ],
    }),
    array_type: Some(CHANNELSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ChannelSettings {
    fn type_info() -> &'static TypeInfo {
        CHANNELSETTINGS_TYPE_INFO
    }
}


pub const CHANNELSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChannelSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ChannelSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum StreamInstallFromBlurayType {
    #[default]
    StreamInstall_DisablePlayFromBluray = 0,
    StreamInstall_InstallChunkPlayFromBluray = 1,
    StreamInstall_FullPlayFromBluray = 2,
}

pub const STREAMINSTALLFROMBLURAYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallFromBlurayType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(STREAMINSTALLFROMBLURAYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StreamInstallFromBlurayType {
    fn type_info() -> &'static TypeInfo {
        STREAMINSTALLFROMBLURAYTYPE_TYPE_INFO
    }
}


pub const STREAMINSTALLFROMBLURAYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallFromBlurayType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("StreamInstallFromBlurayType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum HardwareProfile {
    #[default]
    Hardware_Autodetect = 0,
    Hardware_LowEnd = 1,
    Hardware_Medium = 2,
    Hardware_HighEnd = 3,
    Hardware_Maximum = 4,
}

pub const HARDWAREPROFILE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HardwareProfile",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(HARDWAREPROFILE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for HardwareProfile {
    fn type_info() -> &'static TypeInfo {
        HARDWAREPROFILE_TYPE_INFO
    }
}


pub const HARDWAREPROFILE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HardwareProfile-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("HardwareProfile-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const CORELOGLEVEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreLogLevel",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(CORELOGLEVEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CoreLogLevel {
    fn type_info() -> &'static TypeInfo {
        CORELOGLEVEL_TYPE_INFO
    }
}


pub const CORELOGLEVEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreLogLevel-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("CoreLogLevel-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ContentPreset {
    pub content_name: String,
    pub content_type: String,
    pub content_parameters: DataContainer,
}

pub const CONTENTPRESET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ContentPreset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ContentName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ContentPreset, content_name),
            },
            FieldInfoData {
                name: "ContentType",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ContentPreset, content_type),
            },
            FieldInfoData {
                name: "ContentParameters",
                flags: MemberInfoFlags::new(0),
                field_type: DATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(ContentPreset, content_parameters),
            },
        ],
    }),
    array_type: Some(CONTENTPRESET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ContentPreset {
    fn type_info() -> &'static TypeInfo {
        CONTENTPRESET_TYPE_INFO
    }
}


pub const CONTENTPRESET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ContentPreset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ContentPreset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ContentPresetContainer {
    pub presets: Vec<ContentPreset>,
}

pub const CONTENTPRESETCONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ContentPresetContainer",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Presets",
                flags: MemberInfoFlags::new(144),
                field_type: CONTENTPRESET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ContentPresetContainer, presets),
            },
        ],
    }),
    array_type: Some(CONTENTPRESETCONTAINER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ContentPresetContainer {
    fn type_info() -> &'static TypeInfo {
        CONTENTPRESETCONTAINER_TYPE_INFO
    }
}


pub const CONTENTPRESETCONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ContentPresetContainer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ContentPresetContainer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TimeSpan {
    pub internal: i64,
}

pub const TIMESPAN_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimeSpan",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Internal",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TimeSpan, internal),
            },
        ],
    }),
    array_type: Some(TIMESPAN_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimeSpan {
    fn type_info() -> &'static TypeInfo {
        TIMESPAN_TYPE_INFO
    }
}


pub const TIMESPAN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimeSpan-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TimeSpan-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ThreadSettings {
    pub processor_count: i32,
    pub max_live_edit_processor_count: i32,
    pub job_thread_priority: i32,
    pub free_processor_count: i32,
}

pub const THREADSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ThreadSettings",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ProcessorCount",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ThreadSettings, processor_count),
            },
            FieldInfoData {
                name: "MaxLiveEditProcessorCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ThreadSettings, max_live_edit_processor_count),
            },
            FieldInfoData {
                name: "JobThreadPriority",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ThreadSettings, job_thread_priority),
            },
            FieldInfoData {
                name: "FreeProcessorCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ThreadSettings, free_processor_count),
            },
        ],
    }),
    array_type: Some(THREADSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ThreadSettings {
    fn type_info() -> &'static TypeInfo {
        THREADSETTINGS_TYPE_INFO
    }
}


pub const THREADSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ThreadSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ThreadSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TemplateDescriptorRegistryAsset {
    pub descriptors: Vec<TemplateDescriptorData>,
}

pub const TEMPLATEDESCRIPTORREGISTRYASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateDescriptorRegistryAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Descriptors",
                flags: MemberInfoFlags::new(144),
                field_type: TEMPLATEDESCRIPTORDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TemplateDescriptorRegistryAsset, descriptors),
            },
        ],
    }),
    array_type: Some(TEMPLATEDESCRIPTORREGISTRYASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TemplateDescriptorRegistryAsset {
    fn type_info() -> &'static TypeInfo {
        TEMPLATEDESCRIPTORREGISTRYASSET_TYPE_INFO
    }
}


pub const TEMPLATEDESCRIPTORREGISTRYASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateDescriptorRegistryAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TemplateDescriptorRegistryAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TemplateDescriptorData {
    pub target_asset: Asset,
    pub exposed_objects: Vec<TemplateExposedObject>,
}

pub const TEMPLATEDESCRIPTORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateDescriptorData",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TargetAsset",
                flags: MemberInfoFlags::new(0),
                field_type: ASSET_TYPE_INFO,
                rust_offset: offset_of!(TemplateDescriptorData, target_asset),
            },
            FieldInfoData {
                name: "ExposedObjects",
                flags: MemberInfoFlags::new(144),
                field_type: TEMPLATEEXPOSEDOBJECT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TemplateDescriptorData, exposed_objects),
            },
        ],
    }),
    array_type: Some(TEMPLATEDESCRIPTORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TemplateDescriptorData {
    fn type_info() -> &'static TypeInfo {
        TEMPLATEDESCRIPTORDATA_TYPE_INFO
    }
}


pub const TEMPLATEDESCRIPTORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateDescriptorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TemplateDescriptorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TemplateExposedObject {
    pub object: DataContainer,
    pub object_ref: Guid,
}

pub const TEMPLATEEXPOSEDOBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateExposedObject",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Object",
                flags: MemberInfoFlags::new(0),
                field_type: DATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(TemplateExposedObject, object),
            },
            FieldInfoData {
                name: "ObjectRef",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(TemplateExposedObject, object_ref),
            },
        ],
    }),
    array_type: Some(TEMPLATEEXPOSEDOBJECT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TemplateExposedObject {
    fn type_info() -> &'static TypeInfo {
        TEMPLATEEXPOSEDOBJECT_TYPE_INFO
    }
}


pub const TEMPLATEEXPOSEDOBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateExposedObject-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TemplateExposedObject-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TemplateAsset {
    pub objects: Vec<TemplateTarget>,
}

pub const TEMPLATEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Objects",
                flags: MemberInfoFlags::new(144),
                field_type: TEMPLATETARGET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TemplateAsset, objects),
            },
        ],
    }),
    array_type: Some(TEMPLATEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TemplateAsset {
    fn type_info() -> &'static TypeInfo {
        TEMPLATEASSET_TYPE_INFO
    }
}


pub const TEMPLATEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TemplateAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TemplateTarget {
    pub target_ref: Guid,
    pub template: Template,
}

pub const TEMPLATETARGET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateTarget",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TargetRef",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(TemplateTarget, target_ref),
            },
            FieldInfoData {
                name: "Template",
                flags: MemberInfoFlags::new(0),
                field_type: TEMPLATE_TYPE_INFO,
                rust_offset: offset_of!(TemplateTarget, template),
            },
        ],
    }),
    array_type: Some(TEMPLATETARGET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TemplateTarget {
    fn type_info() -> &'static TypeInfo {
        TEMPLATETARGET_TYPE_INFO
    }
}


pub const TEMPLATETARGET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplateTarget-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TemplateTarget-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Template {
}

pub const TEMPLATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Template",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TEMPLATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Template {
    fn type_info() -> &'static TypeInfo {
        TEMPLATE_TYPE_INFO
    }
}


pub const TEMPLATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Template-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Template-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SuperbundleLayoutSettings {
}

pub const SUPERBUNDLELAYOUTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SuperbundleLayoutSettings",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SUPERBUNDLELAYOUTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SuperbundleLayoutSettings {
    fn type_info() -> &'static TypeInfo {
        SUPERBUNDLELAYOUTSETTINGS_TYPE_INFO
    }
}


pub const SUPERBUNDLELAYOUTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SuperbundleLayoutSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SuperbundleLayoutSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum InstallPackageLayerType {
    #[default]
    InstallPackageLayerType_Any = 4294967295,
    InstallPackageLayerType_Zero = 0,
    InstallPackageLayerType_One = 1,
}

pub const INSTALLPACKAGELAYERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InstallPackageLayerType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(INSTALLPACKAGELAYERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InstallPackageLayerType {
    fn type_info() -> &'static TypeInfo {
        INSTALLPACKAGELAYERTYPE_TYPE_INFO
    }
}


pub const INSTALLPACKAGELAYERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InstallPackageLayerType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("InstallPackageLayerType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum InstallPackageType {
    #[default]
    InstallPackageType_Default = 0,
    InstallPackageType_AlwaysInstalled = 1,
    InstallPackageType_MandatoryDLC = 2,
    InstallPackageType_OptionalDLC = 3,
    InstallPackageType_Exclude = 4,
}

pub const INSTALLPACKAGETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InstallPackageType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(INSTALLPACKAGETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InstallPackageType {
    fn type_info() -> &'static TypeInfo {
        INSTALLPACKAGETYPE_TYPE_INFO
    }
}


pub const INSTALLPACKAGETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InstallPackageType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("InstallPackageType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
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

pub const PLATFORMSCALABLEBOOL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformScalableBool",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Default",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableBool, default),
            },
            FieldInfoData {
                name: "Xenon",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableBool, xenon),
            },
            FieldInfoData {
                name: "Ps3",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableBool, ps3),
            },
            FieldInfoData {
                name: "Gen4a",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableBool, gen4a),
            },
            FieldInfoData {
                name: "Gen4b",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableBool, gen4b),
            },
            FieldInfoData {
                name: "Android",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableBool, android),
            },
            FieldInfoData {
                name: "iOS",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableBool, i_o_s),
            },
            FieldInfoData {
                name: "OSX",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableBool, o_s_x),
            },
            FieldInfoData {
                name: "Linux",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableBool, linux),
            },
        ],
    }),
    array_type: Some(PLATFORMSCALABLEBOOL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PlatformScalableBool {
    fn type_info() -> &'static TypeInfo {
        PLATFORMSCALABLEBOOL_TYPE_INFO
    }
}


pub const PLATFORMSCALABLEBOOL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformScalableBool-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PlatformScalableBool-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const PLATFORMSCALABLEFLOAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformScalableFloat",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Default",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableFloat, default),
            },
            FieldInfoData {
                name: "Xenon",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableFloat, xenon),
            },
            FieldInfoData {
                name: "Ps3",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableFloat, ps3),
            },
            FieldInfoData {
                name: "Gen4a",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableFloat, gen4a),
            },
            FieldInfoData {
                name: "Gen4b",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableFloat, gen4b),
            },
            FieldInfoData {
                name: "Android",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableFloat, android),
            },
            FieldInfoData {
                name: "iOS",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableFloat, i_o_s),
            },
            FieldInfoData {
                name: "OSX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableFloat, o_s_x),
            },
            FieldInfoData {
                name: "Linux",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableFloat, linux),
            },
        ],
    }),
    array_type: Some(PLATFORMSCALABLEFLOAT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PlatformScalableFloat {
    fn type_info() -> &'static TypeInfo {
        PLATFORMSCALABLEFLOAT_TYPE_INFO
    }
}


pub const PLATFORMSCALABLEFLOAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformScalableFloat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PlatformScalableFloat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
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

pub const PLATFORMSCALABLEINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformScalableInt",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Default",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableInt, default),
            },
            FieldInfoData {
                name: "Xenon",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableInt, xenon),
            },
            FieldInfoData {
                name: "Ps3",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableInt, ps3),
            },
            FieldInfoData {
                name: "Gen4a",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableInt, gen4a),
            },
            FieldInfoData {
                name: "Gen4b",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableInt, gen4b),
            },
            FieldInfoData {
                name: "Android",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableInt, android),
            },
            FieldInfoData {
                name: "iOS",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableInt, i_o_s),
            },
            FieldInfoData {
                name: "OSX",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableInt, o_s_x),
            },
            FieldInfoData {
                name: "Linux",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PlatformScalableInt, linux),
            },
        ],
    }),
    array_type: Some(PLATFORMSCALABLEINT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PlatformScalableInt {
    fn type_info() -> &'static TypeInfo {
        PLATFORMSCALABLEINT_TYPE_INFO
    }
}


pub const PLATFORMSCALABLEINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformScalableInt-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PlatformScalableInt-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum QualityScalableEnabled {
    #[default]
    QualityScalableEnabled_Low = 0,
    QualityScalableEnabled_Medium = 1,
    QualityScalableEnabled_High = 2,
    QualityScalableEnabled_Ultra = 3,
    QualityScalableEnabled_Cinematic = 4,
    QualityScalableEnabled_Disabled = 5,
}

pub const QUALITYSCALABLEENABLED_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableEnabled",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(QUALITYSCALABLEENABLED_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for QualityScalableEnabled {
    fn type_info() -> &'static TypeInfo {
        QUALITYSCALABLEENABLED_TYPE_INFO
    }
}


pub const QUALITYSCALABLEENABLED_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableEnabled-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("QualityScalableEnabled-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct QualityScalableBool {
    pub low: bool,
    pub medium: bool,
    pub high: bool,
    pub ultra: bool,
    pub cinematic: bool,
}

pub const QUALITYSCALABLEBOOL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableBool",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Low",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(QualityScalableBool, low),
            },
            FieldInfoData {
                name: "Medium",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(QualityScalableBool, medium),
            },
            FieldInfoData {
                name: "High",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(QualityScalableBool, high),
            },
            FieldInfoData {
                name: "Ultra",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(QualityScalableBool, ultra),
            },
            FieldInfoData {
                name: "Cinematic",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(QualityScalableBool, cinematic),
            },
        ],
    }),
    array_type: Some(QUALITYSCALABLEBOOL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for QualityScalableBool {
    fn type_info() -> &'static TypeInfo {
        QUALITYSCALABLEBOOL_TYPE_INFO
    }
}


pub const QUALITYSCALABLEBOOL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableBool-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("QualityScalableBool-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct QualityScalableFloat {
    pub low: f32,
    pub medium: f32,
    pub high: f32,
    pub ultra: f32,
    pub cinematic: f32,
}

pub const QUALITYSCALABLEFLOAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableFloat",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Low",
                flags: MemberInfoFlags::new(8192),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(QualityScalableFloat, low),
            },
            FieldInfoData {
                name: "Medium",
                flags: MemberInfoFlags::new(8192),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(QualityScalableFloat, medium),
            },
            FieldInfoData {
                name: "High",
                flags: MemberInfoFlags::new(8192),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(QualityScalableFloat, high),
            },
            FieldInfoData {
                name: "Ultra",
                flags: MemberInfoFlags::new(8192),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(QualityScalableFloat, ultra),
            },
            FieldInfoData {
                name: "Cinematic",
                flags: MemberInfoFlags::new(8192),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(QualityScalableFloat, cinematic),
            },
        ],
    }),
    array_type: Some(QUALITYSCALABLEFLOAT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for QualityScalableFloat {
    fn type_info() -> &'static TypeInfo {
        QUALITYSCALABLEFLOAT_TYPE_INFO
    }
}


pub const QUALITYSCALABLEFLOAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableFloat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("QualityScalableFloat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct QualityScalableInt {
    pub low: i32,
    pub medium: i32,
    pub high: i32,
    pub ultra: i32,
    pub cinematic: i32,
}

pub const QUALITYSCALABLEINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableInt",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Low",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(QualityScalableInt, low),
            },
            FieldInfoData {
                name: "Medium",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(QualityScalableInt, medium),
            },
            FieldInfoData {
                name: "High",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(QualityScalableInt, high),
            },
            FieldInfoData {
                name: "Ultra",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(QualityScalableInt, ultra),
            },
            FieldInfoData {
                name: "Cinematic",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(QualityScalableInt, cinematic),
            },
        ],
    }),
    array_type: Some(QUALITYSCALABLEINT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for QualityScalableInt {
    fn type_info() -> &'static TypeInfo {
        QUALITYSCALABLEINT_TYPE_INFO
    }
}


pub const QUALITYSCALABLEINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityScalableInt-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("QualityScalableInt-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const QUALITYLEVEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityLevel",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(QUALITYLEVEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for QualityLevel {
    fn type_info() -> &'static TypeInfo {
        QUALITYLEVEL_TYPE_INFO
    }
}


pub const QUALITYLEVEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QualityLevel-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("QualityLevel-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ResourceManagerSettings {
    pub cas_bundle_read_buffer_size_kb: i32,
    pub cas_bundle_decompress_buffer_size_kb: i32,
    pub cas_bundle_decompress_buffer_count: i32,
    pub bundle_profiling_enable: bool,
}

pub const RESOURCEMANAGERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceManagerSettings",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CasBundleReadBufferSizeKb",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ResourceManagerSettings, cas_bundle_read_buffer_size_kb),
            },
            FieldInfoData {
                name: "CasBundleDecompressBufferSizeKb",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ResourceManagerSettings, cas_bundle_decompress_buffer_size_kb),
            },
            FieldInfoData {
                name: "CasBundleDecompressBufferCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ResourceManagerSettings, cas_bundle_decompress_buffer_count),
            },
            FieldInfoData {
                name: "BundleProfilingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ResourceManagerSettings, bundle_profiling_enable),
            },
        ],
    }),
    array_type: Some(RESOURCEMANAGERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ResourceManagerSettings {
    fn type_info() -> &'static TypeInfo {
        RESOURCEMANAGERSETTINGS_TYPE_INFO
    }
}


pub const RESOURCEMANAGERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceManagerSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ResourceManagerSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
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

pub const RENDERINGOVERRIDES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderingOverrides",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLOVERRIDE_TYPE_INFO,
                rust_offset: offset_of!(RenderingOverrides, shadow_enable),
            },
            FieldInfoData {
                name: "SunShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLOVERRIDE_TYPE_INFO,
                rust_offset: offset_of!(RenderingOverrides, sun_shadow_enable),
            },
            FieldInfoData {
                name: "LocalShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLOVERRIDE_TYPE_INFO,
                rust_offset: offset_of!(RenderingOverrides, local_shadow_enable),
            },
            FieldInfoData {
                name: "DynamicReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLOVERRIDE_TYPE_INFO,
                rust_offset: offset_of!(RenderingOverrides, dynamic_reflection_enable),
            },
            FieldInfoData {
                name: "StaticReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLOVERRIDE_TYPE_INFO,
                rust_offset: offset_of!(RenderingOverrides, static_reflection_enable),
            },
            FieldInfoData {
                name: "PlanarShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLOVERRIDE_TYPE_INFO,
                rust_offset: offset_of!(RenderingOverrides, planar_shadow_enable),
            },
            FieldInfoData {
                name: "HologramEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLOVERRIDE_TYPE_INFO,
                rust_offset: offset_of!(RenderingOverrides, hologram_enable),
            },
            FieldInfoData {
                name: "HologramProjectorIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RenderingOverrides, hologram_projector_index),
            },
            FieldInfoData {
                name: "DistantShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLOVERRIDE_TYPE_INFO,
                rust_offset: offset_of!(RenderingOverrides, distant_shadow_cache_enable),
            },
            FieldInfoData {
                name: "DynamicDistantShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLOVERRIDE_TYPE_INFO,
                rust_offset: offset_of!(RenderingOverrides, dynamic_distant_shadow_cache_enable),
            },
            FieldInfoData {
                name: "LocalShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLOVERRIDE_TYPE_INFO,
                rust_offset: offset_of!(RenderingOverrides, local_shadow_cache_enable),
            },
            FieldInfoData {
                name: "RootShaderEffect",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLOVERRIDE_TYPE_INFO,
                rust_offset: offset_of!(RenderingOverrides, root_shader_effect),
            },
        ],
    }),
    array_type: Some(RENDERINGOVERRIDES_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for RenderingOverrides {
    fn type_info() -> &'static TypeInfo {
        RENDERINGOVERRIDES_TYPE_INFO
    }
}


pub const RENDERINGOVERRIDES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderingOverrides-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RenderingOverrides-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ChunkOnlyResourceType {
}

pub const CHUNKONLYRESOURCETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChunkOnlyResourceType",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(CHUNKONLYRESOURCETYPE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ChunkOnlyResourceType {
    fn type_info() -> &'static TypeInfo {
        CHUNKONLYRESOURCETYPE_TYPE_INFO
    }
}


pub const CHUNKONLYRESOURCETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ChunkOnlyResourceType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ChunkOnlyResourceType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IResourceObject {
}

pub const IRESOURCEOBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IResourceObject",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(IRESOURCEOBJECT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IResourceObject {
    fn type_info() -> &'static TypeInfo {
        IRESOURCEOBJECT_TYPE_INFO
    }
}


pub const IRESOURCEOBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IResourceObject-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("IResourceObject-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProxyResourceObject {
}

pub const PROXYRESOURCEOBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProxyResourceObject",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(PROXYRESOURCEOBJECT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ProxyResourceObject {
    fn type_info() -> &'static TypeInfo {
        PROXYRESOURCEOBJECT_TYPE_INFO
    }
}


pub const PROXYRESOURCEOBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProxyResourceObject-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ProxyResourceObject-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum BoolOverride {
    #[default]
    BoolOverride_Inherit = 0,
    BoolOverride_Enable = 1,
    BoolOverride_Disable = 2,
}

pub const BOOLOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolOverride",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(BOOLOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BoolOverride {
    fn type_info() -> &'static TypeInfo {
        BOOLOVERRIDE_TYPE_INFO
    }
}


pub const BOOLOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("BoolOverride-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DataContainerPolicyPipelineResultBase {
    pub secondary_result: DataContainer,
}

pub const DATACONTAINERPOLICYPIPELINERESULTBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainerPolicyPipelineResultBase",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SETDATARESULTLISTENER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SecondaryResult",
                flags: MemberInfoFlags::new(0),
                field_type: DATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(DataContainerPolicyPipelineResultBase, secondary_result),
            },
        ],
    }),
    array_type: Some(DATACONTAINERPOLICYPIPELINERESULTBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DataContainerPolicyPipelineResultBase {
    fn type_info() -> &'static TypeInfo {
        DATACONTAINERPOLICYPIPELINERESULTBASE_TYPE_INFO
    }
}


pub const DATACONTAINERPOLICYPIPELINERESULTBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainerPolicyPipelineResultBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DataContainerPolicyPipelineResultBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SetDataResultListener {
}

pub const SETDATARESULTLISTENER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetDataResultListener",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SETDATARESULTLISTENER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SetDataResultListener {
    fn type_info() -> &'static TypeInfo {
        SETDATARESULTLISTENER_TYPE_INFO
    }
}


pub const SETDATARESULTLISTENER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SetDataResultListener-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SetDataResultListener-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DataBusData {
    pub flags: u16,
    pub property_connections: Vec<PropertyConnection>,
    pub link_connections: Vec<LinkConnection>,
    pub interface: DynamicDataContainer,
}

pub const DATABUSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataBusData",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Flags",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(DataBusData, flags),
            },
            FieldInfoData {
                name: "PropertyConnections",
                flags: MemberInfoFlags::new(144),
                field_type: PROPERTYCONNECTION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DataBusData, property_connections),
            },
            FieldInfoData {
                name: "LinkConnections",
                flags: MemberInfoFlags::new(144),
                field_type: LINKCONNECTION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DataBusData, link_connections),
            },
            FieldInfoData {
                name: "Interface",
                flags: MemberInfoFlags::new(0),
                field_type: DYNAMICDATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(DataBusData, interface),
            },
        ],
    }),
    array_type: Some(DATABUSDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DataBusData {
    fn type_info() -> &'static TypeInfo {
        DATABUSDATA_TYPE_INFO
    }
}


pub const DATABUSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataBusData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DataBusData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DynamicDataContainer {
    pub fields: Vec<DataField>,
}

pub const DYNAMICDATACONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicDataContainer",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Fields",
                flags: MemberInfoFlags::new(144),
                field_type: DATAFIELD_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DynamicDataContainer, fields),
            },
        ],
    }),
    array_type: Some(DYNAMICDATACONTAINER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DynamicDataContainer {
    fn type_info() -> &'static TypeInfo {
        DYNAMICDATACONTAINER_TYPE_INFO
    }
}


pub const DYNAMICDATACONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicDataContainer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DynamicDataContainer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DataField {
    pub value: String,
    pub value_ref: DataContainer,
    pub id: i32,
    pub access_type: FieldAccessType,
}

pub const DATAFIELD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataField",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DataField, value),
            },
            FieldInfoData {
                name: "ValueRef",
                flags: MemberInfoFlags::new(0),
                field_type: DATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(DataField, value_ref),
            },
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DataField, id),
            },
            FieldInfoData {
                name: "AccessType",
                flags: MemberInfoFlags::new(0),
                field_type: FIELDACCESSTYPE_TYPE_INFO,
                rust_offset: offset_of!(DataField, access_type),
            },
        ],
    }),
    array_type: Some(DATAFIELD_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DataField {
    fn type_info() -> &'static TypeInfo {
        DATAFIELD_TYPE_INFO
    }
}


pub const DATAFIELD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataField-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DataField-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum FieldAccessType {
    #[default]
    FieldAccessType_Source = 0,
    FieldAccessType_Target = 1,
    FieldAccessType_SourceAndTarget = 2,
}

pub const FIELDACCESSTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FieldAccessType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(FIELDACCESSTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FieldAccessType {
    fn type_info() -> &'static TypeInfo {
        FIELDACCESSTYPE_TYPE_INFO
    }
}


pub const FIELDACCESSTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FieldAccessType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FieldAccessType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinkConnection {
    pub source: DataContainer,
    pub target: DataContainer,
    pub source_field_id: i32,
    pub target_field_id: i32,
}

pub const LINKCONNECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkConnection",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Source",
                flags: MemberInfoFlags::new(0),
                field_type: DATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(LinkConnection, source),
            },
            FieldInfoData {
                name: "Target",
                flags: MemberInfoFlags::new(0),
                field_type: DATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(LinkConnection, target),
            },
            FieldInfoData {
                name: "SourceFieldId",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LinkConnection, source_field_id),
            },
            FieldInfoData {
                name: "TargetFieldId",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LinkConnection, target_field_id),
            },
        ],
    }),
    array_type: Some(LINKCONNECTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinkConnection {
    fn type_info() -> &'static TypeInfo {
        LINKCONNECTION_TYPE_INFO
    }
}


pub const LINKCONNECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkConnection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("LinkConnection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PropertyConnection {
    pub source: DataContainer,
    pub target: DataContainer,
    pub source_field_id: i32,
    pub target_field_id: i32,
    pub flags: u32,
}

pub const PROPERTYCONNECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyConnection",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Source",
                flags: MemberInfoFlags::new(0),
                field_type: DATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(PropertyConnection, source),
            },
            FieldInfoData {
                name: "Target",
                flags: MemberInfoFlags::new(0),
                field_type: DATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(PropertyConnection, target),
            },
            FieldInfoData {
                name: "SourceFieldId",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PropertyConnection, source_field_id),
            },
            FieldInfoData {
                name: "TargetFieldId",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PropertyConnection, target_field_id),
            },
            FieldInfoData {
                name: "Flags",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PropertyConnection, flags),
            },
        ],
    }),
    array_type: Some(PROPERTYCONNECTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PropertyConnection {
    fn type_info() -> &'static TypeInfo {
        PROPERTYCONNECTION_TYPE_INFO
    }
}


pub const PROPERTYCONNECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyConnection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PropertyConnection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum InputPropertyType {
    #[default]
    InputPropertyType_Default = 0,
    InputPropertyType_Interface = 1,
    InputPropertyType_Exposed = 2,
    InputPropertyType_Invalid = 3,
}

pub const INPUTPROPERTYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputPropertyType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(INPUTPROPERTYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InputPropertyType {
    fn type_info() -> &'static TypeInfo {
        INPUTPROPERTYTYPE_TYPE_INFO
    }
}


pub const INPUTPROPERTYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputPropertyType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("InputPropertyType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PropertyConnectionTargetType {
    #[default]
    PropertyConnectionTargetType_Invalid = 0,
    PropertyConnectionTargetType_ClientAndServer = 1,
    PropertyConnectionTargetType_Client = 2,
    PropertyConnectionTargetType_Server = 3,
    PropertyConnectionTargetType_NetworkedClient = 4,
    PropertyConnectionTargetType_NetworkedClientAndServer = 5,
}

pub const PROPERTYCONNECTIONTARGETTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyConnectionTargetType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(PROPERTYCONNECTIONTARGETTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PropertyConnectionTargetType {
    fn type_info() -> &'static TypeInfo {
        PROPERTYCONNECTIONTARGETTYPE_TYPE_INFO
    }
}


pub const PROPERTYCONNECTIONTARGETTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyConnectionTargetType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PropertyConnectionTargetType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SettingsBundleAsset {
    pub is_dedicated_server_bundle: bool,
    pub settings: Vec<SystemSettings>,
}

pub const SETTINGSBUNDLEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SettingsBundleAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "IsDedicatedServerBundle",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SettingsBundleAsset, is_dedicated_server_bundle),
            },
            FieldInfoData {
                name: "Settings",
                flags: MemberInfoFlags::new(144),
                field_type: SYSTEMSETTINGS_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SettingsBundleAsset, settings),
            },
        ],
    }),
    array_type: Some(SETTINGSBUNDLEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SettingsBundleAsset {
    fn type_info() -> &'static TypeInfo {
        SETTINGSBUNDLEASSET_TYPE_INFO
    }
}


pub const SETTINGSBUNDLEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SettingsBundleAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SettingsBundleAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SystemSettings {
    pub platform: GamePlatform,
}

pub const SYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SystemSettings",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLATFORM_TYPE_INFO,
                rust_offset: offset_of!(SystemSettings, platform),
            },
        ],
    }),
    array_type: Some(SYSTEMSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SystemSettings {
    fn type_info() -> &'static TypeInfo {
        SYSTEMSETTINGS_TYPE_INFO
    }
}


pub const SYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SystemSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SystemSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DataBusPeer {
    pub flags: u32,
}

pub const DATABUSPEER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataBusPeer",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEDATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Flags",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DataBusPeer, flags),
            },
        ],
    }),
    array_type: Some(DATABUSPEER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DataBusPeer {
    fn type_info() -> &'static TypeInfo {
        DATABUSPEER_TYPE_INFO
    }
}


pub const DATABUSPEER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataBusPeer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DataBusPeer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameDataContainer {
}

pub const GAMEDATACONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameDataContainer",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GAMEDATACONTAINER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameDataContainer {
    fn type_info() -> &'static TypeInfo {
        GAMEDATACONTAINER_TYPE_INFO
    }
}


pub const GAMEDATACONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameDataContainer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("GameDataContainer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum Realm {
    #[default]
    Realm_Client = 0,
    Realm_Server = 1,
    Realm_ClientAndServer = 2,
    Realm_None = 3,
    Realm_Pipeline = 4,
}

pub const REALM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Realm",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(REALM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for Realm {
    fn type_info() -> &'static TypeInfo {
        REALM_TYPE_INFO
    }
}


pub const REALM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Realm-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Realm-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const LANGUAGEFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanguageFormat",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(LANGUAGEFORMAT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LanguageFormat {
    fn type_info() -> &'static TypeInfo {
        LANGUAGEFORMAT_TYPE_INFO
    }
}


pub const LANGUAGEFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanguageFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("LanguageFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DataContainerPolicyAsset {
}

pub const DATACONTAINERPOLICYASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainerPolicyAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DATACONTAINERPOLICYASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DataContainerPolicyAsset {
    fn type_info() -> &'static TypeInfo {
        DATACONTAINERPOLICYASSET_TYPE_INFO
    }
}


pub const DATACONTAINERPOLICYASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainerPolicyAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DataContainerPolicyAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Asset {
    pub name: String,
}

pub const ASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Asset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(Asset, name),
            },
        ],
    }),
    array_type: Some(ASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Asset {
    fn type_info() -> &'static TypeInfo {
        ASSET_TYPE_INFO
    }
}


pub const ASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Asset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Asset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NativeFunctionTypeInfoAsset {
    pub function: TypeRef,
}

pub const NATIVEFUNCTIONTYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NativeFunctionTypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FUNCTIONTYPEINFOASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Function",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(NativeFunctionTypeInfoAsset, function),
            },
        ],
    }),
    array_type: Some(NATIVEFUNCTIONTYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NativeFunctionTypeInfoAsset {
    fn type_info() -> &'static TypeInfo {
        NATIVEFUNCTIONTYPEINFOASSET_TYPE_INFO
    }
}


pub const NATIVEFUNCTIONTYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NativeFunctionTypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("NativeFunctionTypeInfoAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldStructValue {
    pub value: BoxedValueRef,
}

pub const TYPEINFOFIELDSTRUCTVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldStructValue",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: BOXEDVALUEREF_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldStructValue, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDSTRUCTVALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldStructValue {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDSTRUCTVALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDSTRUCTVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldStructValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldStructValue-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldRefValue {
    pub value: DataContainer,
}

pub const TYPEINFOFIELDREFVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldRefValue",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: DATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldRefValue, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDREFVALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldRefValue {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDREFVALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDREFVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldRefValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldRefValue-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldTypeRefValue {
    pub value: TypeRef,
}

pub const TYPEINFOFIELDTYPEREFVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldTypeRefValue",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldTypeRefValue, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDTYPEREFVALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldTypeRefValue {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDTYPEREFVALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDTYPEREFVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldTypeRefValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldTypeRefValue-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldSHA1Value {
    pub value: SHA1,
}

pub const TYPEINFOFIELDSHA1VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldSHA1Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: SHA1_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldSHA1Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDSHA1VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldSHA1Value {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDSHA1VALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDSHA1VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldSHA1Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldSHA1Value-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldGuidValue {
    pub value: Guid,
}

pub const TYPEINFOFIELDGUIDVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldGuidValue",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldGuidValue, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDGUIDVALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldGuidValue {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDGUIDVALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDGUIDVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldGuidValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldGuidValue-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldEnumValue {
    pub enum_info: TypeRef,
    pub value: u32,
}

pub const TYPEINFOFIELDENUMVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldEnumValue",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EnumInfo",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldEnumValue, enum_info),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldEnumValue, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDENUMVALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldEnumValue {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDENUMVALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDENUMVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldEnumValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldEnumValue-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldStringValue {
    pub value: String,
}

pub const TYPEINFOFIELDSTRINGVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldStringValue",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldStringValue, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDSTRINGVALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldStringValue {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDSTRINGVALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDSTRINGVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldStringValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldStringValue-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TypeInfoFieldFloat64Value {
    pub value: f64,
}

pub const TYPEINFOFIELDFLOAT64VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldFloat64Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT64_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldFloat64Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDFLOAT64VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldFloat64Value {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDFLOAT64VALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDFLOAT64VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldFloat64Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldFloat64Value-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TypeInfoFieldFloat32Value {
    pub value: f32,
}

pub const TYPEINFOFIELDFLOAT32VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldFloat32Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldFloat32Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDFLOAT32VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldFloat32Value {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDFLOAT32VALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDFLOAT32VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldFloat32Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldFloat32Value-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldUint64Value {
    pub value: u64,
}

pub const TYPEINFOFIELDUINT64VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint64Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldUint64Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDUINT64VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldUint64Value {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDUINT64VALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDUINT64VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint64Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldUint64Value-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldUint32Value {
    pub value: u32,
}

pub const TYPEINFOFIELDUINT32VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint32Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldUint32Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDUINT32VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldUint32Value {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDUINT32VALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDUINT32VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint32Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldUint32Value-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldUint16Value {
    pub value: u16,
}

pub const TYPEINFOFIELDUINT16VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint16Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldUint16Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDUINT16VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldUint16Value {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDUINT16VALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDUINT16VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint16Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldUint16Value-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldUint8Value {
    pub value: u8,
}

pub const TYPEINFOFIELDUINT8VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint8Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldUint8Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDUINT8VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldUint8Value {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDUINT8VALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDUINT8VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldUint8Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldUint8Value-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldInt64Value {
    pub value: i64,
}

pub const TYPEINFOFIELDINT64VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt64Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldInt64Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDINT64VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldInt64Value {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDINT64VALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDINT64VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt64Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldInt64Value-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldInt32Value {
    pub value: i32,
}

pub const TYPEINFOFIELDINT32VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt32Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldInt32Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDINT32VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldInt32Value {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDINT32VALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDINT32VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt32Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldInt32Value-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldInt16Value {
    pub value: i16,
}

pub const TYPEINFOFIELDINT16VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt16Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: INT16_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldInt16Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDINT16VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldInt16Value {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDINT16VALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDINT16VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt16Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldInt16Value-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldInt8Value {
    pub value: i8,
}

pub const TYPEINFOFIELDINT8VALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt8Value",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: INT8_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldInt8Value, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDINT8VALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldInt8Value {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDINT8VALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDINT8VALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldInt8Value-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldInt8Value-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldBooleanValue {
    pub value: bool,
}

pub const TYPEINFOFIELDBOOLEANVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldBooleanValue",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOFIELDVALUE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldBooleanValue, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDBOOLEANVALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldBooleanValue {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDBOOLEANVALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDBOOLEANVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldBooleanValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldBooleanValue-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PrimitiveTypeInfoAsset {
}

pub const PRIMITIVETYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrimitiveTypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRIMITIVETYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PrimitiveTypeInfoAsset {
    fn type_info() -> &'static TypeInfo {
        PRIMITIVETYPEINFOASSET_TYPE_INFO
    }
}


pub const PRIMITIVETYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrimitiveTypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PrimitiveTypeInfoAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnumTypeInfoAsset {
    pub enumerators: Vec<EnumTypeEnumeratorData>,
}

pub const ENUMTYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumTypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enumerators",
                flags: MemberInfoFlags::new(144),
                field_type: ENUMTYPEENUMERATORDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EnumTypeInfoAsset, enumerators),
            },
        ],
    }),
    array_type: Some(ENUMTYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnumTypeInfoAsset {
    fn type_info() -> &'static TypeInfo {
        ENUMTYPEINFOASSET_TYPE_INFO
    }
}


pub const ENUMTYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumTypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("EnumTypeInfoAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnumTypeEnumeratorData {
    pub name: String,
    pub value: i32,
    pub attributes: Vec<TypeInfoAttribute>,
}

pub const ENUMTYPEENUMERATORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumTypeEnumeratorData",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EnumTypeEnumeratorData, name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EnumTypeEnumeratorData, value),
            },
            FieldInfoData {
                name: "Attributes",
                flags: MemberInfoFlags::new(144),
                field_type: TYPEINFOATTRIBUTE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EnumTypeEnumeratorData, attributes),
            },
        ],
    }),
    array_type: Some(ENUMTYPEENUMERATORDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnumTypeEnumeratorData {
    fn type_info() -> &'static TypeInfo {
        ENUMTYPEENUMERATORDATA_TYPE_INFO
    }
}


pub const ENUMTYPEENUMERATORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnumTypeEnumeratorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("EnumTypeEnumeratorData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FunctionTypeInfoAsset {
    pub parameters: Vec<TypeInfoParameterDataContainer>,
    pub owner: ClassInfoAsset,
}

pub const FUNCTIONTYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FunctionTypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Parameters",
                flags: MemberInfoFlags::new(144),
                field_type: TYPEINFOPARAMETERDATACONTAINER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(FunctionTypeInfoAsset, parameters),
            },
            FieldInfoData {
                name: "Owner",
                flags: MemberInfoFlags::new(0),
                field_type: CLASSINFOASSET_TYPE_INFO,
                rust_offset: offset_of!(FunctionTypeInfoAsset, owner),
            },
        ],
    }),
    array_type: Some(FUNCTIONTYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FunctionTypeInfoAsset {
    fn type_info() -> &'static TypeInfo {
        FUNCTIONTYPEINFOASSET_TYPE_INFO
    }
}


pub const FUNCTIONTYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FunctionTypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FunctionTypeInfoAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoParameterDataContainer {
    pub name: String,
    pub type_ref: TypeInfoRef,
    pub is_array: bool,
    pub qualifier: TypeInfoParameterQualifier,
    pub default_value: DataContainer,
}

pub const TYPEINFOPARAMETERDATACONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoParameterDataContainer",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoParameterDataContainer, name),
            },
            FieldInfoData {
                name: "TypeRef",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEINFOREF_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoParameterDataContainer, type_ref),
            },
            FieldInfoData {
                name: "IsArray",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoParameterDataContainer, is_array),
            },
            FieldInfoData {
                name: "Qualifier",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEINFOPARAMETERQUALIFIER_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoParameterDataContainer, qualifier),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: DATACONTAINER_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoParameterDataContainer, default_value),
            },
        ],
    }),
    array_type: Some(TYPEINFOPARAMETERDATACONTAINER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoParameterDataContainer {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOPARAMETERDATACONTAINER_TYPE_INFO
    }
}


pub const TYPEINFOPARAMETERDATACONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoParameterDataContainer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoParameterDataContainer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TypeInfoParameterQualifier {
    #[default]
    TypeInfoParameterQualifier_In = 0,
    TypeInfoParameterQualifier_Out = 1,
    TypeInfoParameterQualifier_InRef = 2,
    TypeInfoParameterQualifier_OutRef = 3,
}

pub const TYPEINFOPARAMETERQUALIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoParameterQualifier",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(TYPEINFOPARAMETERQUALIFIER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TypeInfoParameterQualifier {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOPARAMETERQUALIFIER_TYPE_INFO
    }
}


pub const TYPEINFOPARAMETERQUALIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoParameterQualifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoParameterQualifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FunctionTypeInfoRef {
    pub asset: FunctionTypeInfoAsset,
    pub type_info: TypeRef,
}

pub const FUNCTIONTYPEINFOREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FunctionTypeInfoRef",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: FUNCTIONTYPEINFOASSET_TYPE_INFO,
                rust_offset: offset_of!(FunctionTypeInfoRef, asset),
            },
            FieldInfoData {
                name: "TypeInfo",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(FunctionTypeInfoRef, type_info),
            },
        ],
    }),
    array_type: Some(FUNCTIONTYPEINFOREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FunctionTypeInfoRef {
    fn type_info() -> &'static TypeInfo {
        FUNCTIONTYPEINFOREF_TYPE_INFO
    }
}


pub const FUNCTIONTYPEINFOREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FunctionTypeInfoRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FunctionTypeInfoRef-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ValueTypeInfoAsset {
}

pub const VALUETYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValueTypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPLEXTYPEINFOASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VALUETYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ValueTypeInfoAsset {
    fn type_info() -> &'static TypeInfo {
        VALUETYPEINFOASSET_TYPE_INFO
    }
}


pub const VALUETYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ValueTypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ValueTypeInfoAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClassInfoAsset {
    pub super_class_ref: ClassInfoRef,
    pub is_abstract: bool,
    pub is_sealed: bool,
}

pub const CLASSINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClassInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPLEXTYPEINFOASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SuperClassRef",
                flags: MemberInfoFlags::new(0),
                field_type: CLASSINFOREF_TYPE_INFO,
                rust_offset: offset_of!(ClassInfoAsset, super_class_ref),
            },
            FieldInfoData {
                name: "IsAbstract",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClassInfoAsset, is_abstract),
            },
            FieldInfoData {
                name: "IsSealed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClassInfoAsset, is_sealed),
            },
        ],
    }),
    array_type: Some(CLASSINFOASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClassInfoAsset {
    fn type_info() -> &'static TypeInfo {
        CLASSINFOASSET_TYPE_INFO
    }
}


pub const CLASSINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClassInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ClassInfoAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClassInfoRef {
    pub asset: ClassInfoAsset,
    pub type_info: TypeRef,
}

pub const CLASSINFOREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClassInfoRef",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: CLASSINFOASSET_TYPE_INFO,
                rust_offset: offset_of!(ClassInfoRef, asset),
            },
            FieldInfoData {
                name: "TypeInfo",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(ClassInfoRef, type_info),
            },
        ],
    }),
    array_type: Some(CLASSINFOREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClassInfoRef {
    fn type_info() -> &'static TypeInfo {
        CLASSINFOREF_TYPE_INFO
    }
}


pub const CLASSINFOREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClassInfoRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ClassInfoRef-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ComplexTypeInfoAsset {
    pub field_collections: Vec<TypeInfoFieldCollectionRef>,
    pub alignment: u32,
}

pub const COMPLEXTYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComplexTypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TYPEINFOASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FieldCollections",
                flags: MemberInfoFlags::new(144),
                field_type: TYPEINFOFIELDCOLLECTIONREF_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ComplexTypeInfoAsset, field_collections),
            },
            FieldInfoData {
                name: "Alignment",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ComplexTypeInfoAsset, alignment),
            },
        ],
    }),
    array_type: Some(COMPLEXTYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ComplexTypeInfoAsset {
    fn type_info() -> &'static TypeInfo {
        COMPLEXTYPEINFOASSET_TYPE_INFO
    }
}


pub const COMPLEXTYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ComplexTypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ComplexTypeInfoAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldCollectionRef {
    pub collection: TypeInfoFieldCollection,
}

pub const TYPEINFOFIELDCOLLECTIONREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldCollectionRef",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Collection",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEINFOFIELDCOLLECTION_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldCollectionRef, collection),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDCOLLECTIONREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldCollectionRef {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDCOLLECTIONREF_TYPE_INFO
    }
}


pub const TYPEINFOFIELDCOLLECTIONREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldCollectionRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldCollectionRef-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldCollection {
    pub fields: Vec<TypeInfoFieldData>,
    pub default_values: Vec<TypeInfoFieldValue>,
}

pub const TYPEINFOFIELDCOLLECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldCollection",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Fields",
                flags: MemberInfoFlags::new(144),
                field_type: TYPEINFOFIELDDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldCollection, fields),
            },
            FieldInfoData {
                name: "DefaultValues",
                flags: MemberInfoFlags::new(144),
                field_type: TYPEINFOFIELDVALUE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldCollection, default_values),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDCOLLECTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldCollection {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDCOLLECTION_TYPE_INFO
    }
}


pub const TYPEINFOFIELDCOLLECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldCollection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldCollection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldValue {
    pub field: String,
}

pub const TYPEINFOFIELDVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldValue",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Field",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldValue, field),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDVALUE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TypeInfoFieldValue {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDVALUE_TYPE_INFO
    }
}


pub const TYPEINFOFIELDVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldValue-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoFieldData {
    pub name: String,
    pub type_ref: TypeInfoRef,
    pub is_array: bool,
    pub protection_level: ProtectionLevel,
    pub memory_sort_index: u32,
    pub is_meta: bool,
    pub is_exposed: bool,
    pub access_type: AccessType,
    pub always_persist: bool,
    pub attributes: Vec<TypeInfoAttribute>,
}

pub const TYPEINFOFIELDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldData",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldData, name),
            },
            FieldInfoData {
                name: "TypeRef",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEINFOREF_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldData, type_ref),
            },
            FieldInfoData {
                name: "IsArray",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldData, is_array),
            },
            FieldInfoData {
                name: "ProtectionLevel",
                flags: MemberInfoFlags::new(0),
                field_type: PROTECTIONLEVEL_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldData, protection_level),
            },
            FieldInfoData {
                name: "MemorySortIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldData, memory_sort_index),
            },
            FieldInfoData {
                name: "IsMeta",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldData, is_meta),
            },
            FieldInfoData {
                name: "IsExposed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldData, is_exposed),
            },
            FieldInfoData {
                name: "AccessType",
                flags: MemberInfoFlags::new(0),
                field_type: ACCESSTYPE_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldData, access_type),
            },
            FieldInfoData {
                name: "AlwaysPersist",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldData, always_persist),
            },
            FieldInfoData {
                name: "Attributes",
                flags: MemberInfoFlags::new(144),
                field_type: TYPEINFOATTRIBUTE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoFieldData, attributes),
            },
        ],
    }),
    array_type: Some(TYPEINFOFIELDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoFieldData {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOFIELDDATA_TYPE_INFO
    }
}


pub const TYPEINFOFIELDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoFieldData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoFieldData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AccessType {
    #[default]
    AccessType_Member = 0,
    AccessType_Const = 1,
}

pub const ACCESSTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AccessType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(ACCESSTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AccessType {
    fn type_info() -> &'static TypeInfo {
        ACCESSTYPE_TYPE_INFO
    }
}


pub const ACCESSTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AccessType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("AccessType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ProtectionLevel {
    #[default]
    ProtectionLevel_Private = 0,
    ProtectionLevel_Protected = 1,
    ProtectionLevel_Public = 2,
}

pub const PROTECTIONLEVEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProtectionLevel",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(PROTECTIONLEVEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ProtectionLevel {
    fn type_info() -> &'static TypeInfo {
        PROTECTIONLEVEL_TYPE_INFO
    }
}


pub const PROTECTIONLEVEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProtectionLevel-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ProtectionLevel-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoRef {
    pub asset: TypeInfoAsset,
    pub type_info: TypeRef,
}

pub const TYPEINFOREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoRef",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEINFOASSET_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoRef, asset),
            },
            FieldInfoData {
                name: "TypeInfo",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoRef, type_info),
            },
        ],
    }),
    array_type: Some(TYPEINFOREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoRef {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOREF_TYPE_INFO
    }
}


pub const TYPEINFOREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoRef-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoAsset {
    pub module_name: String,
    pub type_name: String,
    pub is_meta: bool,
    pub attributes: Vec<TypeInfoAttribute>,
    pub is_native: bool,
}

pub const TYPEINFOASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ModuleName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoAsset, module_name),
            },
            FieldInfoData {
                name: "TypeName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoAsset, type_name),
            },
            FieldInfoData {
                name: "IsMeta",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoAsset, is_meta),
            },
            FieldInfoData {
                name: "Attributes",
                flags: MemberInfoFlags::new(144),
                field_type: TYPEINFOATTRIBUTE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoAsset, attributes),
            },
            FieldInfoData {
                name: "IsNative",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoAsset, is_native),
            },
        ],
    }),
    array_type: Some(TYPEINFOASSET_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TypeInfoAsset {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOASSET_TYPE_INFO
    }
}


pub const TYPEINFOASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoAttribute {
    pub name: String,
    pub arguments: Vec<TypeInfoAttributeArgument>,
    pub is_native: bool,
}

pub const TYPEINFOATTRIBUTE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoAttribute",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoAttribute, name),
            },
            FieldInfoData {
                name: "Arguments",
                flags: MemberInfoFlags::new(144),
                field_type: TYPEINFOATTRIBUTEARGUMENT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoAttribute, arguments),
            },
            FieldInfoData {
                name: "IsNative",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoAttribute, is_native),
            },
        ],
    }),
    array_type: Some(TYPEINFOATTRIBUTE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoAttribute {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOATTRIBUTE_TYPE_INFO
    }
}


pub const TYPEINFOATTRIBUTE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoAttribute-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoAttribute-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeInfoAttributeArgument {
    pub name: String,
    pub value: String,
}

pub const TYPEINFOATTRIBUTEARGUMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoAttributeArgument",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoAttributeArgument, name),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TypeInfoAttributeArgument, value),
            },
        ],
    }),
    array_type: Some(TYPEINFOATTRIBUTEARGUMENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TypeInfoAttributeArgument {
    fn type_info() -> &'static TypeInfo {
        TYPEINFOATTRIBUTEARGUMENT_TYPE_INFO
    }
}


pub const TYPEINFOATTRIBUTEARGUMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeInfoAttributeArgument-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeInfoAttributeArgument-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FunctionDelegateRef {
    pub type_info: TypeRef,
}

pub const FUNCTIONDELEGATEREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FunctionDelegateRef",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TypeInfo",
                flags: MemberInfoFlags::new(0),
                field_type: TYPEREF_TYPE_INFO,
                rust_offset: offset_of!(FunctionDelegateRef, type_info),
            },
        ],
    }),
    array_type: Some(FUNCTIONDELEGATEREF_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FunctionDelegateRef {
    fn type_info() -> &'static TypeInfo {
        FUNCTIONDELEGATEREF_TYPE_INFO
    }
}


pub const FUNCTIONDELEGATEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FunctionDelegateRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FunctionDelegateRef-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventDelay {
}

pub const EVENTDELAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventDelay",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(EVENTDELAY_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EventDelay {
    fn type_info() -> &'static TypeInfo {
        EVENTDELAY_TYPE_INFO
    }
}


pub const EVENTDELAY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventDelay-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("EventDelay-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TimingViewSettings {
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

pub const TIMINGVIEWSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimingViewSettings",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, enable),
            },
            FieldInfoData {
                name: "FrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, frame_count),
            },
            FieldInfoData {
                name: "FrameDelayCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, frame_delay_count),
            },
            FieldInfoData {
                name: "FrameCountToReport",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, frame_count_to_report),
            },
            FieldInfoData {
                name: "TimeRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, time_range),
            },
            FieldInfoData {
                name: "TimeOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, time_offset),
            },
            FieldInfoData {
                name: "LogThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, log_threshold),
            },
            FieldInfoData {
                name: "AutolockThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, autolock_threshold),
            },
            FieldInfoData {
                name: "AutolockNameFilter",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, autolock_name_filter),
            },
            FieldInfoData {
                name: "LegendScreenOffset",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, legend_screen_offset),
            },
            FieldInfoData {
                name: "LegendColumnWidth",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, legend_column_width),
            },
            FieldInfoData {
                name: "LegendDisplayMode",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, legend_display_mode),
            },
            FieldInfoData {
                name: "AverageFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, average_frame_count),
            },
            FieldInfoData {
                name: "DrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, draw_enable),
            },
            FieldInfoData {
                name: "DrawBarsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, draw_bars_enable),
            },
            FieldInfoData {
                name: "DrawLegendEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, draw_legend_enable),
            },
            FieldInfoData {
                name: "DrawCpuLegendEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, draw_cpu_legend_enable),
            },
            FieldInfoData {
                name: "DrawGpuLegendEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, draw_gpu_legend_enable),
            },
            FieldInfoData {
                name: "DrawSpuLegendEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, draw_spu_legend_enable),
            },
            FieldInfoData {
                name: "SnoopEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, snoop_enable),
            },
            FieldInfoData {
                name: "SnoopOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, snoop_only),
            },
            FieldInfoData {
                name: "SnoopEventName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, snoop_event_name),
            },
            FieldInfoData {
                name: "SortByTime",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, sort_by_time),
            },
            FieldInfoData {
                name: "FilterEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, filter_enable),
            },
            FieldInfoData {
                name: "FilterEventName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, filter_event_name),
            },
            FieldInfoData {
                name: "BarMinTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT64_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, bar_min_time),
            },
            FieldInfoData {
                name: "BarHeight",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, bar_height),
            },
            FieldInfoData {
                name: "BarPad",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, bar_pad),
            },
            FieldInfoData {
                name: "BarSyncProcessor",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, bar_sync_processor),
            },
            FieldInfoData {
                name: "MaxCpuLegendColumnCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, max_cpu_legend_column_count),
            },
            FieldInfoData {
                name: "MaxGpuLegendColumnCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, max_gpu_legend_column_count),
            },
            FieldInfoData {
                name: "MaxSpuLegendColumnCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, max_spu_legend_column_count),
            },
            FieldInfoData {
                name: "MaxFrameEventCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, max_frame_event_count),
            },
            FieldInfoData {
                name: "CollectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimingViewSettings, collection_enable),
            },
        ],
    }),
    array_type: Some(TIMINGVIEWSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimingViewSettings {
    fn type_info() -> &'static TypeInfo {
        TIMINGVIEWSETTINGS_TYPE_INFO
    }
}


pub const TIMINGVIEWSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimingViewSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TimingViewSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PerfJournalSettings {
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

pub const PERFJOURNALSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfJournalSettings",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfJournalSettings, enable),
            },
            FieldInfoData {
                name: "JournalExpensiveStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfJournalSettings, journal_expensive_stats),
            },
            FieldInfoData {
                name: "JournalAllSPU",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfJournalSettings, journal_all_s_p_u),
            },
            FieldInfoData {
                name: "JournalSampleInterval",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PerfJournalSettings, journal_sample_interval),
            },
            FieldInfoData {
                name: "JournalReportAverage",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfJournalSettings, journal_report_average),
            },
            FieldInfoData {
                name: "JournalFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PerfJournalSettings, journal_frame_count),
            },
            FieldInfoData {
                name: "AutoJournalEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfJournalSettings, auto_journal_enable),
            },
            FieldInfoData {
                name: "AutoJournalScreenshot",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfJournalSettings, auto_journal_screenshot),
            },
            FieldInfoData {
                name: "AutoJournalThresholdMs",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerfJournalSettings, auto_journal_threshold_ms),
            },
            FieldInfoData {
                name: "AutoJournalMinFrames",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PerfJournalSettings, auto_journal_min_frames),
            },
            FieldInfoData {
                name: "TraceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfJournalSettings, trace_enable),
            },
            FieldInfoData {
                name: "AutoJournalContinuousCapture",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfJournalSettings, auto_journal_continuous_capture),
            },
            FieldInfoData {
                name: "FloatsAlwaysHaveDecimal",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfJournalSettings, floats_always_have_decimal),
            },
        ],
    }),
    array_type: Some(PERFJOURNALSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PerfJournalSettings {
    fn type_info() -> &'static TypeInfo {
        PERFJOURNALSETTINGS_TYPE_INFO
    }
}


pub const PERFJOURNALSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfJournalSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PerfJournalSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PerfHudSettings {
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

pub const PERFHUDSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfHudSettings",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfHudSettings, enable),
            },
            FieldInfoData {
                name: "DrawCpuEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfHudSettings, draw_cpu_enable),
            },
            FieldInfoData {
                name: "DrawGpuEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfHudSettings, draw_gpu_enable),
            },
            FieldInfoData {
                name: "DrawSpuEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfHudSettings, draw_spu_enable),
            },
            FieldInfoData {
                name: "SimpleSummaryMode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfHudSettings, simple_summary_mode),
            },
            FieldInfoData {
                name: "TimerCategorySetName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(PerfHudSettings, timer_category_set_name),
            },
            FieldInfoData {
                name: "TimerCategorySetsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfHudSettings, timer_category_sets_enable),
            },
            FieldInfoData {
                name: "DrawUngroupedTimings",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfHudSettings, draw_ungrouped_timings),
            },
            FieldInfoData {
                name: "DrawConfigFile",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfHudSettings, draw_config_file),
            },
            FieldInfoData {
                name: "HudRightMargin",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PerfHudSettings, hud_right_margin),
            },
            FieldInfoData {
                name: "HudTopMargin",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PerfHudSettings, hud_top_margin),
            },
            FieldInfoData {
                name: "HudAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(PerfHudSettings, hud_alpha),
            },
            FieldInfoData {
                name: "HudCompact",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfHudSettings, hud_compact),
            },
        ],
    }),
    array_type: Some(PERFHUDSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PerfHudSettings {
    fn type_info() -> &'static TypeInfo {
        PERFHUDSETTINGS_TYPE_INFO
    }
}


pub const PERFHUDSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfHudSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("PerfHudSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreLogUpdateSettingsMessage {
}

pub const CORELOGUPDATESETTINGSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreLogUpdateSettingsMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreLogUpdateSettingsMessage {
    fn type_info() -> &'static TypeInfo {
        CORELOGUPDATESETTINGSMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SplitScreenSetPrimaryLocalPlayerIdMessage {
}

pub const SPLITSCREENSETPRIMARYLOCALPLAYERIDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SplitScreenSetPrimaryLocalPlayerIdMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for SplitScreenSetPrimaryLocalPlayerIdMessage {
    fn type_info() -> &'static TypeInfo {
        SPLITSCREENSETPRIMARYLOCALPLAYERIDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StreamInstallRequestResumeMessage {
}

pub const STREAMINSTALLREQUESTRESUMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallRequestResumeMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for StreamInstallRequestResumeMessage {
    fn type_info() -> &'static TypeInfo {
        STREAMINSTALLREQUESTRESUMEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StreamInstallRequestSuspendMessage {
}

pub const STREAMINSTALLREQUESTSUSPENDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallRequestSuspendMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for StreamInstallRequestSuspendMessage {
    fn type_info() -> &'static TypeInfo {
        STREAMINSTALLREQUESTSUSPENDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StreamInstallGroupInstallationProgressMessage {
}

pub const STREAMINSTALLGROUPINSTALLATIONPROGRESSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallGroupInstallationProgressMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for StreamInstallGroupInstallationProgressMessage {
    fn type_info() -> &'static TypeInfo {
        STREAMINSTALLGROUPINSTALLATIONPROGRESSMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StreamInstallGroupInstalledMessage {
}

pub const STREAMINSTALLGROUPINSTALLEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallGroupInstalledMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for StreamInstallGroupInstalledMessage {
    fn type_info() -> &'static TypeInfo {
        STREAMINSTALLGROUPINSTALLEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StreamInstallInstallationProgressMessage {
}

pub const STREAMINSTALLINSTALLATIONPROGRESSMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallInstallationProgressMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for StreamInstallInstallationProgressMessage {
    fn type_info() -> &'static TypeInfo {
        STREAMINSTALLINSTALLATIONPROGRESSMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StreamInstallInstallationFailedMessage {
}

pub const STREAMINSTALLINSTALLATIONFAILEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallInstallationFailedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for StreamInstallInstallationFailedMessage {
    fn type_info() -> &'static TypeInfo {
        STREAMINSTALLINSTALLATIONFAILEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StreamInstallInstallationCompletedMessage {
}

pub const STREAMINSTALLINSTALLATIONCOMPLETEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamInstallInstallationCompletedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for StreamInstallInstallationCompletedMessage {
    fn type_info() -> &'static TypeInfo {
        STREAMINSTALLINSTALLATIONCOMPLETEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreApplicationActivationMessage {
}

pub const COREAPPLICATIONACTIVATIONMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreApplicationActivationMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreApplicationActivationMessage {
    fn type_info() -> &'static TypeInfo {
        COREAPPLICATIONACTIVATIONMESSAGE_TYPE_INFO
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ApplicationActivationState {
    #[default]
    ApplicationActivationState_CodeActivated = 0,
    ApplicationActivationState_Deactivated = 1,
    ApplicationActivationState_PointerActivated = 2,
    ApplicationActivationState_Invalid = 255,
}

pub const APPLICATIONACTIVATIONSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ApplicationActivationState",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(APPLICATIONACTIVATIONSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ApplicationActivationState {
    fn type_info() -> &'static TypeInfo {
        APPLICATIONACTIVATIONSTATE_TYPE_INFO
    }
}


pub const APPLICATIONACTIVATIONSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ApplicationActivationState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ApplicationActivationState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreCleanupMessage {
}

pub const CORECLEANUPMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreCleanupMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreCleanupMessage {
    fn type_info() -> &'static TypeInfo {
        CORECLEANUPMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreSublevelStartStreamOutMessage {
}

pub const CORESUBLEVELSTARTSTREAMOUTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreSublevelStartStreamOutMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreSublevelStartStreamOutMessage {
    fn type_info() -> &'static TypeInfo {
        CORESUBLEVELSTARTSTREAMOUTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CorePanicMessage {
}

pub const COREPANICMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CorePanicMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CorePanicMessage {
    fn type_info() -> &'static TypeInfo {
        COREPANICMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreQuitMessage {
}

pub const COREQUITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreQuitMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreQuitMessage {
    fn type_info() -> &'static TypeInfo {
        COREQUITMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreUpdateClipboardMessage {
}

pub const COREUPDATECLIPBOARDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreUpdateClipboardMessage",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreUpdateClipboardMessage {
    fn type_info() -> &'static TypeInfo {
        COREUPDATECLIPBOARDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreMainThreadInitMessage {
}

pub const COREMAINTHREADINITMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreMainThreadInitMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreMainThreadInitMessage {
    fn type_info() -> &'static TypeInfo {
        COREMAINTHREADINITMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreHibernateMessage {
}

pub const COREHIBERNATEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreHibernateMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreHibernateMessage {
    fn type_info() -> &'static TypeInfo {
        COREHIBERNATEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CoreQuittingInitiatedMessage {
}

pub const COREQUITTINGINITIATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CoreQuittingInitiatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for CoreQuittingInitiatedMessage {
    fn type_info() -> &'static TypeInfo {
        COREQUITTINGINITIATEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SparseTransformArray {
    pub indices: Vec<u16>,
    pub transforms: Vec<LinearTransform>,
    pub count: u32,
}

pub const SPARSETRANSFORMARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SparseTransformArray",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Indices",
                flags: MemberInfoFlags::new(144),
                field_type: UINT16_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SparseTransformArray, indices),
            },
            FieldInfoData {
                name: "Transforms",
                flags: MemberInfoFlags::new(144),
                field_type: LINEARTRANSFORM_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SparseTransformArray, transforms),
            },
            FieldInfoData {
                name: "Count",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(SparseTransformArray, count),
            },
        ],
    }),
    array_type: Some(SPARSETRANSFORMARRAY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SparseTransformArray {
    fn type_info() -> &'static TypeInfo {
        SPARSETRANSFORMARRAY_TYPE_INFO
    }
}


pub const SPARSETRANSFORMARRAY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SparseTransformArray-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SparseTransformArray-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const SPLINECURVE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SplineCurve",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "XValues0",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SplineCurve, x_values0),
            },
            FieldInfoData {
                name: "XValues1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SplineCurve, x_values1),
            },
            FieldInfoData {
                name: "XValues2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SplineCurve, x_values2),
            },
            FieldInfoData {
                name: "YValues0",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SplineCurve, y_values0),
            },
            FieldInfoData {
                name: "YValues1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SplineCurve, y_values1),
            },
            FieldInfoData {
                name: "YValues2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SplineCurve, y_values2),
            },
            FieldInfoData {
                name: "YValues3",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SplineCurve, y_values3),
            },
            FieldInfoData {
                name: "GValues0",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SplineCurve, g_values0),
            },
            FieldInfoData {
                name: "GValues1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SplineCurve, g_values1),
            },
            FieldInfoData {
                name: "GValues2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SplineCurve, g_values2),
            },
            FieldInfoData {
                name: "GValues3",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SplineCurve, g_values3),
            },
            FieldInfoData {
                name: "GValues4",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SplineCurve, g_values4),
            },
            FieldInfoData {
                name: "GValues5",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(SplineCurve, g_values5),
            },
            FieldInfoData {
                name: "SplineType",
                flags: MemberInfoFlags::new(0),
                field_type: SPLINETYPE_TYPE_INFO,
                rust_offset: offset_of!(SplineCurve, spline_type),
            },
        ],
    }),
    array_type: Some(SPLINECURVE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SplineCurve {
    fn type_info() -> &'static TypeInfo {
        SPLINECURVE_TYPE_INFO
    }
}


pub const SPLINECURVE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SplineCurve-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SplineCurve-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SplineType {
    #[default]
    SplineType_5ControlPoints = 5,
    SplineType_9ControlPoints = 9,
    SplineType_13ControlPoints = 13,
}

pub const SPLINETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SplineType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(SPLINETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SplineType {
    fn type_info() -> &'static TypeInfo {
        SPLINETYPE_TYPE_INFO
    }
}


pub const SPLINETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SplineType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SplineType-Array"),
    array_type: None,
    alignment: 8,
};



pub const EVALUATEFLOATCURVE_FLOAT32_FLOAT32_FLOATCURVE__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EvaluateFloatCurve(Float32,Float32,FloatCurve)",
    flags: MemberInfoFlags::new(793),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub const EVALUATEAUDIOCURVE_FLOAT32_FLOAT32_AUDIOCURVE__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EvaluateAudioCurve(Float32,Float32,AudioCurve)",
    flags: MemberInfoFlags::new(793),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FloatCurveCollectionAsset {
    pub curves: Vec<FloatCurve>,
}

pub const FLOATCURVECOLLECTIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurveCollectionAsset",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Curves",
                flags: MemberInfoFlags::new(144),
                field_type: FLOATCURVE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(FloatCurveCollectionAsset, curves),
            },
        ],
    }),
    array_type: Some(FLOATCURVECOLLECTIONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatCurveCollectionAsset {
    fn type_info() -> &'static TypeInfo {
        FLOATCURVECOLLECTIONASSET_TYPE_INFO
    }
}


pub const FLOATCURVECOLLECTIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurveCollectionAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FloatCurveCollectionAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FloatCurve {
    pub points: Vec<FloatCurvePoint>,
    pub min_x: f32,
    pub max_x: f32,
}

pub const FLOATCURVE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurve",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Points",
                flags: MemberInfoFlags::new(144),
                field_type: FLOATCURVEPOINT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(FloatCurve, points),
            },
            FieldInfoData {
                name: "MinX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatCurve, min_x),
            },
            FieldInfoData {
                name: "MaxX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatCurve, max_x),
            },
        ],
    }),
    array_type: Some(FLOATCURVE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatCurve {
    fn type_info() -> &'static TypeInfo {
        FLOATCURVE_TYPE_INFO
    }
}


pub const FLOATCURVE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurve-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FloatCurve-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum FloatCurveType {
    #[default]
    FloatCurveType_Spline = 0,
    FloatCurveType_Smooth = 1,
    FloatCurveType_Linear = 2,
    FloatCurveType_NearestValue = 3,
}

pub const FLOATCURVETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurveType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(FLOATCURVETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FloatCurveType {
    fn type_info() -> &'static TypeInfo {
        FLOATCURVETYPE_TYPE_INFO
    }
}


pub const FLOATCURVETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurveType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FloatCurveType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FloatCurvePoint {
    pub x: f32,
    pub y: f32,
    pub in_tangent_offset_x: f32,
    pub in_tangent_offset_y: f32,
    pub out_tangent_offset_x: f32,
    pub out_tangent_offset_y: f32,
    pub curve_type: FloatCurveType,
}

pub const FLOATCURVEPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurvePoint",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatCurvePoint, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatCurvePoint, y),
            },
            FieldInfoData {
                name: "InTangentOffsetX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatCurvePoint, in_tangent_offset_x),
            },
            FieldInfoData {
                name: "InTangentOffsetY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatCurvePoint, in_tangent_offset_y),
            },
            FieldInfoData {
                name: "OutTangentOffsetX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatCurvePoint, out_tangent_offset_x),
            },
            FieldInfoData {
                name: "OutTangentOffsetY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FloatCurvePoint, out_tangent_offset_y),
            },
            FieldInfoData {
                name: "CurveType",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATCURVETYPE_TYPE_INFO,
                rust_offset: offset_of!(FloatCurvePoint, curve_type),
            },
        ],
    }),
    array_type: Some(FLOATCURVEPOINT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for FloatCurvePoint {
    fn type_info() -> &'static TypeInfo {
        FLOATCURVEPOINT_TYPE_INFO
    }
}


pub const FLOATCURVEPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatCurvePoint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FloatCurvePoint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EffectCurvePoint {
    pub x: f32,
    pub y: f32,
    pub k: f32,
}

pub const EFFECTCURVEPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectCurvePoint",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EffectCurvePoint, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EffectCurvePoint, y),
            },
            FieldInfoData {
                name: "K",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EffectCurvePoint, k),
            },
        ],
    }),
    array_type: Some(EFFECTCURVEPOINT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EffectCurvePoint {
    fn type_info() -> &'static TypeInfo {
        EFFECTCURVEPOINT_TYPE_INFO
    }
}


pub const EFFECTCURVEPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EffectCurvePoint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("EffectCurvePoint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AudioCurve {
    pub points: Vec<AudioCurvePoint>,
    pub curve_type: AudioCurveType,
}

pub const AUDIOCURVE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurve",
    flags: MemberInfoFlags::new(73),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Points",
                flags: MemberInfoFlags::new(144),
                field_type: AUDIOCURVEPOINT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(AudioCurve, points),
            },
            FieldInfoData {
                name: "CurveType",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOCURVETYPE_TYPE_INFO,
                rust_offset: offset_of!(AudioCurve, curve_type),
            },
        ],
    }),
    array_type: Some(AUDIOCURVE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioCurve {
    fn type_info() -> &'static TypeInfo {
        AUDIOCURVE_TYPE_INFO
    }
}


pub const AUDIOCURVE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurve-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("AudioCurve-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AudioCurveType {
    #[default]
    AudioCurveType_Spline = 0,
    AudioCurveType_Smooth = 1,
    AudioCurveType_Linear = 2,
    AudioCurveType_NearestValue = 3,
}

pub const AUDIOCURVETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurveType",
    flags: MemberInfoFlags::new(49429),
    module: "Core",
    data: TypeInfoData::Enum,
    array_type: Some(AUDIOCURVETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AudioCurveType {
    fn type_info() -> &'static TypeInfo {
        AUDIOCURVETYPE_TYPE_INFO
    }
}


pub const AUDIOCURVETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurveType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("AudioCurveType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AudioCurvePoint {
    pub x: f32,
    pub y: f32,
    pub k: f32,
}

pub const AUDIOCURVEPOINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurvePoint",
    flags: MemberInfoFlags::new(36937),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioCurvePoint, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioCurvePoint, y),
            },
            FieldInfoData {
                name: "K",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioCurvePoint, k),
            },
        ],
    }),
    array_type: Some(AUDIOCURVEPOINT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for AudioCurvePoint {
    fn type_info() -> &'static TypeInfo {
        AUDIOCURVEPOINT_TYPE_INFO
    }
}


pub const AUDIOCURVEPOINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCurvePoint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("AudioCurvePoint-Array"),
    array_type: None,
    alignment: 8,
};



pub const FBASSERT_BOOLEAN__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FbAssert(Boolean)",
    flags: MemberInfoFlags::new(793),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DebugRenderSettings {
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

pub const DEBUGRENDERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugRenderSettings",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebugRenderSettings, enable),
            },
            FieldInfoData {
                name: "DrawStatsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebugRenderSettings, draw_stats_enable),
            },
            FieldInfoData {
                name: "TextViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebugRenderSettings, text_view_distance),
            },
            FieldInfoData {
                name: "TextQueueMaxLineCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DebugRenderSettings, text_queue_max_line_count),
            },
            FieldInfoData {
                name: "TextQueueTimeVisible",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DebugRenderSettings, text_queue_time_visible),
            },
            FieldInfoData {
                name: "TextQueueLocationTop",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebugRenderSettings, text_queue_location_top),
            },
            FieldInfoData {
                name: "ProcessJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(DebugRenderSettings, process_job_count),
            },
            FieldInfoData {
                name: "DxMaxVertexCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DebugRenderSettings, dx_max_vertex_count),
            },
            FieldInfoData {
                name: "DxLine2dAntialiasingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebugRenderSettings, dx_line2d_antialiasing_enable),
            },
            FieldInfoData {
                name: "DxLine3dAntialiasingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DebugRenderSettings, dx_line3d_antialiasing_enable),
            },
        ],
    }),
    array_type: Some(DEBUGRENDERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DebugRenderSettings {
    fn type_info() -> &'static TypeInfo {
        DEBUGRENDERSETTINGS_TYPE_INFO
    }
}


pub const DEBUGRENDERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugRenderSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DebugRenderSettings-Array"),
    array_type: None,
    alignment: 8,
};



pub const GETSCREENHEIGHT_UINT32__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetScreenHeight(Uint32)",
    flags: MemberInfoFlags::new(793),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub const GETSCREENWIDTH_UINT32__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetScreenWidth(Uint32)",
    flags: MemberInfoFlags::new(793),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub const DRAWTEXT2D_INT32_INT32_FLOAT32_CSTRING__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DrawText2d(Int32,Int32,Float32,CString)",
    flags: MemberInfoFlags::new(793),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RuntimeModule {
}

pub const RUNTIMEMODULE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeModule",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(RUNTIMEMODULE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RuntimeModule {
    fn type_info() -> &'static TypeInfo {
        RUNTIMEMODULE_TYPE_INFO
    }
}


pub const RUNTIMEMODULE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RuntimeModule-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("RuntimeModule-Array"),
    array_type: None,
    alignment: 8,
};



pub const BOXEDVALUEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxedValueRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("BoxedValueRef-Array"),
    array_type: None,
    alignment: 8,
};



pub const BOXEDVALUEREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxedValueRef",
    flags: MemberInfoFlags::new(17229),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(BOXEDVALUEREF_ARRAY_TYPE_INFO),
    alignment: 8,
};



pub const TYPEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("TypeRef-Array"),
    array_type: None,
    alignment: 8,
};



pub const TYPEREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TypeRef",
    flags: MemberInfoFlags::new(17197),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(TYPEREF_ARRAY_TYPE_INFO),
    alignment: 8,
};



pub const DBOBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DbObject-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DbObject-Array"),
    array_type: None,
    alignment: 8,
};



pub const DBOBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DbObject",
    flags: MemberInfoFlags::new(45),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(DBOBJECT_ARRAY_TYPE_INFO),
    alignment: 8,
};



pub const FILEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FileRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("FileRef-Array"),
    array_type: None,
    alignment: 8,
};



pub const FILEREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FileRef",
    flags: MemberInfoFlags::new(16685),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(FILEREF_ARRAY_TYPE_INFO),
    alignment: 8,
};



pub const STRING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "String-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("String-Array"),
    array_type: None,
    alignment: 8,
};



pub const STRING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "String",
    flags: MemberInfoFlags::new(16589),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(STRING_ARRAY_TYPE_INFO),
    alignment: 8,
};



pub const CSTRING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CString-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("CString-Array"),
    array_type: None,
    alignment: 8,
};



pub const CSTRING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CString",
    flags: MemberInfoFlags::new(16621),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(CSTRING_ARRAY_TYPE_INFO),
    alignment: 8,
};



pub const VOID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Void-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Void-Array"),
    array_type: None,
    alignment: 8,
};



pub const VOID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Void",
    flags: MemberInfoFlags::new(13),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(VOID_ARRAY_TYPE_INFO),
    alignment: 0,
};



pub const RESOURCEREF_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceRef-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("ResourceRef-Array"),
    array_type: None,
    alignment: 8,
};



pub const RESOURCEREF_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResourceRef",
    flags: MemberInfoFlags::new(17133),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(RESOURCEREF_ARRAY_TYPE_INFO),
    alignment: 8,
};



pub const SHA1_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SHA1-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("SHA1-Array"),
    array_type: None,
    alignment: 8,
};



pub const SHA1_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SHA1",
    flags: MemberInfoFlags::new(49869),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(SHA1_ARRAY_TYPE_INFO),
    alignment: 1,
};



pub const GUID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Guid-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Guid-Array"),
    array_type: None,
    alignment: 8,
};



pub const GUID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Guid",
    flags: MemberInfoFlags::new(49837),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(GUID_ARRAY_TYPE_INFO),
    alignment: 4,
};



pub const FLOAT64_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Float64-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Float64-Array"),
    array_type: None,
    alignment: 8,
};



pub const FLOAT64_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Float64",
    flags: MemberInfoFlags::new(49805),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(FLOAT64_ARRAY_TYPE_INFO),
    alignment: 8,
};



pub const FLOAT32_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Float32-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Float32-Array"),
    array_type: None,
    alignment: 8,
};



pub const FLOAT32_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Float32",
    flags: MemberInfoFlags::new(49773),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(FLOAT32_ARRAY_TYPE_INFO),
    alignment: 4,
};



pub const INT64_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int64-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Int64-Array"),
    array_type: None,
    alignment: 8,
};



pub const INT64_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int64",
    flags: MemberInfoFlags::new(49709),
    module: "Core",
    data: TypeInfoData::Int64,
    array_type: Some(INT64_ARRAY_TYPE_INFO),
    alignment: 8,
};



pub const UINT64_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Uint64-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Uint64-Array"),
    array_type: None,
    alignment: 8,
};



pub const UINT64_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Uint64",
    flags: MemberInfoFlags::new(49741),
    module: "Core",
    data: TypeInfoData::Uint64,
    array_type: Some(UINT64_ARRAY_TYPE_INFO),
    alignment: 8,
};



pub const INT32_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int32-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Int32-Array"),
    array_type: None,
    alignment: 8,
};



pub const INT32_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int32",
    flags: MemberInfoFlags::new(49645),
    module: "Core",
    data: TypeInfoData::Int32,
    array_type: Some(INT32_ARRAY_TYPE_INFO),
    alignment: 4,
};



pub const UINT32_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Uint32-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Uint32-Array"),
    array_type: None,
    alignment: 8,
};



pub const UINT32_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Uint32",
    flags: MemberInfoFlags::new(49677),
    module: "Core",
    data: TypeInfoData::Uint32,
    array_type: Some(UINT32_ARRAY_TYPE_INFO),
    alignment: 4,
};



pub const INT16_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int16-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Int16-Array"),
    array_type: None,
    alignment: 8,
};



pub const INT16_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int16",
    flags: MemberInfoFlags::new(49581),
    module: "Core",
    data: TypeInfoData::Int16,
    array_type: Some(INT16_ARRAY_TYPE_INFO),
    alignment: 2,
};



pub const UINT16_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Uint16-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Uint16-Array"),
    array_type: None,
    alignment: 8,
};



pub const UINT16_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Uint16",
    flags: MemberInfoFlags::new(49613),
    module: "Core",
    data: TypeInfoData::Uint16,
    array_type: Some(UINT16_ARRAY_TYPE_INFO),
    alignment: 2,
};



pub const INT8_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int8-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Int8-Array"),
    array_type: None,
    alignment: 8,
};



pub const INT8_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Int8",
    flags: MemberInfoFlags::new(49517),
    module: "Core",
    data: TypeInfoData::Int8,
    array_type: Some(INT8_ARRAY_TYPE_INFO),
    alignment: 1,
};



pub const UINT8_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Uint8-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Uint8-Array"),
    array_type: None,
    alignment: 8,
};



pub const UINT8_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Uint8",
    flags: MemberInfoFlags::new(49549),
    module: "Core",
    data: TypeInfoData::Uint8,
    array_type: Some(UINT8_ARRAY_TYPE_INFO),
    alignment: 1,
};



pub const BOOLEAN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Boolean-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Boolean-Array"),
    array_type: None,
    alignment: 8,
};



pub const BOOLEAN_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Boolean",
    flags: MemberInfoFlags::new(49485),
    module: "Core",
    data: TypeInfoData::Unknown,
    array_type: Some(BOOLEAN_ARRAY_TYPE_INFO),
    alignment: 1,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AxisAlignedBox {
    pub min: Vec3,
    pub max: Vec3,
}

pub const AXISALIGNEDBOX_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AxisAlignedBox",
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "min",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AxisAlignedBox, min),
            },
            FieldInfoData {
                name: "max",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(AxisAlignedBox, max),
            },
        ],
    }),
    array_type: Some(AXISALIGNEDBOX_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for AxisAlignedBox {
    fn type_info() -> &'static TypeInfo {
        AXISALIGNEDBOX_TYPE_INFO
    }
}


pub const AXISALIGNEDBOX_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AxisAlignedBox-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("AxisAlignedBox-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
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

pub const MAT4_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Mat4",
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "m11",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Mat4, m11),
            },
            FieldInfoData {
                name: "m12",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Mat4, m12),
            },
            FieldInfoData {
                name: "m13",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Mat4, m13),
            },
            FieldInfoData {
                name: "m14",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Mat4, m14),
            },
            FieldInfoData {
                name: "m21",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Mat4, m21),
            },
            FieldInfoData {
                name: "m22",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Mat4, m22),
            },
            FieldInfoData {
                name: "m23",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Mat4, m23),
            },
            FieldInfoData {
                name: "m24",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Mat4, m24),
            },
            FieldInfoData {
                name: "m31",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Mat4, m31),
            },
            FieldInfoData {
                name: "m32",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Mat4, m32),
            },
            FieldInfoData {
                name: "m33",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Mat4, m33),
            },
            FieldInfoData {
                name: "m34",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Mat4, m34),
            },
            FieldInfoData {
                name: "m41",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Mat4, m41),
            },
            FieldInfoData {
                name: "m42",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Mat4, m42),
            },
            FieldInfoData {
                name: "m43",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Mat4, m43),
            },
            FieldInfoData {
                name: "m44",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Mat4, m44),
            },
        ],
    }),
    array_type: Some(MAT4_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Mat4 {
    fn type_info() -> &'static TypeInfo {
        MAT4_TYPE_INFO
    }
}


pub const MAT4_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Mat4-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Mat4-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LinearTransform {
    pub right: Vec3,
    pub up: Vec3,
    pub forward: Vec3,
    pub trans: Vec3,
}

pub const LINEARTRANSFORM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearTransform",
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "right",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LinearTransform, right),
            },
            FieldInfoData {
                name: "up",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LinearTransform, up),
            },
            FieldInfoData {
                name: "forward",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LinearTransform, forward),
            },
            FieldInfoData {
                name: "trans",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(LinearTransform, trans),
            },
        ],
    }),
    array_type: Some(LINEARTRANSFORM_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LinearTransform {
    fn type_info() -> &'static TypeInfo {
        LINEARTRANSFORM_TYPE_INFO
    }
}


pub const LINEARTRANSFORM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinearTransform-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("LinearTransform-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Plane {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub const PLANE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Plane",
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Plane, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Plane, y),
            },
            FieldInfoData {
                name: "z",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Plane, z),
            },
            FieldInfoData {
                name: "w",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Plane, w),
            },
        ],
    }),
    array_type: Some(PLANE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Plane {
    fn type_info() -> &'static TypeInfo {
        PLANE_TYPE_INFO
    }
}


pub const PLANE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Plane-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Plane-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub const QUAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Quat",
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Quat, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Quat, y),
            },
            FieldInfoData {
                name: "z",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Quat, z),
            },
            FieldInfoData {
                name: "w",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Quat, w),
            },
        ],
    }),
    array_type: Some(QUAT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Quat {
    fn type_info() -> &'static TypeInfo {
        QUAT_TYPE_INFO
    }
}


pub const QUAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Quat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Quat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub const VEC4_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4",
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Vec4, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Vec4, y),
            },
            FieldInfoData {
                name: "z",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Vec4, z),
            },
            FieldInfoData {
                name: "w",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Vec4, w),
            },
        ],
    }),
    array_type: Some(VEC4_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Vec4 {
    fn type_info() -> &'static TypeInfo {
        VEC4_TYPE_INFO
    }
}


pub const VEC4_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Vec4-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub const VEC3_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3",
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Vec3, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Vec3, y),
            },
            FieldInfoData {
                name: "z",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Vec3, z),
            },
        ],
    }),
    array_type: Some(VEC3_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Vec3 {
    fn type_info() -> &'static TypeInfo {
        VEC3_TYPE_INFO
    }
}


pub const VEC3_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Vec3-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub const VEC2_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2",
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Vec2, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Vec2, y),
            },
        ],
    }),
    array_type: Some(VEC2_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for Vec2 {
    fn type_info() -> &'static TypeInfo {
        VEC2_TYPE_INFO
    }
}


pub const VEC2_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Vec2-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub const VEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec",
    flags: MemberInfoFlags::new(53321),
    module: "Core",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "x",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Vec, x),
            },
            FieldInfoData {
                name: "y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Vec, y),
            },
            FieldInfoData {
                name: "z",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Vec, z),
            },
            FieldInfoData {
                name: "w",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Vec, w),
            },
        ],
    }),
    array_type: Some(VEC_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Vec {
    fn type_info() -> &'static TypeInfo {
        VEC_TYPE_INFO
    }
}


pub const VEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("Vec-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventDispatcher {
}

pub const EVENTDISPATCHER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventDispatcher",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(EVENTDISPATCHER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EventDispatcher {
    fn type_info() -> &'static TypeInfo {
        EVENTDISPATCHER_TYPE_INFO
    }
}


pub const EVENTDISPATCHER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventDispatcher-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("EventDispatcher-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DataContainer {
}

pub const DATACONTAINER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainer",
    flags: MemberInfoFlags::new(101),
    module: "Core",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(DATACONTAINER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DataContainer {
    fn type_info() -> &'static TypeInfo {
        DATACONTAINER_TYPE_INFO
    }
}


pub const DATACONTAINER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DataContainer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Core",
    data: TypeInfoData::Array("DataContainer-Array"),
    array_type: None,
    alignment: 8,
};


