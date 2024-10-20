use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_render_types(registry: &mut TypeRegistry) {
    registry.register_type(BASETEXTURE_TYPE_INFO);
    registry.register_type(BASETEXTURE_ARRAY_TYPE_INFO);
    registry.register_type(BASERENDERBUFFER_TYPE_INFO);
    registry.register_type(BASERENDERBUFFER_ARRAY_TYPE_INFO);
    registry.register_type(ASSETREFTEXTURE_TYPE_INFO);
    registry.register_type(ASSETREFTEXTURE_ARRAY_TYPE_INFO);
    registry.register_type(DX11TEXTURE_TYPE_INFO);
    registry.register_type(DX11TEXTURE_ARRAY_TYPE_INFO);
    registry.register_type(DX11SHADERPROGRAMDATABASE_TYPE_INFO);
    registry.register_type(DX11SHADERPROGRAMDATABASE_ARRAY_TYPE_INFO);
    registry.register_type(DX11RENDERTARGETVIEW_TYPE_INFO);
    registry.register_type(DX11RENDERTARGETVIEW_ARRAY_TYPE_INFO);
    registry.register_type(DX11RENDERBUFFER_TYPE_INFO);
    registry.register_type(DX11RENDERBUFFER_ARRAY_TYPE_INFO);
    registry.register_type(BASESHADERPROGRAMDATABASE_TYPE_INFO);
    registry.register_type(BASESHADERPROGRAMDATABASE_ARRAY_TYPE_INFO);
    registry.register_type(NULLRAYTRACESYSTEM_TYPE_INFO);
    registry.register_type(NULLRAYTRACESYSTEM_ARRAY_TYPE_INFO);
    registry.register_type(NULLRAYTRACESCENEBUILDER_TYPE_INFO);
    registry.register_type(NULLRAYTRACESCENEBUILDER_ARRAY_TYPE_INFO);
    registry.register_type(DX12PCRAYTRACESYSTEM_TYPE_INFO);
    registry.register_type(DX12PCRAYTRACESYSTEM_ARRAY_TYPE_INFO);
    registry.register_type(DX12RAYTRACESCENEBUILDER_TYPE_INFO);
    registry.register_type(DX12RAYTRACESCENEBUILDER_ARRAY_TYPE_INFO);
    registry.register_type(DX12RENDERTARGETVIEW_TYPE_INFO);
    registry.register_type(DX12RENDERTARGETVIEW_ARRAY_TYPE_INFO);
    registry.register_type(DX12RENDERSTATEOBJECT_TYPE_INFO);
    registry.register_type(DX12RENDERSTATEOBJECT_ARRAY_TYPE_INFO);
    registry.register_type(DX12NVIDIACOMPUTEPSODESCTYPE_TYPE_INFO);
    registry.register_type(DX12NVIDIACOMPUTEPSODESCTYPE_ARRAY_TYPE_INFO);
    registry.register_type(DX12RENDERBUFFER_TYPE_INFO);
    registry.register_type(DX12RENDERBUFFER_ARRAY_TYPE_INFO);
    registry.register_type(MESHSET_TYPE_INFO);
    registry.register_type(MESHSET_ARRAY_TYPE_INFO);
    registry.register_type(RENDERTESTSETTINGS_TYPE_INFO);
    registry.register_type(RENDERTESTSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(PERFOVERLAYSETTINGS_TYPE_INFO);
    registry.register_type(PERFOVERLAYSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(TEXTURECOMPRESSSETTINGS_TYPE_INFO);
    registry.register_type(TEXTURECOMPRESSSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(TEXTURECOMPRESSQUALITYMODE_TYPE_INFO);
    registry.register_type(TEXTURECOMPRESSQUALITYMODE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERCONSTANTSYSTEMBUFFER_TYPE_INFO);
    registry.register_type(SHADERCONSTANTSYSTEMBUFFER_ARRAY_TYPE_INFO);
    registry.register_type(SHADERCONSTANTSYSTEMTEXTURE_TYPE_INFO);
    registry.register_type(SHADERCONSTANTSYSTEMTEXTURE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERTIMETYPE_TYPE_INFO);
    registry.register_type(SHADERTIMETYPE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERINTERPOLATIONTYPE_TYPE_INFO);
    registry.register_type(SHADERINTERPOLATIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERDEPTHBIASGROUP_TYPE_INFO);
    registry.register_type(SHADERDEPTHBIASGROUP_ARRAY_TYPE_INFO);
    registry.register_type(SHADERTEXTURECOORDTYPE_TYPE_INFO);
    registry.register_type(SHADERTEXTURECOORDTYPE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERVALUEFORMAT_TYPE_INFO);
    registry.register_type(SHADERVALUEFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(SHADERSAMPLERFORMAT_TYPE_INFO);
    registry.register_type(SHADERSAMPLERFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(SHADERVALUETYPE_TYPE_INFO);
    registry.register_type(SHADERVALUETYPE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERGEOMETRYSPACE_TYPE_INFO);
    registry.register_type(SHADERGEOMETRYSPACE_ARRAY_TYPE_INFO);
    registry.register_type(SUBSURFACEPROFILETYPE_TYPE_INFO);
    registry.register_type(SUBSURFACEPROFILETYPE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERSHADOWMAPMETHOD_TYPE_INFO);
    registry.register_type(SHADERSHADOWMAPMETHOD_ARRAY_TYPE_INFO);
    registry.register_type(SHADEROBJECTLIGHTING_TYPE_INFO);
    registry.register_type(SHADEROBJECTLIGHTING_ARRAY_TYPE_INFO);
    registry.register_type(SHADERDEBUGRENDERMODE_TYPE_INFO);
    registry.register_type(SHADERDEBUGRENDERMODE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERRENDERMODE_TYPE_INFO);
    registry.register_type(SHADERRENDERMODE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERSKINNINGMETHOD_TYPE_INFO);
    registry.register_type(SHADERSKINNINGMETHOD_ARRAY_TYPE_INFO);
    registry.register_type(SHADERINSTANCINGMETHOD_TYPE_INFO);
    registry.register_type(SHADERINSTANCINGMETHOD_ARRAY_TYPE_INFO);
    registry.register_type(SHADERTEXTURESTREAMINGSUPPORT_TYPE_INFO);
    registry.register_type(SHADERTEXTURESTREAMINGSUPPORT_ARRAY_TYPE_INFO);
    registry.register_type(SHADERBLENDMODE_TYPE_INFO);
    registry.register_type(SHADERBLENDMODE_ARRAY_TYPE_INFO);
    registry.register_type(SURFACESHADERTYPE_TYPE_INFO);
    registry.register_type(SURFACESHADERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(PBRDEFAULTMATERIALMODEL_TYPE_INFO);
    registry.register_type(PBRDEFAULTMATERIALMODEL_ARRAY_TYPE_INFO);
    registry.register_type(MATERIALMODEL_TYPE_INFO);
    registry.register_type(MATERIALMODEL_ARRAY_TYPE_INFO);
    registry.register_type(FORWARDLIGHTINGSUPPORTDATA_TYPE_INFO);
    registry.register_type(FORWARDLIGHTINGSUPPORTDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALLIGHTSHADOWQUALITY_TYPE_INFO);
    registry.register_type(LOCALLIGHTSHADOWQUALITY_ARRAY_TYPE_INFO);
    registry.register_type(DOUBLESIDEDLIGHTINGMODE_TYPE_INFO);
    registry.register_type(DOUBLESIDEDLIGHTINGMODE_ARRAY_TYPE_INFO);
    registry.register_type(CUSTOMFORWARDSHADERVALUEPARAMETER_TYPE_INFO);
    registry.register_type(CUSTOMFORWARDSHADERVALUEPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(CUSTOMFORWARDSHADERSAMPLERPARAMETER_TYPE_INFO);
    registry.register_type(CUSTOMFORWARDSHADERSAMPLERPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(CUSTOMFORWARDMATERIALMODEL_TYPE_INFO);
    registry.register_type(CUSTOMFORWARDMATERIALMODEL_ARRAY_TYPE_INFO);
    registry.register_type(SHADERSAMPLERPARAMETERBASE_TYPE_INFO);
    registry.register_type(SHADERSAMPLERPARAMETERBASE_ARRAY_TYPE_INFO);
    registry.register_type(UNITTYPE_TYPE_INFO);
    registry.register_type(UNITTYPE_ARRAY_TYPE_INFO);
    registry.register_type(DISTORTIONSPACETYPE_TYPE_INFO);
    registry.register_type(DISTORTIONSPACETYPE_ARRAY_TYPE_INFO);
    registry.register_type(TRANSPARENTFOGMODE_TYPE_INFO);
    registry.register_type(TRANSPARENTFOGMODE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERBRANCHMETHOD_TYPE_INFO);
    registry.register_type(SHADERBRANCHMETHOD_ARRAY_TYPE_INFO);
    registry.register_type(SHADERCOMPARISONOPERATOR_TYPE_INFO);
    registry.register_type(SHADERCOMPARISONOPERATOR_ARRAY_TYPE_INFO);
    registry.register_type(BLENDSHADERMODE_TYPE_INFO);
    registry.register_type(BLENDSHADERMODE_ARRAY_TYPE_INFO);
    registry.register_type(CURVESHADERTYPE_TYPE_INFO);
    registry.register_type(CURVESHADERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(EYEVECTORSPACE_TYPE_INFO);
    registry.register_type(EYEVECTORSPACE_ARRAY_TYPE_INFO);
    registry.register_type(PIXELNORMALSPACE_TYPE_INFO);
    registry.register_type(PIXELNORMALSPACE_ARRAY_TYPE_INFO);
    registry.register_type(VERTEXNORMALSPACE_TYPE_INFO);
    registry.register_type(VERTEXNORMALSPACE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPOSITIONSPACE_TYPE_INFO);
    registry.register_type(SHADERPOSITIONSPACE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERVALUEPARAMETERTYPE_TYPE_INFO);
    registry.register_type(SHADERVALUEPARAMETERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPORTTYPE_TYPE_INFO);
    registry.register_type(SHADERPORTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERGRAPH_TYPE_INFO);
    registry.register_type(SHADERGRAPH_ARRAY_TYPE_INFO);
    registry.register_type(SHADERBUFFERDEFINITION_TYPE_INFO);
    registry.register_type(SHADERBUFFERDEFINITION_ARRAY_TYPE_INFO);
    registry.register_type(SURFACESHADERPRESET_TYPE_INFO);
    registry.register_type(SURFACESHADERPRESET_ARRAY_TYPE_INFO);
    registry.register_type(SURFACESHADERINSTANCEDATA_TYPE_INFO);
    registry.register_type(SURFACESHADERINSTANCEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SHADERTESSELLATIONTYPE_TYPE_INFO);
    registry.register_type(SHADERTESSELLATIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(TESSELLATIONSHADERFRAGMENTASSET_TYPE_INFO);
    registry.register_type(TESSELLATIONSHADERFRAGMENTASSET_ARRAY_TYPE_INFO);
    registry.register_type(TESSELLATIONSHADERFRAGMENTDEFINITION_TYPE_INFO);
    registry.register_type(TESSELLATIONSHADERFRAGMENTDEFINITION_ARRAY_TYPE_INFO);
    registry.register_type(VERTEXSHADERFRAGMENTASSET_TYPE_INFO);
    registry.register_type(VERTEXSHADERFRAGMENTASSET_ARRAY_TYPE_INFO);
    registry.register_type(DISPATCHBLENDMODE_TYPE_INFO);
    registry.register_type(DISPATCHBLENDMODE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERCONSTANTFUNCTION_TYPE_INFO);
    registry.register_type(SHADERCONSTANTFUNCTION_ARRAY_TYPE_INFO);
    registry.register_type(RVMSYSTEMSETTINGS_TYPE_INFO);
    registry.register_type(RVMSYSTEMSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(RVMLEGACYLIGHTMAPINSTANCE_TYPE_INFO);
    registry.register_type(RVMLEGACYLIGHTPROBES_TYPE_INFO);
    registry.register_type(RVMLEGACYPERMUTATIONDEBUGINFO_TYPE_INFO);
    registry.register_type(RVMLEGACYFORWARDLIGHTSTATE_TYPE_INFO);
    registry.register_type(FRUSTUMSOA_TYPE_INFO);
    registry.register_type(LODVIEWSTATE_TYPE_INFO);
    registry.register_type(TESSELLATIONVIEWSTATE_TYPE_INFO);
    registry.register_type(STENCILSTATE_TYPE_INFO);
    registry.register_type(REFLECTIONSTATE_TYPE_INFO);
    registry.register_type(FOGSTATE_TYPE_INFO);
    registry.register_type(PROJECTIONSTATE_TYPE_INFO);
    registry.register_type(VIEWSTATE_TYPE_INFO);
    registry.register_type(OUTDOORLIGHTSTATE_TYPE_INFO);
    registry.register_type(RVMLEGACYDATABASE_TYPE_INFO);
    registry.register_type(RVMLEGACYDATABASE_ARRAY_TYPE_INFO);
    registry.register_type(RVMDATABASELOADER_TYPE_INFO);
    registry.register_type(RVMDATABASELOADER_ARRAY_TYPE_INFO);
    registry.register_type(IRAYTRACESYSTEM_TYPE_INFO);
    registry.register_type(IRAYTRACESYSTEM_ARRAY_TYPE_INFO);
    registry.register_type(IRAYTRACESCENEBUILDER_TYPE_INFO);
    registry.register_type(IRAYTRACESCENEBUILDER_ARRAY_TYPE_INFO);
    registry.register_type(RVMSTATSSETTINGS_TYPE_INFO);
    registry.register_type(RVMSTATSSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(RVMVIEWPORTRECT_TYPE_INFO);
    registry.register_type(RVMVIEWPORTRECT_ARRAY_TYPE_INFO);
    registry.register_type(RVMINDEXBUFFERFORMAT_TYPE_INFO);
    registry.register_type(RVMINDEXBUFFERFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(RVMLEVELOFDETAIL_TYPE_INFO);
    registry.register_type(RVMLEVELOFDETAIL_ARRAY_TYPE_INFO);
    registry.register_type(RVMLEGACYOUTDOORLIGHTSTATUS_TYPE_INFO);
    registry.register_type(RVMLEGACYOUTDOORLIGHTSTATUS_ARRAY_TYPE_INFO);
    registry.register_type(RVMDEBUGDATABASECOLLECTION_TYPE_INFO);
    registry.register_type(RVMDEBUGDATABASECOLLECTION_ARRAY_TYPE_INFO);
    registry.register_type(RVMDEBUGDATABASEASSET_TYPE_INFO);
    registry.register_type(RVMDEBUGDATABASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(RVMDATABASEASSET_TYPE_INFO);
    registry.register_type(RVMDATABASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(RVMSLOTHANDLE_TYPE_INFO);
    registry.register_type(RVMSLOTHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(RTRVMRAYTRACESTATEOBJECT_TYPE_INFO);
    registry.register_type(RTRVMRAYTRACESTATEOBJECT_ARRAY_TYPE_INFO);
    registry.register_type(RTRVMRAYTRACESCENE_TYPE_INFO);
    registry.register_type(RTRVMRAYTRACESCENE_ARRAY_TYPE_INFO);
    registry.register_type(NVSHADOWLIBMAP_TYPE_INFO);
    registry.register_type(NVSHADOWLIBMAP_ARRAY_TYPE_INFO);
    registry.register_type(NVSHADOWMAPRENDERTYPE_TYPE_INFO);
    registry.register_type(NVSHADOWMAPRENDERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(NVSHADOWLIBDRAWDATA_TYPE_INFO);
    registry.register_type(NVSHADOWLIBDRAWDATA_ARRAY_TYPE_INFO);
    registry.register_type(NVSHADOWLIBPSOPARAMS_TYPE_INFO);
    registry.register_type(NVSHADOWLIBPSOPARAMS_ARRAY_TYPE_INFO);
    registry.register_type(NVSHADOWLIBCONTEXT_TYPE_INFO);
    registry.register_type(NVSHADOWLIBCONTEXT_ARRAY_TYPE_INFO);
    registry.register_type(RAYTRACESETTINGS_TYPE_INFO);
    registry.register_type(RAYTRACESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(GLOBALPOSTPROCESSSETTINGS_TYPE_INFO);
    registry.register_type(GLOBALPOSTPROCESSSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(BLURMETHOD_TYPE_INFO);
    registry.register_type(BLURMETHOD_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICAOMETHOD_TYPE_INFO);
    registry.register_type(DYNAMICAOMETHOD_ARRAY_TYPE_INFO);
    registry.register_type(POSTPROCESSDEBUGMODE_TYPE_INFO);
    registry.register_type(POSTPROCESSDEBUGMODE_ARRAY_TYPE_INFO);
    registry.register_type(FFTBLOOMSETTINGS_TYPE_INFO);
    registry.register_type(FFTBLOOMSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(SKINNEDPROCEDURALANIMATIONDATA_TYPE_INFO);
    registry.register_type(SKINNEDPROCEDURALANIMATIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(SKINNEDPROCEDURALANIMATIONBONE_TYPE_INFO);
    registry.register_type(SKINNEDPROCEDURALANIMATIONBONE_ARRAY_TYPE_INFO);
    registry.register_type(SKINNEDPROCEDURALANIMATIONROOTPOSE_TYPE_INFO);
    registry.register_type(SKINNEDPROCEDURALANIMATIONROOTPOSE_ARRAY_TYPE_INFO);
    registry.register_type(SKINNEDPROCEDURALANIMATIONEXPRESSION_TYPE_INFO);
    registry.register_type(SKINNEDPROCEDURALANIMATIONEXPRESSION_ARRAY_TYPE_INFO);
    registry.register_type(SKINNEDPROCEDURALANIMATIONEXPRESSIONGRAPH_TYPE_INFO);
    registry.register_type(SKINNEDPROCEDURALANIMATIONEXPRESSIONGRAPH_ARRAY_TYPE_INFO);
    registry.register_type(SPAEXPRESSIONEDITORBONE_TYPE_INFO);
    registry.register_type(SPAEXPRESSIONEDITORBONE_ARRAY_TYPE_INFO);
    registry.register_type(SPAEXPRESSIONRUNTIMEPARAMETER_TYPE_INFO);
    registry.register_type(SPAEXPRESSIONRUNTIMEPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(SPAEXPRESSIONEDITORPARAMETER_TYPE_INFO);
    registry.register_type(SPAEXPRESSIONEDITORPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(SPAEXPRESSIONPARAMETERTYPE_TYPE_INFO);
    registry.register_type(SPAEXPRESSIONPARAMETERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(MESHSUBSETCATEGORYFLAGS_TYPE_INFO);
    registry.register_type(MESHSUBSETCATEGORYFLAGS_ARRAY_TYPE_INFO);
    registry.register_type(MESHSUBSETCATEGORY_TYPE_INFO);
    registry.register_type(MESHSUBSETCATEGORY_ARRAY_TYPE_INFO);
    registry.register_type(MESHLIMITS_TYPE_INFO);
    registry.register_type(MESHLIMITS_ARRAY_TYPE_INFO);
    registry.register_type(MESHSTREAMINGSETTINGS_TYPE_INFO);
    registry.register_type(MESHSTREAMINGSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(MESHSETTINGS_TYPE_INFO);
    registry.register_type(MESHSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(SKINNINGMESHCOMPUTEOUTPUT_TYPE_INFO);
    registry.register_type(SKINNINGMESHCOMPUTEOUTPUT_ARRAY_TYPE_INFO);
    registry.register_type(SKINNINGMESHCOMPUTEINPUT_TYPE_INFO);
    registry.register_type(SKINNINGMESHCOMPUTEINPUT_ARRAY_TYPE_INFO);
    registry.register_type(VERTEXNORMALMESHCOMPUTEOUTPUT_TYPE_INFO);
    registry.register_type(VERTEXNORMALMESHCOMPUTEOUTPUT_ARRAY_TYPE_INFO);
    registry.register_type(VERTEXNORMALMESHCOMPUTEINPUT_TYPE_INFO);
    registry.register_type(VERTEXNORMALMESHCOMPUTEINPUT_ARRAY_TYPE_INFO);
    registry.register_type(FACENORMALMESHCOMPUTEOUTPUT_TYPE_INFO);
    registry.register_type(FACENORMALMESHCOMPUTEOUTPUT_ARRAY_TYPE_INFO);
    registry.register_type(FACENORMALMESHCOMPUTEINPUT_TYPE_INFO);
    registry.register_type(FACENORMALMESHCOMPUTEINPUT_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICMORPHMESHCOMPUTEOUTPUT_TYPE_INFO);
    registry.register_type(DYNAMICMORPHMESHCOMPUTEOUTPUT_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICMORPHMESHCOMPUTEINPUT_TYPE_INFO);
    registry.register_type(DYNAMICMORPHMESHCOMPUTEINPUT_ARRAY_TYPE_INFO);
    registry.register_type(VERTEXELEMENTINFOSLOT_TYPE_INFO);
    registry.register_type(VERTEXELEMENTINFOSLOT_ARRAY_TYPE_INFO);
    registry.register_type(MESHCOMPUTEASSET_TYPE_INFO);
    registry.register_type(MESHCOMPUTEASSET_ARRAY_TYPE_INFO);
    registry.register_type(MESHCOMPUTERUNTIMEOUTPUTNODE_TYPE_INFO);
    registry.register_type(MESHCOMPUTERUNTIMEOUTPUTNODE_ARRAY_TYPE_INFO);
    registry.register_type(MESHCOMPUTERUNTIMEBUFFER_TYPE_INFO);
    registry.register_type(MESHCOMPUTERUNTIMEBUFFER_ARRAY_TYPE_INFO);
    registry.register_type(VERTEXELEMENTINFO_TYPE_INFO);
    registry.register_type(VERTEXELEMENTINFO_ARRAY_TYPE_INFO);
    registry.register_type(MESHCOMPUTERUNTIMEBUFFERTYPE_TYPE_INFO);
    registry.register_type(MESHCOMPUTERUNTIMEBUFFERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(MESHCOMPUTERUNTIMENODE_TYPE_INFO);
    registry.register_type(MESHCOMPUTERUNTIMENODE_ARRAY_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIAL_TYPE_INFO);
    registry.register_type(RADIOSITYMATERIAL_ARRAY_TYPE_INFO);
    registry.register_type(MESHVARIATIONDATABASE_TYPE_INFO);
    registry.register_type(MESHVARIATIONDATABASE_ARRAY_TYPE_INFO);
    registry.register_type(MESHVARIATIONDATABASEREDIRECTENTRY_TYPE_INFO);
    registry.register_type(MESHVARIATIONDATABASEREDIRECTENTRY_ARRAY_TYPE_INFO);
    registry.register_type(MESHVARIATIONDATABASEENTRY_TYPE_INFO);
    registry.register_type(MESHVARIATIONDATABASEENTRY_ARRAY_TYPE_INFO);
    registry.register_type(MESHVARIATIONDATABASEMATERIAL_TYPE_INFO);
    registry.register_type(MESHVARIATIONDATABASEMATERIAL_ARRAY_TYPE_INFO);
    registry.register_type(MESHMATERIALVARIATION_TYPE_INFO);
    registry.register_type(MESHMATERIALVARIATION_ARRAY_TYPE_INFO);
    registry.register_type(MESHMATERIAL_TYPE_INFO);
    registry.register_type(MESHMATERIAL_ARRAY_TYPE_INFO);
    registry.register_type(COMPOSITEMESHASSET_TYPE_INFO);
    registry.register_type(COMPOSITEMESHASSET_ARRAY_TYPE_INFO);
    registry.register_type(SKINNEDMESHASSET_TYPE_INFO);
    registry.register_type(SKINNEDMESHASSET_ARRAY_TYPE_INFO);
    registry.register_type(RIGIDMESHASSET_TYPE_INFO);
    registry.register_type(RIGIDMESHASSET_ARRAY_TYPE_INFO);
    registry.register_type(MESHASSET_TYPE_INFO);
    registry.register_type(MESHASSET_ARRAY_TYPE_INFO);
    registry.register_type(LODDISTANCES_TYPE_INFO);
    registry.register_type(LODDISTANCES_ARRAY_TYPE_INFO);
    registry.register_type(LODFADEDISTANCEFACTORS_TYPE_INFO);
    registry.register_type(LODFADEDISTANCEFACTORS_ARRAY_TYPE_INFO);
    registry.register_type(CUSTOMVERTEXSHADER_TYPE_INFO);
    registry.register_type(CUSTOMVERTEXSHADER_ARRAY_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONTYPECUSTOM_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONTYPECUSTOM_ARRAY_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONTYPECUSTOMWIND_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONTYPECUSTOMWIND_ARRAY_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONTYPECUSTOMBASE_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONTYPECUSTOMBASE_ARRAY_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONTYPEWIND_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONTYPEWIND_ARRAY_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONWINDMETHOD_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONWINDMETHOD_ARRAY_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONTYPEWIGGLE_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONTYPEWIGGLE_ARRAY_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONWIGGLEMETHOD_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONWIGGLEMETHOD_ARRAY_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONTYPESIMPLE_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONTYPESIMPLE_ARRAY_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONMETHOD_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONMETHOD_ARRAY_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONENABLE_TYPE_INFO);
    registry.register_type(PROCEDURALANIMATIONENABLE_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENTYPE_TYPE_INFO);
    registry.register_type(ENLIGHTENTYPE_ARRAY_TYPE_INFO);
    registry.register_type(MESHLODGROUP_TYPE_INFO);
    registry.register_type(MESHLODGROUP_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIBLTYPE_TYPE_INFO);
    registry.register_type(LOCALIBLTYPE_ARRAY_TYPE_INFO);
    registry.register_type(PBRANALYTICLIGHTSHAPE_TYPE_INFO);
    registry.register_type(PBRANALYTICLIGHTSHAPE_ARRAY_TYPE_INFO);
    registry.register_type(PBRLIGHTTYPE_TYPE_INFO);
    registry.register_type(PBRLIGHTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(DX12COMPUTEPSODESCTYPE_TYPE_INFO);
    registry.register_type(DX12COMPUTEPSODESCTYPE_ARRAY_TYPE_INFO);
    registry.register_type(DX12GRAPHICSPSODESCTYPE_TYPE_INFO);
    registry.register_type(DX12GRAPHICSPSODESCTYPE_ARRAY_TYPE_INFO);
    registry.register_type(DX12PSODESCTYPE_TYPE_INFO);
    registry.register_type(DX12PSODESCTYPE_ARRAY_TYPE_INFO);
    registry.register_type(RVMBACKENDFACTORY_TYPE_INFO);
    registry.register_type(RVMBACKENDFACTORY_ARRAY_TYPE_INFO);
    registry.register_type(RVMBACKEND_TYPE_INFO);
    registry.register_type(RVMBACKEND_ARRAY_TYPE_INFO);
    registry.register_type(RVMDATABASE_TYPE_INFO);
    registry.register_type(RVMDATABASE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERBLOCKMESHVARIATIONENTRY_TYPE_INFO);
    registry.register_type(SHADERBLOCKMESHVARIATIONENTRY_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPERSISTENTPARAMDBBLOCK_TYPE_INFO);
    registry.register_type(SHADERPERSISTENTPARAMDBBLOCK_ARRAY_TYPE_INFO);
    registry.register_type(MESHPARAMDBBLOCK_TYPE_INFO);
    registry.register_type(MESHPARAMDBBLOCK_ARRAY_TYPE_INFO);
    registry.register_type(SHADERSTATICPARAMDBBLOCK_TYPE_INFO);
    registry.register_type(SHADERSTATICPARAMDBBLOCK_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPARAMDBBLOCK_TYPE_INFO);
    registry.register_type(SHADERPARAMDBBLOCK_ARRAY_TYPE_INFO);
    registry.register_type(SHADERBLOCKENTRY_TYPE_INFO);
    registry.register_type(SHADERBLOCKENTRY_ARRAY_TYPE_INFO);
    registry.register_type(SHADERBLOCKDEPOTITEM_TYPE_INFO);
    registry.register_type(SHADERBLOCKDEPOTITEM_ARRAY_TYPE_INFO);
    registry.register_type(SHADERBLOCKDEPOTRESOURCE_TYPE_INFO);
    registry.register_type(SHADERBLOCKDEPOTRESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(ATLASTEXTURE_TYPE_INFO);
    registry.register_type(ATLASTEXTURE_ARRAY_TYPE_INFO);
    registry.register_type(WIN32SHAREDSURFACEWINDOW_TYPE_INFO);
    registry.register_type(WIN32SHAREDSURFACEWINDOW_ARRAY_TYPE_INFO);
    registry.register_type(WIN32GAMEVIEWWINDOW_TYPE_INFO);
    registry.register_type(WIN32GAMEVIEWWINDOW_ARRAY_TYPE_INFO);
    registry.register_type(WIN32RENDERWINDOW_TYPE_INFO);
    registry.register_type(WIN32RENDERWINDOW_ARRAY_TYPE_INFO);
    registry.register_type(NULLTEXTURE_TYPE_INFO);
    registry.register_type(NULLTEXTURE_ARRAY_TYPE_INFO);
    registry.register_type(DX12TEXTURE_TYPE_INFO);
    registry.register_type(DX12TEXTURE_ARRAY_TYPE_INFO);
    registry.register_type(DX12SHADERPROGRAMDATABASE_TYPE_INFO);
    registry.register_type(DX12SHADERPROGRAMDATABASE_ARRAY_TYPE_INFO);
    registry.register_type(RENDERADAPTERARCHITECTURE_TYPE_INFO);
    registry.register_type(RENDERADAPTERARCHITECTURE_ARRAY_TYPE_INFO);
    registry.register_type(RENDERADAPTERFAMILY_TYPE_INFO);
    registry.register_type(RENDERADAPTERFAMILY_ARRAY_TYPE_INFO);
    registry.register_type(RENDERADAPTERVENDOR_TYPE_INFO);
    registry.register_type(RENDERADAPTERVENDOR_ARRAY_TYPE_INFO);
    registry.register_type(RENDERFEATURELEVEL_TYPE_INFO);
    registry.register_type(RENDERFEATURELEVEL_ARRAY_TYPE_INFO);
    registry.register_type(IRENDERSTATEOBJECT_TYPE_INFO);
    registry.register_type(IRENDERSTATEOBJECT_ARRAY_TYPE_INFO);
    registry.register_type(IRENDERTARGETVIEW_TYPE_INFO);
    registry.register_type(IRENDERTARGETVIEW_ARRAY_TYPE_INFO);
    registry.register_type(IRENDERBUFFER_TYPE_INFO);
    registry.register_type(IRENDERBUFFER_ARRAY_TYPE_INFO);
    registry.register_type(IRENDERRESOURCE_TYPE_INFO);
    registry.register_type(IRENDERRESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(COMPOSITIONSETTINGS_TYPE_INFO);
    registry.register_type(COMPOSITIONSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(COMPOSITIONDISTORTIONTYPE_TYPE_INFO);
    registry.register_type(COMPOSITIONDISTORTIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(GAMERENDERSETTINGS_TYPE_INFO);
    registry.register_type(GAMERENDERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(DISPLAYMAPPINGSHOULDERTYPE_TYPE_INFO);
    registry.register_type(DISPLAYMAPPINGSHOULDERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(SCALERESAMPLEMODE_TYPE_INFO);
    registry.register_type(SCALERESAMPLEMODE_ARRAY_TYPE_INFO);
    registry.register_type(RESOLUTIONSETGENERATOR_TYPE_INFO);
    registry.register_type(RESOLUTIONSETGENERATOR_ARRAY_TYPE_INFO);
    registry.register_type(RESOLUTIONREGULATOR_TYPE_INFO);
    registry.register_type(RESOLUTIONREGULATOR_ARRAY_TYPE_INFO);
    registry.register_type(RENDERDLAASUPPORTCHANGEDMESSAGE_TYPE_INFO);
    registry.register_type(ENLIGHTENLIGHTPROBEMODE_TYPE_INFO);
    registry.register_type(ENLIGHTENLIGHTPROBEMODE_ARRAY_TYPE_INFO);
    registry.register_type(SUPPORTEDLIGHTMAPMODES_TYPE_INFO);
    registry.register_type(SUPPORTEDLIGHTMAPMODES_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENOUTPUTFORMAT_TYPE_INFO);
    registry.register_type(ENLIGHTENOUTPUTFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENSHADERDATABASEASSET_TYPE_INFO);
    registry.register_type(ENLIGHTENSHADERDATABASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(STATICENLIGHTENDATA_TYPE_INFO);
    registry.register_type(STATICENLIGHTENDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENGLOBALCONFIGASSET_TYPE_INFO);
    registry.register_type(ENLIGHTENGLOBALCONFIGASSET_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENDATAASSET_TYPE_INFO);
    registry.register_type(ENLIGHTENDATAASSET_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENTRANSPARENCYMODE_TYPE_INFO);
    registry.register_type(ENLIGHTENTRANSPARENCYMODE_ARRAY_TYPE_INFO);
    registry.register_type(GIBAKEMODE_TYPE_INFO);
    registry.register_type(GIBAKEMODE_ARRAY_TYPE_INFO);
    registry.register_type(VERTEXELEMENTCLASSIFICATION_TYPE_INFO);
    registry.register_type(VERTEXELEMENTCLASSIFICATION_ARRAY_TYPE_INFO);
    registry.register_type(VERTEXELEMENTUSAGE_TYPE_INFO);
    registry.register_type(VERTEXELEMENTUSAGE_ARRAY_TYPE_INFO);
    registry.register_type(VERTEXELEMENTFORMAT_TYPE_INFO);
    registry.register_type(VERTEXELEMENTFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(TEXTUREFILTER_TYPE_INFO);
    registry.register_type(TEXTUREFILTER_ARRAY_TYPE_INFO);
    registry.register_type(TEXTURETYPE_TYPE_INFO);
    registry.register_type(TEXTURETYPE_ARRAY_TYPE_INFO);
    registry.register_type(TEXTUREATLASASSET_TYPE_INFO);
    registry.register_type(TEXTUREATLASASSET_ARRAY_TYPE_INFO);
    registry.register_type(SHADERTEXTUREDECOMPRESSION_TYPE_INFO);
    registry.register_type(SHADERTEXTUREDECOMPRESSION_ARRAY_TYPE_INFO);
    registry.register_type(RENDERTEXTUREASSET_TYPE_INFO);
    registry.register_type(RENDERTEXTUREASSET_ARRAY_TYPE_INFO);
    registry.register_type(RENDERTEXTUREOUTPUTTYPE_TYPE_INFO);
    registry.register_type(RENDERTEXTUREOUTPUTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(GRADINGLUTASSET_TYPE_INFO);
    registry.register_type(GRADINGLUTASSET_ARRAY_TYPE_INFO);
    registry.register_type(ATLASTEXTUREASSET_TYPE_INFO);
    registry.register_type(ATLASTEXTUREASSET_ARRAY_TYPE_INFO);
    registry.register_type(TEXTUREARRAYASSET_TYPE_INFO);
    registry.register_type(TEXTUREARRAYASSET_ARRAY_TYPE_INFO);
    registry.register_type(TEXTUREASSET_TYPE_INFO);
    registry.register_type(TEXTUREASSET_ARRAY_TYPE_INFO);
    registry.register_type(RESIZEFILTER_TYPE_INFO);
    registry.register_type(RESIZEFILTER_ARRAY_TYPE_INFO);
    registry.register_type(GENERATEMIPMAPSFILTERTYPE_TYPE_INFO);
    registry.register_type(GENERATEMIPMAPSFILTERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERPROGRAMFLAGS_TYPE_INFO);
    registry.register_type(SHADERPROGRAMFLAGS_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICTEXTUREARRAYSETTINGS_TYPE_INFO);
    registry.register_type(DYNAMICTEXTUREARRAYSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICTEXTUREATLASSETTINGS_TYPE_INFO);
    registry.register_type(DYNAMICTEXTUREATLASSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(TEXTURESETTINGS_TYPE_INFO);
    registry.register_type(TEXTURESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(TEXTURESTREAMINGSETTINGS_TYPE_INFO);
    registry.register_type(TEXTURESTREAMINGSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(DISPLAYDYNAMICRANGE_TYPE_INFO);
    registry.register_type(DISPLAYDYNAMICRANGE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERRENDERPATH_TYPE_INFO);
    registry.register_type(SHADERRENDERPATH_ARRAY_TYPE_INFO);
    registry.register_type(STATEOBJECTSHADERSTAGETYPE_TYPE_INFO);
    registry.register_type(STATEOBJECTSHADERSTAGETYPE_ARRAY_TYPE_INFO);
    registry.register_type(SHADERSTAGETYPE_TYPE_INFO);
    registry.register_type(SHADERSTAGETYPE_ARRAY_TYPE_INFO);
    registry.register_type(STENCILOPERATION_TYPE_INFO);
    registry.register_type(STENCILOPERATION_ARRAY_TYPE_INFO);
    registry.register_type(DEPTHSTENCILCOMPAREFUNC_TYPE_INFO);
    registry.register_type(DEPTHSTENCILCOMPAREFUNC_ARRAY_TYPE_INFO);
    registry.register_type(RENDERFILLMODE_TYPE_INFO);
    registry.register_type(RENDERFILLMODE_ARRAY_TYPE_INFO);
    registry.register_type(RENDERDEPTHMODE_TYPE_INFO);
    registry.register_type(RENDERDEPTHMODE_ARRAY_TYPE_INFO);
    registry.register_type(RENDERCLEARMASK_TYPE_INFO);
    registry.register_type(RENDERCLEARMASK_ARRAY_TYPE_INFO);
    registry.register_type(RENDERWRITEMASK_TYPE_INFO);
    registry.register_type(RENDERWRITEMASK_ARRAY_TYPE_INFO);
    registry.register_type(RENDERBLENDOP_TYPE_INFO);
    registry.register_type(RENDERBLENDOP_ARRAY_TYPE_INFO);
    registry.register_type(RENDERBLENDMODE_TYPE_INFO);
    registry.register_type(RENDERBLENDMODE_ARRAY_TYPE_INFO);
    registry.register_type(RENDERCULLMODE_TYPE_INFO);
    registry.register_type(RENDERCULLMODE_ARRAY_TYPE_INFO);
    registry.register_type(PRIMITIVETYPE_TYPE_INFO);
    registry.register_type(PRIMITIVETYPE_ARRAY_TYPE_INFO);
    registry.register_type(RENDERBORDERCOLOR_TYPE_INFO);
    registry.register_type(RENDERBORDERCOLOR_ARRAY_TYPE_INFO);
    registry.register_type(RENDERCHANNELFORMAT_TYPE_INFO);
    registry.register_type(RENDERCHANNELFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(RENDERNUMERICFORMAT_TYPE_INFO);
    registry.register_type(RENDERNUMERICFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(RENDERFORMAT_TYPE_INFO);
    registry.register_type(RENDERFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(BASEDISPLAYSETTINGS_TYPE_INFO);
    registry.register_type(BASEDISPLAYSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(PS4DISPLAYSETTINGS_TYPE_INFO);
    registry.register_type(PS4DISPLAYSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(DX12DISPLAYSETTINGS_TYPE_INFO);
    registry.register_type(DX12DISPLAYSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(STABLEPOWERSTATE_TYPE_INFO);
    registry.register_type(STABLEPOWERSTATE_ARRAY_TYPE_INFO);
    registry.register_type(DX11DISPLAYSETTINGS_TYPE_INFO);
    registry.register_type(DX11DISPLAYSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(DXDISPLAYSETTINGS_TYPE_INFO);
    registry.register_type(DXDISPLAYSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(ITEXTURE_TYPE_INFO);
    registry.register_type(ITEXTURE_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BaseTexture {
}

pub const BASETEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseTexture",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ITEXTURE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BASETEXTURE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BaseTexture {
    fn type_info() -> &'static TypeInfo {
        BASETEXTURE_TYPE_INFO
    }
}


pub const BASETEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("BaseTexture-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BaseRenderBuffer {
}

pub const BASERENDERBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseRenderBuffer",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IRENDERBUFFER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BASERENDERBUFFER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BaseRenderBuffer {
    fn type_info() -> &'static TypeInfo {
        BASERENDERBUFFER_TYPE_INFO
    }
}


pub const BASERENDERBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseRenderBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("BaseRenderBuffer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AssetRefTexture {
}

pub const ASSETREFTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AssetRefTexture",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ITEXTURE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ASSETREFTEXTURE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AssetRefTexture {
    fn type_info() -> &'static TypeInfo {
        ASSETREFTEXTURE_TYPE_INFO
    }
}


pub const ASSETREFTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AssetRefTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("AssetRefTexture-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11Texture {
}

pub const DX11TEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11Texture",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASETEXTURE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11TEXTURE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx11Texture {
    fn type_info() -> &'static TypeInfo {
        DX11TEXTURE_TYPE_INFO
    }
}


pub const DX11TEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11Texture-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Dx11Texture-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11ShaderProgramDatabase {
}

pub const DX11SHADERPROGRAMDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11ShaderProgramDatabase",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASESHADERPROGRAMDATABASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11SHADERPROGRAMDATABASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx11ShaderProgramDatabase {
    fn type_info() -> &'static TypeInfo {
        DX11SHADERPROGRAMDATABASE_TYPE_INFO
    }
}


pub const DX11SHADERPROGRAMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11ShaderProgramDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Dx11ShaderProgramDatabase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RenderTargetView {
}

pub const DX11RENDERTARGETVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RenderTargetView",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IRENDERTARGETVIEW_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11RENDERTARGETVIEW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx11RenderTargetView {
    fn type_info() -> &'static TypeInfo {
        DX11RENDERTARGETVIEW_TYPE_INFO
    }
}


pub const DX11RENDERTARGETVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RenderTargetView-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Dx11RenderTargetView-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx11RenderBuffer {
}

pub const DX11RENDERBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RenderBuffer",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASERENDERBUFFER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX11RENDERBUFFER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx11RenderBuffer {
    fn type_info() -> &'static TypeInfo {
        DX11RENDERBUFFER_TYPE_INFO
    }
}


pub const DX11RENDERBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11RenderBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Dx11RenderBuffer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BaseShaderProgramDatabase {
}

pub const BASESHADERPROGRAMDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseShaderProgramDatabase",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(BASESHADERPROGRAMDATABASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BaseShaderProgramDatabase {
    fn type_info() -> &'static TypeInfo {
        BASESHADERPROGRAMDATABASE_TYPE_INFO
    }
}


pub const BASESHADERPROGRAMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseShaderProgramDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("BaseShaderProgramDatabase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NullRaytraceSystem {
}

pub const NULLRAYTRACESYSTEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NullRaytraceSystem",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IRAYTRACESYSTEM_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(NULLRAYTRACESYSTEM_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for NullRaytraceSystem {
    fn type_info() -> &'static TypeInfo {
        NULLRAYTRACESYSTEM_TYPE_INFO
    }
}


pub const NULLRAYTRACESYSTEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NullRaytraceSystem-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("NullRaytraceSystem-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NullRaytraceSceneBuilder {
}

pub const NULLRAYTRACESCENEBUILDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NullRaytraceSceneBuilder",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IRAYTRACESCENEBUILDER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(NULLRAYTRACESCENEBUILDER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for NullRaytraceSceneBuilder {
    fn type_info() -> &'static TypeInfo {
        NULLRAYTRACESCENEBUILDER_TYPE_INFO
    }
}


pub const NULLRAYTRACESCENEBUILDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NullRaytraceSceneBuilder-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("NullRaytraceSceneBuilder-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PcRaytraceSystem {
}

pub const DX12PCRAYTRACESYSTEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRaytraceSystem",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IRAYTRACESYSTEM_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12PCRAYTRACESYSTEM_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx12PcRaytraceSystem {
    fn type_info() -> &'static TypeInfo {
        DX12PCRAYTRACESYSTEM_TYPE_INFO
    }
}


pub const DX12PCRAYTRACESYSTEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PcRaytraceSystem-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Dx12PcRaytraceSystem-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RaytraceSceneBuilder {
}

pub const DX12RAYTRACESCENEBUILDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RaytraceSceneBuilder",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IRAYTRACESCENEBUILDER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RAYTRACESCENEBUILDER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx12RaytraceSceneBuilder {
    fn type_info() -> &'static TypeInfo {
        DX12RAYTRACESCENEBUILDER_TYPE_INFO
    }
}


pub const DX12RAYTRACESCENEBUILDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RaytraceSceneBuilder-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Dx12RaytraceSceneBuilder-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RenderTargetView {
}

pub const DX12RENDERTARGETVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RenderTargetView",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IRENDERTARGETVIEW_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RENDERTARGETVIEW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx12RenderTargetView {
    fn type_info() -> &'static TypeInfo {
        DX12RENDERTARGETVIEW_TYPE_INFO
    }
}


pub const DX12RENDERTARGETVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RenderTargetView-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Dx12RenderTargetView-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RenderStateObject {
}

pub const DX12RENDERSTATEOBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RenderStateObject",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IRENDERSTATEOBJECT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RENDERSTATEOBJECT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx12RenderStateObject {
    fn type_info() -> &'static TypeInfo {
        DX12RENDERSTATEOBJECT_TYPE_INFO
    }
}


pub const DX12RENDERSTATEOBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RenderStateObject-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Dx12RenderStateObject-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12NvidiaComputePsoDescType {
}

pub const DX12NVIDIACOMPUTEPSODESCTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvidiaComputePsoDescType",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DX12COMPUTEPSODESCTYPE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12NVIDIACOMPUTEPSODESCTYPE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12NvidiaComputePsoDescType {
    fn type_info() -> &'static TypeInfo {
        DX12NVIDIACOMPUTEPSODESCTYPE_TYPE_INFO
    }
}


pub const DX12NVIDIACOMPUTEPSODESCTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12NvidiaComputePsoDescType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Dx12NvidiaComputePsoDescType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12RenderBuffer {
}

pub const DX12RENDERBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RenderBuffer",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASERENDERBUFFER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12RENDERBUFFER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx12RenderBuffer {
    fn type_info() -> &'static TypeInfo {
        DX12RENDERBUFFER_TYPE_INFO
    }
}


pub const DX12RENDERBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12RenderBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Dx12RenderBuffer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshSet {
}

pub const MESHSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshSet",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(MESHSET_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshSet {
    fn type_info() -> &'static TypeInfo {
        MESHSET_TYPE_INFO
    }
}


pub const MESHSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshSet-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshSet-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RenderTestSettings {
    pub enable: bool,
    pub schematics_enable: bool,
    pub draw_number_enable: bool,
    pub case: u32,
    pub sub_case: u32,
    pub next: i32,
}

pub const RENDERTESTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderTestSettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RenderTestSettings, enable),
            },
            FieldInfoData {
                name: "SchematicsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RenderTestSettings, schematics_enable),
            },
            FieldInfoData {
                name: "DrawNumberEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RenderTestSettings, draw_number_enable),
            },
            FieldInfoData {
                name: "Case",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RenderTestSettings, case),
            },
            FieldInfoData {
                name: "SubCase",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RenderTestSettings, sub_case),
            },
            FieldInfoData {
                name: "Next",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RenderTestSettings, next),
            },
        ],
    }),
    array_type: Some(RENDERTESTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RenderTestSettings {
    fn type_info() -> &'static TypeInfo {
        RENDERTESTSETTINGS_TYPE_INFO
    }
}


pub const RENDERTESTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderTestSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RenderTestSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PerfOverlaySettings {
    pub enable: bool,
    pub draw_graph: bool,
    pub draw_cpu_graph: bool,
    pub draw_sim_graph: bool,
    pub draw_gpu_graph: bool,
    pub draw_frame_graph: bool,
    pub draw_vblank_graph: bool,
    pub draw_fps: bool,
    pub draw_sim: bool,
    pub draw_gpu: bool,
    pub draw_pixel_throughput: bool,
    pub pixel_throughput_display_format: i32,
    pub draw_fcat: bool,
    pub fcat_width: f32,
    pub legend_display_format: u32,
    pub fps_time_period: f32,
    pub fps_display_alpha: u8,
    pub fps_display_format: i32,
    pub fps_display_scale: f32,
    pub fps_display_offset_x: i32,
    pub fps_display_offset_y: i32,
    pub draw_frame_index_size: f32,
    pub frame_time_source: u32,
    pub target_fps_is60: bool,
    pub target_resolution_scale: f32,
    pub frame_file_log_enable: bool,
}

pub const PERFOVERLAYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfOverlaySettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, enable),
            },
            FieldInfoData {
                name: "DrawGraph",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, draw_graph),
            },
            FieldInfoData {
                name: "DrawCpuGraph",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, draw_cpu_graph),
            },
            FieldInfoData {
                name: "DrawSimGraph",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, draw_sim_graph),
            },
            FieldInfoData {
                name: "DrawGpuGraph",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, draw_gpu_graph),
            },
            FieldInfoData {
                name: "DrawFrameGraph",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, draw_frame_graph),
            },
            FieldInfoData {
                name: "DrawVblankGraph",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, draw_vblank_graph),
            },
            FieldInfoData {
                name: "DrawFps",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, draw_fps),
            },
            FieldInfoData {
                name: "DrawSim",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, draw_sim),
            },
            FieldInfoData {
                name: "DrawGpu",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, draw_gpu),
            },
            FieldInfoData {
                name: "DrawPixelThroughput",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, draw_pixel_throughput),
            },
            FieldInfoData {
                name: "PixelThroughputDisplayFormat",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, pixel_throughput_display_format),
            },
            FieldInfoData {
                name: "DrawFcat",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, draw_fcat),
            },
            FieldInfoData {
                name: "FcatWidth",
                flags: MemberInfoFlags::new(8192),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, fcat_width),
            },
            FieldInfoData {
                name: "LegendDisplayFormat",
                flags: MemberInfoFlags::new(8192),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, legend_display_format),
            },
            FieldInfoData {
                name: "FpsTimePeriod",
                flags: MemberInfoFlags::new(8192),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, fps_time_period),
            },
            FieldInfoData {
                name: "FpsDisplayAlpha",
                flags: MemberInfoFlags::new(8192),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, fps_display_alpha),
            },
            FieldInfoData {
                name: "FpsDisplayFormat",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, fps_display_format),
            },
            FieldInfoData {
                name: "FpsDisplayScale",
                flags: MemberInfoFlags::new(8192),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, fps_display_scale),
            },
            FieldInfoData {
                name: "FpsDisplayOffsetX",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, fps_display_offset_x),
            },
            FieldInfoData {
                name: "FpsDisplayOffsetY",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, fps_display_offset_y),
            },
            FieldInfoData {
                name: "DrawFrameIndexSize",
                flags: MemberInfoFlags::new(8192),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, draw_frame_index_size),
            },
            FieldInfoData {
                name: "FrameTimeSource",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, frame_time_source),
            },
            FieldInfoData {
                name: "TargetFpsIs60",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, target_fps_is60),
            },
            FieldInfoData {
                name: "TargetResolutionScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, target_resolution_scale),
            },
            FieldInfoData {
                name: "FrameFileLogEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PerfOverlaySettings, frame_file_log_enable),
            },
        ],
    }),
    array_type: Some(PERFOVERLAYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PerfOverlaySettings {
    fn type_info() -> &'static TypeInfo {
        PERFOVERLAYSETTINGS_TYPE_INFO
    }
}


pub const PERFOVERLAYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PerfOverlaySettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("PerfOverlaySettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TextureCompressSettings {
    pub view_mode: TextureCompressQualityMode,
    pub texture_compress_job_pool_size: u32,
    pub debug_draw_enable: bool,
    pub debug_draw_alpha_overlay_scale: f32,
}

pub const TEXTURECOMPRESSSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureCompressSettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ViewMode",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTURECOMPRESSQUALITYMODE_TYPE_INFO,
                rust_offset: offset_of!(TextureCompressSettings, view_mode),
            },
            FieldInfoData {
                name: "TextureCompressJobPoolSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureCompressSettings, texture_compress_job_pool_size),
            },
            FieldInfoData {
                name: "DebugDrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureCompressSettings, debug_draw_enable),
            },
            FieldInfoData {
                name: "DebugDrawAlphaOverlayScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TextureCompressSettings, debug_draw_alpha_overlay_scale),
            },
        ],
    }),
    array_type: Some(TEXTURECOMPRESSSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TextureCompressSettings {
    fn type_info() -> &'static TypeInfo {
        TEXTURECOMPRESSSETTINGS_TYPE_INFO
    }
}


pub const TEXTURECOMPRESSSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureCompressSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("TextureCompressSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TextureCompressQualityMode {
    #[default]
    TextureCompressQualityMode_Default = 0,
    TextureCompressQualityMode_Simple = 1,
    TextureCompressQualityMode_HighQuality = 2,
}

pub const TEXTURECOMPRESSQUALITYMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureCompressQualityMode",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(TEXTURECOMPRESSQUALITYMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TextureCompressQualityMode {
    fn type_info() -> &'static TypeInfo {
        TEXTURECOMPRESSQUALITYMODE_TYPE_INFO
    }
}


pub const TEXTURECOMPRESSQUALITYMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureCompressQualityMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("TextureCompressQualityMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderConstantSystemBuffer {
    #[default]
    ShaderConstantSystemBuffer_TiledForwardLightPunctual = 0,
    ShaderConstantSystemBuffer_TiledForwardLightPunctualShadow = 1,
    ShaderConstantSystemBuffer_TiledForwardLightArea = 2,
    ShaderConstantSystemBuffer_TiledForwardLightAreaShadow = 3,
    ShaderConstantSystemBuffer_TiledForwardLightLRV = 4,
    ShaderConstantSystemBuffer_TiledForwardLightLPR = 5,
    ShaderConstantSystemBuffer_TiledForwardLightGrid = 6,
    ShaderConstantSystemBuffer_TiledForwardLightIndex = 7,
    ShaderConstantSystemBuffer_SkinningBuffer = 8,
    ShaderConstantSystemBuffer_PrevSkinningBuffer = 9,
    ShaderConstantSystemBufferCount = 10,
}

pub const SHADERCONSTANTSYSTEMBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderConstantSystemBuffer",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERCONSTANTSYSTEMBUFFER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderConstantSystemBuffer {
    fn type_info() -> &'static TypeInfo {
        SHADERCONSTANTSYSTEMBUFFER_TYPE_INFO
    }
}


pub const SHADERCONSTANTSYSTEMBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderConstantSystemBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderConstantSystemBuffer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderConstantSystemTexture {
    #[default]
    ShaderConstantSystemTexture_DepthBufferTexture = 0,
    ShaderConstantSystemTexture_NormalBufferTexture = 1,
    ShaderConstantSystemTexture_LocalIBLTextureArray = 2,
    ShaderConstantSystemTexture_LocalPRTextureArray = 3,
    ShaderConstantSystemTexture_LocalPRDepthTextureArray = 4,
    ShaderConstantSystemTexture_IESTextureArray = 5,
    ShaderConstantSystemTexture_NormalBasisGBufferTexture = 6,
    ShaderConstantSystemTexture_LightShaftTexture = 7,
    ShaderConstantSystemTexture_LocalLightShadowAtlasTexture = 8,
    ShaderConstantSystemTexture_DirectionalShadowTextureArray = 9,
    ShaderConstantSystemTexture_EmitterDirectionalTransmittanceTexture = 10,
    ShaderConstantSystemTexture_PreIntegratedFGTexture = 11,
    ShaderConstantSystemTexture_ForwardReflectionTexture = 12,
    ShaderConstantSystemTexture_ForwardReflectionDepthBufferTexture = 13,
    ShaderConstantSystemTextureCount = 14,
}

pub const SHADERCONSTANTSYSTEMTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderConstantSystemTexture",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERCONSTANTSYSTEMTEXTURE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderConstantSystemTexture {
    fn type_info() -> &'static TypeInfo {
        SHADERCONSTANTSYSTEMTEXTURE_TYPE_INFO
    }
}


pub const SHADERCONSTANTSYSTEMTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderConstantSystemTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderConstantSystemTexture-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderTimeType {
    #[default]
    ShaderTimeType_Game = 0,
    ShaderTimeType_Real = 1,
    ShaderTimeType_World = 2,
}

pub const SHADERTIMETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderTimeType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERTIMETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderTimeType {
    fn type_info() -> &'static TypeInfo {
        SHADERTIMETYPE_TYPE_INFO
    }
}


pub const SHADERTIMETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderTimeType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderTimeType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderInterpolationType {
    #[default]
    ShaderInterpolationType_Linear = 0,
    ShaderInterpolationType_Centroid = 1,
    ShaderInterpolationType_NoInterpolation = 2,
    ShaderInterpolationType_NoPerspective = 3,
    ShaderInterpolationType_Sample = 4,
    ShaderInterpolationType_Count = 5,
    ShaderInterpolationType_Mask = 255,
    ShaderInterpolationType_DomainManual = 256,
    ShaderInterpolationType_VertexOnly = 512,
    ShaderInterpolationType_VertexHullOnly = 1024,
    ShaderInterpolationType_VertexHullDomainOnly = 2048,
}

pub const SHADERINTERPOLATIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderInterpolationType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERINTERPOLATIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderInterpolationType {
    fn type_info() -> &'static TypeInfo {
        SHADERINTERPOLATIONTYPE_TYPE_INFO
    }
}


pub const SHADERINTERPOLATIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderInterpolationType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderInterpolationType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderDepthBiasGroup {
    #[default]
    ShaderDepthBiasGroup_Default = 0,
    ShaderDepthBiasGroup_Decal = 1,
    ShaderDepthBiasGroup_EmitterOcclusion = 2,
    ShaderDepthBiasGroup_EdgeModel = 3,
    ShaderDepthBiasGroup_TerrainDecal = 4,
    ShaderDepthBiasGroup_TerrainDecalZPass = 5,
    ShaderDepthBiasGroup_DistantShadowCache_LowestBias = 6,
    ShaderDepthBiasGroup_DistantShadowCache_LowerBias = 7,
    ShaderDepthBiasGroup_DistantShadowCache_NormalBias = 8,
    ShaderDepthBiasGroup_DistantShadowCache_HigherBias = 9,
    ShaderDepthBiasGroup_DistantShadowCache_HighestBias = 10,
    ShaderDepthBiasGroup_Shadow16Bit = 11,
    ShaderDepthBiasGroup_Shadow24Bit = 12,
    ShaderDepthBiasGroup_Shadow32Bit = 13,
    ShaderDepthBiasGroup_ZPass = 14,
    ShaderDepthBiasGroup_Emissive = 15,
    ShaderDepthBiasGroup_VelocityVector = 16,
    ShaderDepthBiasGroupCount = 17,
}

pub const SHADERDEPTHBIASGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderDepthBiasGroup",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERDEPTHBIASGROUP_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderDepthBiasGroup {
    fn type_info() -> &'static TypeInfo {
        SHADERDEPTHBIASGROUP_TYPE_INFO
    }
}


pub const SHADERDEPTHBIASGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderDepthBiasGroup-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderDepthBiasGroup-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderTextureCoordType {
    #[default]
    ShaderTextureCoordType_Unknown = 0,
    ShaderTextureCoordType_VertexElement = 1,
    ShaderTextureCoordType_WorldPos = 2,
}

pub const SHADERTEXTURECOORDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderTextureCoordType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERTEXTURECOORDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderTextureCoordType {
    fn type_info() -> &'static TypeInfo {
        SHADERTEXTURECOORDTYPE_TYPE_INFO
    }
}


pub const SHADERTEXTURECOORDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderTextureCoordType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderTextureCoordType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderValueFormat {
    #[default]
    ShaderValueFormat_Half = 0,
    ShaderValueFormat_Float = 1,
    ShaderValueFormat_Int = 2,
    ShaderValueFormat_UInt = 3,
    ShaderValueFormat_Bool = 4,
}

pub const SHADERVALUEFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderValueFormat",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERVALUEFORMAT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderValueFormat {
    fn type_info() -> &'static TypeInfo {
        SHADERVALUEFORMAT_TYPE_INFO
    }
}


pub const SHADERVALUEFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderValueFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderValueFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderSamplerFormat {
    #[default]
    ShaderSamplerFormat_None = 0,
    ShaderSamplerFormat_Half1 = 1,
    ShaderSamplerFormat_Half2 = 2,
    ShaderSamplerFormat_Half3 = 3,
    ShaderSamplerFormat_Half4 = 4,
    ShaderSamplerFormat_Float1 = 5,
    ShaderSamplerFormat_Float2 = 6,
    ShaderSamplerFormat_Float3 = 7,
    ShaderSamplerFormat_Float4 = 8,
    ShaderSamplerFormat_Int1 = 9,
    ShaderSamplerFormat_Int2 = 10,
    ShaderSamplerFormat_Int3 = 11,
    ShaderSamplerFormat_Int4 = 12,
    ShaderSamplerFormat_UInt1 = 13,
    ShaderSamplerFormat_UInt2 = 14,
    ShaderSamplerFormat_UInt3 = 15,
    ShaderSamplerFormat_UInt4 = 16,
    ShaderSamplerFormat_UShort1 = 17,
    ShaderSamplerFormat_UShort2 = 18,
    ShaderSamplerFormat_UShort3 = 19,
    ShaderSamplerFormat_UShort4 = 20,
    ShaderSamplerFormat_Short1 = 21,
    ShaderSamplerFormat_Short2 = 22,
    ShaderSamplerFormat_Short3 = 23,
    ShaderSamplerFormat_Short4 = 24,
}

pub const SHADERSAMPLERFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderSamplerFormat",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERSAMPLERFORMAT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderSamplerFormat {
    fn type_info() -> &'static TypeInfo {
        SHADERSAMPLERFORMAT_TYPE_INFO
    }
}


pub const SHADERSAMPLERFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderSamplerFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderSamplerFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderValueType {
    #[default]
    ShaderValueType_None = 0,
    ShaderValueType_Half1 = 1,
    ShaderValueType_Half1x2 = 2,
    ShaderValueType_Half1x3 = 3,
    ShaderValueType_Half1x4 = 4,
    ShaderValueType_Half2 = 5,
    ShaderValueType_Half2x2 = 6,
    ShaderValueType_Half2x3 = 7,
    ShaderValueType_Half2x4 = 8,
    ShaderValueType_Half3 = 9,
    ShaderValueType_Half3x2 = 10,
    ShaderValueType_Half3x3 = 11,
    ShaderValueType_Half3x4 = 12,
    ShaderValueType_Half4 = 13,
    ShaderValueType_Half4x2 = 14,
    ShaderValueType_Half4x3 = 15,
    ShaderValueType_Half4x4 = 16,
    ShaderValueType_Float1 = 17,
    ShaderValueType_Float1x2 = 18,
    ShaderValueType_Float1x3 = 19,
    ShaderValueType_Float1x4 = 20,
    ShaderValueType_Float2 = 21,
    ShaderValueType_Float2x2 = 22,
    ShaderValueType_Float2x3 = 23,
    ShaderValueType_Float2x4 = 24,
    ShaderValueType_Float3 = 25,
    ShaderValueType_Float3x2 = 26,
    ShaderValueType_Float3x3 = 27,
    ShaderValueType_Float3x4 = 28,
    ShaderValueType_Float4 = 29,
    ShaderValueType_Float4x2 = 30,
    ShaderValueType_Float4x3 = 31,
    ShaderValueType_Float4x4 = 32,
    ShaderValueType_Int1 = 33,
    ShaderValueType_Int1x2 = 34,
    ShaderValueType_Int1x3 = 35,
    ShaderValueType_Int1x4 = 36,
    ShaderValueType_Int2 = 37,
    ShaderValueType_Int2x2 = 38,
    ShaderValueType_Int2x3 = 39,
    ShaderValueType_Int2x4 = 40,
    ShaderValueType_Int3 = 41,
    ShaderValueType_Int3x2 = 42,
    ShaderValueType_Int3x3 = 43,
    ShaderValueType_Int3x4 = 44,
    ShaderValueType_Int4 = 45,
    ShaderValueType_Int4x2 = 46,
    ShaderValueType_Int4x3 = 47,
    ShaderValueType_Int4x4 = 48,
    ShaderValueType_UInt1 = 49,
    ShaderValueType_UInt1x2 = 50,
    ShaderValueType_UInt1x3 = 51,
    ShaderValueType_UInt1x4 = 52,
    ShaderValueType_UInt2 = 53,
    ShaderValueType_UInt2x2 = 54,
    ShaderValueType_UInt2x3 = 55,
    ShaderValueType_UInt2x4 = 56,
    ShaderValueType_UInt3 = 57,
    ShaderValueType_UInt3x2 = 58,
    ShaderValueType_UInt3x3 = 59,
    ShaderValueType_UInt3x4 = 60,
    ShaderValueType_UInt4 = 61,
    ShaderValueType_UInt4x2 = 62,
    ShaderValueType_UInt4x3 = 63,
    ShaderValueType_UInt4x4 = 64,
    ShaderValueType_Bool1 = 65,
    ShaderValueType_Bool1x2 = 66,
    ShaderValueType_Bool1x3 = 67,
    ShaderValueType_Bool1x4 = 68,
    ShaderValueType_Bool2 = 69,
    ShaderValueType_Bool2x2 = 70,
    ShaderValueType_Bool2x3 = 71,
    ShaderValueType_Bool2x4 = 72,
    ShaderValueType_Bool3 = 73,
    ShaderValueType_Bool3x2 = 74,
    ShaderValueType_Bool3x3 = 75,
    ShaderValueType_Bool3x4 = 76,
    ShaderValueType_Bool4 = 77,
    ShaderValueType_Bool4x2 = 78,
    ShaderValueType_Bool4x3 = 79,
    ShaderValueType_Bool4x4 = 80,
    ShaderValueType_Sampler1d = 81,
    ShaderValueType_Sampler1dArray = 82,
    ShaderValueType_Sampler2d = 83,
    ShaderValueType_Sampler2dArray = 84,
    ShaderValueType_Sampler3d = 85,
    ShaderValueType_SamplerCube = 86,
    ShaderValueType_SamplerCubeArray = 87,
    ShaderValueType_BufferTemplate = 88,
    ShaderValueType_StructuredBufferTemplate = 89,
    ShaderValueType_ByteBufferTemplate = 90,
    ShaderValueType_ByteAddressBuffer = 91,
}

pub const SHADERVALUETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderValueType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERVALUETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderValueType {
    fn type_info() -> &'static TypeInfo {
        SHADERVALUETYPE_TYPE_INFO
    }
}


pub const SHADERVALUETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderValueType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderValueType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderGeometrySpace {
    #[default]
    ShaderGeometrySpace_Object = 0,
    ShaderGeometrySpace_ObjectScaled = 1,
    ShaderGeometrySpace_World = 2,
    ShaderGeometrySpace_Screen = 3,
    ShaderGeometrySpace_Manual = 4,
    ShaderGeometrySpaceCount = 5,
}

pub const SHADERGEOMETRYSPACE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderGeometrySpace",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERGEOMETRYSPACE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderGeometrySpace {
    fn type_info() -> &'static TypeInfo {
        SHADERGEOMETRYSPACE_TYPE_INFO
    }
}


pub const SHADERGEOMETRYSPACE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderGeometrySpace-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderGeometrySpace-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SubSurfaceProfileType {
    #[default]
    SubSurfaceProfileType_Profile0 = 0,
    SubSurfaceProfileType_Profile1 = 1,
    SubSurfaceProfileType_Profile2 = 2,
    SubSurfaceProfileType_Profile3 = 3,
    SubSurfaceProfileType_Profile4 = 4,
    SubSurfaceProfileType_Profile5 = 5,
    SubSurfaceProfileType_Profile6 = 6,
    SubSurfaceProfileType_ProfileOATS = 7,
    SubSurfaceProfileTypeCount = 8,
}

pub const SUBSURFACEPROFILETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSurfaceProfileType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SUBSURFACEPROFILETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SubSurfaceProfileType {
    fn type_info() -> &'static TypeInfo {
        SUBSURFACEPROFILETYPE_TYPE_INFO
    }
}


pub const SUBSURFACEPROFILETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SubSurfaceProfileType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("SubSurfaceProfileType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderShadowmapMethod {
    #[default]
    ShaderShadowmapMethod_None = 0,
    ShaderShadowmapMethod_CascadedBox = 1,
    ShaderShadowmapMethodCount = 2,
}

pub const SHADERSHADOWMAPMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderShadowmapMethod",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERSHADOWMAPMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderShadowmapMethod {
    fn type_info() -> &'static TypeInfo {
        SHADERSHADOWMAPMETHOD_TYPE_INFO
    }
}


pub const SHADERSHADOWMAPMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderShadowmapMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderShadowmapMethod-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderObjectLighting {
    #[default]
    ShaderObjectLighting_None = 0,
    ShaderObjectLighting_LightProbe = 1,
    ShaderObjectLighting_LightMap = 2,
    ShaderObjectLighting_RgbDirLightMap = 3,
    ShaderObjectLighting_LightProbe_LightMap = 4,
    ShaderObjectLighting_LightProbe_RgbDirLightMap = 5,
    ShaderObjectLightingCount = 6,
}

pub const SHADEROBJECTLIGHTING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderObjectLighting",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADEROBJECTLIGHTING_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderObjectLighting {
    fn type_info() -> &'static TypeInfo {
        SHADEROBJECTLIGHTING_TYPE_INFO
    }
}


pub const SHADEROBJECTLIGHTING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderObjectLighting-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderObjectLighting-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderDebugRenderMode {
    #[default]
    ShaderDebugRenderMode_None = 0,
    ShaderDebugRenderMode_Overdraw = 1,
    ShaderDebugRenderMode_OverdrawDepthTest = 2,
    ShaderDebugRenderMode_ShaderCost = 3,
}

pub const SHADERDEBUGRENDERMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderDebugRenderMode",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERDEBUGRENDERMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderDebugRenderMode {
    fn type_info() -> &'static TypeInfo {
        SHADERDEBUGRENDERMODE_TYPE_INFO
    }
}


pub const SHADERDEBUGRENDERMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderDebugRenderMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderDebugRenderMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderRenderMode {
    #[default]
    ShaderRenderMode_Forward = 0,
    ShaderRenderMode_ForwardSimple = 1,
    ShaderRenderMode_ZOnly = 2,
    ShaderRenderMode_DeferredShadingGBufferLayout0 = 3,
    ShaderRenderMode_DeferredShadingGBufferLayout1 = 4,
    ShaderRenderMode_DeferredShadingGBufferLayout2 = 5,
    ShaderRenderMode_DeferredShadingGBufferLayout3 = 6,
    ShaderRenderMode_Raytrace = 7,
    ShaderRenderMode_RaytraceForward = 8,
    ShaderRenderMode_ForwardEmissive = 9,
    ShaderRenderMode_VelocityVector = 10,
    ShaderRenderMode_MeshPick = 11,
    ShaderRenderMode_DistortionVector = 12,
    ShaderRenderMode_DebugMulti = 13,
    ShaderRenderMode_ForwardOpaque_RuntimeOnly = 14,
    ShaderRenderMode_TextureSpace = 15,
    ShaderRenderMode_GBufferSimple = 16,
    ShaderRenderModeCount = 17,
}

pub const SHADERRENDERMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderRenderMode",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERRENDERMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderRenderMode {
    fn type_info() -> &'static TypeInfo {
        SHADERRENDERMODE_TYPE_INFO
    }
}


pub const SHADERRENDERMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderRenderMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderRenderMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderSkinningMethod {
    #[default]
    ShaderSkinningMethod_None = 0,
    ShaderSkinningMethod_Linear1Bone = 1,
    ShaderSkinningMethod_Linear2Bone = 2,
    ShaderSkinningMethod_Linear4Bone = 4,
    ShaderSkinningMethod_Linear6Bone = 6,
    ShaderSkinningMethod_Linear8Bone = 8,
    ShaderSkinningMethod_Null = 9,
    ShaderSkinningMethod_DualQuaternion4Bone = 10,
    ShaderSkinningMethodCount = 11,
}

pub const SHADERSKINNINGMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderSkinningMethod",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERSKINNINGMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderSkinningMethod {
    fn type_info() -> &'static TypeInfo {
        SHADERSKINNINGMETHOD_TYPE_INFO
    }
}


pub const SHADERSKINNINGMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderSkinningMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderSkinningMethod-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderInstancingMethod {
    #[default]
    ShaderInstancingMethod_None = 0,
    ShaderInstancingMethod_ObjectTransform4x3Half = 1,
    ShaderInstancingMethod_ObjectTransform4x3InstanceData4x1Half = 2,
    ShaderInstancingMethod_ObjectTransform4x3InstanceData4x2Half = 3,
    ShaderInstancingMethod_WorldTransform4x3Float = 4,
    ShaderInstancingMethod_WorldTransform4x3FloatInstanceData4x2Half = 5,
    ShaderInstancingMethod_PrevWorldTransform4x3FloatInstanceData4x2Half = 6,
    ShaderInstancingMethod_ObjectTranslationScaleHalf = 7,
    ShaderInstancingMethod_ObjectTranslationScaleHalfInstanceData4x1Half = 8,
    ShaderInstancingMethod_ObjectTranslationScaleHalfInstanceData4x2Half = 9,
    ShaderInstancingMethod_PositionStream = 10,
    ShaderInstancingMethod_PositionTbnStream = 11,
    ShaderInstancingMethod_PrevPositionStream = 12,
    ShaderInstancingMethod_LinearMediaStreaming = 13,
    ShaderInstancingMethod_PositionStreamAux = 14,
    ShaderInstancingMethod_DxBuffer = 15,
    ShaderInstancingMethod_DxBufferInstanceData4x1Float = 16,
    ShaderInstancingMethod_DxBufferInstanceData4x2Float = 17,
    ShaderInstancingMethod_Manual = 18,
    ShaderInstancingMethodCount = 19,
}

pub const SHADERINSTANCINGMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderInstancingMethod",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERINSTANCINGMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderInstancingMethod {
    fn type_info() -> &'static TypeInfo {
        SHADERINSTANCINGMETHOD_TYPE_INFO
    }
}


pub const SHADERINSTANCINGMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderInstancingMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderInstancingMethod-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderTextureStreamingSupport {
    #[default]
    ShaderTextureStreamingSupport_None = 0,
    ShaderTextureStreamingSupport_Warn = 1,
    ShaderTextureStreamingSupport_Full = 2,
}

pub const SHADERTEXTURESTREAMINGSUPPORT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderTextureStreamingSupport",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERTEXTURESTREAMINGSUPPORT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderTextureStreamingSupport {
    fn type_info() -> &'static TypeInfo {
        SHADERTEXTURESTREAMINGSUPPORT_TYPE_INFO
    }
}


pub const SHADERTEXTURESTREAMINGSUPPORT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderTextureStreamingSupport-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderTextureStreamingSupport-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderBlendMode {
    #[default]
    ShaderBlendMode_Lerp = 0,
    ShaderBlendMode_Additive = 1,
    ShaderBlendMode_Multiply = 2,
    ShaderBlendMode_LerpPremultiplied = 3,
    ShaderBlendMode_PremultipliedColor = 4,
    ShaderBlendMode_DecalLerpNormal = 5,
    ShaderBlendMode_DecalLerpSmoothness = 6,
    ShaderBlendMode_DecalMultiplySmoothness = 7,
    ShaderBlendMode_DecalAddSmoothness = 8,
    ShaderBlendMode_DecalLerpNormalSmoothness = 9,
    ShaderBlendMode_DecalLerpReflectance = 10,
    ShaderBlendMode_DecalMultiplyReflectance = 11,
    ShaderBlendMode_DecalAddReflectance = 12,
    ShaderBlendMode_DecalLerpMetalMask = 13,
    ShaderBlendMode_DecalMultiplyMetalMask = 14,
    ShaderBlendMode_DecalAddMetalMask = 15,
    ShaderBlendMode_DecalLerpBaseColor = 16,
    ShaderBlendMode_DecalMultiplyBaseColor = 17,
    ShaderBlendMode_DecalAddBaseColor = 18,
    ShaderBlendMode_DecalLerpBaseColorMetalMaskReflectance = 19,
    ShaderBlendMode_DecalMultiplyBaseColorMetalMaskReflectance = 20,
    ShaderBlendMode_DecalLerpNormalSmoothnessMultiplyOthers = 21,
    ShaderBlendMode_DecalLerpNormalSmoothnessMultiplyOthersAddEmissive = 22,
    ShaderBlendMode_DecalLerpEverythingAddEmissive = 23,
    ShaderBlendMode_DecalAddBaseColorAddEmissive = 24,
    ShaderBlendMode_DecalLerpEverything = 25,
}

pub const SHADERBLENDMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlendMode",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERBLENDMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderBlendMode {
    fn type_info() -> &'static TypeInfo {
        SHADERBLENDMODE_TYPE_INFO
    }
}


pub const SHADERBLENDMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlendMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderBlendMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SurfaceShaderType {
    #[default]
    SurfaceShaderType_Opaque = 0,
    SurfaceShaderType_OpaqueAlphaTest = 1,
    SurfaceShaderType_OpaqueAlphaTestSimple = 2,
    SurfaceShaderType_Transparent = 3,
    SurfaceShaderType_TransparentDecal = 4,
    SurfaceShaderType_TransparentDepth = 5,
    SurfaceShaderType_TransparentEdge = 6,
    SurfaceShaderTypeCount = 7,
}

pub const SURFACESHADERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceShaderType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SURFACESHADERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SurfaceShaderType {
    fn type_info() -> &'static TypeInfo {
        SURFACESHADERTYPE_TYPE_INFO
    }
}


pub const SURFACESHADERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceShaderType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("SurfaceShaderType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PBRDefaultMaterialModel {
    #[default]
    PBRDefaultMaterialModel_Standard = 0,
    PBRDefaultMaterialModel_SubSurface = 1,
    PBRDefaultMaterialModel_Coated = 2,
}

pub const PBRDEFAULTMATERIALMODEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PBRDefaultMaterialModel",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(PBRDEFAULTMATERIALMODEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PBRDefaultMaterialModel {
    fn type_info() -> &'static TypeInfo {
        PBRDEFAULTMATERIALMODEL_TYPE_INFO
    }
}


pub const PBRDEFAULTMATERIALMODEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PBRDefaultMaterialModel-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("PBRDefaultMaterialModel-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MaterialModel {
    #[default]
    MaterialModel_Standard = 0,
    MaterialModel_Metallic = 1,
    MaterialModel_Skin = 2,
    MaterialModel_DynamicEnvmap = 3,
    MaterialModel_Hair = 4,
    MaterialModel_Translucent = 5,
}

pub const MATERIALMODEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialModel",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(MATERIALMODEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MaterialModel {
    fn type_info() -> &'static TypeInfo {
        MATERIALMODEL_TYPE_INFO
    }
}


pub const MATERIALMODEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MaterialModel-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MaterialModel-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ForwardLightingSupportData {
    pub specular_enable: bool,
    pub local_planar_reflections_enable: bool,
    pub local_reflection_volume_enable: bool,
    pub distant_reflection_volume_enable: bool,
    pub outdoor_light_enable: bool,
    pub sun_specular_enable: bool,
    pub dynamic_lights_enable: bool,
    pub punctual_lights_enable: bool,
    pub punctual_shadow_lights_enable: bool,
    pub punctual_shadow_lights_quality: LocalLightShadowQuality,
    pub area_lights_enable: bool,
    pub area_shadow_lights_enable: bool,
    pub area_shadow_lights_quality: LocalLightShadowQuality,
}

pub const FORWARDLIGHTINGSUPPORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForwardLightingSupportData",
    flags: MemberInfoFlags::new(36937),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "SpecularEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForwardLightingSupportData, specular_enable),
            },
            FieldInfoData {
                name: "LocalPlanarReflectionsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForwardLightingSupportData, local_planar_reflections_enable),
            },
            FieldInfoData {
                name: "LocalReflectionVolumeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForwardLightingSupportData, local_reflection_volume_enable),
            },
            FieldInfoData {
                name: "DistantReflectionVolumeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForwardLightingSupportData, distant_reflection_volume_enable),
            },
            FieldInfoData {
                name: "OutdoorLightEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForwardLightingSupportData, outdoor_light_enable),
            },
            FieldInfoData {
                name: "SunSpecularEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForwardLightingSupportData, sun_specular_enable),
            },
            FieldInfoData {
                name: "DynamicLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForwardLightingSupportData, dynamic_lights_enable),
            },
            FieldInfoData {
                name: "PunctualLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForwardLightingSupportData, punctual_lights_enable),
            },
            FieldInfoData {
                name: "PunctualShadowLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForwardLightingSupportData, punctual_shadow_lights_enable),
            },
            FieldInfoData {
                name: "PunctualShadowLightsQuality",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALLIGHTSHADOWQUALITY_TYPE_INFO,
                rust_offset: offset_of!(ForwardLightingSupportData, punctual_shadow_lights_quality),
            },
            FieldInfoData {
                name: "AreaLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForwardLightingSupportData, area_lights_enable),
            },
            FieldInfoData {
                name: "AreaShadowLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ForwardLightingSupportData, area_shadow_lights_enable),
            },
            FieldInfoData {
                name: "AreaShadowLightsQuality",
                flags: MemberInfoFlags::new(0),
                field_type: LOCALLIGHTSHADOWQUALITY_TYPE_INFO,
                rust_offset: offset_of!(ForwardLightingSupportData, area_shadow_lights_quality),
            },
        ],
    }),
    array_type: Some(FORWARDLIGHTINGSUPPORTDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for ForwardLightingSupportData {
    fn type_info() -> &'static TypeInfo {
        FORWARDLIGHTINGSUPPORTDATA_TYPE_INFO
    }
}


pub const FORWARDLIGHTINGSUPPORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ForwardLightingSupportData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ForwardLightingSupportData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LocalLightShadowQuality {
    #[default]
    LocalLightShadowQuality_Global = 0,
    LocalLightShadowQuality_Low = 1,
    LocalLightShadowQuality_High = 2,
}

pub const LOCALLIGHTSHADOWQUALITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalLightShadowQuality",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(LOCALLIGHTSHADOWQUALITY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LocalLightShadowQuality {
    fn type_info() -> &'static TypeInfo {
        LOCALLIGHTSHADOWQUALITY_TYPE_INFO
    }
}


pub const LOCALLIGHTSHADOWQUALITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalLightShadowQuality-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("LocalLightShadowQuality-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DoubleSidedLightingMode {
    #[default]
    DoubleSidedLightingMode_None = 0,
    DoubleSidedLightingMode_Flip = 1,
    DoubleSidedLightingMode_Mirror = 2,
}

pub const DOUBLESIDEDLIGHTINGMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DoubleSidedLightingMode",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(DOUBLESIDEDLIGHTINGMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DoubleSidedLightingMode {
    fn type_info() -> &'static TypeInfo {
        DOUBLESIDEDLIGHTINGMODE_TYPE_INFO
    }
}


pub const DOUBLESIDEDLIGHTINGMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DoubleSidedLightingMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("DoubleSidedLightingMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CustomForwardShaderValueParameter {
    pub description: String,
    pub name: String,
    pub value_type: ShaderValueType,
    pub default_value: super::core::Vec4,
}

pub const CUSTOMFORWARDSHADERVALUEPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomForwardShaderValueParameter",
    flags: MemberInfoFlags::new(73),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Description",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CustomForwardShaderValueParameter, description),
            },
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CustomForwardShaderValueParameter, name),
            },
            FieldInfoData {
                name: "ValueType",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERVALUETYPE_TYPE_INFO,
                rust_offset: offset_of!(CustomForwardShaderValueParameter, value_type),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(CustomForwardShaderValueParameter, default_value),
            },
        ],
    }),
    array_type: Some(CUSTOMFORWARDSHADERVALUEPARAMETER_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CustomForwardShaderValueParameter {
    fn type_info() -> &'static TypeInfo {
        CUSTOMFORWARDSHADERVALUEPARAMETER_TYPE_INFO
    }
}


pub const CUSTOMFORWARDSHADERVALUEPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomForwardShaderValueParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("CustomForwardShaderValueParameter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CustomForwardShaderSamplerParameter {
    pub description: String,
    pub texture_type: TextureType,
    pub name: String,
    pub min_filter: TextureFilter,
    pub mag_filter: TextureFilter,
    pub mip_filter: TextureFilter,
    pub address_u: super::render_base::TextureAddress,
    pub address_v: super::render_base::TextureAddress,
    pub address_w: super::render_base::TextureAddress,
    pub anisotropic_quality: super::core::QualityScalableEnabled,
    pub border_color: RenderBorderColor,
    pub texture_format: RenderFormat,
    pub hw_pcf_filter_enable: bool,
    pub value_type: ShaderValueType,
}

pub const CUSTOMFORWARDSHADERSAMPLERPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomForwardShaderSamplerParameter",
    flags: MemberInfoFlags::new(73),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Description",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CustomForwardShaderSamplerParameter, description),
            },
            FieldInfoData {
                name: "TextureType",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTURETYPE_TYPE_INFO,
                rust_offset: offset_of!(CustomForwardShaderSamplerParameter, texture_type),
            },
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CustomForwardShaderSamplerParameter, name),
            },
            FieldInfoData {
                name: "MinFilter",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREFILTER_TYPE_INFO,
                rust_offset: offset_of!(CustomForwardShaderSamplerParameter, min_filter),
            },
            FieldInfoData {
                name: "MagFilter",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREFILTER_TYPE_INFO,
                rust_offset: offset_of!(CustomForwardShaderSamplerParameter, mag_filter),
            },
            FieldInfoData {
                name: "MipFilter",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREFILTER_TYPE_INFO,
                rust_offset: offset_of!(CustomForwardShaderSamplerParameter, mip_filter),
            },
            FieldInfoData {
                name: "AddressU",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREADDRESS_TYPE_INFO,
                rust_offset: offset_of!(CustomForwardShaderSamplerParameter, address_u),
            },
            FieldInfoData {
                name: "AddressV",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREADDRESS_TYPE_INFO,
                rust_offset: offset_of!(CustomForwardShaderSamplerParameter, address_v),
            },
            FieldInfoData {
                name: "AddressW",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREADDRESS_TYPE_INFO,
                rust_offset: offset_of!(CustomForwardShaderSamplerParameter, address_w),
            },
            FieldInfoData {
                name: "AnisotropicQuality",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(CustomForwardShaderSamplerParameter, anisotropic_quality),
            },
            FieldInfoData {
                name: "BorderColor",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERBORDERCOLOR_TYPE_INFO,
                rust_offset: offset_of!(CustomForwardShaderSamplerParameter, border_color),
            },
            FieldInfoData {
                name: "TextureFormat",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERFORMAT_TYPE_INFO,
                rust_offset: offset_of!(CustomForwardShaderSamplerParameter, texture_format),
            },
            FieldInfoData {
                name: "HwPcfFilterEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CustomForwardShaderSamplerParameter, hw_pcf_filter_enable),
            },
            FieldInfoData {
                name: "ValueType",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERVALUETYPE_TYPE_INFO,
                rust_offset: offset_of!(CustomForwardShaderSamplerParameter, value_type),
            },
        ],
    }),
    array_type: Some(CUSTOMFORWARDSHADERSAMPLERPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CustomForwardShaderSamplerParameter {
    fn type_info() -> &'static TypeInfo {
        CUSTOMFORWARDSHADERSAMPLERPARAMETER_TYPE_INFO
    }
}


pub const CUSTOMFORWARDSHADERSAMPLERPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomForwardShaderSamplerParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("CustomForwardShaderSamplerParameter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CustomForwardMaterialModel {
    #[default]
    CustomForwardMaterialModel_None = 0,
    CustomForwardMaterialModel_Anisotropic = 1,
    CustomForwardMaterialModel_Hair = 2,
    CustomForwardMaterialModel_AdvancedHair = 3,
}

pub const CUSTOMFORWARDMATERIALMODEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomForwardMaterialModel",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(CUSTOMFORWARDMATERIALMODEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CustomForwardMaterialModel {
    fn type_info() -> &'static TypeInfo {
        CUSTOMFORWARDMATERIALMODEL_TYPE_INFO
    }
}


pub const CUSTOMFORWARDMATERIALMODEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomForwardMaterialModel-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("CustomForwardMaterialModel-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderSamplerParameterBase {
    pub texture_type: TextureType,
    pub name: String,
    pub min_filter: TextureFilter,
    pub mag_filter: TextureFilter,
    pub mip_filter: TextureFilter,
    pub address_u: super::render_base::TextureAddress,
    pub address_v: super::render_base::TextureAddress,
    pub address_w: super::render_base::TextureAddress,
    pub anisotropic_quality: super::core::QualityScalableEnabled,
    pub border_color: RenderBorderColor,
    pub texture_format: RenderFormat,
    pub hw_pcf_filter_enable: bool,
    pub value_type: ShaderValueType,
}

pub const SHADERSAMPLERPARAMETERBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderSamplerParameterBase",
    flags: MemberInfoFlags::new(73),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TextureType",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTURETYPE_TYPE_INFO,
                rust_offset: offset_of!(ShaderSamplerParameterBase, texture_type),
            },
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ShaderSamplerParameterBase, name),
            },
            FieldInfoData {
                name: "MinFilter",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREFILTER_TYPE_INFO,
                rust_offset: offset_of!(ShaderSamplerParameterBase, min_filter),
            },
            FieldInfoData {
                name: "MagFilter",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREFILTER_TYPE_INFO,
                rust_offset: offset_of!(ShaderSamplerParameterBase, mag_filter),
            },
            FieldInfoData {
                name: "MipFilter",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREFILTER_TYPE_INFO,
                rust_offset: offset_of!(ShaderSamplerParameterBase, mip_filter),
            },
            FieldInfoData {
                name: "AddressU",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREADDRESS_TYPE_INFO,
                rust_offset: offset_of!(ShaderSamplerParameterBase, address_u),
            },
            FieldInfoData {
                name: "AddressV",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREADDRESS_TYPE_INFO,
                rust_offset: offset_of!(ShaderSamplerParameterBase, address_v),
            },
            FieldInfoData {
                name: "AddressW",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREADDRESS_TYPE_INFO,
                rust_offset: offset_of!(ShaderSamplerParameterBase, address_w),
            },
            FieldInfoData {
                name: "AnisotropicQuality",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYSCALABLEENABLED_TYPE_INFO,
                rust_offset: offset_of!(ShaderSamplerParameterBase, anisotropic_quality),
            },
            FieldInfoData {
                name: "BorderColor",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERBORDERCOLOR_TYPE_INFO,
                rust_offset: offset_of!(ShaderSamplerParameterBase, border_color),
            },
            FieldInfoData {
                name: "TextureFormat",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERFORMAT_TYPE_INFO,
                rust_offset: offset_of!(ShaderSamplerParameterBase, texture_format),
            },
            FieldInfoData {
                name: "HwPcfFilterEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ShaderSamplerParameterBase, hw_pcf_filter_enable),
            },
            FieldInfoData {
                name: "ValueType",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERVALUETYPE_TYPE_INFO,
                rust_offset: offset_of!(ShaderSamplerParameterBase, value_type),
            },
        ],
    }),
    array_type: Some(SHADERSAMPLERPARAMETERBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderSamplerParameterBase {
    fn type_info() -> &'static TypeInfo {
        SHADERSAMPLERPARAMETERBASE_TYPE_INFO
    }
}


pub const SHADERSAMPLERPARAMETERBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderSamplerParameterBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderSamplerParameterBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UnitType {
    #[default]
    UnitType_Meter = 0,
    UnitType_Undefined = 1,
}

pub const UNITTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UnitType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(UNITTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UnitType {
    fn type_info() -> &'static TypeInfo {
        UNITTYPE_TYPE_INFO
    }
}


pub const UNITTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UnitType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("UnitType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DistortionSpaceType {
    #[default]
    DistortionSpaceType_CameraSpace = 0,
    DistortionSpaceType_ScreenSpace = 1,
}

pub const DISTORTIONSPACETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistortionSpaceType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(DISTORTIONSPACETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DistortionSpaceType {
    fn type_info() -> &'static TypeInfo {
        DISTORTIONSPACETYPE_TYPE_INFO
    }
}


pub const DISTORTIONSPACETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DistortionSpaceType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("DistortionSpaceType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TransparentFogMode {
    #[default]
    TransparentFogMode_Nop = 0,
    TransparentFogMode_FadeOnly = 1,
    TransparentFogMode_FogOnly = 2,
    TransparentFogMode_FogAndFade = 3,
}

pub const TRANSPARENTFOGMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransparentFogMode",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(TRANSPARENTFOGMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TransparentFogMode {
    fn type_info() -> &'static TypeInfo {
        TRANSPARENTFOGMODE_TYPE_INFO
    }
}


pub const TRANSPARENTFOGMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransparentFogMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("TransparentFogMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderBranchMethod {
    #[default]
    SbmStatic = 0,
    SbmFlat = 1,
    SbmDynamicIfElse = 2,
    SbmDynamicIfTrue = 3,
    SbmDynamicIfFalse = 4,
}

pub const SHADERBRANCHMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBranchMethod",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERBRANCHMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderBranchMethod {
    fn type_info() -> &'static TypeInfo {
        SHADERBRANCHMETHOD_TYPE_INFO
    }
}


pub const SHADERBRANCHMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBranchMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderBranchMethod-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderComparisonOperator {
    #[default]
    ScoEquals = 0,
    ScoNotEquals = 1,
    ScoLess = 2,
    ScoLessEquals = 3,
    ScoGreater = 4,
    ScoGreaterEquals = 5,
    ScoNone = 6,
}

pub const SHADERCOMPARISONOPERATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderComparisonOperator",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERCOMPARISONOPERATOR_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderComparisonOperator {
    fn type_info() -> &'static TypeInfo {
        SHADERCOMPARISONOPERATOR_TYPE_INFO
    }
}


pub const SHADERCOMPARISONOPERATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderComparisonOperator-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderComparisonOperator-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum BlendShaderMode {
    #[default]
    BsmLerp = 0,
    BsmAdd = 1,
    BsmSubtract = 2,
    BsmMultiply = 3,
    BsmMultiply2x = 4,
    BsmScreen = 5,
    BsmDifference = 6,
    BsmLighten = 7,
    BsmDarken = 8,
    BsmOverlay = 9,
}

pub const BLENDSHADERMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlendShaderMode",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(BLENDSHADERMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BlendShaderMode {
    fn type_info() -> &'static TypeInfo {
        BLENDSHADERMODE_TYPE_INFO
    }
}


pub const BLENDSHADERMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlendShaderMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("BlendShaderMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CurveShaderType {
    #[default]
    CstSine = 0,
    CstSineNormalized = 1,
    CstSawtooth = 2,
    CstTriangle = 3,
    CstSquare = 4,
}

pub const CURVESHADERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveShaderType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(CURVESHADERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CurveShaderType {
    fn type_info() -> &'static TypeInfo {
        CURVESHADERTYPE_TYPE_INFO
    }
}


pub const CURVESHADERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveShaderType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("CurveShaderType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EyeVectorSpace {
    #[default]
    EyeVectorSpace_World = 0,
    EyeVectorSpace_Object = 1,
    EyeVectorSpace_Tangent = 2,
}

pub const EYEVECTORSPACE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EyeVectorSpace",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(EYEVECTORSPACE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EyeVectorSpace {
    fn type_info() -> &'static TypeInfo {
        EYEVECTORSPACE_TYPE_INFO
    }
}


pub const EYEVECTORSPACE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EyeVectorSpace-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("EyeVectorSpace-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PixelNormalSpace {
    #[default]
    PnsTangent = 0,
}

pub const PIXELNORMALSPACE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PixelNormalSpace",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(PIXELNORMALSPACE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PixelNormalSpace {
    fn type_info() -> &'static TypeInfo {
        PIXELNORMALSPACE_TYPE_INFO
    }
}


pub const PIXELNORMALSPACE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PixelNormalSpace-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("PixelNormalSpace-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum VertexNormalSpace {
    #[default]
    VnsObject = 0,
    VnsInstance = 1,
    VnsWorld = 2,
}

pub const VERTEXNORMALSPACE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexNormalSpace",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(VERTEXNORMALSPACE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VertexNormalSpace {
    fn type_info() -> &'static TypeInfo {
        VERTEXNORMALSPACE_TYPE_INFO
    }
}


pub const VERTEXNORMALSPACE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexNormalSpace-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("VertexNormalSpace-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderPositionSpace {
    #[default]
    ShaderPositionSpace_Object = 0,
    ShaderPositionSpace_World = 1,
}

pub const SHADERPOSITIONSPACE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderPositionSpace",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERPOSITIONSPACE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderPositionSpace {
    fn type_info() -> &'static TypeInfo {
        SHADERPOSITIONSPACE_TYPE_INFO
    }
}


pub const SHADERPOSITIONSPACE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderPositionSpace-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderPositionSpace-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderValueParameterType {
    #[default]
    SvptLiteral = 0,
    SvptExternalStatic = 1,
    SvptExternalPermutation = 2,
    SvptExternalConstant = 3,
    SvptExternalSubMaterial = 4,
}

pub const SHADERVALUEPARAMETERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderValueParameterType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERVALUEPARAMETERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderValueParameterType {
    fn type_info() -> &'static TypeInfo {
        SHADERVALUEPARAMETERTYPE_TYPE_INFO
    }
}


pub const SHADERVALUEPARAMETERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderValueParameterType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderValueParameterType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderPortType {
    #[default]
    SptBool = 0,
    SptUInteger = 1,
    SptUInt2 = 2,
    SptUInt3 = 3,
    SptUInt4 = 4,
    SptInteger = 5,
    SptInt2 = 6,
    SptInt3 = 7,
    SptInt4 = 8,
    SptScalar = 9,
    SptVec2 = 10,
    SptVec3 = 11,
    SptVec4 = 12,
    SptColor = 13,
}

pub const SHADERPORTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderPortType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERPORTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderPortType {
    fn type_info() -> &'static TypeInfo {
        SHADERPORTTYPE_TYPE_INFO
    }
}


pub const SHADERPORTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderPortType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderPortType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderGraph {
}

pub const SHADERGRAPH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderGraph",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SURFACESHADERBASEASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHADERGRAPH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderGraph {
    fn type_info() -> &'static TypeInfo {
        SHADERGRAPH_TYPE_INFO
    }
}


pub const SHADERGRAPH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderGraph-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderGraph-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderBufferDefinition {
    pub buffer_type: i32,
    pub parameter_name: String,
    pub is_raw: bool,
    pub raw_component_count: u32,
    pub typed_format: ShaderValueType,
}

pub const SHADERBUFFERDEFINITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBufferDefinition",
    flags: MemberInfoFlags::new(73),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BufferType",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ShaderBufferDefinition, buffer_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(ShaderBufferDefinition, parameter_name),
            },
            FieldInfoData {
                name: "IsRaw",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ShaderBufferDefinition, is_raw),
            },
            FieldInfoData {
                name: "RawComponentCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(ShaderBufferDefinition, raw_component_count),
            },
            FieldInfoData {
                name: "TypedFormat",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERVALUETYPE_TYPE_INFO,
                rust_offset: offset_of!(ShaderBufferDefinition, typed_format),
            },
        ],
    }),
    array_type: Some(SHADERBUFFERDEFINITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ShaderBufferDefinition {
    fn type_info() -> &'static TypeInfo {
        SHADERBUFFERDEFINITION_TYPE_INFO
    }
}


pub const SHADERBUFFERDEFINITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBufferDefinition-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderBufferDefinition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SurfaceShaderPreset {
    pub shader_preset: super::render_base::SurfaceShaderInstanceDataStruct,
}

pub const SURFACESHADERPRESET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceShaderPreset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SURFACESHADERBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ShaderPreset",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(SurfaceShaderPreset, shader_preset),
            },
        ],
    }),
    array_type: Some(SURFACESHADERPRESET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SurfaceShaderPreset {
    fn type_info() -> &'static TypeInfo {
        SURFACESHADERPRESET_TYPE_INFO
    }
}


pub const SURFACESHADERPRESET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceShaderPreset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("SurfaceShaderPreset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SurfaceShaderInstanceData {
    pub shader: super::render_base::SurfaceShaderBaseAsset,
}

pub const SURFACESHADERINSTANCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceShaderInstanceData",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(SurfaceShaderInstanceData, shader),
            },
        ],
    }),
    array_type: Some(SURFACESHADERINSTANCEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SurfaceShaderInstanceData {
    fn type_info() -> &'static TypeInfo {
        SURFACESHADERINSTANCEDATA_TYPE_INFO
    }
}


pub const SURFACESHADERINSTANCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceShaderInstanceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("SurfaceShaderInstanceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderTessellationType {
    #[default]
    ShaderTessellationType_None = 0,
    ShaderTessellationType_Phong = 1,
    ShaderTessellationType_DisplacementMappingPhong = 2,
    ShaderTessellationType_DisplacementMapping = 3,
}

pub const SHADERTESSELLATIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderTessellationType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERTESSELLATIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderTessellationType {
    fn type_info() -> &'static TypeInfo {
        SHADERTESSELLATIONTYPE_TYPE_INFO
    }
}


pub const SHADERTESSELLATIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderTessellationType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderTessellationType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TessellationShaderFragmentAsset {
    pub source_file: super::core::FileRef,
    pub h_s_control_point_function_name: String,
    pub h_s_patch_constant_function_name: String,
    pub d_s_function_name: String,
    pub v_s_output_struct_name: String,
    pub h_s_control_point_output_struct_name: String,
    pub h_s_patch_constant_output_struct_name: String,
    pub d_s_output_struct_name: String,
    pub definitions: Vec<TessellationShaderFragmentDefinition>,
}

pub const TESSELLATIONSHADERFRAGMENTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TessellationShaderFragmentAsset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SourceFile",
                flags: MemberInfoFlags::new(0),
                field_type: FILEREF_TYPE_INFO,
                rust_offset: offset_of!(TessellationShaderFragmentAsset, source_file),
            },
            FieldInfoData {
                name: "HSControlPointFunctionName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TessellationShaderFragmentAsset, h_s_control_point_function_name),
            },
            FieldInfoData {
                name: "HSPatchConstantFunctionName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TessellationShaderFragmentAsset, h_s_patch_constant_function_name),
            },
            FieldInfoData {
                name: "DSFunctionName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TessellationShaderFragmentAsset, d_s_function_name),
            },
            FieldInfoData {
                name: "VSOutputStructName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TessellationShaderFragmentAsset, v_s_output_struct_name),
            },
            FieldInfoData {
                name: "HSControlPointOutputStructName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TessellationShaderFragmentAsset, h_s_control_point_output_struct_name),
            },
            FieldInfoData {
                name: "HSPatchConstantOutputStructName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TessellationShaderFragmentAsset, h_s_patch_constant_output_struct_name),
            },
            FieldInfoData {
                name: "DSOutputStructName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TessellationShaderFragmentAsset, d_s_output_struct_name),
            },
            FieldInfoData {
                name: "Definitions",
                flags: MemberInfoFlags::new(144),
                field_type: TESSELLATIONSHADERFRAGMENTDEFINITION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TessellationShaderFragmentAsset, definitions),
            },
        ],
    }),
    array_type: Some(TESSELLATIONSHADERFRAGMENTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TessellationShaderFragmentAsset {
    fn type_info() -> &'static TypeInfo {
        TESSELLATIONSHADERFRAGMENTASSET_TYPE_INFO
    }
}


pub const TESSELLATIONSHADERFRAGMENTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TessellationShaderFragmentAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("TessellationShaderFragmentAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TessellationShaderFragmentDefinition {
    pub definition: String,
    pub enabled: bool,
}

pub const TESSELLATIONSHADERFRAGMENTDEFINITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TessellationShaderFragmentDefinition",
    flags: MemberInfoFlags::new(73),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Definition",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TessellationShaderFragmentDefinition, definition),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TessellationShaderFragmentDefinition, enabled),
            },
        ],
    }),
    array_type: Some(TESSELLATIONSHADERFRAGMENTDEFINITION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TessellationShaderFragmentDefinition {
    fn type_info() -> &'static TypeInfo {
        TESSELLATIONSHADERFRAGMENTDEFINITION_TYPE_INFO
    }
}


pub const TESSELLATIONSHADERFRAGMENTDEFINITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TessellationShaderFragmentDefinition-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("TessellationShaderFragmentDefinition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VertexShaderFragmentAsset {
    pub source_file: super::core::FileRef,
    pub function: String,
    pub pipeline_generated_source_code: String,
    pub extra_input_files: Vec<String>,
    pub extra_input_assets: Vec<String>,
}

pub const VERTEXSHADERFRAGMENTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderFragmentAsset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SourceFile",
                flags: MemberInfoFlags::new(0),
                field_type: FILEREF_TYPE_INFO,
                rust_offset: offset_of!(VertexShaderFragmentAsset, source_file),
            },
            FieldInfoData {
                name: "Function",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VertexShaderFragmentAsset, function),
            },
            FieldInfoData {
                name: "PipelineGeneratedSourceCode",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VertexShaderFragmentAsset, pipeline_generated_source_code),
            },
            FieldInfoData {
                name: "ExtraInputFiles",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(VertexShaderFragmentAsset, extra_input_files),
            },
            FieldInfoData {
                name: "ExtraInputAssets",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(VertexShaderFragmentAsset, extra_input_assets),
            },
        ],
    }),
    array_type: Some(VERTEXSHADERFRAGMENTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VertexShaderFragmentAsset {
    fn type_info() -> &'static TypeInfo {
        VERTEXSHADERFRAGMENTASSET_TYPE_INFO
    }
}


pub const VERTEXSHADERFRAGMENTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexShaderFragmentAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("VertexShaderFragmentAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DispatchBlendMode {
    #[default]
    DispatchBlendMode_NoBlend = 0,
    DispatchBlendMode_ZOnly = 1,
    DispatchBlendMode_DebugOverdraw = 2,
    DispatchBlendMode_DebugShaderCost = 3,
    DispatchBlendMode_Lerp = 4,
    DispatchBlendMode_Additive = 5,
    DispatchBlendMode_Multiply = 6,
    DispatchBlendMode_DualSourceColor = 7,
    DispatchBlendMode_TransparentDofLerp = 8,
    DispatchBlendMode_TransparentDofAdditive = 9,
    DispatchBlendMode_TransparentDofMultiply = 10,
    DispatchBlendMode_DecalLerpNormal = 11,
    DispatchBlendMode_DecalLerpSmoothness = 12,
    DispatchBlendMode_DecalMultiplySmoothness = 13,
    DispatchBlendMode_DecalAddSmoothness = 14,
    DispatchBlendMode_DecalLerpNormalSmoothness = 15,
    DispatchBlendMode_DecalLerpReflectance = 16,
    DispatchBlendMode_DecalMultiplyReflectance = 17,
    DispatchBlendMode_DecalAddReflectance = 18,
    DispatchBlendMode_DecalLerpMetalMask = 19,
    DispatchBlendMode_DecalMultiplyMetalMask = 20,
    DispatchBlendMode_DecalAddMetalMask = 21,
    DispatchBlendMode_DecalLerpBaseColor = 22,
    DispatchBlendMode_DecalMultiplyBaseColor = 23,
    DispatchBlendMode_DecalAddBaseColor = 24,
    DispatchBlendMode_DecalLerpBaseColorMetalMaskReflectance = 25,
    DispatchBlendMode_DecalMultiplyBaseColorMetalMaskReflectance = 26,
    DispatchBlendMode_DecalLerpNormalSmoothnessMultiplyOthers = 27,
    DispatchBlendMode_DecalLerpNormalSmoothnessMultiplyOthersAddEmissive = 28,
    DispatchBlendMode_DecalLerpEverythingAddEmissive = 29,
    DispatchBlendMode_DecalAddBaseColorAddEmissive = 30,
    DispatchBlendMode_DecalLerpEverything = 31,
    DispatchBlendModeCount = 32,
}

pub const DISPATCHBLENDMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DispatchBlendMode",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(DISPATCHBLENDMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DispatchBlendMode {
    fn type_info() -> &'static TypeInfo {
        DISPATCHBLENDMODE_TYPE_INFO
    }
}


pub const DISPATCHBLENDMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DispatchBlendMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("DispatchBlendMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderConstantFunction {
    #[default]
    ShaderConstantFunction_ViewMatrix = 0,
    ShaderConstantFunction_ViewProjMatrix = 1,
    ShaderConstantFunction_WorldViewMatrix = 2,
    ShaderConstantFunction_WorldViewProjMatrix = 3,
    ShaderConstantFunction_CrViewProjMatrix = 4,
    ShaderConstantFunction_CrWorldViewProjMatrix = 5,
    ShaderConstantFunction_CameraPos = 6,
    ShaderConstantFunction_ViewSettings = 7,
    ShaderConstantFunction_ExposureMultipliers = 8,
    ShaderConstantFunction_PrevViewProjMatrix = 9,
    ShaderConstantFunction_CrPrevViewProjMatrix = 10,
    ShaderConstantFunction_PrevCameraPos = 11,
    ShaderConstantFunction_PrevWorldViewProjMatrix = 12,
    ShaderConstantFunction_WorldMatrix = 13,
    ShaderConstantFunction_PrevWorldMatrix = 14,
    ShaderConstantFunction_LightProbe = 15,
    ShaderConstantFunction_LightMapUvTransform = 16,
    ShaderConstantFunction_LightMapUvTranslation = 17,
    ShaderConstantFunction_LightMapIrradianceTexture = 18,
    ShaderConstantFunction_LightMapDirectionTexture = 19,
    ShaderConstantFunction_LightMapDirectionTextureG = 20,
    ShaderConstantFunction_LightMapDirectionTextureB = 21,
    ShaderConstantFunction_LightMapSkyVisibilityTexture = 22,
    ShaderConstantFunction_SecondaryLightMapIrradianceTexture = 23,
    ShaderConstantFunction_SecondaryLightMapDirectionTexture = 24,
    ShaderConstantFunction_SecondaryLightMapDirectionTextureG = 25,
    ShaderConstantFunction_SecondaryLightMapDirectionTextureB = 26,
    ShaderConstantFunction_SecondaryLightMapSkyVisibilityTexture = 27,
    ShaderConstantFunction_LightMapPreExposure = 28,
    ShaderConstantFunction_DepthBufferTexture = 29,
    ShaderConstantFunction_NormalBufferTexture = 30,
    ShaderConstantFunction_ProjectionKxKyKzKw = 31,
    ShaderConstantFunction_Time = 32,
    ShaderConstantFunction_ScreenSize = 33,
    ShaderConstantFunction_OutdoorLightDir = 34,
    ShaderConstantFunction_OutdoorLightAngularRadius = 35,
    ShaderConstantFunction_OutdoorLightHemisphereDir = 36,
    ShaderConstantFunction_OutdoorLightKeyLuminance = 37,
    ShaderConstantFunction_OutdoorLightIlluminanceAndSpecularScale = 38,
    ShaderConstantFunction_OutdoorLightKeySpecularColorAndHemisphereVisibility = 39,
    ShaderConstantFunction_OutdoorLightTopColor = 40,
    ShaderConstantFunction_OutdoorLightBottomColor = 41,
    ShaderConstantFunction_OutdoorLightShadowTransform = 42,
    ShaderConstantFunction_OutdoorLightShadowTransformArray = 43,
    ShaderConstantFunction_OutdoorLightShadowTransformArrayRange = 44,
    ShaderConstantFunction_OutdoorLightShadowmapSizeAndInvSize = 45,
    ShaderConstantFunction_OutdoorLightShadowmapTexture = 46,
    ShaderConstantFunction_OutdoorLightShadowmapTextureArray = 47,
    ShaderConstantFunction_OutdoorLightSkyEnvmap = 48,
    ShaderConstantFunction_OutdoorLightCustomEnvmap = 49,
    ShaderConstantFunction_OutdoorLightDynamicEnvmap = 50,
    ShaderConstantFunction_OutdoorLightTopColorEnvmap = 51,
    ShaderConstantFunction_OutdoorLightBottomColorEnvmap = 52,
    ShaderConstantFunction_OutdoorLightKeyIlluminanceEnvmap = 53,
    ShaderConstantFunction_OutdoorLightEnvmapsMipmapCount = 54,
    ShaderConstantFunction_OutdoorLightSkyEnvmap8BitTexInvScale = 55,
    ShaderConstantFunction_OutdoorLightCustomEnvmapScaleAmbientAndSkyVisExponent = 56,
    ShaderConstantFunction_OutdoorLightCloudShadowType = 57,
    ShaderConstantFunction_OutdoorLightCloudShadowOffsetScaleExponentAndFade = 58,
    ShaderConstantFunction_OutdoorLightCloudShadowTexture = 59,
    ShaderConstantFunction_OutdoorLightSecondaryCloudShadowOffsetScaleExponent = 60,
    ShaderConstantFunction_OutdoorLightSecondaryCloudShadowTexture = 61,
    ShaderConstantFunction_PointLightCount = 62,
    ShaderConstantFunction_PointLightsPositionAndRadius = 63,
    ShaderConstantFunction_PointLightsColorAndAttenuation = 64,
    ShaderConstantFunction_SpotLightCount = 65,
    ShaderConstantFunction_SpotLightsPositionAndRadius = 66,
    ShaderConstantFunction_SpotLightsColorAndAttenuation = 67,
    ShaderConstantFunction_SpotLightsDirection = 68,
    ShaderConstantFunction_SpotLightsConeAngles = 69,
    ShaderConstantFunction_TransparentStartAndSlopeAndClamp = 70,
    ShaderConstantFunction_TransparentCurve = 71,
    ShaderConstantFunction_FogParams = 72,
    ShaderConstantFunction_FogForwardScatteringParamsLuminanceScaleFogEnable = 73,
    ShaderConstantFunction_FogForwardScatteringColorPresence = 74,
    ShaderConstantFunction_FogForwardScatteringSunDir = 75,
    ShaderConstantFunction_FogCoefficients = 76,
    ShaderConstantFunction_FogColorCoefficients = 77,
    ShaderConstantFunction_FogColor = 78,
    ShaderConstantFunction_FogStartDistance = 79,
    ShaderConstantFunction_FogHeightFogCoefficients = 80,
    ShaderConstantFunction_FogMiscParam = 81,
    ShaderConstantFunction_FogEnabledModeSkyModeUseLight2 = 82,
    ShaderConstantFunction_FogSkyGradientUVRanges = 83,
    ShaderConstantFunction_MieGMaxDistanceTransTexDepthMieCoef = 84,
    ShaderConstantFunction_Light0Dir = 85,
    ShaderConstantFunction_Light1Dir = 86,
    ShaderConstantFunction_RayleighScatteringCoefficient = 87,
    ShaderConstantFunction_RayleighPolarizationFilter = 88,
    ShaderConstantFunction_MiePolarizationFilter = 89,
    ShaderConstantFunction_HeightFogColorMulMinTransmittance = 90,
    ShaderConstantFunction_HeightFogColorAdd = 91,
    ShaderConstantFunction_HorizonScatteringTexture = 92,
    ShaderConstantFunction_AerialPerspectiveTexture1 = 93,
    ShaderConstantFunction_AerialPerspectiveTexture2 = 94,
    ShaderConstantFunction_TransmittanceTexture = 95,
    ShaderConstantFunction_AcosLutTexture = 96,
    ShaderConstantFunction_FogSkyGradientTexture = 97,
    ShaderConstantFunction_FogSkyGradientBlendTexture = 98,
    ShaderConstantFunction_LightShaftTexture = 99,
    ShaderConstantFunction_CloudAerialPerspectiveTexture = 100,
    ShaderConstantFunction_FogExclusionVolumeSum = 101,
    ShaderConstantFunction_FogExclusionVolumeMin = 102,
    ShaderConstantFunction_EmitterDirectionalTransmittanceTexture = 103,
    ShaderConstantFunction_ParticipatingMediaCameraVolumeTexture = 104,
    ShaderConstantFunction_ParticipatingMediaCameraVolumeDepthScaleFactor = 105,
    ShaderConstantFunction_ParticipatingMediaEnableUvScaleInvDepthRange = 106,
    ShaderConstantFunction_ParticipatingMediaCameraViewProjMatrix = 107,
    ShaderConstantFunction_SceneTexture = 108,
    ShaderConstantFunction_NormalBasisGBufferTexture = 109,
    ShaderConstantFunction_NormalBasisTransforms = 110,
    ShaderConstantFunction_DxVectorBufferOffsets = 111,
    ShaderConstantFunction_DistortionMaxValue = 112,
    ShaderConstantFunction_ObjectScale = 113,
    ShaderConstantFunction_ObjectWorldPosition = 114,
    ShaderConstantFunction_TessellationScreenSize = 115,
    ShaderConstantFunction_TessellationCrViewProjMatrix = 116,
    ShaderConstantFunction_TessellationCameraPos = 117,
    ShaderConstantFunction_UserData0 = 118,
    ShaderConstantFunction_UserData1 = 119,
    ShaderConstantFunction_PrevUserData0 = 120,
    ShaderConstantFunction_ForwardShadingPreIntegratedDL = 121,
    ShaderConstantFunction_ForwardShadingNoBackdropPreIntegratedDL = 122,
    ShaderConstantFunction_ForwardShadingPreIntegratedFG = 123,
    ShaderConstantFunction_ForwardShadingIBLConstant = 124,
    ShaderConstantFunction_ForwardShadingLocalIBLTextureArray = 125,
    ShaderConstantFunction_ForwardShadingLocalIBLDataArray = 126,
    ShaderConstantFunction_ForwardShadingPunctualLightDataArray = 127,
    ShaderConstantFunction_ForwardShadingPunctualShadowLightDataArray = 128,
    ShaderConstantFunction_ForwardShadingAreaLightDataArray = 129,
    ShaderConstantFunction_ForwardShadingAreaShadowLightDataArray = 130,
    ShaderConstantFunction_ForwardShadingIESTextureArray = 131,
    ShaderConstantFunction_ForwardShadingLightConstants = 132,
    ShaderConstantFunction_ForwardShadingLightGridBuffer = 133,
    ShaderConstantFunction_ForwardShadingLightIndexBuffer = 134,
    ShaderConstantFunction_ForwardShadingLocalPRTextureArray = 135,
    ShaderConstantFunction_ForwardShadingLocalPRDepthTextureArray = 136,
    ShaderConstantFunction_ForwardShadingLocalPRDataArray = 137,
    ShaderConstantFunction_DistantShadowCacheCount = 138,
    ShaderConstantFunction_DistantShadowCacheTexture = 139,
    ShaderConstantFunction_DistantShadowCacheSizeAndInvSize = 140,
    ShaderConstantFunction_DistantShadowCacheTransform = 141,
    ShaderConstantFunction_ForwardShadingLocalLightShadowAtlasTexture = 142,
    ShaderConstantFunction_ForwardShadingLocalLightShadowAtlasSizeAndInvSize = 143,
    ShaderConstantFunction_ForwardReflectionTexture = 144,
    ShaderConstantFunction_ForwardReflectionDepthBufferTexture = 145,
    ShaderConstantFunction_LodFade = 146,
    ShaderConstantFunction_DepthBiasValues = 147,
    ShaderConstantFunction_MeshId = 148,
    ShaderConstantFunction_GradientAdjust = 149,
    ShaderConstantFunction_AutoCheckerboardGradientAdjust = 150,
    ShaderConstantFunction_SkinningBuffer = 151,
    ShaderConstantFunction_PrevSkinningBuffer = 152,
    ShaderConstantFunction_SkinningOffsets = 153,
    ShaderConstantFunction_RaytraceConstants = 154,
    ShaderConstantFunction_CsSkinningParams = 155,
    ShaderConstantFunction_CsSkinningBuffer = 156,
    ShaderConstantFunction_CsSkinningRwBuffer = 157,
    ShaderConstantFunction_IndexBuffer = 158,
    ShaderConstantFunction_VertexBuffer0 = 159,
    ShaderConstantFunction_VertexBuffer1 = 160,
    ShaderConstantFunction_VertexBuffer2 = 161,
    ShaderConstantFunction_VertexBuffer3 = 162,
    ShaderConstantFunction_VertexBuffer4 = 163,
    ShaderConstantFunction_VertexBuffer5 = 164,
    ShaderConstantFunction_VertexBuffer6 = 165,
    ShaderConstantFunction_VertexBuffer7 = 166,
    ShaderConstantFunctionCount = 167,
}

pub const SHADERCONSTANTFUNCTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderConstantFunction",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERCONSTANTFUNCTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderConstantFunction {
    fn type_info() -> &'static TypeInfo {
        SHADERCONSTANTFUNCTION_TYPE_INFO
    }
}


pub const SHADERCONSTANTFUNCTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderConstantFunction-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderConstantFunction-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RvmSystemSettings {
    pub default_quality_level: super::core::QualityLevel,
    pub default_sink_batch_range_count: u32,
    pub default_sink_context_reorder_count: u32,
    pub default_sink_sort_method: u32,
    pub default_sink_batch_range_distance: u64,
    pub default_sink_context_reorder_distance: u64,
    pub max_cache_growth_per_frame: u32,
    pub load_debug_databases: bool,
    pub legacy_validation_enabled: bool,
    pub global_caching_enabled: bool,
    pub local_caching_enabled: bool,
    pub global_caching_force_enabled: bool,
    pub local_caching_force_enabled: bool,
    pub analyzer_early_validate_enabled: bool,
    pub analyzer_error_is_fatal: bool,
    pub live_edit_skip_initial_load: bool,
    pub batch_execution_queue_timeslice_length_ms: f32,
    pub pre_cache_warmup_frame_count: u16,
    pub cleanup_step_count: u32,
    pub dispatch_batch_size: u32,
}

pub const RVMSYSTEMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSystemSettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DefaultQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, default_quality_level),
            },
            FieldInfoData {
                name: "DefaultSinkBatchRangeCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, default_sink_batch_range_count),
            },
            FieldInfoData {
                name: "DefaultSinkContextReorderCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, default_sink_context_reorder_count),
            },
            FieldInfoData {
                name: "DefaultSinkSortMethod",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, default_sink_sort_method),
            },
            FieldInfoData {
                name: "DefaultSinkBatchRangeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, default_sink_batch_range_distance),
            },
            FieldInfoData {
                name: "DefaultSinkContextReorderDistance",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, default_sink_context_reorder_distance),
            },
            FieldInfoData {
                name: "MaxCacheGrowthPerFrame",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, max_cache_growth_per_frame),
            },
            FieldInfoData {
                name: "LoadDebugDatabases",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, load_debug_databases),
            },
            FieldInfoData {
                name: "LegacyValidationEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, legacy_validation_enabled),
            },
            FieldInfoData {
                name: "GlobalCachingEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, global_caching_enabled),
            },
            FieldInfoData {
                name: "LocalCachingEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, local_caching_enabled),
            },
            FieldInfoData {
                name: "GlobalCachingForceEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, global_caching_force_enabled),
            },
            FieldInfoData {
                name: "LocalCachingForceEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, local_caching_force_enabled),
            },
            FieldInfoData {
                name: "AnalyzerEarlyValidateEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, analyzer_early_validate_enabled),
            },
            FieldInfoData {
                name: "AnalyzerErrorIsFatal",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, analyzer_error_is_fatal),
            },
            FieldInfoData {
                name: "LiveEditSkipInitialLoad",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, live_edit_skip_initial_load),
            },
            FieldInfoData {
                name: "BatchExecutionQueueTimesliceLengthMs",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, batch_execution_queue_timeslice_length_ms),
            },
            FieldInfoData {
                name: "PreCacheWarmupFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, pre_cache_warmup_frame_count),
            },
            FieldInfoData {
                name: "CleanupStepCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, cleanup_step_count),
            },
            FieldInfoData {
                name: "DispatchBatchSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RvmSystemSettings, dispatch_batch_size),
            },
        ],
    }),
    array_type: Some(RVMSYSTEMSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmSystemSettings {
    fn type_info() -> &'static TypeInfo {
        RVMSYSTEMSETTINGS_TYPE_INFO
    }
}


pub const RVMSYSTEMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSystemSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RvmSystemSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmLegacyLightMapInstance {
}

pub const RVMLEGACYLIGHTMAPINSTANCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyLightMapInstance",
    flags: MemberInfoFlags::new(53321),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMLEGACYLIGHTMAPINSTANCE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmLegacyLightMapInstance {
    fn type_info() -> &'static TypeInfo {
        RVMLEGACYLIGHTMAPINSTANCE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmLegacyLightProbes {
}

pub const RVMLEGACYLIGHTPROBES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyLightProbes",
    flags: MemberInfoFlags::new(53321),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMLEGACYLIGHTPROBES_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmLegacyLightProbes {
    fn type_info() -> &'static TypeInfo {
        RVMLEGACYLIGHTPROBES_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmLegacyPermutationDebugInfo {
}

pub const RVMLEGACYPERMUTATIONDEBUGINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyPermutationDebugInfo",
    flags: MemberInfoFlags::new(53321),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMLEGACYPERMUTATIONDEBUGINFO_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmLegacyPermutationDebugInfo {
    fn type_info() -> &'static TypeInfo {
        RVMLEGACYPERMUTATIONDEBUGINFO_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmLegacyForwardLightState {
}

pub const RVMLEGACYFORWARDLIGHTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyForwardLightState",
    flags: MemberInfoFlags::new(53321),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMLEGACYFORWARDLIGHTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RvmLegacyForwardLightState {
    fn type_info() -> &'static TypeInfo {
        RVMLEGACYFORWARDLIGHTSTATE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FrustumSoA {
}

pub const FRUSTUMSOA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FrustumSoA",
    flags: MemberInfoFlags::new(53321),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(FRUSTUMSOA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FrustumSoA {
    fn type_info() -> &'static TypeInfo {
        FRUSTUMSOA_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LodViewState {
}

pub const LODVIEWSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LodViewState",
    flags: MemberInfoFlags::new(53321),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(LODVIEWSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for LodViewState {
    fn type_info() -> &'static TypeInfo {
        LODVIEWSTATE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TessellationViewState {
}

pub const TESSELLATIONVIEWSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TessellationViewState",
    flags: MemberInfoFlags::new(53321),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(TESSELLATIONVIEWSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TessellationViewState {
    fn type_info() -> &'static TypeInfo {
        TESSELLATIONVIEWSTATE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StencilState {
}

pub const STENCILSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StencilState",
    flags: MemberInfoFlags::new(53321),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(STENCILSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for StencilState {
    fn type_info() -> &'static TypeInfo {
        STENCILSTATE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReflectionState {
}

pub const REFLECTIONSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ReflectionState",
    flags: MemberInfoFlags::new(53321),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(REFLECTIONSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ReflectionState {
    fn type_info() -> &'static TypeInfo {
        REFLECTIONSTATE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FogState {
}

pub const FOGSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FogState",
    flags: MemberInfoFlags::new(53321),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(FOGSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FogState {
    fn type_info() -> &'static TypeInfo {
        FOGSTATE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProjectionState {
}

pub const PROJECTIONSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProjectionState",
    flags: MemberInfoFlags::new(53321),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(PROJECTIONSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ProjectionState {
    fn type_info() -> &'static TypeInfo {
        PROJECTIONSTATE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ViewState {
}

pub const VIEWSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ViewState",
    flags: MemberInfoFlags::new(53321),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(VIEWSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ViewState {
    fn type_info() -> &'static TypeInfo {
        VIEWSTATE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OutdoorLightState {
}

pub const OUTDOORLIGHTSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OutdoorLightState",
    flags: MemberInfoFlags::new(53321),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(OUTDOORLIGHTSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for OutdoorLightState {
    fn type_info() -> &'static TypeInfo {
        OUTDOORLIGHTSTATE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmLegacyDatabase {
}

pub const RVMLEGACYDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyDatabase",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RVMDATABASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RVMLEGACYDATABASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RvmLegacyDatabase {
    fn type_info() -> &'static TypeInfo {
        RVMLEGACYDATABASE_TYPE_INFO
    }
}


pub const RVMLEGACYDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RvmLegacyDatabase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmDatabaseLoader {
}

pub const RVMDATABASELOADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDatabaseLoader",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(RVMDATABASELOADER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RvmDatabaseLoader {
    fn type_info() -> &'static TypeInfo {
        RVMDATABASELOADER_TYPE_INFO
    }
}


pub const RVMDATABASELOADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDatabaseLoader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RvmDatabaseLoader-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IRaytraceSystem {
}

pub const IRAYTRACESYSTEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IRaytraceSystem",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(IRAYTRACESYSTEM_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IRaytraceSystem {
    fn type_info() -> &'static TypeInfo {
        IRAYTRACESYSTEM_TYPE_INFO
    }
}


pub const IRAYTRACESYSTEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IRaytraceSystem-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("IRaytraceSystem-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IRaytraceSceneBuilder {
}

pub const IRAYTRACESCENEBUILDER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IRaytraceSceneBuilder",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(IRAYTRACESCENEBUILDER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IRaytraceSceneBuilder {
    fn type_info() -> &'static TypeInfo {
        IRAYTRACESCENEBUILDER_TYPE_INFO
    }
}


pub const IRAYTRACESCENEBUILDER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IRaytraceSceneBuilder-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("IRaytraceSceneBuilder-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RvmStatsSettings {
    pub frame_view_enabled: bool,
    pub frame_view_draw_graph: bool,
    pub frame_view_timers: String,
    pub frame_view_pos_x: i32,
    pub frame_view_pos_y: i32,
    pub frame_view_text_scale: f32,
    pub frame_view_graph_scale: f32,
    pub frame_view_background_opacity: f32,
    pub sink_view_enabled: bool,
    pub sink_view_timers: String,
    pub sink_view_filter_by: String,
    pub sink_view_filter_name: String,
    pub sink_view_sort_by: String,
    pub sink_view_pos_x: i32,
    pub sink_view_pos_y: i32,
    pub sink_view_text_scale: f32,
    pub sink_view_background_opacity: f32,
    pub permutation_filter: String,
    pub program_filter: String,
    pub capture_frame_count: i32,
    pub draw_uncached_count: i32,
}

pub const RVMSTATSSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmStatsSettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FrameViewEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, frame_view_enabled),
            },
            FieldInfoData {
                name: "FrameViewDrawGraph",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, frame_view_draw_graph),
            },
            FieldInfoData {
                name: "FrameViewTimers",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, frame_view_timers),
            },
            FieldInfoData {
                name: "FrameViewPosX",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, frame_view_pos_x),
            },
            FieldInfoData {
                name: "FrameViewPosY",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, frame_view_pos_y),
            },
            FieldInfoData {
                name: "FrameViewTextScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, frame_view_text_scale),
            },
            FieldInfoData {
                name: "FrameViewGraphScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, frame_view_graph_scale),
            },
            FieldInfoData {
                name: "FrameViewBackgroundOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, frame_view_background_opacity),
            },
            FieldInfoData {
                name: "SinkViewEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, sink_view_enabled),
            },
            FieldInfoData {
                name: "SinkViewTimers",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, sink_view_timers),
            },
            FieldInfoData {
                name: "SinkViewFilterBy",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, sink_view_filter_by),
            },
            FieldInfoData {
                name: "SinkViewFilterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, sink_view_filter_name),
            },
            FieldInfoData {
                name: "SinkViewSortBy",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, sink_view_sort_by),
            },
            FieldInfoData {
                name: "SinkViewPosX",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, sink_view_pos_x),
            },
            FieldInfoData {
                name: "SinkViewPosY",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, sink_view_pos_y),
            },
            FieldInfoData {
                name: "SinkViewTextScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, sink_view_text_scale),
            },
            FieldInfoData {
                name: "SinkViewBackgroundOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, sink_view_background_opacity),
            },
            FieldInfoData {
                name: "PermutationFilter",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, permutation_filter),
            },
            FieldInfoData {
                name: "ProgramFilter",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, program_filter),
            },
            FieldInfoData {
                name: "CaptureFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, capture_frame_count),
            },
            FieldInfoData {
                name: "DrawUncachedCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RvmStatsSettings, draw_uncached_count),
            },
        ],
    }),
    array_type: Some(RVMSTATSSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmStatsSettings {
    fn type_info() -> &'static TypeInfo {
        RVMSTATSSETTINGS_TYPE_INFO
    }
}


pub const RVMSTATSSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmStatsSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RvmStatsSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmViewportRect {
    pub left: u16,
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
}

pub const RVMVIEWPORTRECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmViewportRect",
    flags: MemberInfoFlags::new(36937),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Left",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(RvmViewportRect, left),
            },
            FieldInfoData {
                name: "Top",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(RvmViewportRect, top),
            },
            FieldInfoData {
                name: "Right",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(RvmViewportRect, right),
            },
            FieldInfoData {
                name: "Bottom",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(RvmViewportRect, bottom),
            },
        ],
    }),
    array_type: Some(RVMVIEWPORTRECT_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmViewportRect {
    fn type_info() -> &'static TypeInfo {
        RVMVIEWPORTRECT_TYPE_INFO
    }
}


pub const RVMVIEWPORTRECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmViewportRect-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RvmViewportRect-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RvmIndexBufferFormat {
    #[default]
    RvmIndexBufferFormat_U16 = 0,
    RvmIndexBufferFormat_U32 = 1,
}

pub const RVMINDEXBUFFERFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmIndexBufferFormat",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RVMINDEXBUFFERFORMAT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmIndexBufferFormat {
    fn type_info() -> &'static TypeInfo {
        RVMINDEXBUFFERFORMAT_TYPE_INFO
    }
}


pub const RVMINDEXBUFFERFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmIndexBufferFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RvmIndexBufferFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RvmLevelOfDetail {
    #[default]
    RvmLevelOfDetail_Low = 0,
    RvmLevelOfDetail_High = 1,
}

pub const RVMLEVELOFDETAIL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLevelOfDetail",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RVMLEVELOFDETAIL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmLevelOfDetail {
    fn type_info() -> &'static TypeInfo {
        RVMLEVELOFDETAIL_TYPE_INFO
    }
}


pub const RVMLEVELOFDETAIL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLevelOfDetail-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RvmLevelOfDetail-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RvmLegacyOutdoorLightStatus {
    #[default]
    RvmLegacyOutdoorLightStatus_Disabled = 0,
    RvmLegacyOutdoorLightStatus_NoMethod_LowQuality = 1,
    RvmLegacyOutdoorLightStatus_NoMethod_HighQuality = 2,
    RvmLegacyOutdoorLightStatus_CascadedBoxMethod_LowQuality = 3,
    RvmLegacyOutdoorLightStatus_CascadedBoxMethod_HighQuality = 4,
    RvmLegacyOutdoorLightStatusCount = 5,
}

pub const RVMLEGACYOUTDOORLIGHTSTATUS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyOutdoorLightStatus",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RVMLEGACYOUTDOORLIGHTSTATUS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RvmLegacyOutdoorLightStatus {
    fn type_info() -> &'static TypeInfo {
        RVMLEGACYOUTDOORLIGHTSTATUS_TYPE_INFO
    }
}


pub const RVMLEGACYOUTDOORLIGHTSTATUS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmLegacyOutdoorLightStatus-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RvmLegacyOutdoorLightStatus-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmDebugDatabaseCollection {
    pub debug_database_bundles: Vec<super::core::Asset>,
}

pub const RVMDEBUGDATABASECOLLECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDebugDatabaseCollection",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DebugDatabaseBundles",
                flags: MemberInfoFlags::new(144),
                field_type: ASSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(RvmDebugDatabaseCollection, debug_database_bundles),
            },
        ],
    }),
    array_type: Some(RVMDEBUGDATABASECOLLECTION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmDebugDatabaseCollection {
    fn type_info() -> &'static TypeInfo {
        RVMDEBUGDATABASECOLLECTION_TYPE_INFO
    }
}


pub const RVMDEBUGDATABASECOLLECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDebugDatabaseCollection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RvmDebugDatabaseCollection-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmDebugDatabaseAsset {
    pub original_asset_name: String,
    pub resource_type_name: String,
    pub cookie_data_video_size: u32,
    pub cookie_data_system_size: u32,
    pub cookie_data_fixup_size: u32,
    pub cookie_data_hash: u32,
    pub named_blob_hash: u64,
}

pub const RVMDEBUGDATABASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDebugDatabaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "OriginalAssetName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(RvmDebugDatabaseAsset, original_asset_name),
            },
            FieldInfoData {
                name: "ResourceTypeName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(RvmDebugDatabaseAsset, resource_type_name),
            },
            FieldInfoData {
                name: "CookieDataVideoSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RvmDebugDatabaseAsset, cookie_data_video_size),
            },
            FieldInfoData {
                name: "CookieDataSystemSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RvmDebugDatabaseAsset, cookie_data_system_size),
            },
            FieldInfoData {
                name: "CookieDataFixupSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RvmDebugDatabaseAsset, cookie_data_fixup_size),
            },
            FieldInfoData {
                name: "CookieDataHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RvmDebugDatabaseAsset, cookie_data_hash),
            },
            FieldInfoData {
                name: "NamedBlobHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(RvmDebugDatabaseAsset, named_blob_hash),
            },
        ],
    }),
    array_type: Some(RVMDEBUGDATABASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmDebugDatabaseAsset {
    fn type_info() -> &'static TypeInfo {
        RVMDEBUGDATABASEASSET_TYPE_INFO
    }
}


pub const RVMDEBUGDATABASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDebugDatabaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RvmDebugDatabaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmDatabaseAsset {
}

pub const RVMDATABASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDatabaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RVMDATABASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RvmDatabaseAsset {
    fn type_info() -> &'static TypeInfo {
        RVMDATABASEASSET_TYPE_INFO
    }
}


pub const RVMDATABASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDatabaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RvmDatabaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmSlotHandle {
}

pub const RVMSLOTHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSlotHandle",
    flags: MemberInfoFlags::new(36937),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RVMSLOTHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for RvmSlotHandle {
    fn type_info() -> &'static TypeInfo {
        RVMSLOTHANDLE_TYPE_INFO
    }
}


pub const RVMSLOTHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmSlotHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RvmSlotHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RtRvmRaytraceStateObject {
}

pub const RTRVMRAYTRACESTATEOBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RtRvmRaytraceStateObject",
    flags: MemberInfoFlags::new(36937),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RTRVMRAYTRACESTATEOBJECT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RtRvmRaytraceStateObject {
    fn type_info() -> &'static TypeInfo {
        RTRVMRAYTRACESTATEOBJECT_TYPE_INFO
    }
}


pub const RTRVMRAYTRACESTATEOBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RtRvmRaytraceStateObject-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RtRvmRaytraceStateObject-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RtRvmRaytraceScene {
}

pub const RTRVMRAYTRACESCENE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RtRvmRaytraceScene",
    flags: MemberInfoFlags::new(36937),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(RTRVMRAYTRACESCENE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RtRvmRaytraceScene {
    fn type_info() -> &'static TypeInfo {
        RTRVMRAYTRACESCENE_TYPE_INFO
    }
}


pub const RTRVMRAYTRACESCENE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RtRvmRaytraceScene-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RtRvmRaytraceScene-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NvShadowLibMap {
}

pub const NVSHADOWLIBMAP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NvShadowLibMap",
    flags: MemberInfoFlags::new(36937),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(NVSHADOWLIBMAP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NvShadowLibMap {
    fn type_info() -> &'static TypeInfo {
        NVSHADOWLIBMAP_TYPE_INFO
    }
}


pub const NVSHADOWLIBMAP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NvShadowLibMap-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("NvShadowLibMap-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum NvShadowMapRenderType {
    #[default]
    NvShadowMapRenderType_Depth = 0,
    NvShadowMapRenderType_FT = 2,
}

pub const NVSHADOWMAPRENDERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NvShadowMapRenderType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(NVSHADOWMAPRENDERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for NvShadowMapRenderType {
    fn type_info() -> &'static TypeInfo {
        NVSHADOWMAPRENDERTYPE_TYPE_INFO
    }
}


pub const NVSHADOWMAPRENDERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NvShadowMapRenderType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("NvShadowMapRenderType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NvShadowLibDrawData {
}

pub const NVSHADOWLIBDRAWDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NvShadowLibDrawData",
    flags: MemberInfoFlags::new(36937),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(NVSHADOWLIBDRAWDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NvShadowLibDrawData {
    fn type_info() -> &'static TypeInfo {
        NVSHADOWLIBDRAWDATA_TYPE_INFO
    }
}


pub const NVSHADOWLIBDRAWDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NvShadowLibDrawData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("NvShadowLibDrawData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NvShadowLibPsoParams {
}

pub const NVSHADOWLIBPSOPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NvShadowLibPsoParams",
    flags: MemberInfoFlags::new(36937),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(NVSHADOWLIBPSOPARAMS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for NvShadowLibPsoParams {
    fn type_info() -> &'static TypeInfo {
        NVSHADOWLIBPSOPARAMS_TYPE_INFO
    }
}


pub const NVSHADOWLIBPSOPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NvShadowLibPsoParams-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("NvShadowLibPsoParams-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NvShadowLibContext {
}

pub const NVSHADOWLIBCONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NvShadowLibContext",
    flags: MemberInfoFlags::new(36937),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(NVSHADOWLIBCONTEXT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NvShadowLibContext {
    fn type_info() -> &'static TypeInfo {
        NVSHADOWLIBCONTEXT_TYPE_INFO
    }
}


pub const NVSHADOWLIBCONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NvShadowLibContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("NvShadowLibContext-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RaytraceSettings {
    pub blas_optimize_memory_enable: bool,
    pub raytrace_terrain_cull_radius: f32,
    pub raytrace_terrain_triangle_density: u32,
}

pub const RAYTRACESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RaytraceSettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BlasOptimizeMemoryEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(RaytraceSettings, blas_optimize_memory_enable),
            },
            FieldInfoData {
                name: "RaytraceTerrainCullRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RaytraceSettings, raytrace_terrain_cull_radius),
            },
            FieldInfoData {
                name: "RaytraceTerrainTriangleDensity",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(RaytraceSettings, raytrace_terrain_triangle_density),
            },
        ],
    }),
    array_type: Some(RAYTRACESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RaytraceSettings {
    fn type_info() -> &'static TypeInfo {
        RAYTRACESETTINGS_TYPE_INFO
    }
}


pub const RAYTRACESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RaytraceSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RaytraceSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GlobalPostProcessSettings {
    pub debug_mode: PostProcessDebugMode,
    pub debug_mode_step: u32,
    pub hdr_blur_enable: bool,
    pub e_v_clamp_enable: bool,
    pub adaptation_time_enable: bool,
    pub force_e_v_compensation_enable: bool,
    pub force_e_v_compensation: f32,
    pub force_e_v_enable: bool,
    pub force_e_v: f32,
    pub draw_debug_info: bool,
    pub draw_exposure_debug_info: bool,
    pub render_target_load_opts_enable: bool,
    pub blur_enable: bool,
    pub quarter_downsampling_enable: bool,
    pub blur_blend_enable: bool,
    pub bloom_enable: bool,
    pub bloom_test_enable: bool,
    pub blur_pyramid_enable: bool,
    pub blur_pyramid_quarter_res_enable: bool,
    pub blur_pyramid_final_level: u32,
    pub blur_pyramid_hdr_enable: bool,
    pub blur_pyramid_fast_hdr_enable: bool,
    pub blur_pyramid_ldr_range: f32,
    pub blur_pyramid_single_pass_enable: bool,
    pub debug_color_graph_enable: bool,
    pub debug_color_graph_min_value: f32,
    pub debug_color_graph_max_value: f32,
    pub debug_color_graph_line_number: i32,
    pub auto_exposure_method: super::render_base::AutoExposureMethod,
    pub auto_exposure_histogram_bin_count: u32,
    pub auto_exposure_histogram_mip_used: u32,
    pub auto_exposure_histogram_min_value: f32,
    pub auto_exposure_histogram_max_value: f32,
    pub downsample_log_average_enable: bool,
    pub downsample_average_start_mipmap: u32,
    pub downsample_before_blur_enable: bool,
    pub force_dof_enable: i32,
    pub force_dof_blur_factor: f32,
    pub force_dof_blur_add: f32,
    pub force_dof_focus_distance: f32,
    pub force_simple_dof_near_start: f32,
    pub force_simple_dof_near_end: f32,
    pub force_simple_dof_far_start: f32,
    pub force_simple_dof_far_end: f32,
    pub force_simple_dof_blur_max: f32,
    pub force_sprite_dof_near_start: f32,
    pub force_sprite_dof_near_end: f32,
    pub force_sprite_dof_far_start: f32,
    pub force_sprite_dof_far_end: f32,
    pub force_sprite_dof_blur_max: f32,
    pub force_bloom_scale: super::core::Vec3,
    pub force_vignette_scale: super::core::Vec2,
    pub force_vignette_exponent: f32,
    pub force_vignette_color: super::core::Vec4,
    pub vignette_enable: bool,
    pub fxaa_compute_debug: bool,
    pub fxaa_compute_sub_pixel_removal: f32,
    pub fxaa_compute_contrast_threshold: f32,
    pub force_tonemap_method: i32,
    pub color_grading_enable: bool,
    pub color_grading_debug_enable: bool,
    pub color_transform_enable: bool,
    pub color_grading_force_update_always: bool,
    pub color_grading_high_quality_mode: super::render_base::ColorGradingQualityMode,
    pub force_chromostereopsis_enable: i32,
    pub force_chromostereopsis_offset: i32,
    pub force_chromostereopsis_scale: f32,
    pub film_grain_enable: bool,
    pub film_grain_texture_scale: super::core::Vec2,
    pub film_grain_color_scale: super::core::Vec3,
    pub film_grain_linear_filtering_enable: bool,
    pub film_grain_random_enable: bool,
    pub lens_scope_enable: bool,
    pub lens_scope_color_scale: f32,
    pub half_res_edge_detect_threshold: f32,
    pub brightness: super::core::Vec3,
    pub contrast: super::core::Vec3,
    pub saturation: super::core::Vec3,
    pub hue: f32,
    pub u_i_brightness_norm: f32,
    pub user_brightness_min: f32,
    pub user_brightness_max: f32,
    pub user_brightness_add_scale: f32,
    pub user_brightness_mul_scale: f32,
    pub user_brightness_l_u_t_enable: bool,
    pub draw_debug_user_brightness_l_u_t: bool,
    pub l_u_t_gamma_r: f32,
    pub l_u_t_gamma_g: f32,
    pub l_u_t_gamma_b: f32,
    pub l_u_t_gamma_curb_offset: f32,
    pub blur_method: BlurMethod,
    pub sprite_dof_enable: bool,
    pub sprite_dof_merge_enable: bool,
    pub sprite_dof_foreground_enable: bool,
    pub sprite_dof_depth_filter_enable: bool,
    pub sprite_dof_buffer32bit_enable: bool,
    pub sprite_dof_half_resolution_enable: bool,
    pub sprite_dof_min_radius_layer1: f32,
    pub sprite_dof_min_radius_layer2: f32,
    pub sprite_dof_max_radius_gather_pass: f32,
    pub sprite_dof_near_gather_enable: bool,
    pub sprite_dof_merge_color_threshold: f32,
    pub sprite_dof_merge_radius_threshold: f32,
    pub sprite_dof_depth_discontinuity_threshold: f32,
    pub sprite_dof_active_layer: u32,
    pub sprite_dof_infocus_multiplier: f32,
    pub sprite_dof_max_blur_scale: f32,
    pub sprite_dof_energy_scaler: f32,
    pub sprite_dof_best_upsampling_enable: bool,
    pub sprite_dof_multilayer_foreground_enable: bool,
    pub sprite_dof_multilayer_foreground_count: u32,
    pub sprite_dof_multilayer_foreground_coc_span: f32,
    pub sprite_dof_foreground_reweight_exponent: f32,
    pub sprite_dof_multilayer_foreground_layer_extension: f32,
    pub sprite_dof_packed_bokeh_enable: bool,
    pub sprite_dof_bicubic_sample_enable: bool,
    pub sprite_dof_weight_threshold: f32,
    pub sprite_dof_multilayer_foreground_active_layer: u32,
    pub sprite_dof_debug_enable: bool,
    pub sprite_dof_use_async_compute: bool,
    pub sprite_dof_optical_vignetting_enable: bool,
    pub circular_dof_enable: bool,
    pub circular_dof_enable_high_res: bool,
    pub circular_dof_enable_far_blur_high_quality: bool,
    pub circular_dof_enable_anti_banding: bool,
    pub circular_dof_near_blending_speed: f32,
    pub circular_dof_far_blending_speed: f32,
    pub dynamic_a_o_enable: bool,
    pub dynamic_a_o_method: DynamicAOMethod,
    pub ssao_blur_enable: bool,
    pub screen_space_raytrace_enable: bool,
    pub screen_space_raytrace_deferred_resolve_enable: bool,
    pub screen_space_raytrace_use_velocity_vectors_for_temporal: bool,
    pub screen_space_raytrace_separate_coverage_enable: bool,
    pub screen_space_raytrace_fullres_enable: bool,
    pub screen_space_raytrace_debug: i32,
    pub screen_space_raytrace_quality: i32,
    pub screen_space_raytrace_camera_cut_enable: bool,
    pub screen_space_raytrace_async_compute_enable: bool,
    pub ironsights_dof_enable: bool,
    pub ironsights_dof_resolution_factor: u32,
    pub force_ironsights_dof_active: bool,
    pub ironsights_blur_filter: super::render_base::BlurFilter,
    pub ironsights_blur_filter720p: super::render_base::BlurFilter,
    pub ironsights_h_d_r_compression: f32,
    pub ironsights_co_c_scale: f32,
    pub override_ironsights_dof_params: bool,
    pub override_ironsights_hip_fade: f32,
    pub override_ironsights_start_fade: f32,
    pub override_ironsights_focal_distance: f32,
    pub override_ironsights_dof_circle_blur: bool,
    pub override_ironsights_dof_circle_distance: f32,
    pub override_ironsights_dof_circle_fade_distance: f32,
    pub force_lens_scope_active: bool,
    pub dynamic_a_o_horizon_based: bool,
    pub dynamic_a_o_sample_temporal_count: u32,
    pub dynamic_a_o_sample_step_count: u32,
    pub dynamic_a_o_sample_dir_count: u32,
    pub dynamic_a_o_max_footprint_radius: f32,
    pub dynamic_a_o_bilateral_blur_enable: bool,
    pub dynamic_a_o_bilateral_blur_radius: u32,
    pub dynamic_a_o_bilateral_blur_sharpness: f32,
    pub dynamic_a_o_normal_enable: bool,
    pub dynamic_a_o_normal_influence: f32,
    pub dynamic_a_o_use_async_compute: bool,
    pub dynamic_a_o_half_res_enable: bool,
    pub dynamic_a_o_upscale_enable: bool,
    pub dynamic_a_o_edge_blur_enable: bool,
    pub dynamic_a_o_edge_blur_type: u32,
    pub dynamic_a_o_edge_blur_groups: u32,
    pub advanced_a_o_local_samples: u32,
    pub advanced_a_o_distant_samples: u32,
    pub dynamic_a_o_temporal_filter_enable: bool,
    pub dynamic_a_o_temporal_history_sharpening: bool,
    pub dynamic_a_o_temporal_disocclusion_rejection_factor: f32,
    pub dynamic_a_o_temporal_motion_sharpening_factor: f32,
    pub dynamic_a_o_temporal_responsiveness: f32,
    pub dynamic_a_o_temporal_antiflicker_strength: f32,
    pub draw_debug_dynamic_a_o_temporal_enable: bool,
    pub draw_debug_dynamic_a_o_temporal_accumulation_count: u32,
    pub draw_debug_dynamic_a_o_temporal_debug_mode: u32,
    pub draw_debug_dynamic_a_o_temporal_max_distance: f32,
    pub chromatic_aberration_allowed: bool,
    pub lens_distortion_allowed: bool,
}

pub const GLOBALPOSTPROCESSSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GlobalPostProcessSettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DebugMode",
                flags: MemberInfoFlags::new(0),
                field_type: POSTPROCESSDEBUGMODE_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, debug_mode),
            },
            FieldInfoData {
                name: "DebugModeStep",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, debug_mode_step),
            },
            FieldInfoData {
                name: "HdrBlurEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, hdr_blur_enable),
            },
            FieldInfoData {
                name: "EVClampEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, e_v_clamp_enable),
            },
            FieldInfoData {
                name: "AdaptationTimeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, adaptation_time_enable),
            },
            FieldInfoData {
                name: "ForceEVCompensationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_e_v_compensation_enable),
            },
            FieldInfoData {
                name: "ForceEVCompensation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_e_v_compensation),
            },
            FieldInfoData {
                name: "ForceEVEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_e_v_enable),
            },
            FieldInfoData {
                name: "ForceEV",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_e_v),
            },
            FieldInfoData {
                name: "DrawDebugInfo",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, draw_debug_info),
            },
            FieldInfoData {
                name: "DrawExposureDebugInfo",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, draw_exposure_debug_info),
            },
            FieldInfoData {
                name: "RenderTargetLoadOptsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, render_target_load_opts_enable),
            },
            FieldInfoData {
                name: "BlurEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, blur_enable),
            },
            FieldInfoData {
                name: "QuarterDownsamplingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, quarter_downsampling_enable),
            },
            FieldInfoData {
                name: "BlurBlendEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, blur_blend_enable),
            },
            FieldInfoData {
                name: "BloomEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, bloom_enable),
            },
            FieldInfoData {
                name: "BloomTestEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, bloom_test_enable),
            },
            FieldInfoData {
                name: "BlurPyramidEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, blur_pyramid_enable),
            },
            FieldInfoData {
                name: "BlurPyramidQuarterResEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, blur_pyramid_quarter_res_enable),
            },
            FieldInfoData {
                name: "BlurPyramidFinalLevel",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, blur_pyramid_final_level),
            },
            FieldInfoData {
                name: "BlurPyramidHdrEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, blur_pyramid_hdr_enable),
            },
            FieldInfoData {
                name: "BlurPyramidFastHdrEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, blur_pyramid_fast_hdr_enable),
            },
            FieldInfoData {
                name: "BlurPyramidLdrRange",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, blur_pyramid_ldr_range),
            },
            FieldInfoData {
                name: "BlurPyramidSinglePassEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, blur_pyramid_single_pass_enable),
            },
            FieldInfoData {
                name: "DebugColorGraphEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, debug_color_graph_enable),
            },
            FieldInfoData {
                name: "DebugColorGraphMinValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, debug_color_graph_min_value),
            },
            FieldInfoData {
                name: "DebugColorGraphMaxValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, debug_color_graph_max_value),
            },
            FieldInfoData {
                name: "DebugColorGraphLineNumber",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, debug_color_graph_line_number),
            },
            FieldInfoData {
                name: "AutoExposureMethod",
                flags: MemberInfoFlags::new(0),
                field_type: AUTOEXPOSUREMETHOD_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, auto_exposure_method),
            },
            FieldInfoData {
                name: "AutoExposureHistogramBinCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, auto_exposure_histogram_bin_count),
            },
            FieldInfoData {
                name: "AutoExposureHistogramMipUsed",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, auto_exposure_histogram_mip_used),
            },
            FieldInfoData {
                name: "AutoExposureHistogramMinValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, auto_exposure_histogram_min_value),
            },
            FieldInfoData {
                name: "AutoExposureHistogramMaxValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, auto_exposure_histogram_max_value),
            },
            FieldInfoData {
                name: "DownsampleLogAverageEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, downsample_log_average_enable),
            },
            FieldInfoData {
                name: "DownsampleAverageStartMipmap",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, downsample_average_start_mipmap),
            },
            FieldInfoData {
                name: "DownsampleBeforeBlurEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, downsample_before_blur_enable),
            },
            FieldInfoData {
                name: "ForceDofEnable",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_dof_enable),
            },
            FieldInfoData {
                name: "ForceDofBlurFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_dof_blur_factor),
            },
            FieldInfoData {
                name: "ForceDofBlurAdd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_dof_blur_add),
            },
            FieldInfoData {
                name: "ForceDofFocusDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_dof_focus_distance),
            },
            FieldInfoData {
                name: "ForceSimpleDofNearStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_simple_dof_near_start),
            },
            FieldInfoData {
                name: "ForceSimpleDofNearEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_simple_dof_near_end),
            },
            FieldInfoData {
                name: "ForceSimpleDofFarStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_simple_dof_far_start),
            },
            FieldInfoData {
                name: "ForceSimpleDofFarEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_simple_dof_far_end),
            },
            FieldInfoData {
                name: "ForceSimpleDofBlurMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_simple_dof_blur_max),
            },
            FieldInfoData {
                name: "ForceSpriteDofNearStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_sprite_dof_near_start),
            },
            FieldInfoData {
                name: "ForceSpriteDofNearEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_sprite_dof_near_end),
            },
            FieldInfoData {
                name: "ForceSpriteDofFarStart",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_sprite_dof_far_start),
            },
            FieldInfoData {
                name: "ForceSpriteDofFarEnd",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_sprite_dof_far_end),
            },
            FieldInfoData {
                name: "ForceSpriteDofBlurMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_sprite_dof_blur_max),
            },
            FieldInfoData {
                name: "ForceBloomScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_bloom_scale),
            },
            FieldInfoData {
                name: "ForceVignetteScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_vignette_scale),
            },
            FieldInfoData {
                name: "ForceVignetteExponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_vignette_exponent),
            },
            FieldInfoData {
                name: "ForceVignetteColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_vignette_color),
            },
            FieldInfoData {
                name: "VignetteEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, vignette_enable),
            },
            FieldInfoData {
                name: "FxaaComputeDebug",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, fxaa_compute_debug),
            },
            FieldInfoData {
                name: "FxaaComputeSubPixelRemoval",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, fxaa_compute_sub_pixel_removal),
            },
            FieldInfoData {
                name: "FxaaComputeContrastThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, fxaa_compute_contrast_threshold),
            },
            FieldInfoData {
                name: "ForceTonemapMethod",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_tonemap_method),
            },
            FieldInfoData {
                name: "ColorGradingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, color_grading_enable),
            },
            FieldInfoData {
                name: "ColorGradingDebugEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, color_grading_debug_enable),
            },
            FieldInfoData {
                name: "ColorTransformEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, color_transform_enable),
            },
            FieldInfoData {
                name: "ColorGradingForceUpdateAlways",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, color_grading_force_update_always),
            },
            FieldInfoData {
                name: "ColorGradingHighQualityMode",
                flags: MemberInfoFlags::new(0),
                field_type: COLORGRADINGQUALITYMODE_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, color_grading_high_quality_mode),
            },
            FieldInfoData {
                name: "ForceChromostereopsisEnable",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_chromostereopsis_enable),
            },
            FieldInfoData {
                name: "ForceChromostereopsisOffset",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_chromostereopsis_offset),
            },
            FieldInfoData {
                name: "ForceChromostereopsisScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_chromostereopsis_scale),
            },
            FieldInfoData {
                name: "FilmGrainEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, film_grain_enable),
            },
            FieldInfoData {
                name: "FilmGrainTextureScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, film_grain_texture_scale),
            },
            FieldInfoData {
                name: "FilmGrainColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, film_grain_color_scale),
            },
            FieldInfoData {
                name: "FilmGrainLinearFilteringEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, film_grain_linear_filtering_enable),
            },
            FieldInfoData {
                name: "FilmGrainRandomEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, film_grain_random_enable),
            },
            FieldInfoData {
                name: "LensScopeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, lens_scope_enable),
            },
            FieldInfoData {
                name: "LensScopeColorScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, lens_scope_color_scale),
            },
            FieldInfoData {
                name: "HalfResEdgeDetectThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, half_res_edge_detect_threshold),
            },
            FieldInfoData {
                name: "Brightness",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, brightness),
            },
            FieldInfoData {
                name: "Contrast",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, contrast),
            },
            FieldInfoData {
                name: "Saturation",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, saturation),
            },
            FieldInfoData {
                name: "Hue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, hue),
            },
            FieldInfoData {
                name: "UIBrightnessNorm",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, u_i_brightness_norm),
            },
            FieldInfoData {
                name: "UserBrightnessMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, user_brightness_min),
            },
            FieldInfoData {
                name: "UserBrightnessMax",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, user_brightness_max),
            },
            FieldInfoData {
                name: "UserBrightnessAddScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, user_brightness_add_scale),
            },
            FieldInfoData {
                name: "UserBrightnessMulScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, user_brightness_mul_scale),
            },
            FieldInfoData {
                name: "UserBrightnessLUTEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, user_brightness_l_u_t_enable),
            },
            FieldInfoData {
                name: "DrawDebugUserBrightnessLUT",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, draw_debug_user_brightness_l_u_t),
            },
            FieldInfoData {
                name: "LUTGammaR",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, l_u_t_gamma_r),
            },
            FieldInfoData {
                name: "LUTGammaG",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, l_u_t_gamma_g),
            },
            FieldInfoData {
                name: "LUTGammaB",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, l_u_t_gamma_b),
            },
            FieldInfoData {
                name: "LUTGammaCurbOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, l_u_t_gamma_curb_offset),
            },
            FieldInfoData {
                name: "BlurMethod",
                flags: MemberInfoFlags::new(8192),
                field_type: BLURMETHOD_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, blur_method),
            },
            FieldInfoData {
                name: "SpriteDofEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_enable),
            },
            FieldInfoData {
                name: "SpriteDofMergeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_merge_enable),
            },
            FieldInfoData {
                name: "SpriteDofForegroundEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_foreground_enable),
            },
            FieldInfoData {
                name: "SpriteDofDepthFilterEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_depth_filter_enable),
            },
            FieldInfoData {
                name: "SpriteDofBuffer32bitEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_buffer32bit_enable),
            },
            FieldInfoData {
                name: "SpriteDofHalfResolutionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_half_resolution_enable),
            },
            FieldInfoData {
                name: "SpriteDofMinRadiusLayer1",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_min_radius_layer1),
            },
            FieldInfoData {
                name: "SpriteDofMinRadiusLayer2",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_min_radius_layer2),
            },
            FieldInfoData {
                name: "SpriteDofMaxRadiusGatherPass",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_max_radius_gather_pass),
            },
            FieldInfoData {
                name: "SpriteDofNearGatherEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_near_gather_enable),
            },
            FieldInfoData {
                name: "SpriteDofMergeColorThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_merge_color_threshold),
            },
            FieldInfoData {
                name: "SpriteDofMergeRadiusThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_merge_radius_threshold),
            },
            FieldInfoData {
                name: "SpriteDofDepthDiscontinuityThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_depth_discontinuity_threshold),
            },
            FieldInfoData {
                name: "SpriteDofActiveLayer",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_active_layer),
            },
            FieldInfoData {
                name: "SpriteDofInfocusMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_infocus_multiplier),
            },
            FieldInfoData {
                name: "SpriteDofMaxBlurScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_max_blur_scale),
            },
            FieldInfoData {
                name: "SpriteDofEnergyScaler",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_energy_scaler),
            },
            FieldInfoData {
                name: "SpriteDofBestUpsamplingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_best_upsampling_enable),
            },
            FieldInfoData {
                name: "SpriteDofMultilayerForegroundEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_multilayer_foreground_enable),
            },
            FieldInfoData {
                name: "SpriteDofMultilayerForegroundCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_multilayer_foreground_count),
            },
            FieldInfoData {
                name: "SpriteDofMultilayerForegroundCocSpan",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_multilayer_foreground_coc_span),
            },
            FieldInfoData {
                name: "SpriteDofForegroundReweightExponent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_foreground_reweight_exponent),
            },
            FieldInfoData {
                name: "SpriteDofMultilayerForegroundLayerExtension",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_multilayer_foreground_layer_extension),
            },
            FieldInfoData {
                name: "SpriteDofPackedBokehEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_packed_bokeh_enable),
            },
            FieldInfoData {
                name: "SpriteDofBicubicSampleEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_bicubic_sample_enable),
            },
            FieldInfoData {
                name: "SpriteDofWeightThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_weight_threshold),
            },
            FieldInfoData {
                name: "SpriteDofMultilayerForegroundActiveLayer",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_multilayer_foreground_active_layer),
            },
            FieldInfoData {
                name: "SpriteDofDebugEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_debug_enable),
            },
            FieldInfoData {
                name: "SpriteDofUseAsyncCompute",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_use_async_compute),
            },
            FieldInfoData {
                name: "SpriteDofOpticalVignettingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, sprite_dof_optical_vignetting_enable),
            },
            FieldInfoData {
                name: "CircularDofEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, circular_dof_enable),
            },
            FieldInfoData {
                name: "CircularDofEnableHighRes",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, circular_dof_enable_high_res),
            },
            FieldInfoData {
                name: "CircularDofEnableFarBlurHighQuality",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, circular_dof_enable_far_blur_high_quality),
            },
            FieldInfoData {
                name: "CircularDofEnableAntiBanding",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, circular_dof_enable_anti_banding),
            },
            FieldInfoData {
                name: "CircularDofNearBlendingSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, circular_dof_near_blending_speed),
            },
            FieldInfoData {
                name: "CircularDofFarBlendingSpeed",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, circular_dof_far_blending_speed),
            },
            FieldInfoData {
                name: "DynamicAOEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_enable),
            },
            FieldInfoData {
                name: "DynamicAOMethod",
                flags: MemberInfoFlags::new(8192),
                field_type: DYNAMICAOMETHOD_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_method),
            },
            FieldInfoData {
                name: "SsaoBlurEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, ssao_blur_enable),
            },
            FieldInfoData {
                name: "ScreenSpaceRaytraceEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, screen_space_raytrace_enable),
            },
            FieldInfoData {
                name: "ScreenSpaceRaytraceDeferredResolveEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, screen_space_raytrace_deferred_resolve_enable),
            },
            FieldInfoData {
                name: "ScreenSpaceRaytraceUseVelocityVectorsForTemporal",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, screen_space_raytrace_use_velocity_vectors_for_temporal),
            },
            FieldInfoData {
                name: "ScreenSpaceRaytraceSeparateCoverageEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, screen_space_raytrace_separate_coverage_enable),
            },
            FieldInfoData {
                name: "ScreenSpaceRaytraceFullresEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, screen_space_raytrace_fullres_enable),
            },
            FieldInfoData {
                name: "ScreenSpaceRaytraceDebug",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, screen_space_raytrace_debug),
            },
            FieldInfoData {
                name: "ScreenSpaceRaytraceQuality",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, screen_space_raytrace_quality),
            },
            FieldInfoData {
                name: "ScreenSpaceRaytraceCameraCutEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, screen_space_raytrace_camera_cut_enable),
            },
            FieldInfoData {
                name: "ScreenSpaceRaytraceAsyncComputeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, screen_space_raytrace_async_compute_enable),
            },
            FieldInfoData {
                name: "IronsightsDofEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, ironsights_dof_enable),
            },
            FieldInfoData {
                name: "IronsightsDofResolutionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, ironsights_dof_resolution_factor),
            },
            FieldInfoData {
                name: "ForceIronsightsDofActive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_ironsights_dof_active),
            },
            FieldInfoData {
                name: "IronsightsBlurFilter",
                flags: MemberInfoFlags::new(0),
                field_type: BLURFILTER_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, ironsights_blur_filter),
            },
            FieldInfoData {
                name: "IronsightsBlurFilter720p",
                flags: MemberInfoFlags::new(0),
                field_type: BLURFILTER_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, ironsights_blur_filter720p),
            },
            FieldInfoData {
                name: "IronsightsHDRCompression",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, ironsights_h_d_r_compression),
            },
            FieldInfoData {
                name: "IronsightsCoCScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, ironsights_co_c_scale),
            },
            FieldInfoData {
                name: "OverrideIronsightsDofParams",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, override_ironsights_dof_params),
            },
            FieldInfoData {
                name: "OverrideIronsightsHipFade",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, override_ironsights_hip_fade),
            },
            FieldInfoData {
                name: "OverrideIronsightsStartFade",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, override_ironsights_start_fade),
            },
            FieldInfoData {
                name: "OverrideIronsightsFocalDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, override_ironsights_focal_distance),
            },
            FieldInfoData {
                name: "OverrideIronsightsDofCircleBlur",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, override_ironsights_dof_circle_blur),
            },
            FieldInfoData {
                name: "OverrideIronsightsDofCircleDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, override_ironsights_dof_circle_distance),
            },
            FieldInfoData {
                name: "OverrideIronsightsDofCircleFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, override_ironsights_dof_circle_fade_distance),
            },
            FieldInfoData {
                name: "ForceLensScopeActive",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, force_lens_scope_active),
            },
            FieldInfoData {
                name: "DynamicAOHorizonBased",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_horizon_based),
            },
            FieldInfoData {
                name: "DynamicAOSampleTemporalCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_sample_temporal_count),
            },
            FieldInfoData {
                name: "DynamicAOSampleStepCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_sample_step_count),
            },
            FieldInfoData {
                name: "DynamicAOSampleDirCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_sample_dir_count),
            },
            FieldInfoData {
                name: "DynamicAOMaxFootprintRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_max_footprint_radius),
            },
            FieldInfoData {
                name: "DynamicAOBilateralBlurEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_bilateral_blur_enable),
            },
            FieldInfoData {
                name: "DynamicAOBilateralBlurRadius",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_bilateral_blur_radius),
            },
            FieldInfoData {
                name: "DynamicAOBilateralBlurSharpness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_bilateral_blur_sharpness),
            },
            FieldInfoData {
                name: "DynamicAONormalEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_normal_enable),
            },
            FieldInfoData {
                name: "DynamicAONormalInfluence",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_normal_influence),
            },
            FieldInfoData {
                name: "DynamicAOUseAsyncCompute",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_use_async_compute),
            },
            FieldInfoData {
                name: "DynamicAOHalfResEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_half_res_enable),
            },
            FieldInfoData {
                name: "DynamicAOUpscaleEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_upscale_enable),
            },
            FieldInfoData {
                name: "DynamicAOEdgeBlurEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_edge_blur_enable),
            },
            FieldInfoData {
                name: "DynamicAOEdgeBlurType",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_edge_blur_type),
            },
            FieldInfoData {
                name: "DynamicAOEdgeBlurGroups",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_edge_blur_groups),
            },
            FieldInfoData {
                name: "AdvancedAOLocalSamples",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, advanced_a_o_local_samples),
            },
            FieldInfoData {
                name: "AdvancedAODistantSamples",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, advanced_a_o_distant_samples),
            },
            FieldInfoData {
                name: "DynamicAOTemporalFilterEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_temporal_filter_enable),
            },
            FieldInfoData {
                name: "DynamicAOTemporalHistorySharpening",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_temporal_history_sharpening),
            },
            FieldInfoData {
                name: "DynamicAOTemporalDisocclusionRejectionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_temporal_disocclusion_rejection_factor),
            },
            FieldInfoData {
                name: "DynamicAOTemporalMotionSharpeningFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_temporal_motion_sharpening_factor),
            },
            FieldInfoData {
                name: "DynamicAOTemporalResponsiveness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_temporal_responsiveness),
            },
            FieldInfoData {
                name: "DynamicAOTemporalAntiflickerStrength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, dynamic_a_o_temporal_antiflicker_strength),
            },
            FieldInfoData {
                name: "DrawDebugDynamicAOTemporalEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, draw_debug_dynamic_a_o_temporal_enable),
            },
            FieldInfoData {
                name: "DrawDebugDynamicAOTemporalAccumulationCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, draw_debug_dynamic_a_o_temporal_accumulation_count),
            },
            FieldInfoData {
                name: "DrawDebugDynamicAOTemporalDebugMode",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, draw_debug_dynamic_a_o_temporal_debug_mode),
            },
            FieldInfoData {
                name: "DrawDebugDynamicAOTemporalMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, draw_debug_dynamic_a_o_temporal_max_distance),
            },
            FieldInfoData {
                name: "ChromaticAberrationAllowed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, chromatic_aberration_allowed),
            },
            FieldInfoData {
                name: "LensDistortionAllowed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GlobalPostProcessSettings, lens_distortion_allowed),
            },
        ],
    }),
    array_type: Some(GLOBALPOSTPROCESSSETTINGS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for GlobalPostProcessSettings {
    fn type_info() -> &'static TypeInfo {
        GLOBALPOSTPROCESSSETTINGS_TYPE_INFO
    }
}


pub const GLOBALPOSTPROCESSSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GlobalPostProcessSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("GlobalPostProcessSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum BlurMethod {
    #[default]
    BlurMethod_Disabled = 0,
    BlurMethod_Gaussian = 1,
    BlurMethod_Circular = 2,
    BlurMethod_Sprite = 3,
}

pub const BLURMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlurMethod",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(BLURMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BlurMethod {
    fn type_info() -> &'static TypeInfo {
        BLURMETHOD_TYPE_INFO
    }
}


pub const BLURMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlurMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("BlurMethod-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DynamicAOMethod {
    #[default]
    DynamicAOMethod_SSAO = 0,
    DynamicAOMethod_HBAO = 1,
    DynamicAOMethod_AdvancedAO = 2,
}

pub const DYNAMICAOMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicAOMethod",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(DYNAMICAOMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DynamicAOMethod {
    fn type_info() -> &'static TypeInfo {
        DYNAMICAOMETHOD_TYPE_INFO
    }
}


pub const DYNAMICAOMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicAOMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("DynamicAOMethod-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PostProcessDebugMode {
    #[default]
    PpdmDefault = 0,
    PpdmBloom = 1,
    PpdmBloomStep = 2,
    PpdmDofBlur = 3,
    PpdmBlur = 4,
    PpdmBlurStep = 5,
    PpdmDepth = 6,
    PpdmBlurPyramidStep = 7,
}

pub const POSTPROCESSDEBUGMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PostProcessDebugMode",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(POSTPROCESSDEBUGMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PostProcessDebugMode {
    fn type_info() -> &'static TypeInfo {
        POSTPROCESSDEBUGMODE_TYPE_INFO
    }
}


pub const POSTPROCESSDEBUGMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PostProcessDebugMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("PostProcessDebugMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FFTBloomSettings {
    pub enable: bool,
    pub mip_level: i32,
    pub draw_debug_enable: bool,
    pub procedural_kernel_size: i32,
}

pub const FFTBLOOMSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FFTBloomSettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FFTBloomSettings, enable),
            },
            FieldInfoData {
                name: "MipLevel",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(FFTBloomSettings, mip_level),
            },
            FieldInfoData {
                name: "DrawDebugEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FFTBloomSettings, draw_debug_enable),
            },
            FieldInfoData {
                name: "ProceduralKernelSize",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(FFTBloomSettings, procedural_kernel_size),
            },
        ],
    }),
    array_type: Some(FFTBLOOMSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FFTBloomSettings {
    fn type_info() -> &'static TypeInfo {
        FFTBLOOMSETTINGS_TYPE_INFO
    }
}


pub const FFTBLOOMSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FFTBloomSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("FFTBloomSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SkinnedProceduralAnimationData {
    pub expressions: Vec<SkinnedProceduralAnimationExpression>,
    pub cull_distance: f32,
    pub cull_fade_distance: f32,
    pub bones: Vec<SkinnedProceduralAnimationBone>,
    pub root_poses: Vec<SkinnedProceduralAnimationRootPose>,
}

pub const SKINNEDPROCEDURALANIMATIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkinnedProceduralAnimationData",
    flags: MemberInfoFlags::new(73),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Expressions",
                flags: MemberInfoFlags::new(144),
                field_type: SKINNEDPROCEDURALANIMATIONEXPRESSION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SkinnedProceduralAnimationData, expressions),
            },
            FieldInfoData {
                name: "CullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkinnedProceduralAnimationData, cull_distance),
            },
            FieldInfoData {
                name: "CullFadeDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SkinnedProceduralAnimationData, cull_fade_distance),
            },
            FieldInfoData {
                name: "Bones",
                flags: MemberInfoFlags::new(144),
                field_type: SKINNEDPROCEDURALANIMATIONBONE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SkinnedProceduralAnimationData, bones),
            },
            FieldInfoData {
                name: "RootPoses",
                flags: MemberInfoFlags::new(144),
                field_type: SKINNEDPROCEDURALANIMATIONROOTPOSE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SkinnedProceduralAnimationData, root_poses),
            },
        ],
    }),
    array_type: Some(SKINNEDPROCEDURALANIMATIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SkinnedProceduralAnimationData {
    fn type_info() -> &'static TypeInfo {
        SKINNEDPROCEDURALANIMATIONDATA_TYPE_INFO
    }
}


pub const SKINNEDPROCEDURALANIMATIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkinnedProceduralAnimationData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("SkinnedProceduralAnimationData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SkinnedProceduralAnimationBone {
    pub pose: super::core::LinearTransform,
    pub local_pose: super::core::LinearTransform,
    pub parent_index: i32,
}

pub const SKINNEDPROCEDURALANIMATIONBONE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkinnedProceduralAnimationBone",
    flags: MemberInfoFlags::new(36937),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Pose",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SkinnedProceduralAnimationBone, pose),
            },
            FieldInfoData {
                name: "LocalPose",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SkinnedProceduralAnimationBone, local_pose),
            },
            FieldInfoData {
                name: "ParentIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SkinnedProceduralAnimationBone, parent_index),
            },
        ],
    }),
    array_type: Some(SKINNEDPROCEDURALANIMATIONBONE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkinnedProceduralAnimationBone {
    fn type_info() -> &'static TypeInfo {
        SKINNEDPROCEDURALANIMATIONBONE_TYPE_INFO
    }
}


pub const SKINNEDPROCEDURALANIMATIONBONE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkinnedProceduralAnimationBone-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("SkinnedProceduralAnimationBone-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SkinnedProceduralAnimationRootPose {
    pub local_pose: super::core::LinearTransform,
    pub index: i32,
}

pub const SKINNEDPROCEDURALANIMATIONROOTPOSE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkinnedProceduralAnimationRootPose",
    flags: MemberInfoFlags::new(36937),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "LocalPose",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(SkinnedProceduralAnimationRootPose, local_pose),
            },
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SkinnedProceduralAnimationRootPose, index),
            },
        ],
    }),
    array_type: Some(SKINNEDPROCEDURALANIMATIONROOTPOSE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkinnedProceduralAnimationRootPose {
    fn type_info() -> &'static TypeInfo {
        SKINNEDPROCEDURALANIMATIONROOTPOSE_TYPE_INFO
    }
}


pub const SKINNEDPROCEDURALANIMATIONROOTPOSE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkinnedProceduralAnimationRootPose-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("SkinnedProceduralAnimationRootPose-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SkinnedProceduralAnimationExpression {
    pub graph: SkinnedProceduralAnimationExpressionGraph,
    pub runtime_parameters: Vec<SPAExpressionRuntimeParameter>,
    pub bone_input_node_hashes: Vec<i32>,
    pub bone_output_node_hashes: Vec<i32>,
    pub bone_indices: Vec<i32>,
    pub globals_node_hash: i32,
}

pub const SKINNEDPROCEDURALANIMATIONEXPRESSION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkinnedProceduralAnimationExpression",
    flags: MemberInfoFlags::new(73),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Graph",
                flags: MemberInfoFlags::new(0),
                field_type: SKINNEDPROCEDURALANIMATIONEXPRESSIONGRAPH_TYPE_INFO,
                rust_offset: offset_of!(SkinnedProceduralAnimationExpression, graph),
            },
            FieldInfoData {
                name: "RuntimeParameters",
                flags: MemberInfoFlags::new(144),
                field_type: SPAEXPRESSIONRUNTIMEPARAMETER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SkinnedProceduralAnimationExpression, runtime_parameters),
            },
            FieldInfoData {
                name: "BoneInputNodeHashes",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SkinnedProceduralAnimationExpression, bone_input_node_hashes),
            },
            FieldInfoData {
                name: "BoneOutputNodeHashes",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SkinnedProceduralAnimationExpression, bone_output_node_hashes),
            },
            FieldInfoData {
                name: "BoneIndices",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SkinnedProceduralAnimationExpression, bone_indices),
            },
            FieldInfoData {
                name: "GlobalsNodeHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SkinnedProceduralAnimationExpression, globals_node_hash),
            },
        ],
    }),
    array_type: Some(SKINNEDPROCEDURALANIMATIONEXPRESSION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SkinnedProceduralAnimationExpression {
    fn type_info() -> &'static TypeInfo {
        SKINNEDPROCEDURALANIMATIONEXPRESSION_TYPE_INFO
    }
}


pub const SKINNEDPROCEDURALANIMATIONEXPRESSION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkinnedProceduralAnimationExpression-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("SkinnedProceduralAnimationExpression-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SkinnedProceduralAnimationExpressionGraph {
}

pub const SKINNEDPROCEDURALANIMATIONEXPRESSIONGRAPH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkinnedProceduralAnimationExpressionGraph",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EXPRESSIONFUNCTIONTYPEINFOASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SKINNEDPROCEDURALANIMATIONEXPRESSIONGRAPH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SkinnedProceduralAnimationExpressionGraph {
    fn type_info() -> &'static TypeInfo {
        SKINNEDPROCEDURALANIMATIONEXPRESSIONGRAPH_TYPE_INFO
    }
}


pub const SKINNEDPROCEDURALANIMATIONEXPRESSIONGRAPH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkinnedProceduralAnimationExpressionGraph-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("SkinnedProceduralAnimationExpressionGraph-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SPAExpressionEditorBone {
    pub node_id: super::core::Guid,
    pub bone_name: String,
}

pub const SPAEXPRESSIONEDITORBONE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SPAExpressionEditorBone",
    flags: MemberInfoFlags::new(73),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "NodeId",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(SPAExpressionEditorBone, node_id),
            },
            FieldInfoData {
                name: "BoneName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(SPAExpressionEditorBone, bone_name),
            },
        ],
    }),
    array_type: Some(SPAEXPRESSIONEDITORBONE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SPAExpressionEditorBone {
    fn type_info() -> &'static TypeInfo {
        SPAEXPRESSIONEDITORBONE_TYPE_INFO
    }
}


pub const SPAEXPRESSIONEDITORBONE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SPAExpressionEditorBone-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("SPAExpressionEditorBone-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SPAExpressionRuntimeParameter {
    pub node_hash: i32,
    pub float_value: f32,
    pub int_value: i32,
    pub bool_value: bool,
    pub parameter_type: SPAExpressionParameterType,
}

pub const SPAEXPRESSIONRUNTIMEPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SPAExpressionRuntimeParameter",
    flags: MemberInfoFlags::new(36937),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "NodeHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SPAExpressionRuntimeParameter, node_hash),
            },
            FieldInfoData {
                name: "FloatValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SPAExpressionRuntimeParameter, float_value),
            },
            FieldInfoData {
                name: "IntValue",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SPAExpressionRuntimeParameter, int_value),
            },
            FieldInfoData {
                name: "BoolValue",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SPAExpressionRuntimeParameter, bool_value),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: SPAEXPRESSIONPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(SPAExpressionRuntimeParameter, parameter_type),
            },
        ],
    }),
    array_type: Some(SPAEXPRESSIONRUNTIMEPARAMETER_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for SPAExpressionRuntimeParameter {
    fn type_info() -> &'static TypeInfo {
        SPAEXPRESSIONRUNTIMEPARAMETER_TYPE_INFO
    }
}


pub const SPAEXPRESSIONRUNTIMEPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SPAExpressionRuntimeParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("SPAExpressionRuntimeParameter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SPAExpressionEditorParameter {
    pub node_id: super::core::Guid,
    pub parameter_type: SPAExpressionParameterType,
    pub string_value: String,
    pub float_value: f32,
    pub int_value: i32,
    pub bool_value: bool,
}

pub const SPAEXPRESSIONEDITORPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SPAExpressionEditorParameter",
    flags: MemberInfoFlags::new(73),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "NodeId",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(SPAExpressionEditorParameter, node_id),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: SPAEXPRESSIONPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(SPAExpressionEditorParameter, parameter_type),
            },
            FieldInfoData {
                name: "StringValue",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(SPAExpressionEditorParameter, string_value),
            },
            FieldInfoData {
                name: "FloatValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SPAExpressionEditorParameter, float_value),
            },
            FieldInfoData {
                name: "IntValue",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SPAExpressionEditorParameter, int_value),
            },
            FieldInfoData {
                name: "BoolValue",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SPAExpressionEditorParameter, bool_value),
            },
        ],
    }),
    array_type: Some(SPAEXPRESSIONEDITORPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SPAExpressionEditorParameter {
    fn type_info() -> &'static TypeInfo {
        SPAEXPRESSIONEDITORPARAMETER_TYPE_INFO
    }
}


pub const SPAEXPRESSIONEDITORPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SPAExpressionEditorParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("SPAExpressionEditorParameter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SPAExpressionParameterType {
    #[default]
    SPAExpressionParameterType_Invalid = 0,
    SPAExpressionParameterType_Float = 1,
    SPAExpressionParameterType_Int = 2,
    SPAExpressionParameterType_Bool = 3,
    SPAExpressionParameterType_BoneQuery = 4,
}

pub const SPAEXPRESSIONPARAMETERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SPAExpressionParameterType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SPAEXPRESSIONPARAMETERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SPAExpressionParameterType {
    fn type_info() -> &'static TypeInfo {
        SPAEXPRESSIONPARAMETERTYPE_TYPE_INFO
    }
}


pub const SPAEXPRESSIONPARAMETERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SPAExpressionParameterType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("SPAExpressionParameterType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MeshSubsetCategoryFlags {
    #[default]
    MeshSubsetCategoryFlags_Opaque = 1,
    MeshSubsetCategoryFlags_Transparent = 2,
    MeshSubsetCategoryFlags_TransparentDecal = 4,
    MeshSubsetCategoryFlags_ZOnly = 8,
    MeshSubsetCategoryFlags_Shadow = 16,
    MeshSubsetCategoryFlags_DynamicReflection = 32,
    MeshSubsetCategoryFlags_PlanarReflection = 64,
    MeshSubsetCategoryFlags_StaticReflection = 128,
    MeshSubsetCategoryFlags_DistantShadowCache = 4096,
    MeshSubsetCategoryFlags_RootShaderEffect = 16384,
    MeshSubsetCategoryFlags_ShadowOverride = 256,
    MeshSubsetCategoryFlags_DynamicReflectionOverride = 512,
    MeshSubsetCategoryFlags_PlanarReflectionOverride = 1024,
    MeshSubsetCategoryFlags_StaticReflectionOverride = 2048,
    MeshSubsetCategoryFlags_DistantShadowCacheOverride = 8192,
    MeshSubsetCategoryFlags_RootShaderEffectOverride = 32768,
    MeshSubsetCategoryFlags_Normal = 16391,
    MeshSubsetCategoryFlags_All = 65535,
}

pub const MESHSUBSETCATEGORYFLAGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshSubsetCategoryFlags",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(MESHSUBSETCATEGORYFLAGS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshSubsetCategoryFlags {
    fn type_info() -> &'static TypeInfo {
        MESHSUBSETCATEGORYFLAGS_TYPE_INFO
    }
}


pub const MESHSUBSETCATEGORYFLAGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshSubsetCategoryFlags-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshSubsetCategoryFlags-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MeshSubsetCategory {
    #[default]
    MeshSubsetCategory_Opaque = 0,
    MeshSubsetCategory_Transparent = 1,
    MeshSubsetCategory_TransparentDecal = 2,
    MeshSubsetCategory_ZOnly = 3,
    MeshSubsetCategory_Shadow = 4,
    MeshSubsetCategoryCount = 5,
}

pub const MESHSUBSETCATEGORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshSubsetCategory",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(MESHSUBSETCATEGORY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshSubsetCategory {
    fn type_info() -> &'static TypeInfo {
        MESHSUBSETCATEGORY_TYPE_INFO
    }
}


pub const MESHSUBSETCATEGORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshSubsetCategory-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshSubsetCategory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MeshLimits {
    #[default]
    MaxMeshLodCount = 6,
}

pub const MESHLIMITS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshLimits",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(MESHLIMITS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshLimits {
    fn type_info() -> &'static TypeInfo {
        MESHLIMITS_TYPE_INFO
    }
}


pub const MESHLIMITS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshLimits-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshLimits-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MeshStreamingSettings {
    pub enable: bool,
    pub update_enable: bool,
    pub update_job_enable: bool,
    pub priority_job_enable: bool,
    pub priority_spu_job_enable: bool,
    pub use_slow_texture_prio: bool,
    pub instant_unloading_enable: bool,
    pub max_unload_count_per_frame: u32,
    pub async_creates_enable: bool,
    pub dx_immutable_usage_enable: bool,
    pub override_pool_sizes: bool,
    pub pool_size: u32,
    pub pool_headroom_size: u32,
    pub pool_max_alloc_count: u32,
    pub cpu_pool_enabled: bool,
    pub cpu_pool_size_scale: f32,
    pub defrag_enable: bool,
    pub defrag_transfers_enable: bool,
    pub defrag_transfer_limit: u32,
    pub defrag_search_limit: u32,
    pub defrag_job_count: u32,
    pub force_lod: i32,
    pub max_pending_load_count: u32,
    pub prioritize_visible_meshes_first_enable: bool,
    pub prioritize_visible_lods_first_enable: bool,
    pub prioritize_visible_loads_enable: bool,
    pub prioritize_textures_enable: bool,
    pub highest_priority_enable: bool,
    pub prioritize_nearest_point_enable: bool,
    pub two_phase_prio_enable: bool,
    pub distance_min: f32,
    pub draw_instance_boxes_enable: bool,
    pub draw_stats_enable: bool,
    pub draw_missing_list_enable: bool,
    pub draw_priority_list_enable: bool,
    pub draw_loading_list_enable: bool,
    pub draw_mesh_list_enable: bool,
    pub draw_non_streamed_list_enable: bool,
    pub list_view_page_index: u32,
    pub list_view_sort_order: u32,
    pub dump_loaded_list: bool,
    pub dump_loaded_list_file_name: String,
    pub dump_instance_list: bool,
    pub dump_instance_list_file_name: String,
    pub dump_pool_allocations: bool,
    pub reserved_positioned_instance_count: u32,
    pub reserved_distanced_instance_count: u32,
    pub use_sweepable_pool: bool,
    pub sweepable_page_size: u32,
    pub sweepable_page_align: u32,
    pub sweepable_min_pages: u32,
    pub sweepable_reserved_pages: u32,
    pub sweepable_page_allocation_limit: u32,
    pub sweepable_direct_allocation_alignment_waste_threshold: i32,
    pub sweepable_use_virtual_pool: bool,
    pub sweepable_virtual_pool_initial_virtual_size: u32,
    pub sweepable_virtual_pool_extend_virtual_size: u32,
    pub sweepable_virtual_pool_max_delayed_operations: u32,
    pub sweepable_virtual_pool_can_delay_allocations: bool,
}

pub const MESHSTREAMINGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshStreamingSettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, enable),
            },
            FieldInfoData {
                name: "UpdateEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, update_enable),
            },
            FieldInfoData {
                name: "UpdateJobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, update_job_enable),
            },
            FieldInfoData {
                name: "PriorityJobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, priority_job_enable),
            },
            FieldInfoData {
                name: "PrioritySpuJobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, priority_spu_job_enable),
            },
            FieldInfoData {
                name: "UseSlowTexturePrio",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, use_slow_texture_prio),
            },
            FieldInfoData {
                name: "InstantUnloadingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, instant_unloading_enable),
            },
            FieldInfoData {
                name: "MaxUnloadCountPerFrame",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, max_unload_count_per_frame),
            },
            FieldInfoData {
                name: "AsyncCreatesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, async_creates_enable),
            },
            FieldInfoData {
                name: "DxImmutableUsageEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, dx_immutable_usage_enable),
            },
            FieldInfoData {
                name: "OverridePoolSizes",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, override_pool_sizes),
            },
            FieldInfoData {
                name: "PoolSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, pool_size),
            },
            FieldInfoData {
                name: "PoolHeadroomSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, pool_headroom_size),
            },
            FieldInfoData {
                name: "PoolMaxAllocCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, pool_max_alloc_count),
            },
            FieldInfoData {
                name: "CpuPoolEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, cpu_pool_enabled),
            },
            FieldInfoData {
                name: "CpuPoolSizeScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, cpu_pool_size_scale),
            },
            FieldInfoData {
                name: "DefragEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, defrag_enable),
            },
            FieldInfoData {
                name: "DefragTransfersEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, defrag_transfers_enable),
            },
            FieldInfoData {
                name: "DefragTransferLimit",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, defrag_transfer_limit),
            },
            FieldInfoData {
                name: "DefragSearchLimit",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, defrag_search_limit),
            },
            FieldInfoData {
                name: "DefragJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, defrag_job_count),
            },
            FieldInfoData {
                name: "ForceLod",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, force_lod),
            },
            FieldInfoData {
                name: "MaxPendingLoadCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, max_pending_load_count),
            },
            FieldInfoData {
                name: "PrioritizeVisibleMeshesFirstEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, prioritize_visible_meshes_first_enable),
            },
            FieldInfoData {
                name: "PrioritizeVisibleLodsFirstEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, prioritize_visible_lods_first_enable),
            },
            FieldInfoData {
                name: "PrioritizeVisibleLoadsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, prioritize_visible_loads_enable),
            },
            FieldInfoData {
                name: "PrioritizeTexturesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, prioritize_textures_enable),
            },
            FieldInfoData {
                name: "HighestPriorityEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, highest_priority_enable),
            },
            FieldInfoData {
                name: "PrioritizeNearestPointEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, prioritize_nearest_point_enable),
            },
            FieldInfoData {
                name: "TwoPhasePrioEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, two_phase_prio_enable),
            },
            FieldInfoData {
                name: "DistanceMin",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, distance_min),
            },
            FieldInfoData {
                name: "DrawInstanceBoxesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, draw_instance_boxes_enable),
            },
            FieldInfoData {
                name: "DrawStatsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, draw_stats_enable),
            },
            FieldInfoData {
                name: "DrawMissingListEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, draw_missing_list_enable),
            },
            FieldInfoData {
                name: "DrawPriorityListEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, draw_priority_list_enable),
            },
            FieldInfoData {
                name: "DrawLoadingListEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, draw_loading_list_enable),
            },
            FieldInfoData {
                name: "DrawMeshListEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, draw_mesh_list_enable),
            },
            FieldInfoData {
                name: "DrawNonStreamedListEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, draw_non_streamed_list_enable),
            },
            FieldInfoData {
                name: "ListViewPageIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, list_view_page_index),
            },
            FieldInfoData {
                name: "ListViewSortOrder",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, list_view_sort_order),
            },
            FieldInfoData {
                name: "DumpLoadedList",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, dump_loaded_list),
            },
            FieldInfoData {
                name: "DumpLoadedListFileName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, dump_loaded_list_file_name),
            },
            FieldInfoData {
                name: "DumpInstanceList",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, dump_instance_list),
            },
            FieldInfoData {
                name: "DumpInstanceListFileName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, dump_instance_list_file_name),
            },
            FieldInfoData {
                name: "DumpPoolAllocations",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, dump_pool_allocations),
            },
            FieldInfoData {
                name: "ReservedPositionedInstanceCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, reserved_positioned_instance_count),
            },
            FieldInfoData {
                name: "ReservedDistancedInstanceCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, reserved_distanced_instance_count),
            },
            FieldInfoData {
                name: "UseSweepablePool",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, use_sweepable_pool),
            },
            FieldInfoData {
                name: "SweepablePageSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, sweepable_page_size),
            },
            FieldInfoData {
                name: "SweepablePageAlign",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, sweepable_page_align),
            },
            FieldInfoData {
                name: "SweepableMinPages",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, sweepable_min_pages),
            },
            FieldInfoData {
                name: "SweepableReservedPages",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, sweepable_reserved_pages),
            },
            FieldInfoData {
                name: "SweepablePageAllocationLimit",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, sweepable_page_allocation_limit),
            },
            FieldInfoData {
                name: "SweepableDirectAllocationAlignmentWasteThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, sweepable_direct_allocation_alignment_waste_threshold),
            },
            FieldInfoData {
                name: "SweepableUseVirtualPool",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, sweepable_use_virtual_pool),
            },
            FieldInfoData {
                name: "SweepableVirtualPoolInitialVirtualSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, sweepable_virtual_pool_initial_virtual_size),
            },
            FieldInfoData {
                name: "SweepableVirtualPoolExtendVirtualSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, sweepable_virtual_pool_extend_virtual_size),
            },
            FieldInfoData {
                name: "SweepableVirtualPoolMaxDelayedOperations",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, sweepable_virtual_pool_max_delayed_operations),
            },
            FieldInfoData {
                name: "SweepableVirtualPoolCanDelayAllocations",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshStreamingSettings, sweepable_virtual_pool_can_delay_allocations),
            },
        ],
    }),
    array_type: Some(MESHSTREAMINGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshStreamingSettings {
    fn type_info() -> &'static TypeInfo {
        MESHSTREAMINGSETTINGS_TYPE_INFO
    }
}


pub const MESHSTREAMINGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshStreamingSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshStreamingSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MeshSettings {
    pub override_shaders_shader_name: String,
    pub override_shaders_mesh_name: String,
    pub force_lod: i32,
    pub loading_enabled: bool,
    pub force_load_streaming_frame_delay: u32,
    pub lod_fade_enable: bool,
    pub force_shadow_slice_lod_bias: i32,
    pub global_lod_scale: f32,
    pub shadow_distance_scale: f32,
    pub procedural_animation_max_distance: f32,
    pub tessellation_enable: bool,
    pub tessellation_back_face_culling_enable: bool,
    pub tessellation_screen_space_adative_enable: bool,
    pub planar_reflection_tessellation_enable: bool,
    pub dynamic_envmap_tessellation_enable: bool,
    pub tessellation_max_factor: f32,
    pub tessellation_force_tessellation_factor: f32,
    pub tessellation_max_distance: f32,
    pub tessellation_max_distance_fade: f32,
    pub tessellation_max_distance_scale: f32,
    pub tessellation_max_distance_cull_scale: f32,
    pub cast_shadow_quality_level: super::core::QualityLevel,
    pub cast_planar_reflection_quality_level: super::core::QualityLevel,
    pub cast_dynamic_reflection_quality_level: super::core::QualityLevel,
    pub cast_static_reflection_quality_level: super::core::QualityLevel,
}

pub const MESHSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshSettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "OverrideShadersShaderName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, override_shaders_shader_name),
            },
            FieldInfoData {
                name: "OverrideShadersMeshName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, override_shaders_mesh_name),
            },
            FieldInfoData {
                name: "ForceLod",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, force_lod),
            },
            FieldInfoData {
                name: "LoadingEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, loading_enabled),
            },
            FieldInfoData {
                name: "ForceLoadStreamingFrameDelay",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, force_load_streaming_frame_delay),
            },
            FieldInfoData {
                name: "LodFadeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, lod_fade_enable),
            },
            FieldInfoData {
                name: "ForceShadowSliceLodBias",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, force_shadow_slice_lod_bias),
            },
            FieldInfoData {
                name: "GlobalLodScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, global_lod_scale),
            },
            FieldInfoData {
                name: "ShadowDistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, shadow_distance_scale),
            },
            FieldInfoData {
                name: "ProceduralAnimationMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, procedural_animation_max_distance),
            },
            FieldInfoData {
                name: "TessellationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, tessellation_enable),
            },
            FieldInfoData {
                name: "TessellationBackFaceCullingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, tessellation_back_face_culling_enable),
            },
            FieldInfoData {
                name: "TessellationScreenSpaceAdativeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, tessellation_screen_space_adative_enable),
            },
            FieldInfoData {
                name: "PlanarReflectionTessellationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, planar_reflection_tessellation_enable),
            },
            FieldInfoData {
                name: "DynamicEnvmapTessellationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, dynamic_envmap_tessellation_enable),
            },
            FieldInfoData {
                name: "TessellationMaxFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, tessellation_max_factor),
            },
            FieldInfoData {
                name: "TessellationForceTessellationFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, tessellation_force_tessellation_factor),
            },
            FieldInfoData {
                name: "TessellationMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, tessellation_max_distance),
            },
            FieldInfoData {
                name: "TessellationMaxDistanceFade",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, tessellation_max_distance_fade),
            },
            FieldInfoData {
                name: "TessellationMaxDistanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, tessellation_max_distance_scale),
            },
            FieldInfoData {
                name: "TessellationMaxDistanceCullScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, tessellation_max_distance_cull_scale),
            },
            FieldInfoData {
                name: "CastShadowQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, cast_shadow_quality_level),
            },
            FieldInfoData {
                name: "CastPlanarReflectionQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, cast_planar_reflection_quality_level),
            },
            FieldInfoData {
                name: "CastDynamicReflectionQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, cast_dynamic_reflection_quality_level),
            },
            FieldInfoData {
                name: "CastStaticReflectionQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: QUALITYLEVEL_TYPE_INFO,
                rust_offset: offset_of!(MeshSettings, cast_static_reflection_quality_level),
            },
        ],
    }),
    array_type: Some(MESHSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshSettings {
    fn type_info() -> &'static TypeInfo {
        MESHSETTINGS_TYPE_INFO
    }
}


pub const MESHSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SkinningMeshComputeOutput {
    #[default]
    SkinningMeshComputeOutput_SkinnedPositions = 0,
    SkinningMeshComputeOutput_SkinnedTangents = 1,
    SkinningMeshComputeOutput_SkinnedBinormals = 2,
    SkinningMeshComputeOutput_SkinnedNormals = 3,
    SkinningMeshComputeOutputCount = 4,
}

pub const SKINNINGMESHCOMPUTEOUTPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkinningMeshComputeOutput",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SKINNINGMESHCOMPUTEOUTPUT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SkinningMeshComputeOutput {
    fn type_info() -> &'static TypeInfo {
        SKINNINGMESHCOMPUTEOUTPUT_TYPE_INFO
    }
}


pub const SKINNINGMESHCOMPUTEOUTPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkinningMeshComputeOutput-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("SkinningMeshComputeOutput-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SkinningMeshComputeInput {
    #[default]
    SkinningMeshComputeInput_BoneTransforms = 0,
    SkinningMeshComputeInput_BoneWeights = 1,
    SkinningMeshComputeInput_BoneIndices = 2,
    SkinningMeshComputeInput_Positions = 3,
    SkinningMeshComputeInput_Tangents = 4,
    SkinningMeshComputeInput_Binormals = 5,
    SkinningMeshComputeInput_Normals = 6,
    SkinningMeshComputeInputCount = 7,
}

pub const SKINNINGMESHCOMPUTEINPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkinningMeshComputeInput",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SKINNINGMESHCOMPUTEINPUT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SkinningMeshComputeInput {
    fn type_info() -> &'static TypeInfo {
        SKINNINGMESHCOMPUTEINPUT_TYPE_INFO
    }
}


pub const SKINNINGMESHCOMPUTEINPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkinningMeshComputeInput-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("SkinningMeshComputeInput-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum VertexNormalMeshComputeOutput {
    #[default]
    VertexNormalMeshComputeOutput_Normals = 0,
    VertexNormalMeshComputeOutputCount = 1,
}

pub const VERTEXNORMALMESHCOMPUTEOUTPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexNormalMeshComputeOutput",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(VERTEXNORMALMESHCOMPUTEOUTPUT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VertexNormalMeshComputeOutput {
    fn type_info() -> &'static TypeInfo {
        VERTEXNORMALMESHCOMPUTEOUTPUT_TYPE_INFO
    }
}


pub const VERTEXNORMALMESHCOMPUTEOUTPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexNormalMeshComputeOutput-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("VertexNormalMeshComputeOutput-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum VertexNormalMeshComputeInput {
    #[default]
    VertexNormalMeshComputeInput_FaceNormals = 0,
    VertexNormalMeshComputeInput_Adjacency = 1,
    VertexNormalMeshComputeInput_OriginalNormals = 2,
    VertexNormalMeshComputeInputCount = 3,
}

pub const VERTEXNORMALMESHCOMPUTEINPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexNormalMeshComputeInput",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(VERTEXNORMALMESHCOMPUTEINPUT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VertexNormalMeshComputeInput {
    fn type_info() -> &'static TypeInfo {
        VERTEXNORMALMESHCOMPUTEINPUT_TYPE_INFO
    }
}


pub const VERTEXNORMALMESHCOMPUTEINPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexNormalMeshComputeInput-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("VertexNormalMeshComputeInput-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum FaceNormalMeshComputeOutput {
    #[default]
    FaceNormalMeshComputeOutput_FaceNormals = 0,
    FaceNormalMeshComputeOutputCount = 1,
}

pub const FACENORMALMESHCOMPUTEOUTPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceNormalMeshComputeOutput",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(FACENORMALMESHCOMPUTEOUTPUT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FaceNormalMeshComputeOutput {
    fn type_info() -> &'static TypeInfo {
        FACENORMALMESHCOMPUTEOUTPUT_TYPE_INFO
    }
}


pub const FACENORMALMESHCOMPUTEOUTPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceNormalMeshComputeOutput-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("FaceNormalMeshComputeOutput-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum FaceNormalMeshComputeInput {
    #[default]
    FaceNormalMeshComputeInput_Indices = 0,
    FaceNormalMeshComputeInput_Positions = 1,
    FaceNormalMeshComputeInputCount = 2,
}

pub const FACENORMALMESHCOMPUTEINPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceNormalMeshComputeInput",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(FACENORMALMESHCOMPUTEINPUT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FaceNormalMeshComputeInput {
    fn type_info() -> &'static TypeInfo {
        FACENORMALMESHCOMPUTEINPUT_TYPE_INFO
    }
}


pub const FACENORMALMESHCOMPUTEINPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FaceNormalMeshComputeInput-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("FaceNormalMeshComputeInput-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DynamicMorphMeshComputeOutput {
    #[default]
    DynamicMorphMeshComputeOutput_Positions = 0,
    DynamicMorphMeshComputeOutput_Normals = 1,
    DynamicMorphMeshComputeOutputCount = 2,
}

pub const DYNAMICMORPHMESHCOMPUTEOUTPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicMorphMeshComputeOutput",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(DYNAMICMORPHMESHCOMPUTEOUTPUT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DynamicMorphMeshComputeOutput {
    fn type_info() -> &'static TypeInfo {
        DYNAMICMORPHMESHCOMPUTEOUTPUT_TYPE_INFO
    }
}


pub const DYNAMICMORPHMESHCOMPUTEOUTPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicMorphMeshComputeOutput-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("DynamicMorphMeshComputeOutput-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DynamicMorphMeshComputeInput {
    #[default]
    DynamicMorphMeshComputeInput_Positions = 0,
    DynamicMorphMeshComputeInput_Normals = 1,
    DynamicMorphMeshComputeInput_MorphTargets = 2,
    DynamicMorphMeshComputeInput_MorphChannels = 3,
    DynamicMorphMeshComputeInputCount = 4,
}

pub const DYNAMICMORPHMESHCOMPUTEINPUT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicMorphMeshComputeInput",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(DYNAMICMORPHMESHCOMPUTEINPUT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DynamicMorphMeshComputeInput {
    fn type_info() -> &'static TypeInfo {
        DYNAMICMORPHMESHCOMPUTEINPUT_TYPE_INFO
    }
}


pub const DYNAMICMORPHMESHCOMPUTEINPUT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicMorphMeshComputeInput-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("DynamicMorphMeshComputeInput-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VertexElementInfoSlot {
    pub pin_name: String,
    pub format: VertexElementFormat,
    pub usage: VertexElementUsage,
    pub stride: u32,
}

pub const VERTEXELEMENTINFOSLOT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexElementInfoSlot",
    flags: MemberInfoFlags::new(73),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "PinName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VertexElementInfoSlot, pin_name),
            },
            FieldInfoData {
                name: "Format",
                flags: MemberInfoFlags::new(0),
                field_type: VERTEXELEMENTFORMAT_TYPE_INFO,
                rust_offset: offset_of!(VertexElementInfoSlot, format),
            },
            FieldInfoData {
                name: "Usage",
                flags: MemberInfoFlags::new(0),
                field_type: VERTEXELEMENTUSAGE_TYPE_INFO,
                rust_offset: offset_of!(VertexElementInfoSlot, usage),
            },
            FieldInfoData {
                name: "Stride",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VertexElementInfoSlot, stride),
            },
        ],
    }),
    array_type: Some(VERTEXELEMENTINFOSLOT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VertexElementInfoSlot {
    fn type_info() -> &'static TypeInfo {
        VERTEXELEMENTINFOSLOT_TYPE_INFO
    }
}


pub const VERTEXELEMENTINFOSLOT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexElementInfoSlot-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("VertexElementInfoSlot-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshComputeAsset {
    pub runtime_nodes: Vec<MeshComputeRuntimeNode>,
    pub runtime_buffers: Vec<MeshComputeRuntimeBuffer>,
    pub output_node: MeshComputeRuntimeOutputNode,
}

pub const MESHCOMPUTEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeAsset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHCOMPUTEBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RuntimeNodes",
                flags: MemberInfoFlags::new(144),
                field_type: MESHCOMPUTERUNTIMENODE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MeshComputeAsset, runtime_nodes),
            },
            FieldInfoData {
                name: "RuntimeBuffers",
                flags: MemberInfoFlags::new(144),
                field_type: MESHCOMPUTERUNTIMEBUFFER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MeshComputeAsset, runtime_buffers),
            },
            FieldInfoData {
                name: "OutputNode",
                flags: MemberInfoFlags::new(0),
                field_type: MESHCOMPUTERUNTIMEOUTPUTNODE_TYPE_INFO,
                rust_offset: offset_of!(MeshComputeAsset, output_node),
            },
        ],
    }),
    array_type: Some(MESHCOMPUTEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshComputeAsset {
    fn type_info() -> &'static TypeInfo {
        MESHCOMPUTEASSET_TYPE_INFO
    }
}


pub const MESHCOMPUTEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshComputeAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshComputeRuntimeOutputNode {
    pub runtime_node_index: u32,
    pub output_type: i32,
}

pub const MESHCOMPUTERUNTIMEOUTPUTNODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeRuntimeOutputNode",
    flags: MemberInfoFlags::new(36937),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "RuntimeNodeIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshComputeRuntimeOutputNode, runtime_node_index),
            },
            FieldInfoData {
                name: "OutputType",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MeshComputeRuntimeOutputNode, output_type),
            },
        ],
    }),
    array_type: Some(MESHCOMPUTERUNTIMEOUTPUTNODE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MeshComputeRuntimeOutputNode {
    fn type_info() -> &'static TypeInfo {
        MESHCOMPUTERUNTIMEOUTPUTNODE_TYPE_INFO
    }
}


pub const MESHCOMPUTERUNTIMEOUTPUTNODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeRuntimeOutputNode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshComputeRuntimeOutputNode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshComputeRuntimeBuffer {
    pub buffer_type: MeshComputeRuntimeBufferType,
    pub vertex_element: VertexElementInfo,
    pub subset_offsets: Vec<u32>,
    pub element_count: u32,
}

pub const MESHCOMPUTERUNTIMEBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeRuntimeBuffer",
    flags: MemberInfoFlags::new(73),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BufferType",
                flags: MemberInfoFlags::new(0),
                field_type: MESHCOMPUTERUNTIMEBUFFERTYPE_TYPE_INFO,
                rust_offset: offset_of!(MeshComputeRuntimeBuffer, buffer_type),
            },
            FieldInfoData {
                name: "VertexElement",
                flags: MemberInfoFlags::new(0),
                field_type: VERTEXELEMENTINFO_TYPE_INFO,
                rust_offset: offset_of!(MeshComputeRuntimeBuffer, vertex_element),
            },
            FieldInfoData {
                name: "SubsetOffsets",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MeshComputeRuntimeBuffer, subset_offsets),
            },
            FieldInfoData {
                name: "ElementCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshComputeRuntimeBuffer, element_count),
            },
        ],
    }),
    array_type: Some(MESHCOMPUTERUNTIMEBUFFER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshComputeRuntimeBuffer {
    fn type_info() -> &'static TypeInfo {
        MESHCOMPUTERUNTIMEBUFFER_TYPE_INFO
    }
}


pub const MESHCOMPUTERUNTIMEBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeRuntimeBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshComputeRuntimeBuffer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VertexElementInfo {
    pub format: VertexElementFormat,
    pub usage: VertexElementUsage,
    pub stride: u32,
}

pub const VERTEXELEMENTINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexElementInfo",
    flags: MemberInfoFlags::new(36937),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Format",
                flags: MemberInfoFlags::new(0),
                field_type: VERTEXELEMENTFORMAT_TYPE_INFO,
                rust_offset: offset_of!(VertexElementInfo, format),
            },
            FieldInfoData {
                name: "Usage",
                flags: MemberInfoFlags::new(0),
                field_type: VERTEXELEMENTUSAGE_TYPE_INFO,
                rust_offset: offset_of!(VertexElementInfo, usage),
            },
            FieldInfoData {
                name: "Stride",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VertexElementInfo, stride),
            },
        ],
    }),
    array_type: Some(VERTEXELEMENTINFO_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for VertexElementInfo {
    fn type_info() -> &'static TypeInfo {
        VERTEXELEMENTINFO_TYPE_INFO
    }
}


pub const VERTEXELEMENTINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexElementInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("VertexElementInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MeshComputeRuntimeBufferType {
    #[default]
    MeshComputeRuntimeBuffer_Input = 0,
    MeshComputeRuntimeBuffer_Transient = 1,
    MeshComputeRuntimeBuffer_Output = 2,
    MeshComputeRuntimeBuffer_Invalid = 3,
    MeshComputeRuntimeBufferCount = 4,
}

pub const MESHCOMPUTERUNTIMEBUFFERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeRuntimeBufferType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(MESHCOMPUTERUNTIMEBUFFERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MeshComputeRuntimeBufferType {
    fn type_info() -> &'static TypeInfo {
        MESHCOMPUTERUNTIMEBUFFERTYPE_TYPE_INFO
    }
}


pub const MESHCOMPUTERUNTIMEBUFFERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeRuntimeBufferType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshComputeRuntimeBufferType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshComputeRuntimeNode {
    pub node_resource: super::core::ResourceRef,
    pub node_type: i32,
    pub buffer_indices: Vec<u32>,
}

pub const MESHCOMPUTERUNTIMENODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeRuntimeNode",
    flags: MemberInfoFlags::new(73),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "NodeResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(MeshComputeRuntimeNode, node_resource),
            },
            FieldInfoData {
                name: "NodeType",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MeshComputeRuntimeNode, node_type),
            },
            FieldInfoData {
                name: "BufferIndices",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MeshComputeRuntimeNode, buffer_indices),
            },
        ],
    }),
    array_type: Some(MESHCOMPUTERUNTIMENODE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshComputeRuntimeNode {
    fn type_info() -> &'static TypeInfo {
        MESHCOMPUTERUNTIMENODE_TYPE_INFO
    }
}


pub const MESHCOMPUTERUNTIMENODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeRuntimeNode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshComputeRuntimeNode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RadiosityMaterial {
    pub name: String,
    pub color: super::core::Vec3,
    pub emissive_intensity: f32,
    pub opacity: f32,
    pub backface_type: super::render_base::RadiosityBackfaceType,
}

pub const RADIOSITYMATERIAL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterial",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterial, name),
            },
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterial, color),
            },
            FieldInfoData {
                name: "EmissiveIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterial, emissive_intensity),
            },
            FieldInfoData {
                name: "Opacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterial, opacity),
            },
            FieldInfoData {
                name: "BackfaceType",
                flags: MemberInfoFlags::new(0),
                field_type: RADIOSITYBACKFACETYPE_TYPE_INFO,
                rust_offset: offset_of!(RadiosityMaterial, backface_type),
            },
        ],
    }),
    array_type: Some(RADIOSITYMATERIAL_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RadiosityMaterial {
    fn type_info() -> &'static TypeInfo {
        RADIOSITYMATERIAL_TYPE_INFO
    }
}


pub const RADIOSITYMATERIAL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiosityMaterial-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RadiosityMaterial-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MeshVariationDatabase {
    pub entries: Vec<MeshVariationDatabaseEntry>,
    pub redirect_entries: Vec<MeshVariationDatabaseRedirectEntry>,
}

pub const MESHVARIATIONDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshVariationDatabase",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Entries",
                flags: MemberInfoFlags::new(144),
                field_type: MESHVARIATIONDATABASEENTRY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MeshVariationDatabase, entries),
            },
            FieldInfoData {
                name: "RedirectEntries",
                flags: MemberInfoFlags::new(144),
                field_type: MESHVARIATIONDATABASEREDIRECTENTRY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MeshVariationDatabase, redirect_entries),
            },
        ],
    }),
    array_type: Some(MESHVARIATIONDATABASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshVariationDatabase {
    fn type_info() -> &'static TypeInfo {
        MESHVARIATIONDATABASE_TYPE_INFO
    }
}


pub const MESHVARIATIONDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshVariationDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshVariationDatabase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MeshVariationDatabaseRedirectEntry {
    pub mesh: MeshAsset,
    pub variation_asset_name_hash: u32,
}

pub const MESHVARIATIONDATABASEREDIRECTENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshVariationDatabaseRedirectEntry",
    flags: MemberInfoFlags::new(73),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: MESHASSET_TYPE_INFO,
                rust_offset: offset_of!(MeshVariationDatabaseRedirectEntry, mesh),
            },
            FieldInfoData {
                name: "VariationAssetNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshVariationDatabaseRedirectEntry, variation_asset_name_hash),
            },
        ],
    }),
    array_type: Some(MESHVARIATIONDATABASEREDIRECTENTRY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshVariationDatabaseRedirectEntry {
    fn type_info() -> &'static TypeInfo {
        MESHVARIATIONDATABASEREDIRECTENTRY_TYPE_INFO
    }
}


pub const MESHVARIATIONDATABASEREDIRECTENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshVariationDatabaseRedirectEntry-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshVariationDatabaseRedirectEntry-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MeshVariationDatabaseEntry {
    pub mesh: MeshAsset,
    pub variation_asset_name_hash: u32,
    pub materials: Vec<MeshVariationDatabaseMaterial>,
}

pub const MESHVARIATIONDATABASEENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshVariationDatabaseEntry",
    flags: MemberInfoFlags::new(73),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Mesh",
                flags: MemberInfoFlags::new(0),
                field_type: MESHASSET_TYPE_INFO,
                rust_offset: offset_of!(MeshVariationDatabaseEntry, mesh),
            },
            FieldInfoData {
                name: "VariationAssetNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshVariationDatabaseEntry, variation_asset_name_hash),
            },
            FieldInfoData {
                name: "Materials",
                flags: MemberInfoFlags::new(144),
                field_type: MESHVARIATIONDATABASEMATERIAL_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MeshVariationDatabaseEntry, materials),
            },
        ],
    }),
    array_type: Some(MESHVARIATIONDATABASEENTRY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshVariationDatabaseEntry {
    fn type_info() -> &'static TypeInfo {
        MESHVARIATIONDATABASEENTRY_TYPE_INFO
    }
}


pub const MESHVARIATIONDATABASEENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshVariationDatabaseEntry-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshVariationDatabaseEntry-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MeshVariationDatabaseMaterial {
    pub material: MeshMaterial,
    pub material_variation: MeshMaterialVariation,
    pub surface_shader_id: u32,
    pub surface_shader_guid: super::core::Guid,
    pub material_id: u64,
    pub texture_parameters: Vec<super::render_base::TextureShaderParameter>,
}

pub const MESHVARIATIONDATABASEMATERIAL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshVariationDatabaseMaterial",
    flags: MemberInfoFlags::new(73),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Material",
                flags: MemberInfoFlags::new(0),
                field_type: MESHMATERIAL_TYPE_INFO,
                rust_offset: offset_of!(MeshVariationDatabaseMaterial, material),
            },
            FieldInfoData {
                name: "MaterialVariation",
                flags: MemberInfoFlags::new(0),
                field_type: MESHMATERIALVARIATION_TYPE_INFO,
                rust_offset: offset_of!(MeshVariationDatabaseMaterial, material_variation),
            },
            FieldInfoData {
                name: "SurfaceShaderId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshVariationDatabaseMaterial, surface_shader_id),
            },
            FieldInfoData {
                name: "SurfaceShaderGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(MeshVariationDatabaseMaterial, surface_shader_guid),
            },
            FieldInfoData {
                name: "MaterialId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(MeshVariationDatabaseMaterial, material_id),
            },
            FieldInfoData {
                name: "TextureParameters",
                flags: MemberInfoFlags::new(144),
                field_type: TEXTURESHADERPARAMETER_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MeshVariationDatabaseMaterial, texture_parameters),
            },
        ],
    }),
    array_type: Some(MESHVARIATIONDATABASEMATERIAL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshVariationDatabaseMaterial {
    fn type_info() -> &'static TypeInfo {
        MESHVARIATIONDATABASEMATERIAL_TYPE_INFO
    }
}


pub const MESHVARIATIONDATABASEMATERIAL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshVariationDatabaseMaterial-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshVariationDatabaseMaterial-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MeshMaterialVariation {
    pub shader: super::render_base::SurfaceShaderInstanceDataStruct,
}

pub const MESHMATERIALVARIATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshMaterialVariation",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterialVariation, shader),
            },
        ],
    }),
    array_type: Some(MESHMATERIALVARIATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshMaterialVariation {
    fn type_info() -> &'static TypeInfo {
        MESHMATERIALVARIATION_TYPE_INFO
    }
}


pub const MESHMATERIALVARIATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshMaterialVariation-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshMaterialVariation-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MeshMaterial {
    pub shader_instance: SurfaceShaderInstanceData,
    pub shader: super::render_base::SurfaceShaderInstanceDataStruct,
    pub cast_shadow: bool,
    pub tessellation_type: ShaderTessellationType,
    pub tessellation_triangle_size: f32,
    pub tessellation_max_distance: f32,
    pub back_face_cull_epsilon: f32,
    pub shape_factor: f32,
    pub displacement_map: super::render_base::TextureBaseAsset,
    pub displacement_scale: f32,
    pub displacement_bias: f32,
    pub smooth_edge_vertices: bool,
    pub displacement_map_tex_coord: u32,
    pub displacement_object_scale: bool,
    pub texture_space_rendering_enabled: bool,
    pub texture_space_reference_texture: super::render_base::TextureBaseAsset,
    pub texture_space_anchor_distance_texture: super::render_base::TextureBaseAsset,
    pub texture_space_penetration_distance_texture: super::render_base::TextureBaseAsset,
    pub texture_space_num_iterations: u32,
    pub texture_space_anchor_distance_multiplier: f32,
    pub texture_space_anchor_spring: f32,
    pub texture_space_penetration_distance: f32,
    pub texture_space_penetration_factor: f32,
    pub texture_space_aerodynamic_effect_scale: f32,
}

pub const MESHMATERIAL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshMaterial",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ShaderInstance",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATA_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, shader_instance),
            },
            FieldInfoData {
                name: "Shader",
                flags: MemberInfoFlags::new(0),
                field_type: SURFACESHADERINSTANCEDATASTRUCT_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, shader),
            },
            FieldInfoData {
                name: "CastShadow",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, cast_shadow),
            },
            FieldInfoData {
                name: "TessellationType",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERTESSELLATIONTYPE_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, tessellation_type),
            },
            FieldInfoData {
                name: "TessellationTriangleSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, tessellation_triangle_size),
            },
            FieldInfoData {
                name: "TessellationMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, tessellation_max_distance),
            },
            FieldInfoData {
                name: "BackFaceCullEpsilon",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, back_face_cull_epsilon),
            },
            FieldInfoData {
                name: "ShapeFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, shape_factor),
            },
            FieldInfoData {
                name: "DisplacementMap",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, displacement_map),
            },
            FieldInfoData {
                name: "DisplacementScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, displacement_scale),
            },
            FieldInfoData {
                name: "DisplacementBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, displacement_bias),
            },
            FieldInfoData {
                name: "SmoothEdgeVertices",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, smooth_edge_vertices),
            },
            FieldInfoData {
                name: "DisplacementMapTexCoord",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, displacement_map_tex_coord),
            },
            FieldInfoData {
                name: "DisplacementObjectScale",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, displacement_object_scale),
            },
            FieldInfoData {
                name: "TextureSpaceRenderingEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, texture_space_rendering_enabled),
            },
            FieldInfoData {
                name: "TextureSpaceReferenceTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, texture_space_reference_texture),
            },
            FieldInfoData {
                name: "TextureSpaceAnchorDistanceTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, texture_space_anchor_distance_texture),
            },
            FieldInfoData {
                name: "TextureSpacePenetrationDistanceTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, texture_space_penetration_distance_texture),
            },
            FieldInfoData {
                name: "TextureSpaceNumIterations",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, texture_space_num_iterations),
            },
            FieldInfoData {
                name: "TextureSpaceAnchorDistanceMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, texture_space_anchor_distance_multiplier),
            },
            FieldInfoData {
                name: "TextureSpaceAnchorSpring",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, texture_space_anchor_spring),
            },
            FieldInfoData {
                name: "TextureSpacePenetrationDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, texture_space_penetration_distance),
            },
            FieldInfoData {
                name: "TextureSpacePenetrationFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, texture_space_penetration_factor),
            },
            FieldInfoData {
                name: "TextureSpaceAerodynamicEffectScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshMaterial, texture_space_aerodynamic_effect_scale),
            },
        ],
    }),
    array_type: Some(MESHMATERIAL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshMaterial {
    fn type_info() -> &'static TypeInfo {
        MESHMATERIAL_TYPE_INFO
    }
}


pub const MESHMATERIAL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshMaterial-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshMaterial-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CompositeMeshAsset {
}

pub const COMPOSITEMESHASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompositeMeshAsset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(COMPOSITEMESHASSET_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CompositeMeshAsset {
    fn type_info() -> &'static TypeInfo {
        COMPOSITEMESHASSET_TYPE_INFO
    }
}


pub const COMPOSITEMESHASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompositeMeshAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("CompositeMeshAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SkinnedMeshAsset {
    pub bounding_box_position_offset: super::core::Vec3,
    pub bounding_box_size_offset: super::core::Vec3,
    pub can_render_as_rigid_mesh: bool,
    pub skinned_procedural_animation: SkinnedProceduralAnimationData,
}

pub const SKINNEDMESHASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkinnedMeshAsset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BoundingBoxPositionOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkinnedMeshAsset, bounding_box_position_offset),
            },
            FieldInfoData {
                name: "BoundingBoxSizeOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(SkinnedMeshAsset, bounding_box_size_offset),
            },
            FieldInfoData {
                name: "CanRenderAsRigidMesh",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SkinnedMeshAsset, can_render_as_rigid_mesh),
            },
            FieldInfoData {
                name: "SkinnedProceduralAnimation",
                flags: MemberInfoFlags::new(0),
                field_type: SKINNEDPROCEDURALANIMATIONDATA_TYPE_INFO,
                rust_offset: offset_of!(SkinnedMeshAsset, skinned_procedural_animation),
            },
        ],
    }),
    array_type: Some(SKINNEDMESHASSET_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SkinnedMeshAsset {
    fn type_info() -> &'static TypeInfo {
        SKINNEDMESHASSET_TYPE_INFO
    }
}


pub const SKINNEDMESHASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkinnedMeshAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("SkinnedMeshAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RigidMeshAsset {
}

pub const RIGIDMESHASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidMeshAsset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RIGIDMESHASSET_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RigidMeshAsset {
    fn type_info() -> &'static TypeInfo {
        RIGIDMESHASSET_TYPE_INFO
    }
}


pub const RIGIDMESHASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RigidMeshAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RigidMeshAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MeshAsset {
    pub lod_group: MeshLodGroup,
    pub lod_scale: f32,
    pub cull_scale: f32,
    pub shader_quality_switch: i32,
    pub lod_distances_view_dir_x: LodDistances,
    pub lod_distances_view_dir_n_x: LodDistances,
    pub lod_distances_view_dir_y: LodDistances,
    pub lod_distances_view_dir_n_y: LodDistances,
    pub lod_distances_view_dir_z: LodDistances,
    pub lod_distances_view_dir_n_z: LodDistances,
    pub shadow_map_lod_bias: u32,
    pub shadow_map_lod_bias_slice_start: u32,
    pub dynamic_distant_shadow_cache_enable: bool,
    pub compute_graph: super::render_base::MeshComputeBaseAsset,
    pub linear_media_streaming_supported: bool,
    pub streaming_enable: bool,
    pub occluder_mesh_enable: bool,
    pub occluder_high_priority: bool,
    pub occluder_is_conservative: bool,
    pub coverage_value: f32,
    pub destruction_material_enable: bool,
    pub enlighten_type: EnlightenType,
    pub enable_enlighten_static_override: bool,
    pub enable_enlighten_proxy_override: bool,
    pub enlighten_mesh_lod: i32,
    pub lightmap_u_vs_scale_charts: bool,
    pub auto_lightmap_u_vs: bool,
    pub auto_lightmap_u_vs_max_distance: f32,
    pub auto_lightmap_u_vs_expansion_factor: f32,
    pub auto_lightmap_u_vs_max_normal_deviation: f32,
    pub receive_only: bool,
    pub light_probe_sample_offset: super::core::Vec3,
    pub procedural_animation: ProceduralAnimationTypeSimple,
    pub materials: Vec<MeshMaterial>,
    pub name_hash: u32,
    pub mesh_set_resource: super::core::ResourceRef,
    pub occluder_mesh_resource: super::core::ResourceRef,
}

pub const MESHASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshAsset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LodGroup",
                flags: MemberInfoFlags::new(0),
                field_type: MESHLODGROUP_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, lod_group),
            },
            FieldInfoData {
                name: "LodScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, lod_scale),
            },
            FieldInfoData {
                name: "CullScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, cull_scale),
            },
            FieldInfoData {
                name: "ShaderQualitySwitch",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, shader_quality_switch),
            },
            FieldInfoData {
                name: "LodDistancesViewDirX",
                flags: MemberInfoFlags::new(0),
                field_type: LODDISTANCES_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, lod_distances_view_dir_x),
            },
            FieldInfoData {
                name: "LodDistancesViewDirNX",
                flags: MemberInfoFlags::new(0),
                field_type: LODDISTANCES_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, lod_distances_view_dir_n_x),
            },
            FieldInfoData {
                name: "LodDistancesViewDirY",
                flags: MemberInfoFlags::new(0),
                field_type: LODDISTANCES_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, lod_distances_view_dir_y),
            },
            FieldInfoData {
                name: "LodDistancesViewDirNY",
                flags: MemberInfoFlags::new(0),
                field_type: LODDISTANCES_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, lod_distances_view_dir_n_y),
            },
            FieldInfoData {
                name: "LodDistancesViewDirZ",
                flags: MemberInfoFlags::new(0),
                field_type: LODDISTANCES_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, lod_distances_view_dir_z),
            },
            FieldInfoData {
                name: "LodDistancesViewDirNZ",
                flags: MemberInfoFlags::new(0),
                field_type: LODDISTANCES_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, lod_distances_view_dir_n_z),
            },
            FieldInfoData {
                name: "ShadowMapLodBias",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, shadow_map_lod_bias),
            },
            FieldInfoData {
                name: "ShadowMapLodBiasSliceStart",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, shadow_map_lod_bias_slice_start),
            },
            FieldInfoData {
                name: "DynamicDistantShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, dynamic_distant_shadow_cache_enable),
            },
            FieldInfoData {
                name: "ComputeGraph",
                flags: MemberInfoFlags::new(0),
                field_type: MESHCOMPUTEBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, compute_graph),
            },
            FieldInfoData {
                name: "LinearMediaStreamingSupported",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, linear_media_streaming_supported),
            },
            FieldInfoData {
                name: "StreamingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, streaming_enable),
            },
            FieldInfoData {
                name: "OccluderMeshEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, occluder_mesh_enable),
            },
            FieldInfoData {
                name: "OccluderHighPriority",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, occluder_high_priority),
            },
            FieldInfoData {
                name: "OccluderIsConservative",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, occluder_is_conservative),
            },
            FieldInfoData {
                name: "CoverageValue",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, coverage_value),
            },
            FieldInfoData {
                name: "DestructionMaterialEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, destruction_material_enable),
            },
            FieldInfoData {
                name: "EnlightenType",
                flags: MemberInfoFlags::new(0),
                field_type: ENLIGHTENTYPE_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, enlighten_type),
            },
            FieldInfoData {
                name: "EnableEnlightenStaticOverride",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, enable_enlighten_static_override),
            },
            FieldInfoData {
                name: "EnableEnlightenProxyOverride",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, enable_enlighten_proxy_override),
            },
            FieldInfoData {
                name: "EnlightenMeshLod",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, enlighten_mesh_lod),
            },
            FieldInfoData {
                name: "LightmapUVsScaleCharts",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, lightmap_u_vs_scale_charts),
            },
            FieldInfoData {
                name: "AutoLightmapUVs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, auto_lightmap_u_vs),
            },
            FieldInfoData {
                name: "AutoLightmapUVsMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, auto_lightmap_u_vs_max_distance),
            },
            FieldInfoData {
                name: "AutoLightmapUVsExpansionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, auto_lightmap_u_vs_expansion_factor),
            },
            FieldInfoData {
                name: "AutoLightmapUVsMaxNormalDeviation",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, auto_lightmap_u_vs_max_normal_deviation),
            },
            FieldInfoData {
                name: "ReceiveOnly",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, receive_only),
            },
            FieldInfoData {
                name: "LightProbeSampleOffset",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, light_probe_sample_offset),
            },
            FieldInfoData {
                name: "ProceduralAnimation",
                flags: MemberInfoFlags::new(0),
                field_type: PROCEDURALANIMATIONTYPESIMPLE_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, procedural_animation),
            },
            FieldInfoData {
                name: "Materials",
                flags: MemberInfoFlags::new(144),
                field_type: MESHMATERIAL_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, materials),
            },
            FieldInfoData {
                name: "NameHash",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, name_hash),
            },
            FieldInfoData {
                name: "MeshSetResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, mesh_set_resource),
            },
            FieldInfoData {
                name: "OccluderMeshResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(MeshAsset, occluder_mesh_resource),
            },
        ],
    }),
    array_type: Some(MESHASSET_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MeshAsset {
    fn type_info() -> &'static TypeInfo {
        MESHASSET_TYPE_INFO
    }
}


pub const MESHASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LodDistances {
    pub lod1: f32,
    pub lod2: f32,
    pub lod3: f32,
    pub lod4: f32,
    pub lod5: f32,
    pub lod6: f32,
}

pub const LODDISTANCES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LodDistances",
    flags: MemberInfoFlags::new(36937),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Lod1",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LodDistances, lod1),
            },
            FieldInfoData {
                name: "Lod2",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LodDistances, lod2),
            },
            FieldInfoData {
                name: "Lod3",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LodDistances, lod3),
            },
            FieldInfoData {
                name: "Lod4",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LodDistances, lod4),
            },
            FieldInfoData {
                name: "Lod5",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LodDistances, lod5),
            },
            FieldInfoData {
                name: "Lod6",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LodDistances, lod6),
            },
        ],
    }),
    array_type: Some(LODDISTANCES_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LodDistances {
    fn type_info() -> &'static TypeInfo {
        LODDISTANCES_TYPE_INFO
    }
}


pub const LODDISTANCES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LodDistances-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("LodDistances-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LodFadeDistanceFactors {
    pub lod1: super::core::Vec2,
    pub lod2: super::core::Vec2,
    pub lod3: super::core::Vec2,
    pub lod4: super::core::Vec2,
    pub lod5: super::core::Vec2,
    pub lod6: f32,
}

pub const LODFADEDISTANCEFACTORS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LodFadeDistanceFactors",
    flags: MemberInfoFlags::new(36937),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Lod1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(LodFadeDistanceFactors, lod1),
            },
            FieldInfoData {
                name: "Lod2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(LodFadeDistanceFactors, lod2),
            },
            FieldInfoData {
                name: "Lod3",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(LodFadeDistanceFactors, lod3),
            },
            FieldInfoData {
                name: "Lod4",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(LodFadeDistanceFactors, lod4),
            },
            FieldInfoData {
                name: "Lod5",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(LodFadeDistanceFactors, lod5),
            },
            FieldInfoData {
                name: "Lod6",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LodFadeDistanceFactors, lod6),
            },
        ],
    }),
    array_type: Some(LODFADEDISTANCEFACTORS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LodFadeDistanceFactors {
    fn type_info() -> &'static TypeInfo {
        LODFADEDISTANCEFACTORS_TYPE_INFO
    }
}


pub const LODFADEDISTANCEFACTORS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LodFadeDistanceFactors-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("LodFadeDistanceFactors-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CustomVertexShader {
    pub fragment: VertexShaderFragmentAsset,
    pub fragment_no_batch: VertexShaderFragmentAsset,
    pub fragment_buffered: VertexShaderFragmentAsset,
    pub fragment_m_v: VertexShaderFragmentAsset,
    pub fragment_no_batch_m_v: VertexShaderFragmentAsset,
    pub fragment_deformer_m_v: VertexShaderFragmentAsset,
}

pub const CUSTOMVERTEXSHADER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomVertexShader",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Fragment",
                flags: MemberInfoFlags::new(0),
                field_type: VERTEXSHADERFRAGMENTASSET_TYPE_INFO,
                rust_offset: offset_of!(CustomVertexShader, fragment),
            },
            FieldInfoData {
                name: "FragmentNoBatch",
                flags: MemberInfoFlags::new(0),
                field_type: VERTEXSHADERFRAGMENTASSET_TYPE_INFO,
                rust_offset: offset_of!(CustomVertexShader, fragment_no_batch),
            },
            FieldInfoData {
                name: "FragmentBuffered",
                flags: MemberInfoFlags::new(0),
                field_type: VERTEXSHADERFRAGMENTASSET_TYPE_INFO,
                rust_offset: offset_of!(CustomVertexShader, fragment_buffered),
            },
            FieldInfoData {
                name: "FragmentMV",
                flags: MemberInfoFlags::new(0),
                field_type: VERTEXSHADERFRAGMENTASSET_TYPE_INFO,
                rust_offset: offset_of!(CustomVertexShader, fragment_m_v),
            },
            FieldInfoData {
                name: "FragmentNoBatchMV",
                flags: MemberInfoFlags::new(0),
                field_type: VERTEXSHADERFRAGMENTASSET_TYPE_INFO,
                rust_offset: offset_of!(CustomVertexShader, fragment_no_batch_m_v),
            },
            FieldInfoData {
                name: "FragmentDeformerMV",
                flags: MemberInfoFlags::new(0),
                field_type: VERTEXSHADERFRAGMENTASSET_TYPE_INFO,
                rust_offset: offset_of!(CustomVertexShader, fragment_deformer_m_v),
            },
        ],
    }),
    array_type: Some(CUSTOMVERTEXSHADER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CustomVertexShader {
    fn type_info() -> &'static TypeInfo {
        CUSTOMVERTEXSHADER_TYPE_INFO
    }
}


pub const CUSTOMVERTEXSHADER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomVertexShader-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("CustomVertexShader-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ProceduralAnimationTypeCustom {
    pub custom_animation_parameters: super::core::Vec4,
}

pub const PROCEDURALANIMATIONTYPECUSTOM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationTypeCustom",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCEDURALANIMATIONTYPECUSTOMBASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CustomAnimationParameters",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ProceduralAnimationTypeCustom, custom_animation_parameters),
            },
        ],
    }),
    array_type: Some(PROCEDURALANIMATIONTYPECUSTOM_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ProceduralAnimationTypeCustom {
    fn type_info() -> &'static TypeInfo {
        PROCEDURALANIMATIONTYPECUSTOM_TYPE_INFO
    }
}


pub const PROCEDURALANIMATIONTYPECUSTOM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationTypeCustom-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ProceduralAnimationTypeCustom-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ProceduralAnimationTypeCustomWind {
    pub custom_params_z: f32,
    pub custom_params_w: f32,
    pub custom_wind_influence_multiplier: f32,
}

pub const PROCEDURALANIMATIONTYPECUSTOMWIND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationTypeCustomWind",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCEDURALANIMATIONTYPECUSTOMBASE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CustomParamsZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProceduralAnimationTypeCustomWind, custom_params_z),
            },
            FieldInfoData {
                name: "CustomParamsW",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProceduralAnimationTypeCustomWind, custom_params_w),
            },
            FieldInfoData {
                name: "CustomWindInfluenceMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProceduralAnimationTypeCustomWind, custom_wind_influence_multiplier),
            },
        ],
    }),
    array_type: Some(PROCEDURALANIMATIONTYPECUSTOMWIND_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProceduralAnimationTypeCustomWind {
    fn type_info() -> &'static TypeInfo {
        PROCEDURALANIMATIONTYPECUSTOMWIND_TYPE_INFO
    }
}


pub const PROCEDURALANIMATIONTYPECUSTOMWIND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationTypeCustomWind-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ProceduralAnimationTypeCustomWind-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ProceduralAnimationTypeCustomBase {
    pub custom_vertex_fragment: CustomVertexShader,
}

pub const PROCEDURALANIMATIONTYPECUSTOMBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationTypeCustomBase",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCEDURALANIMATIONTYPESIMPLE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CustomVertexFragment",
                flags: MemberInfoFlags::new(0),
                field_type: CUSTOMVERTEXSHADER_TYPE_INFO,
                rust_offset: offset_of!(ProceduralAnimationTypeCustomBase, custom_vertex_fragment),
            },
        ],
    }),
    array_type: Some(PROCEDURALANIMATIONTYPECUSTOMBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProceduralAnimationTypeCustomBase {
    fn type_info() -> &'static TypeInfo {
        PROCEDURALANIMATIONTYPECUSTOMBASE_TYPE_INFO
    }
}


pub const PROCEDURALANIMATIONTYPECUSTOMBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationTypeCustomBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ProceduralAnimationTypeCustomBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ProceduralAnimationTypeWind {
    pub wind_method: ProceduralAnimationWindMethod,
}

pub const PROCEDURALANIMATIONTYPEWIND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationTypeWind",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCEDURALANIMATIONTYPESIMPLE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "WindMethod",
                flags: MemberInfoFlags::new(0),
                field_type: PROCEDURALANIMATIONWINDMETHOD_TYPE_INFO,
                rust_offset: offset_of!(ProceduralAnimationTypeWind, wind_method),
            },
        ],
    }),
    array_type: Some(PROCEDURALANIMATIONTYPEWIND_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProceduralAnimationTypeWind {
    fn type_info() -> &'static TypeInfo {
        PROCEDURALANIMATIONTYPEWIND_TYPE_INFO
    }
}


pub const PROCEDURALANIMATIONTYPEWIND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationTypeWind-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ProceduralAnimationTypeWind-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ProceduralAnimationWindMethod {
    #[default]
    Wind_Cloth = 0,
    Wind_Flag = 1,
}

pub const PROCEDURALANIMATIONWINDMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationWindMethod",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(PROCEDURALANIMATIONWINDMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ProceduralAnimationWindMethod {
    fn type_info() -> &'static TypeInfo {
        PROCEDURALANIMATIONWINDMETHOD_TYPE_INFO
    }
}


pub const PROCEDURALANIMATIONWINDMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationWindMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ProceduralAnimationWindMethod-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ProceduralAnimationTypeWiggle {
    pub wiggle_method: ProceduralAnimationWiggleMethod,
}

pub const PROCEDURALANIMATIONTYPEWIGGLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationTypeWiggle",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROCEDURALANIMATIONTYPESIMPLE_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "WiggleMethod",
                flags: MemberInfoFlags::new(0),
                field_type: PROCEDURALANIMATIONWIGGLEMETHOD_TYPE_INFO,
                rust_offset: offset_of!(ProceduralAnimationTypeWiggle, wiggle_method),
            },
        ],
    }),
    array_type: Some(PROCEDURALANIMATIONTYPEWIGGLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProceduralAnimationTypeWiggle {
    fn type_info() -> &'static TypeInfo {
        PROCEDURALANIMATIONTYPEWIGGLE_TYPE_INFO
    }
}


pub const PROCEDURALANIMATIONTYPEWIGGLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationTypeWiggle-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ProceduralAnimationTypeWiggle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ProceduralAnimationWiggleMethod {
    #[default]
    Wiggle_Palmtree = 0,
    Wiggle_Tree = 1,
    Wiggle_Bush = 2,
}

pub const PROCEDURALANIMATIONWIGGLEMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationWiggleMethod",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(PROCEDURALANIMATIONWIGGLEMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ProceduralAnimationWiggleMethod {
    fn type_info() -> &'static TypeInfo {
        PROCEDURALANIMATIONWIGGLEMETHOD_TYPE_INFO
    }
}


pub const PROCEDURALANIMATIONWIGGLEMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationWiggleMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ProceduralAnimationWiggleMethod-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ProceduralAnimationTypeSimple {
    pub bend_multiplier: f32,
    pub wiggle_speed_multiplier: f32,
    pub wind_influence_multiplier: f32,
    pub procedural_animation_max_distance: f32,
    pub enable_procedural_animation_in_shadow: bool,
}

pub const PROCEDURALANIMATIONTYPESIMPLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationTypeSimple",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BendMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProceduralAnimationTypeSimple, bend_multiplier),
            },
            FieldInfoData {
                name: "WiggleSpeedMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProceduralAnimationTypeSimple, wiggle_speed_multiplier),
            },
            FieldInfoData {
                name: "WindInfluenceMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProceduralAnimationTypeSimple, wind_influence_multiplier),
            },
            FieldInfoData {
                name: "ProceduralAnimationMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ProceduralAnimationTypeSimple, procedural_animation_max_distance),
            },
            FieldInfoData {
                name: "EnableProceduralAnimationInShadow",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ProceduralAnimationTypeSimple, enable_procedural_animation_in_shadow),
            },
        ],
    }),
    array_type: Some(PROCEDURALANIMATIONTYPESIMPLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProceduralAnimationTypeSimple {
    fn type_info() -> &'static TypeInfo {
        PROCEDURALANIMATIONTYPESIMPLE_TYPE_INFO
    }
}


pub const PROCEDURALANIMATIONTYPESIMPLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationTypeSimple-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ProceduralAnimationTypeSimple-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ProceduralAnimationMethod {
    #[default]
    ProceduralAnimation_Off = 0,
    ProceduralAnimation_Simple = 1,
    ProceduralAnimation_Wiggle_Palmtree = 2,
    ProceduralAnimation_Wiggle_Tree = 3,
    ProceduralAnimation_Wiggle_Bush = 4,
    ProceduralAnimation_Wind_Cloth = 5,
    ProceduralAnimation_Wind_Flag = 6,
}

pub const PROCEDURALANIMATIONMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationMethod",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(PROCEDURALANIMATIONMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ProceduralAnimationMethod {
    fn type_info() -> &'static TypeInfo {
        PROCEDURALANIMATIONMETHOD_TYPE_INFO
    }
}


pub const PROCEDURALANIMATIONMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ProceduralAnimationMethod-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ProceduralAnimationEnable {
    #[default]
    ProceduralAnimationEnable_Off = 0,
    ProceduralAnimationEnable_On = 1,
}

pub const PROCEDURALANIMATIONENABLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationEnable",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(PROCEDURALANIMATIONENABLE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ProceduralAnimationEnable {
    fn type_info() -> &'static TypeInfo {
        PROCEDURALANIMATIONENABLE_TYPE_INFO
    }
}


pub const PROCEDURALANIMATIONENABLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProceduralAnimationEnable-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ProceduralAnimationEnable-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EnlightenType {
    #[default]
    EnlightenType_Dynamic = 0,
    EnlightenType_LightProbe = 1,
    EnlightenType_Static = 2,
    EnlightenType_Proxy = 3,
    EnlightenType_Static_Blendable = 4,
}

pub const ENLIGHTENTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(ENLIGHTENTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EnlightenType {
    fn type_info() -> &'static TypeInfo {
        ENLIGHTENTYPE_TYPE_INFO
    }
}


pub const ENLIGHTENTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("EnlightenType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MeshLodGroup {
    pub auto_lod: bool,
    pub lod1_distance: f32,
    pub lod2_distance: f32,
    pub lod3_distance: f32,
    pub lod4_distance: f32,
    pub lod5_distance: f32,
    pub lod6_distance: f32,
    pub shadow_distance: f32,
    pub cull_screen_area: f32,
    pub runtime_short_name: String,
    pub shader_quality_switch: i32,
    pub shader_quality_switch_gen4a: i32,
    pub shader_quality_switch_gen4b: i32,
}

pub const MESHLODGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshLodGroup",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHLODGROUPBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AutoLod",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MeshLodGroup, auto_lod),
            },
            FieldInfoData {
                name: "Lod1Distance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshLodGroup, lod1_distance),
            },
            FieldInfoData {
                name: "Lod2Distance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshLodGroup, lod2_distance),
            },
            FieldInfoData {
                name: "Lod3Distance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshLodGroup, lod3_distance),
            },
            FieldInfoData {
                name: "Lod4Distance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshLodGroup, lod4_distance),
            },
            FieldInfoData {
                name: "Lod5Distance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshLodGroup, lod5_distance),
            },
            FieldInfoData {
                name: "Lod6Distance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshLodGroup, lod6_distance),
            },
            FieldInfoData {
                name: "ShadowDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshLodGroup, shadow_distance),
            },
            FieldInfoData {
                name: "CullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MeshLodGroup, cull_screen_area),
            },
            FieldInfoData {
                name: "RuntimeShortName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(MeshLodGroup, runtime_short_name),
            },
            FieldInfoData {
                name: "ShaderQualitySwitch",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MeshLodGroup, shader_quality_switch),
            },
            FieldInfoData {
                name: "ShaderQualitySwitchGen4a",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MeshLodGroup, shader_quality_switch_gen4a),
            },
            FieldInfoData {
                name: "ShaderQualitySwitchGen4b",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MeshLodGroup, shader_quality_switch_gen4b),
            },
        ],
    }),
    array_type: Some(MESHLODGROUP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshLodGroup {
    fn type_info() -> &'static TypeInfo {
        MESHLODGROUP_TYPE_INFO
    }
}


pub const MESHLODGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshLodGroup-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshLodGroup-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LocalIBLType {
    #[default]
    LocalIBLType_Sphere = 0,
    LocalIBLType_Box = 1,
    LocalIBLTypeCount = 2,
}

pub const LOCALIBLTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalIBLType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(LOCALIBLTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LocalIBLType {
    fn type_info() -> &'static TypeInfo {
        LOCALIBLTYPE_TYPE_INFO
    }
}


pub const LOCALIBLTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalIBLType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("LocalIBLType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PBRAnalyticLightShape {
    #[default]
    PBRAnalyticLightShape_Sphere = 0,
    PBRAnalyticLightShape_Spot = 1,
    PBRAnalyticLightShape_Tube = 2,
    PBRAnalyticLightShape_Rectangular = 3,
    PBRAnalyticLightShapeCount = 4,
}

pub const PBRANALYTICLIGHTSHAPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PBRAnalyticLightShape",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(PBRANALYTICLIGHTSHAPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PBRAnalyticLightShape {
    fn type_info() -> &'static TypeInfo {
        PBRANALYTICLIGHTSHAPE_TYPE_INFO
    }
}


pub const PBRANALYTICLIGHTSHAPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PBRAnalyticLightShape-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("PBRAnalyticLightShape-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PBRLightType {
    #[default]
    PBRLightType_AreaLight = 0,
    PBRLightType_AreaLightShadow = 1,
    PBRLightType_PunctualLight = 2,
    PBRLightType_PunctualLightShadow = 3,
    PBRLightType_LocalIBL = 4,
    PBRLightType_LocalPR = 5,
    PBRLightTypeCount = 6,
}

pub const PBRLIGHTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PBRLightType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(PBRLIGHTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PBRLightType {
    fn type_info() -> &'static TypeInfo {
        PBRLIGHTTYPE_TYPE_INFO
    }
}


pub const PBRLIGHTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PBRLightType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("PBRLightType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12ComputePsoDescType {
}

pub const DX12COMPUTEPSODESCTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12ComputePsoDescType",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DX12PSODESCTYPE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12COMPUTEPSODESCTYPE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12ComputePsoDescType {
    fn type_info() -> &'static TypeInfo {
        DX12COMPUTEPSODESCTYPE_TYPE_INFO
    }
}


pub const DX12COMPUTEPSODESCTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12ComputePsoDescType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Dx12ComputePsoDescType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12GraphicsPsoDescType {
}

pub const DX12GRAPHICSPSODESCTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12GraphicsPsoDescType",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DX12PSODESCTYPE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12GRAPHICSPSODESCTYPE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12GraphicsPsoDescType {
    fn type_info() -> &'static TypeInfo {
        DX12GRAPHICSPSODESCTYPE_TYPE_INFO
    }
}


pub const DX12GRAPHICSPSODESCTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12GraphicsPsoDescType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Dx12GraphicsPsoDescType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12PsoDescType {
}

pub const DX12PSODESCTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PsoDescType",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(DX12PSODESCTYPE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx12PsoDescType {
    fn type_info() -> &'static TypeInfo {
        DX12PSODESCTYPE_TYPE_INFO
    }
}


pub const DX12PSODESCTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12PsoDescType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Dx12PsoDescType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmBackendFactory {
}

pub const RVMBACKENDFACTORY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmBackendFactory",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(RVMBACKENDFACTORY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RvmBackendFactory {
    fn type_info() -> &'static TypeInfo {
        RVMBACKENDFACTORY_TYPE_INFO
    }
}


pub const RVMBACKENDFACTORY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmBackendFactory-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RvmBackendFactory-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmBackend {
}

pub const RVMBACKEND_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmBackend",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(RVMBACKEND_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RvmBackend {
    fn type_info() -> &'static TypeInfo {
        RVMBACKEND_TYPE_INFO
    }
}


pub const RVMBACKEND_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmBackend-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RvmBackend-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RvmDatabase {
}

pub const RVMDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDatabase",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IRESOURCEOBJECT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RVMDATABASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RvmDatabase {
    fn type_info() -> &'static TypeInfo {
        RVMDATABASE_TYPE_INFO
    }
}


pub const RVMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RvmDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RvmDatabase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderBlockMeshVariationEntry {
}

pub const SHADERBLOCKMESHVARIATIONENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockMeshVariationEntry",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SHADERBLOCKDEPOTITEM_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHADERBLOCKMESHVARIATIONENTRY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShaderBlockMeshVariationEntry {
    fn type_info() -> &'static TypeInfo {
        SHADERBLOCKMESHVARIATIONENTRY_TYPE_INFO
    }
}


pub const SHADERBLOCKMESHVARIATIONENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockMeshVariationEntry-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderBlockMeshVariationEntry-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderPersistentParamDbBlock {
}

pub const SHADERPERSISTENTPARAMDBBLOCK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderPersistentParamDbBlock",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SHADERPARAMDBBLOCK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHADERPERSISTENTPARAMDBBLOCK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShaderPersistentParamDbBlock {
    fn type_info() -> &'static TypeInfo {
        SHADERPERSISTENTPARAMDBBLOCK_TYPE_INFO
    }
}


pub const SHADERPERSISTENTPARAMDBBLOCK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderPersistentParamDbBlock-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderPersistentParamDbBlock-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MeshParamDbBlock {
}

pub const MESHPARAMDBBLOCK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshParamDbBlock",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SHADERPERSISTENTPARAMDBBLOCK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MESHPARAMDBBLOCK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshParamDbBlock {
    fn type_info() -> &'static TypeInfo {
        MESHPARAMDBBLOCK_TYPE_INFO
    }
}


pub const MESHPARAMDBBLOCK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshParamDbBlock-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("MeshParamDbBlock-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderStaticParamDbBlock {
}

pub const SHADERSTATICPARAMDBBLOCK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderStaticParamDbBlock",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SHADERPARAMDBBLOCK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHADERSTATICPARAMDBBLOCK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShaderStaticParamDbBlock {
    fn type_info() -> &'static TypeInfo {
        SHADERSTATICPARAMDBBLOCK_TYPE_INFO
    }
}


pub const SHADERSTATICPARAMDBBLOCK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderStaticParamDbBlock-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderStaticParamDbBlock-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderParamDbBlock {
}

pub const SHADERPARAMDBBLOCK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParamDbBlock",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SHADERBLOCKDEPOTITEM_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHADERPARAMDBBLOCK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShaderParamDbBlock {
    fn type_info() -> &'static TypeInfo {
        SHADERPARAMDBBLOCK_TYPE_INFO
    }
}


pub const SHADERPARAMDBBLOCK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderParamDbBlock-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderParamDbBlock-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderBlockEntry {
}

pub const SHADERBLOCKENTRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockEntry",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SHADERBLOCKDEPOTITEM_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SHADERBLOCKENTRY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShaderBlockEntry {
    fn type_info() -> &'static TypeInfo {
        SHADERBLOCKENTRY_TYPE_INFO
    }
}


pub const SHADERBLOCKENTRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockEntry-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderBlockEntry-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderBlockDepotItem {
}

pub const SHADERBLOCKDEPOTITEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockDepotItem",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(SHADERBLOCKDEPOTITEM_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShaderBlockDepotItem {
    fn type_info() -> &'static TypeInfo {
        SHADERBLOCKDEPOTITEM_TYPE_INFO
    }
}


pub const SHADERBLOCKDEPOTITEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockDepotItem-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderBlockDepotItem-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShaderBlockDepotResource {
}

pub const SHADERBLOCKDEPOTRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockDepotResource",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(SHADERBLOCKDEPOTRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ShaderBlockDepotResource {
    fn type_info() -> &'static TypeInfo {
        SHADERBLOCKDEPOTRESOURCE_TYPE_INFO
    }
}


pub const SHADERBLOCKDEPOTRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderBlockDepotResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderBlockDepotResource-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AtlasTexture {
}

pub const ATLASTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AtlasTexture",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(ATLASTEXTURE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for AtlasTexture {
    fn type_info() -> &'static TypeInfo {
        ATLASTEXTURE_TYPE_INFO
    }
}


pub const ATLASTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AtlasTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("AtlasTexture-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Win32SharedSurfaceWindow {
}

pub const WIN32SHAREDSURFACEWINDOW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Win32SharedSurfaceWindow",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WINDOW_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WIN32SHAREDSURFACEWINDOW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Win32SharedSurfaceWindow {
    fn type_info() -> &'static TypeInfo {
        WIN32SHAREDSURFACEWINDOW_TYPE_INFO
    }
}


pub const WIN32SHAREDSURFACEWINDOW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Win32SharedSurfaceWindow-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Win32SharedSurfaceWindow-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Win32GameViewWindow {
}

pub const WIN32GAMEVIEWWINDOW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Win32GameViewWindow",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WIN32RENDERWINDOW_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WIN32GAMEVIEWWINDOW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Win32GameViewWindow {
    fn type_info() -> &'static TypeInfo {
        WIN32GAMEVIEWWINDOW_TYPE_INFO
    }
}


pub const WIN32GAMEVIEWWINDOW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Win32GameViewWindow-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Win32GameViewWindow-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Win32RenderWindow {
}

pub const WIN32RENDERWINDOW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Win32RenderWindow",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WIN32WINDOW_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WIN32RENDERWINDOW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Win32RenderWindow {
    fn type_info() -> &'static TypeInfo {
        WIN32RENDERWINDOW_TYPE_INFO
    }
}


pub const WIN32RENDERWINDOW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Win32RenderWindow-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Win32RenderWindow-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NullTexture {
}

pub const NULLTEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NullTexture",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ITEXTURE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(NULLTEXTURE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for NullTexture {
    fn type_info() -> &'static TypeInfo {
        NULLTEXTURE_TYPE_INFO
    }
}


pub const NULLTEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NullTexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("NullTexture-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12Texture {
}

pub const DX12TEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12Texture",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASETEXTURE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12TEXTURE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx12Texture {
    fn type_info() -> &'static TypeInfo {
        DX12TEXTURE_TYPE_INFO
    }
}


pub const DX12TEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12Texture-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Dx12Texture-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dx12ShaderProgramDatabase {
}

pub const DX12SHADERPROGRAMDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12ShaderProgramDatabase",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASESHADERPROGRAMDATABASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DX12SHADERPROGRAMDATABASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Dx12ShaderProgramDatabase {
    fn type_info() -> &'static TypeInfo {
        DX12SHADERPROGRAMDATABASE_TYPE_INFO
    }
}


pub const DX12SHADERPROGRAMDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12ShaderProgramDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Dx12ShaderProgramDatabase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RenderAdapterArchitecture {
    #[default]
    RenderAdapterArchitecture_Immediate = 0,
    RenderAdapterArchitecture_Tiler = 1,
}

pub const RENDERADAPTERARCHITECTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderAdapterArchitecture",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RENDERADAPTERARCHITECTURE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderAdapterArchitecture {
    fn type_info() -> &'static TypeInfo {
        RENDERADAPTERARCHITECTURE_TYPE_INFO
    }
}


pub const RENDERADAPTERARCHITECTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderAdapterArchitecture-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RenderAdapterArchitecture-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RenderAdapterFamily {
    #[default]
    RenderAdapterFamily_AmdPreGcn = 0,
    RenderAdapterFamily_AmdGcn = 1,
    RenderAdapterFamily_Unknown = 5,
}

pub const RENDERADAPTERFAMILY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderAdapterFamily",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RENDERADAPTERFAMILY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderAdapterFamily {
    fn type_info() -> &'static TypeInfo {
        RENDERADAPTERFAMILY_TYPE_INFO
    }
}


pub const RENDERADAPTERFAMILY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderAdapterFamily-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RenderAdapterFamily-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RenderAdapterVendor {
    #[default]
    RenderAdapterVendor_Nvidia = 4318,
    RenderAdapterVendor_Amd = 4098,
    RenderAdapterVendor_Intel = 32902,
    RenderAdapterVendor_ImgTech = 1,
    RenderAdapterVendor_Unknown = 0,
}

pub const RENDERADAPTERVENDOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderAdapterVendor",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RENDERADAPTERVENDOR_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderAdapterVendor {
    fn type_info() -> &'static TypeInfo {
        RENDERADAPTERVENDOR_TYPE_INFO
    }
}


pub const RENDERADAPTERVENDOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderAdapterVendor-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RenderAdapterVendor-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RenderFeatureLevel {
    #[default]
    RenderFeatureLevel_Dx9 = 0,
    RenderFeatureLevel_Dx10 = 1,
    RenderFeatureLevel_Dx10Plus = 2,
    RenderFeatureLevel_Dx10_1 = 3,
    RenderFeatureLevel_Dx11 = 4,
    RenderFeatureLevel_Dx11_1 = 5,
}

pub const RENDERFEATURELEVEL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderFeatureLevel",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RENDERFEATURELEVEL_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderFeatureLevel {
    fn type_info() -> &'static TypeInfo {
        RENDERFEATURELEVEL_TYPE_INFO
    }
}


pub const RENDERFEATURELEVEL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderFeatureLevel-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RenderFeatureLevel-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IRenderStateObject {
}

pub const IRENDERSTATEOBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IRenderStateObject",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(IRENDERSTATEOBJECT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IRenderStateObject {
    fn type_info() -> &'static TypeInfo {
        IRENDERSTATEOBJECT_TYPE_INFO
    }
}


pub const IRENDERSTATEOBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IRenderStateObject-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("IRenderStateObject-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IRenderTargetView {
}

pub const IRENDERTARGETVIEW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IRenderTargetView",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(IRENDERTARGETVIEW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IRenderTargetView {
    fn type_info() -> &'static TypeInfo {
        IRENDERTARGETVIEW_TYPE_INFO
    }
}


pub const IRENDERTARGETVIEW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IRenderTargetView-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("IRenderTargetView-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IRenderBuffer {
}

pub const IRENDERBUFFER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IRenderBuffer",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IRENDERRESOURCE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(IRENDERBUFFER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IRenderBuffer {
    fn type_info() -> &'static TypeInfo {
        IRENDERBUFFER_TYPE_INFO
    }
}


pub const IRENDERBUFFER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IRenderBuffer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("IRenderBuffer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IRenderResource {
}

pub const IRENDERRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IRenderResource",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IRESOURCEOBJECT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(IRENDERRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IRenderResource {
    fn type_info() -> &'static TypeInfo {
        IRENDERRESOURCE_TYPE_INFO
    }
}


pub const IRENDERRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IRenderResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("IRenderResource-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CompositionSettings {
    pub distortion_enabled: bool,
    pub scanlines_enabled: bool,
    pub chromostereopsis_enabled: bool,
    pub scanline_width: u32,
    pub scanline_dir: u32,
    pub scanline_alpha: f32,
    pub chromostereopsis_red_mult_x: f32,
    pub chromostereopsis_red_mult_y: f32,
    pub chromostereopsis_green_mult_x: f32,
    pub chromostereopsis_green_mult_y: f32,
    pub distortion_scale: f32,
    pub distortion_offset_speed_x: f32,
    pub distortion_offset_speed_y: f32,
    pub debug_force_distortion_scale: f32,
    pub debug_force_distortion_type: i32,
}

pub const COMPOSITIONSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompositionSettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DistortionEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CompositionSettings, distortion_enabled),
            },
            FieldInfoData {
                name: "ScanlinesEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CompositionSettings, scanlines_enabled),
            },
            FieldInfoData {
                name: "ChromostereopsisEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CompositionSettings, chromostereopsis_enabled),
            },
            FieldInfoData {
                name: "ScanlineWidth",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(CompositionSettings, scanline_width),
            },
            FieldInfoData {
                name: "ScanlineDir",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(CompositionSettings, scanline_dir),
            },
            FieldInfoData {
                name: "ScanlineAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CompositionSettings, scanline_alpha),
            },
            FieldInfoData {
                name: "ChromostereopsisRedMultX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CompositionSettings, chromostereopsis_red_mult_x),
            },
            FieldInfoData {
                name: "ChromostereopsisRedMultY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CompositionSettings, chromostereopsis_red_mult_y),
            },
            FieldInfoData {
                name: "ChromostereopsisGreenMultX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CompositionSettings, chromostereopsis_green_mult_x),
            },
            FieldInfoData {
                name: "ChromostereopsisGreenMultY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CompositionSettings, chromostereopsis_green_mult_y),
            },
            FieldInfoData {
                name: "DistortionScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CompositionSettings, distortion_scale),
            },
            FieldInfoData {
                name: "DistortionOffsetSpeedX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CompositionSettings, distortion_offset_speed_x),
            },
            FieldInfoData {
                name: "DistortionOffsetSpeedY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CompositionSettings, distortion_offset_speed_y),
            },
            FieldInfoData {
                name: "DebugForceDistortionScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CompositionSettings, debug_force_distortion_scale),
            },
            FieldInfoData {
                name: "DebugForceDistortionType",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(CompositionSettings, debug_force_distortion_type),
            },
        ],
    }),
    array_type: Some(COMPOSITIONSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CompositionSettings {
    fn type_info() -> &'static TypeInfo {
        COMPOSITIONSETTINGS_TYPE_INFO
    }
}


pub const COMPOSITIONSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompositionSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("CompositionSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum CompositionDistortionType {
    #[default]
    CompositionDistortionType_Disabled = 0,
    CompositionDistortionType_PointSampled = 1,
    CompositionDistortionType_LinearSampled = 2,
}

pub const COMPOSITIONDISTORTIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompositionDistortionType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(COMPOSITIONDISTORTIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CompositionDistortionType {
    fn type_info() -> &'static TypeInfo {
        COMPOSITIONDISTORTIONTYPE_TYPE_INFO
    }
}


pub const COMPOSITIONDISTORTIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompositionDistortionType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("CompositionDistortionType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GameRenderSettings {
    pub enable: bool,
    pub null_renderer_enable: bool,
    pub inactive_skip_frame_count: u32,
    pub job_enable: bool,
    pub build_job_sync_enable: bool,
    pub frame_graph_parallel_execute_enable: bool,
    pub frame_graph_bundle_size_limit: u32,
    pub render_quick_end_job_enable: bool,
    pub draw_debug_dynamic_texture_arrays: bool,
    pub draw_debug_info: bool,
    pub draw_screen_info: bool,
    pub draw_display_info: bool,
    pub resolution_scale: f32,
    pub resolution_scale_min: f32,
    pub resolution_scale_max: f32,
    pub dynamic_resolution_scale_enable: bool,
    pub dynamic_resolution_scale_target_time: f32,
    pub dynamic_resolution_max_step_count: u32,
    pub resolution_regulator: ResolutionRegulator,
    pub resolution_set_generator: ResolutionSetGenerator,
    pub dynamic_resolution_draw_graph_enable: bool,
    pub dynamic_resolution_draw_table_enable: bool,
    pub vsync_enable: bool,
    pub vsync_during_loading_screen_enable: bool,
    pub fullscreen: bool,
    pub force_v_sync_enable: bool,
    pub movie_v_sync_enable: bool,
    pub v_sync_flash_test_enable: bool,
    pub output_brightness_test_enable: bool,
    pub dx11_enable: bool,
    pub dx12_enable: bool,
    pub dx12_use_profile_option_enable: bool,
    pub dxr_enable: i32,
    pub d_l_i_s_p_enable: bool,
    pub d_l_a_a_enable: bool,
    pub use_resolution_scale_from_n_g_x: bool,
    pub d_l_s_s_debug_draw_enable: bool,
    pub d_l_a_a_capture_enable: bool,
    pub d_l_i_s_p_override_sharpness_per_resolution: bool,
    pub d_l_i_s_p_sharpness: f32,
    pub d_l_a_a_reset: bool,
    pub d_l_a_a_motion_vector_scale_x: f32,
    pub d_l_a_a_motion_vector_scale_y: f32,
    pub d_l_a_a_evaluate_feature: bool,
    pub d_l_i_s_p_evaluate_feature: bool,
    pub gen4a_esram_enable: bool,
    pub gen4b_color_remap: bool,
    pub gpu_texture_compressor_enable: bool,
    pub emitters_enable: bool,
    pub entity_render_enable: bool,
    pub debug_renderer_enable: bool,
    pub debug_render_service_enable: bool,
    pub initial_clear_enable: bool,
    pub near_plane: f32,
    pub view_distance: f32,
    pub force_fov: f32,
    pub fov_multiplier: f32,
    pub force_ortho_view_enable: bool,
    pub force_ortho_view_size: f32,
    pub force_square_ortho_view: bool,
    pub destruction_volume_draw_enable: bool,
    pub edge_models_enable: bool,
    pub edge_model_cast_shadows_enable: bool,
    pub edge_model_depth_bias_enable: bool,
    pub edge_model_shadow_depth_bias_enable: bool,
    pub edge_model_screen_area_scale: f32,
    pub edge_model_view_distance: f32,
    pub edge_model_use_main_lod_enable: bool,
    pub edge_model_force_lod: i32,
    pub edge_model_use_lod_box: bool,
    pub edge_model_lod_scale: f32,
    pub edge_model_cull_enable: bool,
    pub edge_model_frustum_cull_enable: bool,
    pub edge_model_draw_boxes: bool,
    pub edge_model_draw_stats: bool,
    pub static_model_enable: bool,
    pub static_model_meshes_enable: bool,
    pub static_model_z_pass_enable: bool,
    pub static_model_part_cull_enable: bool,
    pub static_model_part_frustum_cull_enable: bool,
    pub static_model_part_occlusion_cull_enable: bool,
    pub static_model_part_shadow_cull_enable: bool,
    pub static_model_draw_boxes: bool,
    pub static_model_draw_stats: bool,
    pub static_model_part_occlusion_max_screen_area: f32,
    pub static_model_cull_job_count: u32,
    pub static_model_cull_spu_job_enable: bool,
    pub static_model_surface_shader_terrain_access_enable: bool,
    pub lock_view: bool,
    pub reset_locked_view: bool,
    pub infinite_projection_matrix_enable: bool,
    pub secondary_streaming_view_enable: bool,
    pub fade_enable: bool,
    pub fade_waiting_enable: bool,
    pub force_blur_amount: f32,
    pub force_world_fade_amount: f32,
    pub render_planes_enable: bool,
    pub render_plane_main_enable: bool,
    pub render_plane_overlay_enable: bool,
    pub dedicated_debug_texture: bool,
    pub render_planes_auto_disable: bool,
    pub color_blind_enable: bool,
    pub color_blind_protanopia_factor: f32,
    pub color_blind_deuteranopia_factor: f32,
    pub color_blind_tritanopia_factor: f32,
    pub color_blind_daltonize_factor: f32,
    pub color_blind_brightness_factor: f32,
    pub color_blind_contrast_factor: f32,
    pub render_scale_resample_mode: ScaleResampleMode,
    pub render_scale_resample_enable: bool,
    pub blur_enable: bool,
    pub stereo_crosshair_max_hit_depth: f32,
    pub stereo_crosshair_radius: f32,
    pub stereo_crosshair_damping_factor: f32,
    pub hdr_grading_enable: bool,
    pub display_mapping_enable: bool,
    pub display_mapping_sdr_peak_luma: f32,
    pub display_mapping_hdr10_peak_luma: f32,
    pub display_mapping_shoulder_type: DisplayMappingShoulderType,
    pub hdr_output_prefer_cs: bool,
    pub hdr_live_grading_overlay_opacity: f32,
    pub draw_hdr_calibration_screen: bool,
    pub dolby_vision_metadata_l1_min_luminance_override: f32,
    pub dolby_vision_metadata_l1_max_luminance_override: f32,
    pub dolby_vision_metadata_l2_min_luminance_override: f32,
    pub dolby_vision_metadata_l2_max_luminance_override: f32,
    pub dolby_vision_metadata_l2_avg_luminance_override: f32,
    pub dolby_vision_metadata_luminance_override_enable: bool,
    pub dolby_vision_metadata_debug_overlay_enable: bool,
    pub distortion_max_value_scale: f32,
    pub frame_synthesis: bool,
    pub u_i_shade_in_linear_space_enabled: bool,
    pub brightness_scale: f32,
    pub rvm_enable: bool,
    pub rvm_test_mode: bool,
    pub rvm_on_demand_building_enable: bool,
    pub load_shader_databases: bool,
    pub overlay_drop_shadow_amount: f32,
}

pub const GAMERENDERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameRenderSettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, enable),
            },
            FieldInfoData {
                name: "NullRendererEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, null_renderer_enable),
            },
            FieldInfoData {
                name: "InactiveSkipFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, inactive_skip_frame_count),
            },
            FieldInfoData {
                name: "JobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, job_enable),
            },
            FieldInfoData {
                name: "BuildJobSyncEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, build_job_sync_enable),
            },
            FieldInfoData {
                name: "FrameGraphParallelExecuteEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, frame_graph_parallel_execute_enable),
            },
            FieldInfoData {
                name: "FrameGraphBundleSizeLimit",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, frame_graph_bundle_size_limit),
            },
            FieldInfoData {
                name: "RenderQuickEndJobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, render_quick_end_job_enable),
            },
            FieldInfoData {
                name: "DrawDebugDynamicTextureArrays",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, draw_debug_dynamic_texture_arrays),
            },
            FieldInfoData {
                name: "DrawDebugInfo",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, draw_debug_info),
            },
            FieldInfoData {
                name: "DrawScreenInfo",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, draw_screen_info),
            },
            FieldInfoData {
                name: "DrawDisplayInfo",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, draw_display_info),
            },
            FieldInfoData {
                name: "ResolutionScale",
                flags: MemberInfoFlags::new(8192),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, resolution_scale),
            },
            FieldInfoData {
                name: "ResolutionScaleMin",
                flags: MemberInfoFlags::new(8192),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, resolution_scale_min),
            },
            FieldInfoData {
                name: "ResolutionScaleMax",
                flags: MemberInfoFlags::new(8192),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, resolution_scale_max),
            },
            FieldInfoData {
                name: "DynamicResolutionScaleEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, dynamic_resolution_scale_enable),
            },
            FieldInfoData {
                name: "DynamicResolutionScaleTargetTime",
                flags: MemberInfoFlags::new(8192),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, dynamic_resolution_scale_target_time),
            },
            FieldInfoData {
                name: "DynamicResolutionMaxStepCount",
                flags: MemberInfoFlags::new(8192),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, dynamic_resolution_max_step_count),
            },
            FieldInfoData {
                name: "ResolutionRegulator",
                flags: MemberInfoFlags::new(8192),
                field_type: RESOLUTIONREGULATOR_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, resolution_regulator),
            },
            FieldInfoData {
                name: "ResolutionSetGenerator",
                flags: MemberInfoFlags::new(8192),
                field_type: RESOLUTIONSETGENERATOR_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, resolution_set_generator),
            },
            FieldInfoData {
                name: "DynamicResolutionDrawGraphEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, dynamic_resolution_draw_graph_enable),
            },
            FieldInfoData {
                name: "DynamicResolutionDrawTableEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, dynamic_resolution_draw_table_enable),
            },
            FieldInfoData {
                name: "VsyncEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, vsync_enable),
            },
            FieldInfoData {
                name: "VsyncDuringLoadingScreenEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, vsync_during_loading_screen_enable),
            },
            FieldInfoData {
                name: "Fullscreen",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, fullscreen),
            },
            FieldInfoData {
                name: "ForceVSyncEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, force_v_sync_enable),
            },
            FieldInfoData {
                name: "MovieVSyncEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, movie_v_sync_enable),
            },
            FieldInfoData {
                name: "VSyncFlashTestEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, v_sync_flash_test_enable),
            },
            FieldInfoData {
                name: "OutputBrightnessTestEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, output_brightness_test_enable),
            },
            FieldInfoData {
                name: "Dx11Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, dx11_enable),
            },
            FieldInfoData {
                name: "Dx12Enable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, dx12_enable),
            },
            FieldInfoData {
                name: "Dx12UseProfileOptionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, dx12_use_profile_option_enable),
            },
            FieldInfoData {
                name: "DxrEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, dxr_enable),
            },
            FieldInfoData {
                name: "DLISPEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, d_l_i_s_p_enable),
            },
            FieldInfoData {
                name: "DLAAEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, d_l_a_a_enable),
            },
            FieldInfoData {
                name: "UseResolutionScaleFromNGX",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, use_resolution_scale_from_n_g_x),
            },
            FieldInfoData {
                name: "DLSSDebugDrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, d_l_s_s_debug_draw_enable),
            },
            FieldInfoData {
                name: "DLAACaptureEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, d_l_a_a_capture_enable),
            },
            FieldInfoData {
                name: "DLISPOverrideSharpnessPerResolution",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, d_l_i_s_p_override_sharpness_per_resolution),
            },
            FieldInfoData {
                name: "DLISPSharpness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, d_l_i_s_p_sharpness),
            },
            FieldInfoData {
                name: "DLAAReset",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, d_l_a_a_reset),
            },
            FieldInfoData {
                name: "DLAAMotionVectorScaleX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, d_l_a_a_motion_vector_scale_x),
            },
            FieldInfoData {
                name: "DLAAMotionVectorScaleY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, d_l_a_a_motion_vector_scale_y),
            },
            FieldInfoData {
                name: "DLAAEvaluateFeature",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, d_l_a_a_evaluate_feature),
            },
            FieldInfoData {
                name: "DLISPEvaluateFeature",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, d_l_i_s_p_evaluate_feature),
            },
            FieldInfoData {
                name: "Gen4aEsramEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, gen4a_esram_enable),
            },
            FieldInfoData {
                name: "Gen4bColorRemap",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, gen4b_color_remap),
            },
            FieldInfoData {
                name: "GpuTextureCompressorEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, gpu_texture_compressor_enable),
            },
            FieldInfoData {
                name: "EmittersEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, emitters_enable),
            },
            FieldInfoData {
                name: "EntityRenderEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, entity_render_enable),
            },
            FieldInfoData {
                name: "DebugRendererEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, debug_renderer_enable),
            },
            FieldInfoData {
                name: "DebugRenderServiceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, debug_render_service_enable),
            },
            FieldInfoData {
                name: "InitialClearEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, initial_clear_enable),
            },
            FieldInfoData {
                name: "NearPlane",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, near_plane),
            },
            FieldInfoData {
                name: "ViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, view_distance),
            },
            FieldInfoData {
                name: "ForceFov",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, force_fov),
            },
            FieldInfoData {
                name: "FovMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, fov_multiplier),
            },
            FieldInfoData {
                name: "ForceOrthoViewEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, force_ortho_view_enable),
            },
            FieldInfoData {
                name: "ForceOrthoViewSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, force_ortho_view_size),
            },
            FieldInfoData {
                name: "ForceSquareOrthoView",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, force_square_ortho_view),
            },
            FieldInfoData {
                name: "DestructionVolumeDrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, destruction_volume_draw_enable),
            },
            FieldInfoData {
                name: "EdgeModelsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, edge_models_enable),
            },
            FieldInfoData {
                name: "EdgeModelCastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, edge_model_cast_shadows_enable),
            },
            FieldInfoData {
                name: "EdgeModelDepthBiasEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, edge_model_depth_bias_enable),
            },
            FieldInfoData {
                name: "EdgeModelShadowDepthBiasEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, edge_model_shadow_depth_bias_enable),
            },
            FieldInfoData {
                name: "EdgeModelScreenAreaScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, edge_model_screen_area_scale),
            },
            FieldInfoData {
                name: "EdgeModelViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, edge_model_view_distance),
            },
            FieldInfoData {
                name: "EdgeModelUseMainLodEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, edge_model_use_main_lod_enable),
            },
            FieldInfoData {
                name: "EdgeModelForceLod",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, edge_model_force_lod),
            },
            FieldInfoData {
                name: "EdgeModelUseLodBox",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, edge_model_use_lod_box),
            },
            FieldInfoData {
                name: "EdgeModelLodScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, edge_model_lod_scale),
            },
            FieldInfoData {
                name: "EdgeModelCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, edge_model_cull_enable),
            },
            FieldInfoData {
                name: "EdgeModelFrustumCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, edge_model_frustum_cull_enable),
            },
            FieldInfoData {
                name: "EdgeModelDrawBoxes",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, edge_model_draw_boxes),
            },
            FieldInfoData {
                name: "EdgeModelDrawStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, edge_model_draw_stats),
            },
            FieldInfoData {
                name: "StaticModelEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, static_model_enable),
            },
            FieldInfoData {
                name: "StaticModelMeshesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, static_model_meshes_enable),
            },
            FieldInfoData {
                name: "StaticModelZPassEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, static_model_z_pass_enable),
            },
            FieldInfoData {
                name: "StaticModelPartCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, static_model_part_cull_enable),
            },
            FieldInfoData {
                name: "StaticModelPartFrustumCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, static_model_part_frustum_cull_enable),
            },
            FieldInfoData {
                name: "StaticModelPartOcclusionCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, static_model_part_occlusion_cull_enable),
            },
            FieldInfoData {
                name: "StaticModelPartShadowCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, static_model_part_shadow_cull_enable),
            },
            FieldInfoData {
                name: "StaticModelDrawBoxes",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, static_model_draw_boxes),
            },
            FieldInfoData {
                name: "StaticModelDrawStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, static_model_draw_stats),
            },
            FieldInfoData {
                name: "StaticModelPartOcclusionMaxScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, static_model_part_occlusion_max_screen_area),
            },
            FieldInfoData {
                name: "StaticModelCullJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, static_model_cull_job_count),
            },
            FieldInfoData {
                name: "StaticModelCullSpuJobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, static_model_cull_spu_job_enable),
            },
            FieldInfoData {
                name: "StaticModelSurfaceShaderTerrainAccessEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, static_model_surface_shader_terrain_access_enable),
            },
            FieldInfoData {
                name: "LockView",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, lock_view),
            },
            FieldInfoData {
                name: "ResetLockedView",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, reset_locked_view),
            },
            FieldInfoData {
                name: "InfiniteProjectionMatrixEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, infinite_projection_matrix_enable),
            },
            FieldInfoData {
                name: "SecondaryStreamingViewEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, secondary_streaming_view_enable),
            },
            FieldInfoData {
                name: "FadeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, fade_enable),
            },
            FieldInfoData {
                name: "FadeWaitingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, fade_waiting_enable),
            },
            FieldInfoData {
                name: "ForceBlurAmount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, force_blur_amount),
            },
            FieldInfoData {
                name: "ForceWorldFadeAmount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, force_world_fade_amount),
            },
            FieldInfoData {
                name: "RenderPlanesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, render_planes_enable),
            },
            FieldInfoData {
                name: "RenderPlaneMainEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, render_plane_main_enable),
            },
            FieldInfoData {
                name: "RenderPlaneOverlayEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, render_plane_overlay_enable),
            },
            FieldInfoData {
                name: "DedicatedDebugTexture",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, dedicated_debug_texture),
            },
            FieldInfoData {
                name: "RenderPlanesAutoDisable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, render_planes_auto_disable),
            },
            FieldInfoData {
                name: "ColorBlindEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, color_blind_enable),
            },
            FieldInfoData {
                name: "ColorBlindProtanopiaFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, color_blind_protanopia_factor),
            },
            FieldInfoData {
                name: "ColorBlindDeuteranopiaFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, color_blind_deuteranopia_factor),
            },
            FieldInfoData {
                name: "ColorBlindTritanopiaFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, color_blind_tritanopia_factor),
            },
            FieldInfoData {
                name: "ColorBlindDaltonizeFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, color_blind_daltonize_factor),
            },
            FieldInfoData {
                name: "ColorBlindBrightnessFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, color_blind_brightness_factor),
            },
            FieldInfoData {
                name: "ColorBlindContrastFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, color_blind_contrast_factor),
            },
            FieldInfoData {
                name: "RenderScaleResampleMode",
                flags: MemberInfoFlags::new(0),
                field_type: SCALERESAMPLEMODE_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, render_scale_resample_mode),
            },
            FieldInfoData {
                name: "RenderScaleResampleEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, render_scale_resample_enable),
            },
            FieldInfoData {
                name: "BlurEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, blur_enable),
            },
            FieldInfoData {
                name: "StereoCrosshairMaxHitDepth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, stereo_crosshair_max_hit_depth),
            },
            FieldInfoData {
                name: "StereoCrosshairRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, stereo_crosshair_radius),
            },
            FieldInfoData {
                name: "StereoCrosshairDampingFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, stereo_crosshair_damping_factor),
            },
            FieldInfoData {
                name: "HdrGradingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, hdr_grading_enable),
            },
            FieldInfoData {
                name: "DisplayMappingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, display_mapping_enable),
            },
            FieldInfoData {
                name: "DisplayMappingSdrPeakLuma",
                flags: MemberInfoFlags::new(8192),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, display_mapping_sdr_peak_luma),
            },
            FieldInfoData {
                name: "DisplayMappingHdr10PeakLuma",
                flags: MemberInfoFlags::new(8192),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, display_mapping_hdr10_peak_luma),
            },
            FieldInfoData {
                name: "DisplayMappingShoulderType",
                flags: MemberInfoFlags::new(0),
                field_type: DISPLAYMAPPINGSHOULDERTYPE_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, display_mapping_shoulder_type),
            },
            FieldInfoData {
                name: "HdrOutputPreferCs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, hdr_output_prefer_cs),
            },
            FieldInfoData {
                name: "HdrLiveGradingOverlayOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, hdr_live_grading_overlay_opacity),
            },
            FieldInfoData {
                name: "DrawHdrCalibrationScreen",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, draw_hdr_calibration_screen),
            },
            FieldInfoData {
                name: "DolbyVisionMetadataL1MinLuminanceOverride",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, dolby_vision_metadata_l1_min_luminance_override),
            },
            FieldInfoData {
                name: "DolbyVisionMetadataL1MaxLuminanceOverride",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, dolby_vision_metadata_l1_max_luminance_override),
            },
            FieldInfoData {
                name: "DolbyVisionMetadataL2MinLuminanceOverride",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, dolby_vision_metadata_l2_min_luminance_override),
            },
            FieldInfoData {
                name: "DolbyVisionMetadataL2MaxLuminanceOverride",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, dolby_vision_metadata_l2_max_luminance_override),
            },
            FieldInfoData {
                name: "DolbyVisionMetadataL2AvgLuminanceOverride",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, dolby_vision_metadata_l2_avg_luminance_override),
            },
            FieldInfoData {
                name: "DolbyVisionMetadataLuminanceOverrideEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, dolby_vision_metadata_luminance_override_enable),
            },
            FieldInfoData {
                name: "DolbyVisionMetadataDebugOverlayEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, dolby_vision_metadata_debug_overlay_enable),
            },
            FieldInfoData {
                name: "DistortionMaxValueScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, distortion_max_value_scale),
            },
            FieldInfoData {
                name: "FrameSynthesis",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, frame_synthesis),
            },
            FieldInfoData {
                name: "UIShadeInLinearSpaceEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, u_i_shade_in_linear_space_enabled),
            },
            FieldInfoData {
                name: "BrightnessScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, brightness_scale),
            },
            FieldInfoData {
                name: "RvmEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, rvm_enable),
            },
            FieldInfoData {
                name: "RvmTestMode",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, rvm_test_mode),
            },
            FieldInfoData {
                name: "RvmOnDemandBuildingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, rvm_on_demand_building_enable),
            },
            FieldInfoData {
                name: "LoadShaderDatabases",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, load_shader_databases),
            },
            FieldInfoData {
                name: "OverlayDropShadowAmount",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(GameRenderSettings, overlay_drop_shadow_amount),
            },
        ],
    }),
    array_type: Some(GAMERENDERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GameRenderSettings {
    fn type_info() -> &'static TypeInfo {
        GAMERENDERSETTINGS_TYPE_INFO
    }
}


pub const GAMERENDERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GameRenderSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("GameRenderSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DisplayMappingShoulderType {
    #[default]
    DisplayMappingShoulderType_None = 0,
    DisplayMappingShoulderType_Neutral = 1,
}

pub const DISPLAYMAPPINGSHOULDERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DisplayMappingShoulderType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(DISPLAYMAPPINGSHOULDERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DisplayMappingShoulderType {
    fn type_info() -> &'static TypeInfo {
        DISPLAYMAPPINGSHOULDERTYPE_TYPE_INFO
    }
}


pub const DISPLAYMAPPINGSHOULDERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DisplayMappingShoulderType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("DisplayMappingShoulderType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ScaleResampleMode {
    #[default]
    ScaleResampleMode_Point = 0,
    ScaleResampleMode_Linear = 1,
    ScaleResampleMode_Bicubic = 2,
    ScaleResampleMode_Lanczos = 3,
    ScaleResampleMode_LanczosSeparable = 4,
    ScaleResampleMode_BicubicSharp = 5,
    ScaleResampleMode_BicubicSharpSeparable = 6,
}

pub const SCALERESAMPLEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScaleResampleMode",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SCALERESAMPLEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ScaleResampleMode {
    fn type_info() -> &'static TypeInfo {
        SCALERESAMPLEMODE_TYPE_INFO
    }
}


pub const SCALERESAMPLEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScaleResampleMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ScaleResampleMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ResolutionSetGenerator {
    #[default]
    ResolutionSetGenerator_Normal = 0,
    ResolutionSetGenerator_Diagonal = 1,
    ResolutionSetGenerator_Horizontal = 2,
    ResolutionSetGenerator_Vertical = 3,
    ResolutionSetGenerator_Invalid = 4,
}

pub const RESOLUTIONSETGENERATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResolutionSetGenerator",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RESOLUTIONSETGENERATOR_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ResolutionSetGenerator {
    fn type_info() -> &'static TypeInfo {
        RESOLUTIONSETGENERATOR_TYPE_INFO
    }
}


pub const RESOLUTIONSETGENERATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResolutionSetGenerator-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ResolutionSetGenerator-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ResolutionRegulator {
    #[default]
    ResolutionRegulator_Default = 0,
    ResolutionRegulator_Sine = 1,
    ResolutionRegulator_PingPong = 2,
    ResolutionRegulator_Random = 3,
    ResolutionRegulator_MinMax = 4,
    ResolutionRegulator_Invalid = 5,
}

pub const RESOLUTIONREGULATOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResolutionRegulator",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RESOLUTIONREGULATOR_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ResolutionRegulator {
    fn type_info() -> &'static TypeInfo {
        RESOLUTIONREGULATOR_TYPE_INFO
    }
}


pub const RESOLUTIONREGULATOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResolutionRegulator-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ResolutionRegulator-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RenderDLAASupportChangedMessage {
}

pub const RENDERDLAASUPPORTCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderDLAASupportChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Render",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for RenderDLAASupportChangedMessage {
    fn type_info() -> &'static TypeInfo {
        RENDERDLAASUPPORTCHANGEDMESSAGE_TYPE_INFO
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EnlightenLightProbeMode {
    #[default]
    EnlightenLightProbeMode_ShL1 = 0,
    EnlightenLightProbeMode_DoubleShL1 = 1,
    EnlightenLightProbeMode_ShL2 = 2,
}

pub const ENLIGHTENLIGHTPROBEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenLightProbeMode",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(ENLIGHTENLIGHTPROBEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EnlightenLightProbeMode {
    fn type_info() -> &'static TypeInfo {
        ENLIGHTENLIGHTPROBEMODE_TYPE_INFO
    }
}


pub const ENLIGHTENLIGHTPROBEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenLightProbeMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("EnlightenLightProbeMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum SupportedLightMapModes {
    #[default]
    SupportedLightMapModes_SingleDirection = 1,
    SupportedLightMapModes_RgbDirection = 2,
    SupportedLightMapModes_SingleAndRgb = 3,
}

pub const SUPPORTEDLIGHTMAPMODES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SupportedLightMapModes",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SUPPORTEDLIGHTMAPMODES_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SupportedLightMapModes {
    fn type_info() -> &'static TypeInfo {
        SUPPORTEDLIGHTMAPMODES_TYPE_INFO
    }
}


pub const SUPPORTEDLIGHTMAPMODES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SupportedLightMapModes-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("SupportedLightMapModes-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EnlightenOutputFormat {
    #[default]
    EnlightenOutputFormat_Fp16 = 0,
    EnlightenOutputFormat_R9G9B9E5 = 1,
    EnlightenOutputFormat_R11G11B10 = 2,
    EnlightenOutputFormatCount = 3,
}

pub const ENLIGHTENOUTPUTFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenOutputFormat",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(ENLIGHTENOUTPUTFORMAT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EnlightenOutputFormat {
    fn type_info() -> &'static TypeInfo {
        ENLIGHTENOUTPUTFORMAT_TYPE_INFO
    }
}


pub const ENLIGHTENOUTPUTFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenOutputFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("EnlightenOutputFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnlightenShaderDatabaseAsset {
    pub num_shaders: u32,
    pub database_resource: super::core::ResourceRef,
}

pub const ENLIGHTENSHADERDATABASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenShaderDatabaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENLIGHTENSHADERDATABASEBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "NumShaders",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenShaderDatabaseAsset, num_shaders),
            },
            FieldInfoData {
                name: "DatabaseResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(EnlightenShaderDatabaseAsset, database_resource),
            },
        ],
    }),
    array_type: Some(ENLIGHTENSHADERDATABASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnlightenShaderDatabaseAsset {
    fn type_info() -> &'static TypeInfo {
        ENLIGHTENSHADERDATABASEASSET_TYPE_INFO
    }
}


pub const ENLIGHTENSHADERDATABASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenShaderDatabaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("EnlightenShaderDatabaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StaticEnlightenData {
    pub static_irradiance_texture: TextureAsset,
    pub static_direction_texture: TextureAsset,
    pub static_direction_texture_g: TextureAsset,
    pub static_direction_texture_b: TextureAsset,
    pub static_sky_visibility_texture: TextureAsset,
    pub static_gen4_enable: bool,
    pub ambient_occlusion_texture_compression_enable: bool,
    pub irradiance_texture_compression_enable: bool,
    pub database_resource: super::core::ResourceRef,
}

pub const STATICENLIGHTENDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticEnlightenData",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(STATICENLIGHTENBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StaticIrradianceTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREASSET_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenData, static_irradiance_texture),
            },
            FieldInfoData {
                name: "StaticDirectionTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREASSET_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenData, static_direction_texture),
            },
            FieldInfoData {
                name: "StaticDirectionTextureG",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREASSET_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenData, static_direction_texture_g),
            },
            FieldInfoData {
                name: "StaticDirectionTextureB",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREASSET_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenData, static_direction_texture_b),
            },
            FieldInfoData {
                name: "StaticSkyVisibilityTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREASSET_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenData, static_sky_visibility_texture),
            },
            FieldInfoData {
                name: "StaticGen4Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenData, static_gen4_enable),
            },
            FieldInfoData {
                name: "AmbientOcclusionTextureCompressionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenData, ambient_occlusion_texture_compression_enable),
            },
            FieldInfoData {
                name: "IrradianceTextureCompressionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenData, irradiance_texture_compression_enable),
            },
            FieldInfoData {
                name: "DatabaseResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(StaticEnlightenData, database_resource),
            },
        ],
    }),
    array_type: Some(STATICENLIGHTENDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StaticEnlightenData {
    fn type_info() -> &'static TypeInfo {
        STATICENLIGHTENDATA_TYPE_INFO
    }
}


pub const STATICENLIGHTENDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticEnlightenData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("StaticEnlightenData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EnlightenGlobalConfigAsset {
    pub mix_feature_mode: super::render_base::RadiosityMixMode,
}

pub const ENLIGHTENGLOBALCONFIGASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenGlobalConfigAsset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MixFeatureMode",
                flags: MemberInfoFlags::new(0),
                field_type: RADIOSITYMIXMODE_TYPE_INFO,
                rust_offset: offset_of!(EnlightenGlobalConfigAsset, mix_feature_mode),
            },
        ],
    }),
    array_type: Some(ENLIGHTENGLOBALCONFIGASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnlightenGlobalConfigAsset {
    fn type_info() -> &'static TypeInfo {
        ENLIGHTENGLOBALCONFIGASSET_TYPE_INFO
    }
}


pub const ENLIGHTENGLOBALCONFIGASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenGlobalConfigAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("EnlightenGlobalConfigAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EnlightenDataAsset {
    pub dynamic_enable: bool,
    pub load_debug_data: bool,
    pub group: String,
    pub dynamic_gen4a_enable: bool,
    pub dynamic_gen4b_enable: bool,
    pub dynamic_win32_enable: bool,
    pub dynamic_android_enable: bool,
    pub dynamici_o_s_enable: bool,
    pub dynamic_o_s_x_enable: bool,
    pub debug_chart_texture: TextureAsset,
    pub debug_back_face_texture: TextureAsset,
    pub sky_visibility_texture: TextureAsset,
    pub output_format: EnlightenOutputFormat,
    pub directional_irradiance_rgb_enable: bool,
    pub distant_lightprobe_position: super::core::Vec3,
    pub grid_based_system_generation: bool,
    pub system_grid_size: u32,
    pub system_lightmap_size: i32,
    pub max_system_lightmap_size: i32,
    pub max_lightmap_size: i32,
    pub system_influence_radius: f32,
    pub cluster_size: f32,
    pub ir_budget: u32,
    pub irradiance_quality_multiplier: u32,
    pub voxel_based_leaf_clustering: bool,
    pub pixel_stitching_enable: bool,
    pub edge_stitching_enable: bool,
    pub aligned_edge_stitching_enable: bool,
    pub stitching_distance_multiplier: f32,
    pub edge_stitching_distance_multiplier: f32,
    pub max_stitching_angle: f32,
    pub max_pixel_stitching_angle: f32,
    pub dependency_visibility_threshold: f32,
    pub align_resolution_to_pow2_enable: bool,
    pub samples_per_cluster: u32,
    pub max_cpu_thread_count: u32,
    pub terrain_enable: bool,
    pub terrain_probe_res: u32,
    pub environment_quality: u32,
    pub flux_lightmap_scale: u32,
    pub flux_global_solution: bool,
    pub flux_global_solution_include_group_probe_set: bool,
    pub flux_ambient_occlusion_enable: bool,
    pub flux_ambient_occlusion_radius: f32,
    pub flux_ambient_occlusion_rays: u32,
    pub ambient_occlusion_texture_compression_enable: bool,
    pub default_probe_priority: i32,
    pub flux_use_lightmap_stitching: bool,
    pub flux_lightmap_stitching_distance: f32,
    pub database_resource: super::core::ResourceRef,
}

pub const ENLIGHTENDATAASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenDataAsset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENLIGHTENBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DynamicEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, dynamic_enable),
            },
            FieldInfoData {
                name: "LoadDebugData",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, load_debug_data),
            },
            FieldInfoData {
                name: "Group",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, group),
            },
            FieldInfoData {
                name: "DynamicGen4aEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, dynamic_gen4a_enable),
            },
            FieldInfoData {
                name: "DynamicGen4bEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, dynamic_gen4b_enable),
            },
            FieldInfoData {
                name: "DynamicWin32Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, dynamic_win32_enable),
            },
            FieldInfoData {
                name: "DynamicAndroidEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, dynamic_android_enable),
            },
            FieldInfoData {
                name: "DynamiciOSEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, dynamici_o_s_enable),
            },
            FieldInfoData {
                name: "DynamicOSXEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, dynamic_o_s_x_enable),
            },
            FieldInfoData {
                name: "DebugChartTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREASSET_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, debug_chart_texture),
            },
            FieldInfoData {
                name: "DebugBackFaceTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREASSET_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, debug_back_face_texture),
            },
            FieldInfoData {
                name: "SkyVisibilityTexture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREASSET_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, sky_visibility_texture),
            },
            FieldInfoData {
                name: "OutputFormat",
                flags: MemberInfoFlags::new(0),
                field_type: ENLIGHTENOUTPUTFORMAT_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, output_format),
            },
            FieldInfoData {
                name: "DirectionalIrradianceRgbEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, directional_irradiance_rgb_enable),
            },
            FieldInfoData {
                name: "DistantLightprobePosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, distant_lightprobe_position),
            },
            FieldInfoData {
                name: "GridBasedSystemGeneration",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, grid_based_system_generation),
            },
            FieldInfoData {
                name: "SystemGridSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, system_grid_size),
            },
            FieldInfoData {
                name: "SystemLightmapSize",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, system_lightmap_size),
            },
            FieldInfoData {
                name: "MaxSystemLightmapSize",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, max_system_lightmap_size),
            },
            FieldInfoData {
                name: "MaxLightmapSize",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, max_lightmap_size),
            },
            FieldInfoData {
                name: "SystemInfluenceRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, system_influence_radius),
            },
            FieldInfoData {
                name: "ClusterSize",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, cluster_size),
            },
            FieldInfoData {
                name: "IrBudget",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, ir_budget),
            },
            FieldInfoData {
                name: "IrradianceQualityMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, irradiance_quality_multiplier),
            },
            FieldInfoData {
                name: "VoxelBasedLeafClustering",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, voxel_based_leaf_clustering),
            },
            FieldInfoData {
                name: "PixelStitchingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, pixel_stitching_enable),
            },
            FieldInfoData {
                name: "EdgeStitchingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, edge_stitching_enable),
            },
            FieldInfoData {
                name: "AlignedEdgeStitchingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, aligned_edge_stitching_enable),
            },
            FieldInfoData {
                name: "StitchingDistanceMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, stitching_distance_multiplier),
            },
            FieldInfoData {
                name: "EdgeStitchingDistanceMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, edge_stitching_distance_multiplier),
            },
            FieldInfoData {
                name: "MaxStitchingAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, max_stitching_angle),
            },
            FieldInfoData {
                name: "MaxPixelStitchingAngle",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, max_pixel_stitching_angle),
            },
            FieldInfoData {
                name: "DependencyVisibilityThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, dependency_visibility_threshold),
            },
            FieldInfoData {
                name: "AlignResolutionToPow2Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, align_resolution_to_pow2_enable),
            },
            FieldInfoData {
                name: "SamplesPerCluster",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, samples_per_cluster),
            },
            FieldInfoData {
                name: "MaxCpuThreadCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, max_cpu_thread_count),
            },
            FieldInfoData {
                name: "TerrainEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, terrain_enable),
            },
            FieldInfoData {
                name: "TerrainProbeRes",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, terrain_probe_res),
            },
            FieldInfoData {
                name: "EnvironmentQuality",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, environment_quality),
            },
            FieldInfoData {
                name: "FluxLightmapScale",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, flux_lightmap_scale),
            },
            FieldInfoData {
                name: "FluxGlobalSolution",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, flux_global_solution),
            },
            FieldInfoData {
                name: "FluxGlobalSolutionIncludeGroupProbeSet",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, flux_global_solution_include_group_probe_set),
            },
            FieldInfoData {
                name: "FluxAmbientOcclusionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, flux_ambient_occlusion_enable),
            },
            FieldInfoData {
                name: "FluxAmbientOcclusionRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, flux_ambient_occlusion_radius),
            },
            FieldInfoData {
                name: "FluxAmbientOcclusionRays",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, flux_ambient_occlusion_rays),
            },
            FieldInfoData {
                name: "AmbientOcclusionTextureCompressionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, ambient_occlusion_texture_compression_enable),
            },
            FieldInfoData {
                name: "DefaultProbePriority",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, default_probe_priority),
            },
            FieldInfoData {
                name: "FluxUseLightmapStitching",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, flux_use_lightmap_stitching),
            },
            FieldInfoData {
                name: "FluxLightmapStitchingDistance",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, flux_lightmap_stitching_distance),
            },
            FieldInfoData {
                name: "DatabaseResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(EnlightenDataAsset, database_resource),
            },
        ],
    }),
    array_type: Some(ENLIGHTENDATAASSET_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EnlightenDataAsset {
    fn type_info() -> &'static TypeInfo {
        ENLIGHTENDATAASSET_TYPE_INFO
    }
}


pub const ENLIGHTENDATAASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenDataAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("EnlightenDataAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EnlightenTransparencyMode {
    #[default]
    EnlightenTransparencyMode_UseLightProbeVolumes = 0,
    EnlightenTransparencyMode_UseDefaultLightProbe = 1,
    EnlightenTransparencyMode_UseTransLightProbes = 2,
}

pub const ENLIGHTENTRANSPARENCYMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenTransparencyMode",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(ENLIGHTENTRANSPARENCYMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EnlightenTransparencyMode {
    fn type_info() -> &'static TypeInfo {
        ENLIGHTENTRANSPARENCYMODE_TYPE_INFO
    }
}


pub const ENLIGHTENTRANSPARENCYMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenTransparencyMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("EnlightenTransparencyMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum GiBakeMode {
    #[default]
    GiBakeMode_Local = 0,
    GiBakeMode_Incredibuild = 1,
    GiBakeMode_SNDBS = 2,
}

pub const GIBAKEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GiBakeMode",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(GIBAKEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GiBakeMode {
    fn type_info() -> &'static TypeInfo {
        GIBAKEMODE_TYPE_INFO
    }
}


pub const GIBAKEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GiBakeMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("GiBakeMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum VertexElementClassification {
    #[default]
    VertexElementClassification_PerVertex = 0,
    VertexElementClassification_PerInstance = 1,
}

pub const VERTEXELEMENTCLASSIFICATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexElementClassification",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(VERTEXELEMENTCLASSIFICATION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VertexElementClassification {
    fn type_info() -> &'static TypeInfo {
        VERTEXELEMENTCLASSIFICATION_TYPE_INFO
    }
}


pub const VERTEXELEMENTCLASSIFICATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexElementClassification-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("VertexElementClassification-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum VertexElementUsage {
    #[default]
    VertexElementUsage_Unknown = 0,
    VertexElementUsage_Pos = 1,
    VertexElementUsage_BoneIndices = 2,
    VertexElementUsage_BoneIndices2 = 3,
    VertexElementUsage_BoneWeights = 4,
    VertexElementUsage_BoneWeights2 = 5,
    VertexElementUsage_Normal = 6,
    VertexElementUsage_Tangent = 7,
    VertexElementUsage_Binormal = 8,
    VertexElementUsage_BinormalSign = 9,
    VertexElementUsage_WorldTrans1 = 10,
    VertexElementUsage_WorldTrans2 = 11,
    VertexElementUsage_WorldTrans3 = 12,
    VertexElementUsage_InstanceId = 13,
    VertexElementUsage_InstanceUserData0 = 14,
    VertexElementUsage_PrevInstanceUserData0 = 15,
    VertexElementUsage_InstanceUserData1 = 16,
    VertexElementUsage_PrevWorldTrans1 = 17,
    VertexElementUsage_PrevWorldTrans2 = 18,
    VertexElementUsage_PrevWorldTrans3 = 19,
    VertexElementUsage_XenonIndex = 20,
    VertexElementUsage_XenonBarycentric = 21,
    VertexElementUsage_XenonQuadID = 22,
    VertexElementUsage_Index = 23,
    VertexElementUsage_ViewIndex = 24,
    VertexElementUsage_Color0 = 30,
    VertexElementUsage_Color1 = 31,
    VertexElementUsage_TexCoord0 = 33,
    VertexElementUsage_TexCoord1 = 34,
    VertexElementUsage_TexCoord2 = 35,
    VertexElementUsage_TexCoord3 = 36,
    VertexElementUsage_TexCoord4 = 37,
    VertexElementUsage_TexCoord5 = 38,
    VertexElementUsage_TexCoord6 = 39,
    VertexElementUsage_TexCoord7 = 40,
    VertexElementUsage_DisplacementMapTexCoord = 41,
    VertexElementUsage_RadiosityTexCoord = 42,
    VertexElementUsage_VisInfo = 43,
    VertexElementUsage_SpriteSize = 44,
    VertexElementUsage_PackedTexCoord0 = 45,
    VertexElementUsage_PackedTexCoord1 = 46,
    VertexElementUsage_PackedTexCoord2 = 47,
    VertexElementUsage_PackedTexCoord3 = 48,
    VertexElementUsage_ClipDistance0 = 49,
    VertexElementUsage_ClipDistance1 = 50,
    VertexElementUsage_SubMaterialIndex = 51,
    VertexElementUsage_TangentSpace = 52,
    VertexElementUsage_BranchInfo = 60,
    VertexElementUsage_PosAndScale = 61,
    VertexElementUsage_Rotation = 62,
    VertexElementUsage_SpriteSizeAndUv = 63,
    VertexElementUsage_FadePos = 90,
    VertexElementUsage_SpawnTime = 91,
    VertexElementUsage_RegionIds = 100,
    VertexElementUsage_BlendWeights = 101,
    VertexElementUsage_PosAndSoftMul = 150,
    VertexElementUsage_Alpha = 151,
    VertexElementUsage_Misc0 = 152,
    VertexElementUsage_Misc1 = 153,
    VertexElementUsage_Misc2 = 154,
    VertexElementUsage_Misc3 = 155,
    VertexElementUsage_LeftAndRotation = 156,
    VertexElementUsage_UpAndNormalBlend = 157,
    VertexElementUsage_Hl2BasisL0 = 158,
    VertexElementUsage_Hl2BasisL1 = 159,
    VertexElementUsage_Hl2BasisL2 = 160,
    VertexElementUsage_PosAndRejectCulling = 161,
    VertexElementUsage_Shadow = 162,
    VertexElementUsage_CustomParams = 163,
    VertexElementUsage_PatchUv = 180,
    VertexElementUsage_Height = 181,
    VertexElementUsage_MaskUVs0 = 182,
    VertexElementUsage_MaskUVs1 = 183,
    VertexElementUsage_MaskUVs2 = 184,
    VertexElementUsage_MaskUVs3 = 185,
    VertexElementUsage_UserMasks = 186,
    VertexElementUsage_HeightfieldUv = 187,
    VertexElementUsage_MaskUv = 188,
    VertexElementUsage_GlobalColorUv = 189,
    VertexElementUsage_HeightfieldPixelSizeAndAspect = 190,
    VertexElementUsage_WorldPositionXz = 191,
    VertexElementUsage_TerrainTextureNodeUv = 192,
    VertexElementUsage_ParentTerrainTextureNodeUv = 193,
    VertexElementUsage_PatchMissingNeighbors = 194,
    VertexElementUsage_DeformationIndex = 205,
    VertexElementUsage_DeformationWeight = 206,
    VertexElementUsage_DeformationPosition = 207,
    VertexElementUsage_Delta = 208,
    VertexElementUsage_ElementIndex = 209,
    VertexElementUsage_Uv01 = 210,
    VertexElementUsage_WorldPos = 211,
    VertexElementUsage_EyeVector = 212,
    VertexElementUsage_LightParams1 = 220,
    VertexElementUsage_LightParams2 = 221,
    VertexElementUsage_LightSubParams = 222,
    VertexElementUsage_LightSideVector = 223,
    VertexElementUsage_LightInnerAndOuterAngle = 224,
    VertexElementUsage_LightDir = 225,
    VertexElementUsage_LightMatrix1 = 226,
    VertexElementUsage_LightMatrix2 = 227,
    VertexElementUsage_LightMatrix3 = 228,
    VertexElementUsage_LightMatrix4 = 229,
    VertexElementUsage_Custom = 230,
    VertexElementUsage_DestructionMaskDistance = 240,
    VertexElementUsage_DestructionMaskTexCoord = 241,
    VertexElementUsage_VertIndex = 250,
}

pub const VERTEXELEMENTUSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexElementUsage",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(VERTEXELEMENTUSAGE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VertexElementUsage {
    fn type_info() -> &'static TypeInfo {
        VERTEXELEMENTUSAGE_TYPE_INFO
    }
}


pub const VERTEXELEMENTUSAGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexElementUsage-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("VertexElementUsage-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum VertexElementFormat {
    #[default]
    VertexElementFormat_None = 0,
    VertexElementFormat_Float = 1,
    VertexElementFormat_Float2 = 2,
    VertexElementFormat_Float3 = 3,
    VertexElementFormat_Float4 = 4,
    VertexElementFormat_Half = 5,
    VertexElementFormat_Half2 = 6,
    VertexElementFormat_Half3 = 7,
    VertexElementFormat_Half4 = 8,
    VertexElementFormat_UByteN = 50,
    VertexElementFormat_Byte4 = 10,
    VertexElementFormat_Byte4N = 11,
    VertexElementFormat_UByte4 = 12,
    VertexElementFormat_UByte4N = 13,
    VertexElementFormat_Short = 14,
    VertexElementFormat_Short2 = 15,
    VertexElementFormat_Short3 = 16,
    VertexElementFormat_Short4 = 17,
    VertexElementFormat_ShortN = 18,
    VertexElementFormat_Short2N = 19,
    VertexElementFormat_Short3N = 20,
    VertexElementFormat_Short4N = 21,
    VertexElementFormat_UShort2 = 22,
    VertexElementFormat_UShort4 = 23,
    VertexElementFormat_UShort2N = 24,
    VertexElementFormat_UShort4N = 25,
    VertexElementFormat_Int = 26,
    VertexElementFormat_Int2 = 27,
    VertexElementFormat_Int3 = 51,
    VertexElementFormat_Int4 = 28,
    VertexElementFormat_IntN = 29,
    VertexElementFormat_Int2N = 30,
    VertexElementFormat_Int4N = 31,
    VertexElementFormat_UInt = 32,
    VertexElementFormat_UInt2 = 33,
    VertexElementFormat_UInt3 = 52,
    VertexElementFormat_UInt4 = 34,
    VertexElementFormat_UIntN = 35,
    VertexElementFormat_UInt2N = 36,
    VertexElementFormat_UInt4N = 37,
    VertexElementFormat_Comp3_10_10_10 = 38,
    VertexElementFormat_Comp3N_10_10_10 = 39,
    VertexElementFormat_UComp3_10_10_10 = 40,
    VertexElementFormat_UComp3N_10_10_10 = 41,
    VertexElementFormat_Comp3_11_11_10 = 42,
    VertexElementFormat_Comp3N_11_11_10 = 43,
    VertexElementFormat_UComp3_11_11_10 = 44,
    VertexElementFormat_UComp3N_11_11_10 = 45,
    VertexElementFormat_Comp4_10_10_10_2 = 46,
    VertexElementFormat_Comp4N_10_10_10_2 = 47,
    VertexElementFormat_UComp4_10_10_10_2 = 48,
    VertexElementFormat_UComp4N_10_10_10_2 = 49,
}

pub const VERTEXELEMENTFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexElementFormat",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(VERTEXELEMENTFORMAT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VertexElementFormat {
    fn type_info() -> &'static TypeInfo {
        VERTEXELEMENTFORMAT_TYPE_INFO
    }
}


pub const VERTEXELEMENTFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VertexElementFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("VertexElementFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TextureFilter {
    #[default]
    TfNone = 0,
    TfPoint = 1,
    TfLinear = 2,
    TfAnisotropic = 3,
    TfDefault = 4,
}

pub const TEXTUREFILTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureFilter",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(TEXTUREFILTER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TextureFilter {
    fn type_info() -> &'static TypeInfo {
        TEXTUREFILTER_TYPE_INFO
    }
}


pub const TEXTUREFILTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureFilter-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("TextureFilter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TextureType {
    #[default]
    TextureType_1d = 5,
    TextureType_1dArray = 4,
    TextureType_2d = 0,
    TextureType_2dMS = 7,
    TextureType_2dArray = 3,
    TextureType_Cube = 1,
    TextureType_3d = 2,
    TextureType_CubeArray = 6,
    TextureTypeCount = 8,
}

pub const TEXTURETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(TEXTURETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TextureType {
    fn type_info() -> &'static TypeInfo {
        TEXTURETYPE_TYPE_INFO
    }
}


pub const TEXTURETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("TextureType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextureAtlasAsset {
}

pub const TEXTUREATLASASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureAtlasAsset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TEXTUREATLASASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TextureAtlasAsset {
    fn type_info() -> &'static TypeInfo {
        TEXTUREATLASASSET_TYPE_INFO
    }
}


pub const TEXTUREATLASASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureAtlasAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("TextureAtlasAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderTextureDecompression {
    #[default]
    ShaderTextureDecompression_None = 0,
    ShaderTextureDecompression_NormalAG = 1,
    ShaderTextureDecompression_NormalRG = 2,
    ShaderTextureDecompression_NormalRGA = 3,
    ShaderTextureDecompression_NormalRGorAG = 4,
}

pub const SHADERTEXTUREDECOMPRESSION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderTextureDecompression",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERTEXTUREDECOMPRESSION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderTextureDecompression {
    fn type_info() -> &'static TypeInfo {
        SHADERTEXTUREDECOMPRESSION_TYPE_INFO
    }
}


pub const SHADERTEXTUREDECOMPRESSION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderTextureDecompression-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderTextureDecompression-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RenderTextureAsset {
}

pub const RENDERTEXTUREASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderTextureAsset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RENDERTEXTUREBASEASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RENDERTEXTUREASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RenderTextureAsset {
    fn type_info() -> &'static TypeInfo {
        RENDERTEXTUREASSET_TYPE_INFO
    }
}


pub const RENDERTEXTUREASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderTextureAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RenderTextureAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RenderTextureOutputType {
    #[default]
    RenderTextureOutputType_Graphics = 0,
    RenderTextureOutputType_Compute = 1,
    RenderTextureOutputType_GraphicsAndCompute = 2,
}

pub const RENDERTEXTUREOUTPUTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderTextureOutputType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RENDERTEXTUREOUTPUTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderTextureOutputType {
    fn type_info() -> &'static TypeInfo {
        RENDERTEXTUREOUTPUTTYPE_TYPE_INFO
    }
}


pub const RENDERTEXTUREOUTPUTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderTextureOutputType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RenderTextureOutputType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GradingLutAsset {
}

pub const GRADINGLUTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GradingLutAsset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TEXTUREASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GRADINGLUTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GradingLutAsset {
    fn type_info() -> &'static TypeInfo {
        GRADINGLUTASSET_TYPE_INFO
    }
}


pub const GRADINGLUTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GradingLutAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("GradingLutAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AtlasTextureAsset {
    pub animation_column_count: i32,
    pub animation_frame_count: i32,
    pub left_right_tiles: bool,
    pub resource: super::core::ResourceRef,
}

pub const ATLASTEXTUREASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AtlasTextureAsset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AnimationColumnCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AtlasTextureAsset, animation_column_count),
            },
            FieldInfoData {
                name: "AnimationFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(AtlasTextureAsset, animation_frame_count),
            },
            FieldInfoData {
                name: "LeftRightTiles",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(AtlasTextureAsset, left_right_tiles),
            },
            FieldInfoData {
                name: "Resource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(AtlasTextureAsset, resource),
            },
        ],
    }),
    array_type: Some(ATLASTEXTUREASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AtlasTextureAsset {
    fn type_info() -> &'static TypeInfo {
        ATLASTEXTUREASSET_TYPE_INFO
    }
}


pub const ATLASTEXTUREASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AtlasTextureAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("AtlasTextureAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextureArrayAsset {
}

pub const TEXTUREARRAYASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureArrayAsset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TEXTUREASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TEXTUREARRAYASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TextureArrayAsset {
    fn type_info() -> &'static TypeInfo {
        TEXTUREARRAYASSET_TYPE_INFO
    }
}


pub const TEXTUREARRAYASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureArrayAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("TextureArrayAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextureAsset {
    pub generate_mipmaps_filter: GenerateMipmapsFilterType,
    pub resize_filter: ResizeFilter,
}

pub const TEXTUREASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureAsset",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TEXTUREBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "GenerateMipmapsFilter",
                flags: MemberInfoFlags::new(0),
                field_type: GENERATEMIPMAPSFILTERTYPE_TYPE_INFO,
                rust_offset: offset_of!(TextureAsset, generate_mipmaps_filter),
            },
            FieldInfoData {
                name: "ResizeFilter",
                flags: MemberInfoFlags::new(0),
                field_type: RESIZEFILTER_TYPE_INFO,
                rust_offset: offset_of!(TextureAsset, resize_filter),
            },
        ],
    }),
    array_type: Some(TEXTUREASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TextureAsset {
    fn type_info() -> &'static TypeInfo {
        TEXTUREASSET_TYPE_INFO
    }
}


pub const TEXTUREASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("TextureAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ResizeFilter {
    #[default]
    ResizeFilter_Default = 0,
    ResizeFilter_Box = 1,
    ResizeFilter_Triangle = 2,
    ResizeFilter_Cubic = 3,
    ResizeFilter_MitchellFilter = 4,
    ResizeFilter_Lanczos = 5,
    ResizeFilter_Kaiser = 6,
    ResizeFilter_Nearest = 7,
}

pub const RESIZEFILTER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResizeFilter",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RESIZEFILTER_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ResizeFilter {
    fn type_info() -> &'static TypeInfo {
        RESIZEFILTER_TYPE_INFO
    }
}


pub const RESIZEFILTER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ResizeFilter-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ResizeFilter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum GenerateMipmapsFilterType {
    #[default]
    GenerateMipmapsFilter_Box = 0,
    GenerateMipmapsFilter_BoxRgbSharpenAlpha = 1,
    GenerateMipmapsFilter_BoxRgbKeepAlphaTestCoverage = 2,
    GenerateMipmapsFilter_AlphaWeightedBox = 3,
    GenerateMipmapsFilter_Triangle = 4,
    GenerateMipmapsFilter_Cubic = 5,
    GenerateMipmapsFilter_MitchellFilter = 6,
    GenerateMipmapsFilter_Lanczos = 7,
    GenerateMipmapsFilter_Kaiser = 8,
    GenerateMipmapsFilter_Nearest = 9,
    GenerateMipmapsFilter_BoxSharpened = 10,
}

pub const GENERATEMIPMAPSFILTERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GenerateMipmapsFilterType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(GENERATEMIPMAPSFILTERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for GenerateMipmapsFilterType {
    fn type_info() -> &'static TypeInfo {
        GENERATEMIPMAPSFILTERTYPE_TYPE_INFO
    }
}


pub const GENERATEMIPMAPSFILTERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GenerateMipmapsFilterType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("GenerateMipmapsFilterType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderProgramFlags {
    #[default]
    ShaderProgramFlags_NV_Intrinsics = 1,
}

pub const SHADERPROGRAMFLAGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderProgramFlags",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERPROGRAMFLAGS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderProgramFlags {
    fn type_info() -> &'static TypeInfo {
        SHADERPROGRAMFLAGS_TYPE_INFO
    }
}


pub const SHADERPROGRAMFLAGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderProgramFlags-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderProgramFlags-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DynamicTextureArraySettings {
    pub ies_texture_size: u32,
    pub ies_texture_array_size: u32,
}

pub const DYNAMICTEXTUREARRAYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicTextureArraySettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "IesTextureSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicTextureArraySettings, ies_texture_size),
            },
            FieldInfoData {
                name: "IesTextureArraySize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicTextureArraySettings, ies_texture_array_size),
            },
        ],
    }),
    array_type: Some(DYNAMICTEXTUREARRAYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DynamicTextureArraySettings {
    fn type_info() -> &'static TypeInfo {
        DYNAMICTEXTUREARRAYSETTINGS_TYPE_INFO
    }
}


pub const DYNAMICTEXTUREARRAYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicTextureArraySettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("DynamicTextureArraySettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DynamicTextureAtlasSettings {
    pub emitter_base_width: u32,
    pub emitter_base_height: u32,
    pub emitter_base_mipmap_count: u32,
    pub emitter_base_skipmips_count: u32,
    pub emitter_normal_width: u32,
    pub emitter_normal_height: u32,
    pub emitter_normal_mipmap_count: u32,
    pub emitter_normal_skipmips_count: u32,
    pub emitter_base_pixel_border: f32,
}

pub const DYNAMICTEXTUREATLASSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicTextureAtlasSettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EmitterBaseWidth",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicTextureAtlasSettings, emitter_base_width),
            },
            FieldInfoData {
                name: "EmitterBaseHeight",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicTextureAtlasSettings, emitter_base_height),
            },
            FieldInfoData {
                name: "EmitterBaseMipmapCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicTextureAtlasSettings, emitter_base_mipmap_count),
            },
            FieldInfoData {
                name: "EmitterBaseSkipmipsCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicTextureAtlasSettings, emitter_base_skipmips_count),
            },
            FieldInfoData {
                name: "EmitterNormalWidth",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicTextureAtlasSettings, emitter_normal_width),
            },
            FieldInfoData {
                name: "EmitterNormalHeight",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicTextureAtlasSettings, emitter_normal_height),
            },
            FieldInfoData {
                name: "EmitterNormalMipmapCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicTextureAtlasSettings, emitter_normal_mipmap_count),
            },
            FieldInfoData {
                name: "EmitterNormalSkipmipsCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicTextureAtlasSettings, emitter_normal_skipmips_count),
            },
            FieldInfoData {
                name: "EmitterBasePixelBorder",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DynamicTextureAtlasSettings, emitter_base_pixel_border),
            },
        ],
    }),
    array_type: Some(DYNAMICTEXTUREATLASSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DynamicTextureAtlasSettings {
    fn type_info() -> &'static TypeInfo {
        DYNAMICTEXTUREATLASSETTINGS_TYPE_INFO
    }
}


pub const DYNAMICTEXTUREATLASSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicTextureAtlasSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("DynamicTextureAtlasSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextureSettings {
    pub skip_mipmap_count: u32,
    pub loading_enabled: bool,
    pub render_textures_enabled: bool,
    pub streamable_mipmaps_enable: bool,
}

pub const TEXTURESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureSettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SkipMipmapCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureSettings, skip_mipmap_count),
            },
            FieldInfoData {
                name: "LoadingEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureSettings, loading_enabled),
            },
            FieldInfoData {
                name: "RenderTexturesEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureSettings, render_textures_enabled),
            },
            FieldInfoData {
                name: "StreamableMipmapsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureSettings, streamable_mipmaps_enable),
            },
        ],
    }),
    array_type: Some(TEXTURESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TextureSettings {
    fn type_info() -> &'static TypeInfo {
        TEXTURESETTINGS_TYPE_INFO
    }
}


pub const TEXTURESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("TextureSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TextureStreamingSettings {
    pub enable: bool,
    pub update_enable: bool,
    pub chunk_load_enable: bool,
    pub instant_unloading_enable: bool,
    pub only_wanted_in_pool: bool,
    pub force_wanted_enable: bool,
    pub load_mipmaps_enable: bool,
    pub upload_mipmaps_enable: bool,
    pub unload_in_place_enable: bool,
    pub texture_update_enable: bool,
    pub immutable_usage_enable: bool,
    pub mipmaps_enable: bool,
    pub force_mipmap: i32,
    pub min_mipmap_count: u32,
    pub max_mipmap_count: u32,
    pub mipmap_bias: f32,
    pub max_texture_size_kb: u32,
    pub fade_mipmaps_enable: bool,
    pub fade_mipmap_time: f32,
    pub min_texture_size: u32,
    pub force_non_streamable_textures_in_streamable_pool: bool,
    pub override_pool_size: bool,
    pub pool_size: u32,
    pub pool_headroom_size: u32,
    pub pool_max_alloc_count: u32,
    pub on_demand_pool_size: u32,
    pub on_demand_max_alloc_count: u32,
    pub pool_enable: bool,
    pub defrag_enable: bool,
    pub defrag_transfers_enable: bool,
    pub defrag_frame_transfer_limit: u32,
    pub defrag_frame_search_limit: u32,
    pub defrag_job_count: u32,
    pub force_load_streaming_frame_delay: u32,
    pub force_load_streaming_immediate: bool,
    pub max_pending_load_count: u32,
    pub max_pending_unload_count: u32,
    pub max_frame_texture_create_size: u32,
    pub max_frame_texture_create_count: u32,
    pub priority_threshold: f32,
    pub draw_stats_enable: bool,
    pub draw_stats_offset_x: i32,
    pub draw_stats_offset_y: i32,
    pub draw_texture_group_stats_enable: bool,
    pub draw_texture_format_stats_enable: bool,
    pub draw_loading_list_enable: bool,
    pub draw_priority_list_enable: bool,
    pub list_view_page_index: u32,
    pub dump_loaded_list: bool,
    pub dump_loaded_list_file_name: String,
    pub dump_pool_allocations: bool,
    pub use_sweepable_pool: bool,
    pub sweepable_page_size: u32,
    pub sweepable_page_align: u32,
    pub sweepable_min_pages: u32,
    pub sweepable_reserved_pages: u32,
    pub sweepable_page_allocation_limit: u32,
    pub sweepable_direct_allocation_alignment_waste_threshold: i32,
    pub sweepable_use_virtual_pool: bool,
    pub sweepable_virtual_pool_can_split_large_blocks: bool,
    pub sweepable_virtual_pool_initial_virtual_size: u32,
    pub sweepable_virtual_pool_extend_virtual_size: u32,
    pub sweepable_virtual_pool_max_delayed_operations: u32,
    pub sweepable_virtual_pool_can_delay_allocations: bool,
}

pub const TEXTURESTREAMINGSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureStreamingSettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, enable),
            },
            FieldInfoData {
                name: "UpdateEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, update_enable),
            },
            FieldInfoData {
                name: "ChunkLoadEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, chunk_load_enable),
            },
            FieldInfoData {
                name: "InstantUnloadingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, instant_unloading_enable),
            },
            FieldInfoData {
                name: "OnlyWantedInPool",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, only_wanted_in_pool),
            },
            FieldInfoData {
                name: "ForceWantedEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, force_wanted_enable),
            },
            FieldInfoData {
                name: "LoadMipmapsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, load_mipmaps_enable),
            },
            FieldInfoData {
                name: "UploadMipmapsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, upload_mipmaps_enable),
            },
            FieldInfoData {
                name: "UnloadInPlaceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, unload_in_place_enable),
            },
            FieldInfoData {
                name: "TextureUpdateEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, texture_update_enable),
            },
            FieldInfoData {
                name: "ImmutableUsageEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, immutable_usage_enable),
            },
            FieldInfoData {
                name: "MipmapsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, mipmaps_enable),
            },
            FieldInfoData {
                name: "ForceMipmap",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, force_mipmap),
            },
            FieldInfoData {
                name: "MinMipmapCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, min_mipmap_count),
            },
            FieldInfoData {
                name: "MaxMipmapCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, max_mipmap_count),
            },
            FieldInfoData {
                name: "MipmapBias",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, mipmap_bias),
            },
            FieldInfoData {
                name: "MaxTextureSizeKb",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, max_texture_size_kb),
            },
            FieldInfoData {
                name: "FadeMipmapsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, fade_mipmaps_enable),
            },
            FieldInfoData {
                name: "FadeMipmapTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, fade_mipmap_time),
            },
            FieldInfoData {
                name: "MinTextureSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, min_texture_size),
            },
            FieldInfoData {
                name: "ForceNonStreamableTexturesInStreamablePool",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, force_non_streamable_textures_in_streamable_pool),
            },
            FieldInfoData {
                name: "OverridePoolSize",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, override_pool_size),
            },
            FieldInfoData {
                name: "PoolSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, pool_size),
            },
            FieldInfoData {
                name: "PoolHeadroomSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, pool_headroom_size),
            },
            FieldInfoData {
                name: "PoolMaxAllocCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, pool_max_alloc_count),
            },
            FieldInfoData {
                name: "OnDemandPoolSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, on_demand_pool_size),
            },
            FieldInfoData {
                name: "OnDemandMaxAllocCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, on_demand_max_alloc_count),
            },
            FieldInfoData {
                name: "PoolEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, pool_enable),
            },
            FieldInfoData {
                name: "DefragEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, defrag_enable),
            },
            FieldInfoData {
                name: "DefragTransfersEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, defrag_transfers_enable),
            },
            FieldInfoData {
                name: "DefragFrameTransferLimit",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, defrag_frame_transfer_limit),
            },
            FieldInfoData {
                name: "DefragFrameSearchLimit",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, defrag_frame_search_limit),
            },
            FieldInfoData {
                name: "DefragJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, defrag_job_count),
            },
            FieldInfoData {
                name: "ForceLoadStreamingFrameDelay",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, force_load_streaming_frame_delay),
            },
            FieldInfoData {
                name: "ForceLoadStreamingImmediate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, force_load_streaming_immediate),
            },
            FieldInfoData {
                name: "MaxPendingLoadCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, max_pending_load_count),
            },
            FieldInfoData {
                name: "MaxPendingUnloadCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, max_pending_unload_count),
            },
            FieldInfoData {
                name: "MaxFrameTextureCreateSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, max_frame_texture_create_size),
            },
            FieldInfoData {
                name: "MaxFrameTextureCreateCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, max_frame_texture_create_count),
            },
            FieldInfoData {
                name: "PriorityThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, priority_threshold),
            },
            FieldInfoData {
                name: "DrawStatsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, draw_stats_enable),
            },
            FieldInfoData {
                name: "DrawStatsOffsetX",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, draw_stats_offset_x),
            },
            FieldInfoData {
                name: "DrawStatsOffsetY",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, draw_stats_offset_y),
            },
            FieldInfoData {
                name: "DrawTextureGroupStatsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, draw_texture_group_stats_enable),
            },
            FieldInfoData {
                name: "DrawTextureFormatStatsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, draw_texture_format_stats_enable),
            },
            FieldInfoData {
                name: "DrawLoadingListEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, draw_loading_list_enable),
            },
            FieldInfoData {
                name: "DrawPriorityListEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, draw_priority_list_enable),
            },
            FieldInfoData {
                name: "ListViewPageIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, list_view_page_index),
            },
            FieldInfoData {
                name: "DumpLoadedList",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, dump_loaded_list),
            },
            FieldInfoData {
                name: "DumpLoadedListFileName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, dump_loaded_list_file_name),
            },
            FieldInfoData {
                name: "DumpPoolAllocations",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, dump_pool_allocations),
            },
            FieldInfoData {
                name: "UseSweepablePool",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, use_sweepable_pool),
            },
            FieldInfoData {
                name: "SweepablePageSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, sweepable_page_size),
            },
            FieldInfoData {
                name: "SweepablePageAlign",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, sweepable_page_align),
            },
            FieldInfoData {
                name: "SweepableMinPages",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, sweepable_min_pages),
            },
            FieldInfoData {
                name: "SweepableReservedPages",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, sweepable_reserved_pages),
            },
            FieldInfoData {
                name: "SweepablePageAllocationLimit",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, sweepable_page_allocation_limit),
            },
            FieldInfoData {
                name: "SweepableDirectAllocationAlignmentWasteThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, sweepable_direct_allocation_alignment_waste_threshold),
            },
            FieldInfoData {
                name: "SweepableUseVirtualPool",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, sweepable_use_virtual_pool),
            },
            FieldInfoData {
                name: "SweepableVirtualPoolCanSplitLargeBlocks",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, sweepable_virtual_pool_can_split_large_blocks),
            },
            FieldInfoData {
                name: "SweepableVirtualPoolInitialVirtualSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, sweepable_virtual_pool_initial_virtual_size),
            },
            FieldInfoData {
                name: "SweepableVirtualPoolExtendVirtualSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, sweepable_virtual_pool_extend_virtual_size),
            },
            FieldInfoData {
                name: "SweepableVirtualPoolMaxDelayedOperations",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, sweepable_virtual_pool_max_delayed_operations),
            },
            FieldInfoData {
                name: "SweepableVirtualPoolCanDelayAllocations",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TextureStreamingSettings, sweepable_virtual_pool_can_delay_allocations),
            },
        ],
    }),
    array_type: Some(TEXTURESTREAMINGSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TextureStreamingSettings {
    fn type_info() -> &'static TypeInfo {
        TEXTURESTREAMINGSETTINGS_TYPE_INFO
    }
}


pub const TEXTURESTREAMINGSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureStreamingSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("TextureStreamingSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DisplayDynamicRange {
    #[default]
    DisplayDynamicRange_SDR = 0,
    DisplayDynamicRange_DolbyMaui = 1,
    DisplayDynamicRange_DolbyVision = 2,
    DisplayDynamicRange_LiveGrading = 3,
    DisplayDynamicRange_HDR10 = 4,
    DisplayDynamicRange_HDR10_Float = 5,
    DisplayDynamicRange_Auto = 6,
}

pub const DISPLAYDYNAMICRANGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DisplayDynamicRange",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(DISPLAYDYNAMICRANGE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DisplayDynamicRange {
    fn type_info() -> &'static TypeInfo {
        DISPLAYDYNAMICRANGE_TYPE_INFO
    }
}


pub const DISPLAYDYNAMICRANGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DisplayDynamicRange-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("DisplayDynamicRange-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderRenderPath {
    #[default]
    ShaderRenderPath_Dx10 = 0,
    ShaderRenderPath_Dx10Plus = 1,
    ShaderRenderPath_Dx10_1 = 2,
    ShaderRenderPath_Dx11 = 3,
    ShaderRenderPath_Dx11_1 = 4,
    ShaderRenderPath_Dx12 = 5,
    ShaderRenderPath_GLSL_410 = 6,
    ShaderRenderPath_Gen4a = 7,
    ShaderRenderPath_Gen4aDx12 = 8,
    ShaderRenderPath_Ps4 = 9,
    ShaderRenderPath_GLSL_ES_100 = 10,
    ShaderRenderPath_GLSL_ES_300 = 11,
    ShaderRenderPath_GLSL_ES_310 = 12,
    ShaderRenderPath_BLSL_1_0 = 13,
    ShaderRenderPathCount = 14,
}

pub const SHADERRENDERPATH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderRenderPath",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERRENDERPATH_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderRenderPath {
    fn type_info() -> &'static TypeInfo {
        SHADERRENDERPATH_TYPE_INFO
    }
}


pub const SHADERRENDERPATH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderRenderPath-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderRenderPath-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum StateObjectShaderStageType {
    #[default]
    StateObjectShaderStageType_Vertex = 0,
    StateObjectShaderStageType_Pixel = 1,
    StateObjectShaderStageType_Hull = 2,
    StateObjectShaderStageType_Geometry = 3,
    StateObjectShaderStageType_Export = 4,
    StateObjectShaderStageType_Load = 5,
    StateObjectShaderStageTypeCount = 6,
}

pub const STATEOBJECTSHADERSTAGETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateObjectShaderStageType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(STATEOBJECTSHADERSTAGETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StateObjectShaderStageType {
    fn type_info() -> &'static TypeInfo {
        STATEOBJECTSHADERSTAGETYPE_TYPE_INFO
    }
}


pub const STATEOBJECTSHADERSTAGETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateObjectShaderStageType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("StateObjectShaderStageType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ShaderStageType {
    #[default]
    ShaderStageType_Vertex = 0,
    ShaderStageType_Pixel = 1,
    ShaderStageType_Hull = 2,
    ShaderStageType_Domain = 3,
    ShaderStageType_Geometry = 4,
    ShaderStageType_Compute = 5,
    ShaderStageType_RayGen = 6,
    ShaderStageType_HitGroup = 7,
    ShaderStageType_Miss = 8,
    ShaderStageType_Callable = 9,
    ShaderStageTypeCount = 10,
}

pub const SHADERSTAGETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderStageType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(SHADERSTAGETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ShaderStageType {
    fn type_info() -> &'static TypeInfo {
        SHADERSTAGETYPE_TYPE_INFO
    }
}


pub const SHADERSTAGETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ShaderStageType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ShaderStageType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum StencilOperation {
    #[default]
    StencilOperation_Keep = 0,
    StencilOperation_Zero = 1,
    StencilOperation_Replace = 2,
    StencilOperation_IncrementSaturate = 3,
    StencilOperation_DecrementSaturate = 4,
    StencilOperation_Invert = 5,
    StencilOperation_IncrementWrap = 6,
    StencilOperation_DecrementWrap = 7,
}

pub const STENCILOPERATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StencilOperation",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(STENCILOPERATION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StencilOperation {
    fn type_info() -> &'static TypeInfo {
        STENCILOPERATION_TYPE_INFO
    }
}


pub const STENCILOPERATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StencilOperation-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("StencilOperation-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DepthStencilCompareFunc {
    #[default]
    DepthStencilCompareFunc_Never = 0,
    DepthStencilCompareFunc_Less = 1,
    DepthStencilCompareFunc_Equal = 2,
    DepthStencilCompareFunc_LessEqual = 3,
    DepthStencilCompareFunc_Greater = 4,
    DepthStencilCompareFunc_NotEqual = 5,
    DepthStencilCompareFunc_GreaterEqual = 6,
    DepthStencilCompareFunc_Always = 7,
}

pub const DEPTHSTENCILCOMPAREFUNC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DepthStencilCompareFunc",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(DEPTHSTENCILCOMPAREFUNC_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DepthStencilCompareFunc {
    fn type_info() -> &'static TypeInfo {
        DEPTHSTENCILCOMPAREFUNC_TYPE_INFO
    }
}


pub const DEPTHSTENCILCOMPAREFUNC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DepthStencilCompareFunc-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("DepthStencilCompareFunc-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RenderFillMode {
    #[default]
    RenderFillMode_Solid = 0,
    RenderFillMode_Wireframe = 1,
}

pub const RENDERFILLMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderFillMode",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RENDERFILLMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderFillMode {
    fn type_info() -> &'static TypeInfo {
        RENDERFILLMODE_TYPE_INFO
    }
}


pub const RENDERFILLMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderFillMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RenderFillMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RenderDepthMode {
    #[default]
    RenderDepthMode_Disabled = 0,
    RenderDepthMode_ReadOnly = 1,
    RenderDepthMode_ReadAndWrite = 2,
    RenderDepthMode_WriteOnly = 3,
    RenderDepthMode_ReadOnlyInverted = 4,
}

pub const RENDERDEPTHMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderDepthMode",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RENDERDEPTHMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderDepthMode {
    fn type_info() -> &'static TypeInfo {
        RENDERDEPTHMODE_TYPE_INFO
    }
}


pub const RENDERDEPTHMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderDepthMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RenderDepthMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RenderClearMask {
    #[default]
    RenderClearMask_Color0 = 1,
    RenderClearMask_Color1 = 2,
    RenderClearMask_Color2 = 4,
    RenderClearMask_Color3 = 8,
    RenderClearMask_Color4 = 16,
    RenderClearMask_Color5 = 32,
    RenderClearMask_Color6 = 64,
    RenderClearMask_Color7 = 128,
    RenderClearMask_Depth = 256,
    RenderClearMask_Stencil = 512,
    RenderClearMask_Color = 255,
    RenderClearMask_All = 1023,
    RenderClearMask_None = 0,
}

pub const RENDERCLEARMASK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderClearMask",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RENDERCLEARMASK_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderClearMask {
    fn type_info() -> &'static TypeInfo {
        RENDERCLEARMASK_TYPE_INFO
    }
}


pub const RENDERCLEARMASK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderClearMask-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RenderClearMask-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RenderWriteMask {
    #[default]
    RenderWriteMask_Red = 1,
    RenderWriteMask_Green = 2,
    RenderWriteMask_Blue = 4,
    RenderWriteMask_Alpha = 8,
    RenderWriteMask_None = 0,
    RenderWriteMask_Color = 7,
    RenderWriteMask_All = 15,
}

pub const RENDERWRITEMASK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderWriteMask",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RENDERWRITEMASK_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderWriteMask {
    fn type_info() -> &'static TypeInfo {
        RENDERWRITEMASK_TYPE_INFO
    }
}


pub const RENDERWRITEMASK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderWriteMask-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RenderWriteMask-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RenderBlendOp {
    #[default]
    RenderBlendOp_Add = 0,
    RenderBlendOp_Subtract = 1,
    RenderBlendOp_RevSubtract = 2,
    RenderBlendOp_Min = 3,
    RenderBlendOp_Max = 4,
    RenderBlendOpCount = 5,
}

pub const RENDERBLENDOP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderBlendOp",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RENDERBLENDOP_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderBlendOp {
    fn type_info() -> &'static TypeInfo {
        RENDERBLENDOP_TYPE_INFO
    }
}


pub const RENDERBLENDOP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderBlendOp-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RenderBlendOp-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RenderBlendMode {
    #[default]
    RenderBlendMode_Zero = 0,
    RenderBlendMode_One = 1,
    RenderBlendMode_SourceColor = 2,
    RenderBlendMode_InvSourceColor = 3,
    RenderBlendMode_SourceAlpha = 4,
    RenderBlendMode_InvSourceAlpha = 5,
    RenderBlendMode_DestColor = 6,
    RenderBlendMode_InvDestColor = 7,
    RenderBlendMode_DestAlpha = 8,
    RenderBlendMode_InvDestAlpha = 9,
    RenderBlendMode_SourceAlphaSaturate = 10,
    RenderBlendMode_Constant = 11,
    RenderBlendMode_InvConstant = 12,
    RenderBlendMode_Source1Color = 13,
    RenderBlendMode_InvSource1Color = 14,
    RenderBlendMode_Source1Alpha = 15,
    RenderBlendMode_InvSource1Alpha = 16,
    RenderBlendModeCount = 17,
}

pub const RENDERBLENDMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderBlendMode",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RENDERBLENDMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderBlendMode {
    fn type_info() -> &'static TypeInfo {
        RENDERBLENDMODE_TYPE_INFO
    }
}


pub const RENDERBLENDMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderBlendMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RenderBlendMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RenderCullMode {
    #[default]
    RenderCullMode_None = 0,
    RenderCullMode_Front = 1,
    RenderCullMode_Back = 2,
}

pub const RENDERCULLMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderCullMode",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RENDERCULLMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderCullMode {
    fn type_info() -> &'static TypeInfo {
        RENDERCULLMODE_TYPE_INFO
    }
}


pub const RENDERCULLMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderCullMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RenderCullMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PrimitiveType {
    #[default]
    PrimitiveType_PointList = 0,
    PrimitiveType_LineList = 1,
    PrimitiveType_LineStrip = 2,
    PrimitiveType_TriangleList = 3,
    PrimitiveType_TriangleStrip = 5,
    PrimitiveType_QuadList = 7,
    PrimitiveType_RectList = 8,
    PrimitiveType_TrianglePatch = 9,
    PrimitiveTypeCount = 10,
}

pub const PRIMITIVETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrimitiveType",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(PRIMITIVETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PrimitiveType {
    fn type_info() -> &'static TypeInfo {
        PRIMITIVETYPE_TYPE_INFO
    }
}


pub const PRIMITIVETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PrimitiveType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("PrimitiveType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RenderBorderColor {
    #[default]
    RenderBorderColor_BlackA0 = 0,
    RenderBorderColor_BlackA1 = 1,
    RenderBorderColor_WhiteA1 = 2,
    RenderBorderColor_RedA0 = 3,
    RenderBorderColor_RedA1 = 4,
    RenderBorderColorCount = 5,
}

pub const RENDERBORDERCOLOR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderBorderColor",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RENDERBORDERCOLOR_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderBorderColor {
    fn type_info() -> &'static TypeInfo {
        RENDERBORDERCOLOR_TYPE_INFO
    }
}


pub const RENDERBORDERCOLOR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderBorderColor-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RenderBorderColor-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RenderChannelFormat {
    #[default]
    RenderChannelFormat_R4G4 = 0,
    RenderChannelFormat_R4G4B4A4 = 1,
    RenderChannelFormat_R5G6B5 = 2,
    RenderChannelFormat_B5G6R5 = 3,
    RenderChannelFormat_R5G5B5A1 = 4,
    RenderChannelFormat_R8 = 5,
    RenderChannelFormat_R8G8 = 6,
    RenderChannelFormat_R8G8B8 = 7,
    RenderChannelFormat_R8G8B8A8 = 8,
    RenderChannelFormat_B8G8R8A8 = 9,
    RenderChannelFormat_R10G11B11 = 10,
    RenderChannelFormat_R11G11B10 = 11,
    RenderChannelFormat_R10G10B10A2 = 12,
    RenderChannelFormat_R9G9B9E5 = 13,
    RenderChannelFormat_R16 = 14,
    RenderChannelFormat_R16G16 = 15,
    RenderChannelFormat_R16G16B16A16 = 16,
    RenderChannelFormat_R32 = 17,
    RenderChannelFormat_R32G32 = 18,
    RenderChannelFormat_R32G32B32A32 = 19,
    RenderChannelFormat_BC1 = 20,
    RenderChannelFormat_BC1A = 21,
    RenderChannelFormat_BC2 = 22,
    RenderChannelFormat_BC3 = 23,
    RenderChannelFormat_BC4 = 24,
    RenderChannelFormat_BC5 = 25,
    RenderChannelFormat_BC6U = 26,
    RenderChannelFormat_BC6S = 27,
    RenderChannelFormat_BC7 = 28,
    RenderChannelFormat_ETC1 = 29,
    RenderChannelFormat_ETC2RGB = 30,
    RenderChannelFormat_ETC2RGBA = 31,
    RenderChannelFormat_ETC2RGBA1 = 32,
    RenderChannelFormat_EAC_R11 = 33,
    RenderChannelFormat_EAC_RG11 = 34,
    RenderChannelFormat_PVRTC1_4BPP_RGBA = 35,
    RenderChannelFormat_PVRTC1_4BPP_RGB = 36,
    RenderChannelFormat_PVRTC1_2BPP_RGBA = 37,
    RenderChannelFormat_PVRTC1_2BPP_RGB = 38,
    RenderChannelFormat_PVRTC2_4BPP = 39,
    RenderChannelFormat_PVRTC2_2BPP = 40,
    RenderChannelFormat_ASTC_4x4 = 41,
    RenderChannelFormat_ASTC_5x4 = 42,
    RenderChannelFormat_ASTC_5x5 = 43,
    RenderChannelFormat_ASTC_6x5 = 44,
    RenderChannelFormat_ASTC_6x6 = 45,
    RenderChannelFormat_ASTC_8x5 = 46,
    RenderChannelFormat_ASTC_8x6 = 47,
    RenderChannelFormat_ASTC_8x8 = 48,
    RenderChannelFormat_ASTC_10x5 = 49,
    RenderChannelFormat_ASTC_10x6 = 50,
    RenderChannelFormat_ASTC_10x8 = 51,
    RenderChannelFormat_ASTC_10x10 = 52,
    RenderChannelFormat_ASTC_12x10 = 53,
    RenderChannelFormat_ASTC_12x12 = 54,
    RenderChannelFormat_D24S8 = 55,
    RenderChannelFormat_D32S8 = 56,
    RenderChannelFormat_D16 = 57,
    RenderChannelFormat_D24 = 58,
    RenderChannelFormat_D32 = 59,
    RenderChannelFormatCount = 60,
}

pub const RENDERCHANNELFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderChannelFormat",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RENDERCHANNELFORMAT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderChannelFormat {
    fn type_info() -> &'static TypeInfo {
        RENDERCHANNELFORMAT_TYPE_INFO
    }
}


pub const RENDERCHANNELFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderChannelFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RenderChannelFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RenderNumericFormat {
    #[default]
    RenderNumericFormat_FLOAT = 0,
    RenderNumericFormat_SRGB = 1,
    RenderNumericFormat_UNORM = 2,
    RenderNumericFormat_SNORM = 3,
    RenderNumericFormat_UINT = 4,
    RenderNumericFormat_SINT = 5,
}

pub const RENDERNUMERICFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderNumericFormat",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RENDERNUMERICFORMAT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderNumericFormat {
    fn type_info() -> &'static TypeInfo {
        RENDERNUMERICFORMAT_TYPE_INFO
    }
}


pub const RENDERNUMERICFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderNumericFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RenderNumericFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum RenderFormat {
    #[default]
    RenderFormat_Unknown = 0,
    RenderFormat_R4G4_UNORM = 1,
    RenderFormat_R4G4B4A4_UNORM = 2,
    RenderFormat_R5G6B5_UNORM = 3,
    RenderFormat_B5G6R5_UNORM = 4,
    RenderFormat_R5G5B5A1_UNORM = 5,
    RenderFormat_R8_UNORM = 6,
    RenderFormat_R8_SNORM = 7,
    RenderFormat_R8_SRGB = 8,
    RenderFormat_R8_UINT = 9,
    RenderFormat_R8_SINT = 10,
    RenderFormat_R8G8_UNORM = 11,
    RenderFormat_R8G8_SNORM = 12,
    RenderFormat_R8G8_SRGB = 13,
    RenderFormat_R8G8_UINT = 14,
    RenderFormat_R8G8_SINT = 15,
    RenderFormat_R8G8B8_UNORM = 16,
    RenderFormat_R8G8B8_SRGB = 17,
    RenderFormat_R8G8B8A8_UNORM = 18,
    RenderFormat_R8G8B8A8_SNORM = 19,
    RenderFormat_R8G8B8A8_SRGB = 20,
    RenderFormat_R8G8B8A8_UINT = 21,
    RenderFormat_R8G8B8A8_SINT = 22,
    RenderFormat_B8G8R8A8_UNORM = 23,
    RenderFormat_B8G8R8A8_SRGB = 24,
    RenderFormat_R10G11B11_FLOAT = 25,
    RenderFormat_R11G11B10_FLOAT = 26,
    RenderFormat_R10G10B10A2_UNORM = 27,
    RenderFormat_R10G10B10A2_UINT = 28,
    RenderFormat_R9G9B9E5_FLOAT = 29,
    RenderFormat_R16_FLOAT = 30,
    RenderFormat_R16_UNORM = 31,
    RenderFormat_R16_SNORM = 32,
    RenderFormat_R16_UINT = 33,
    RenderFormat_R16_SINT = 34,
    RenderFormat_R16G16_FLOAT = 35,
    RenderFormat_R16G16_UNORM = 36,
    RenderFormat_R16G16_SNORM = 37,
    RenderFormat_R16G16_UINT = 38,
    RenderFormat_R16G16_SINT = 39,
    RenderFormat_R16G16B16A16_FLOAT = 40,
    RenderFormat_R16G16B16A16_UNORM = 41,
    RenderFormat_R16G16B16A16_SNORM = 42,
    RenderFormat_R16G16B16A16_UINT = 43,
    RenderFormat_R16G16B16A16_SINT = 44,
    RenderFormat_R32_FLOAT = 45,
    RenderFormat_R32_UINT = 46,
    RenderFormat_R32_SINT = 47,
    RenderFormat_R32G32_FLOAT = 48,
    RenderFormat_R32G32_UINT = 49,
    RenderFormat_R32G32_SINT = 50,
    RenderFormat_R32G32B32A32_FLOAT = 51,
    RenderFormat_R32G32B32A32_UINT = 52,
    RenderFormat_R32G32B32A32_SINT = 53,
    RenderFormat_BC1_UNORM = 54,
    RenderFormat_BC1_SRGB = 55,
    RenderFormat_BC1A_UNORM = 56,
    RenderFormat_BC1A_SRGB = 57,
    RenderFormat_BC2_UNORM = 58,
    RenderFormat_BC2_SRGB = 59,
    RenderFormat_BC3_UNORM = 60,
    RenderFormat_BC3_SRGB = 61,
    RenderFormat_BC4_UNORM = 62,
    RenderFormat_BC5_UNORM = 63,
    RenderFormat_BC6U_FLOAT = 64,
    RenderFormat_BC6S_FLOAT = 65,
    RenderFormat_BC7_UNORM = 66,
    RenderFormat_BC7_SRGB = 67,
    RenderFormat_ETC1_UNORM = 68,
    RenderFormat_ETC1_SRGB = 69,
    RenderFormat_ETC2RGB_UNORM = 70,
    RenderFormat_ETC2RGB_SRGB = 71,
    RenderFormat_ETC2RGBA_UNORM = 72,
    RenderFormat_ETC2RGBA_SRGB = 73,
    RenderFormat_ETC2RGBA1_UNORM = 74,
    RenderFormat_ETC2RGBA1_SRGB = 75,
    RenderFormat_EAC_R11_UNORM = 76,
    RenderFormat_EAC_R11_SNORM = 77,
    RenderFormat_EAC_RG11_UNORM = 78,
    RenderFormat_EAC_RG11_SNORM = 79,
    RenderFormat_PVRTC1_4BPP_RGBA_UNORM = 80,
    RenderFormat_PVRTC1_4BPP_RGBA_SRGB = 81,
    RenderFormat_PVRTC1_4BPP_RGB_UNORM = 82,
    RenderFormat_PVRTC1_4BPP_RGB_SRGB = 83,
    RenderFormat_PVRTC1_2BPP_RGBA_UNORM = 84,
    RenderFormat_PVRTC1_2BPP_RGBA_SRGB = 85,
    RenderFormat_PVRTC1_2BPP_RGB_UNORM = 86,
    RenderFormat_PVRTC1_2BPP_RGB_SRGB = 87,
    RenderFormat_PVRTC2_4BPP_UNORM = 88,
    RenderFormat_PVRTC2_4BPP_SRGB = 89,
    RenderFormat_PVRTC2_2BPP_UNORM = 90,
    RenderFormat_PVRTC2_2BPP_SRGB = 91,
    RenderFormat_ASTC_4x4_UNORM = 92,
    RenderFormat_ASTC_4x4_SRGB = 93,
    RenderFormat_ASTC_5x4_UNORM = 94,
    RenderFormat_ASTC_5x4_SRGB = 95,
    RenderFormat_ASTC_5x5_UNORM = 96,
    RenderFormat_ASTC_5x5_SRGB = 97,
    RenderFormat_ASTC_6x5_UNORM = 98,
    RenderFormat_ASTC_6x5_SRGB = 99,
    RenderFormat_ASTC_6x6_UNORM = 100,
    RenderFormat_ASTC_6x6_SRGB = 101,
    RenderFormat_ASTC_8x5_UNORM = 102,
    RenderFormat_ASTC_8x5_SRGB = 103,
    RenderFormat_ASTC_8x6_UNORM = 104,
    RenderFormat_ASTC_8x6_SRGB = 105,
    RenderFormat_ASTC_8x8_UNORM = 106,
    RenderFormat_ASTC_8x8_SRGB = 107,
    RenderFormat_ASTC_10x5_UNORM = 108,
    RenderFormat_ASTC_10x5_SRGB = 109,
    RenderFormat_ASTC_10x6_UNORM = 110,
    RenderFormat_ASTC_10x6_SRGB = 111,
    RenderFormat_ASTC_10x8_UNORM = 112,
    RenderFormat_ASTC_10x8_SRGB = 113,
    RenderFormat_ASTC_10x10_UNORM = 114,
    RenderFormat_ASTC_10x10_SRGB = 115,
    RenderFormat_ASTC_12x10_UNORM = 116,
    RenderFormat_ASTC_12x10_SRGB = 117,
    RenderFormat_ASTC_12x12_UNORM = 118,
    RenderFormat_ASTC_12x12_SRGB = 119,
    RenderFormat_D24_UNORM_S8_UINT = 120,
    RenderFormat_D24_FLOAT_S8_UINT = 121,
    RenderFormat_D32_FLOAT_S8_UINT = 122,
    RenderFormat_D16_UNORM = 123,
    RenderFormat_D24_UNORM = 124,
    RenderFormat_D32_FLOAT = 125,
    RenderFormatCount = 126,
}

pub const RENDERFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderFormat",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(RENDERFORMAT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for RenderFormat {
    fn type_info() -> &'static TypeInfo {
        RENDERFORMAT_TYPE_INFO
    }
}


pub const RENDERFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("RenderFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BaseDisplaySettings {
    pub gpu_profiler_enable: bool,
    pub null_driver_enable: bool,
    pub create_minimal_window: bool,
    pub fullscreen_mode_enable: bool,
    pub fullscreen: bool,
    pub fullscreen_height: u32,
    pub fullscreen_width: u32,
    pub fullscreen_refresh_rate: f32,
    pub preferred_adapter_index: u32,
    pub fullscreen_output_index: i32,
    pub present_interval: i32,
    pub present_enable: bool,
    pub present_immediate_threshold: u32,
    pub window_borders_enable: bool,
    pub v_sync_enable: bool,
    pub triple_buffering_enable: bool,
    pub render_ahead_limit: i32,
    pub gpu_timeout_time: f32,
    pub gpu_timer_count: u32,
    pub automatic_compute_sync_enable: bool,
    pub frame_resource_segment_size: u32,
    pub frame_resource_non_segment_size: u32,
    pub frame_resource_free_enable: bool,
    pub frame_resource_free_frame_count: u32,
    pub frame_resource_free_factor: f32,
    pub draw_frame_memory_stats: bool,
    pub draw_frame_memory_allocations: bool,
    pub framebuffer10_bit_enable: bool,
    pub display_dynamic_range: DisplayDynamicRange,
    pub cpu_heap_stomp_enable: bool,
    pub gpu_heap_stomp_enable: bool,
}

pub const BASEDISPLAYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseDisplaySettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "GpuProfilerEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, gpu_profiler_enable),
            },
            FieldInfoData {
                name: "NullDriverEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, null_driver_enable),
            },
            FieldInfoData {
                name: "CreateMinimalWindow",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, create_minimal_window),
            },
            FieldInfoData {
                name: "FullscreenModeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, fullscreen_mode_enable),
            },
            FieldInfoData {
                name: "Fullscreen",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, fullscreen),
            },
            FieldInfoData {
                name: "FullscreenHeight",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, fullscreen_height),
            },
            FieldInfoData {
                name: "FullscreenWidth",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, fullscreen_width),
            },
            FieldInfoData {
                name: "FullscreenRefreshRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, fullscreen_refresh_rate),
            },
            FieldInfoData {
                name: "PreferredAdapterIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, preferred_adapter_index),
            },
            FieldInfoData {
                name: "FullscreenOutputIndex",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, fullscreen_output_index),
            },
            FieldInfoData {
                name: "PresentInterval",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, present_interval),
            },
            FieldInfoData {
                name: "PresentEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, present_enable),
            },
            FieldInfoData {
                name: "PresentImmediateThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, present_immediate_threshold),
            },
            FieldInfoData {
                name: "WindowBordersEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, window_borders_enable),
            },
            FieldInfoData {
                name: "VSyncEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, v_sync_enable),
            },
            FieldInfoData {
                name: "TripleBufferingEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, triple_buffering_enable),
            },
            FieldInfoData {
                name: "RenderAheadLimit",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, render_ahead_limit),
            },
            FieldInfoData {
                name: "GpuTimeoutTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, gpu_timeout_time),
            },
            FieldInfoData {
                name: "GpuTimerCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, gpu_timer_count),
            },
            FieldInfoData {
                name: "AutomaticComputeSyncEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, automatic_compute_sync_enable),
            },
            FieldInfoData {
                name: "FrameResourceSegmentSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, frame_resource_segment_size),
            },
            FieldInfoData {
                name: "FrameResourceNonSegmentSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, frame_resource_non_segment_size),
            },
            FieldInfoData {
                name: "FrameResourceFreeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, frame_resource_free_enable),
            },
            FieldInfoData {
                name: "FrameResourceFreeFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, frame_resource_free_frame_count),
            },
            FieldInfoData {
                name: "FrameResourceFreeFactor",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, frame_resource_free_factor),
            },
            FieldInfoData {
                name: "DrawFrameMemoryStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, draw_frame_memory_stats),
            },
            FieldInfoData {
                name: "DrawFrameMemoryAllocations",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, draw_frame_memory_allocations),
            },
            FieldInfoData {
                name: "Framebuffer10BitEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, framebuffer10_bit_enable),
            },
            FieldInfoData {
                name: "DisplayDynamicRange",
                flags: MemberInfoFlags::new(0),
                field_type: DISPLAYDYNAMICRANGE_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, display_dynamic_range),
            },
            FieldInfoData {
                name: "CpuHeapStompEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, cpu_heap_stomp_enable),
            },
            FieldInfoData {
                name: "GpuHeapStompEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BaseDisplaySettings, gpu_heap_stomp_enable),
            },
        ],
    }),
    array_type: Some(BASEDISPLAYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BaseDisplaySettings {
    fn type_info() -> &'static TypeInfo {
        BASEDISPLAYSETTINGS_TYPE_INFO
    }
}


pub const BASEDISPLAYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseDisplaySettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("BaseDisplaySettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Ps4DisplaySettings {
    pub submit_job_enable: bool,
    pub compute_queue_enable: bool,
    pub supports_command_buffers: bool,
    pub async_resource_manager_enable: bool,
    pub memory_pools_enable: bool,
    pub c_mask_enable: bool,
    pub c_mask_fast_clear: bool,
    pub dcc_enable: bool,
    pub dcc_decompress: bool,
    pub h_tile_enable: bool,
    pub h_tile_stencil_enable: bool,
    pub h_tile_fast_clear: bool,
    pub h_tile_break: bool,
    pub re_z_enable: bool,
    pub zero_viewport_enable: bool,
    pub state_cache: bool,
    pub screen_width: u32,
    pub screen_height: u32,
    pub command_buffer_debug_mode: u32,
    pub heavy_synchronization: bool,
    pub max_linear_allocated_transient_buffer_size: u32,
    pub mips_stats_enable: bool,
    pub mips_stats_start: bool,
    pub mips_stats_stop: bool,
    pub mips_stats_clear_stats: bool,
    pub mips_stats_report_dump: bool,
    pub mips_stats_freq: u32,
    pub mips_stats_texture_size: u32,
    pub mips_stats_single_capture: bool,
}

pub const PS4DISPLAYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4DisplaySettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASEDISPLAYSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SubmitJobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, submit_job_enable),
            },
            FieldInfoData {
                name: "ComputeQueueEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, compute_queue_enable),
            },
            FieldInfoData {
                name: "SupportsCommandBuffers",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, supports_command_buffers),
            },
            FieldInfoData {
                name: "AsyncResourceManagerEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, async_resource_manager_enable),
            },
            FieldInfoData {
                name: "MemoryPoolsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, memory_pools_enable),
            },
            FieldInfoData {
                name: "CMaskEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, c_mask_enable),
            },
            FieldInfoData {
                name: "CMaskFastClear",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, c_mask_fast_clear),
            },
            FieldInfoData {
                name: "DccEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, dcc_enable),
            },
            FieldInfoData {
                name: "DccDecompress",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, dcc_decompress),
            },
            FieldInfoData {
                name: "HTileEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, h_tile_enable),
            },
            FieldInfoData {
                name: "HTileStencilEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, h_tile_stencil_enable),
            },
            FieldInfoData {
                name: "HTileFastClear",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, h_tile_fast_clear),
            },
            FieldInfoData {
                name: "HTileBreak",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, h_tile_break),
            },
            FieldInfoData {
                name: "ReZEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, re_z_enable),
            },
            FieldInfoData {
                name: "ZeroViewportEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, zero_viewport_enable),
            },
            FieldInfoData {
                name: "StateCache",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, state_cache),
            },
            FieldInfoData {
                name: "ScreenWidth",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, screen_width),
            },
            FieldInfoData {
                name: "ScreenHeight",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, screen_height),
            },
            FieldInfoData {
                name: "CommandBufferDebugMode",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, command_buffer_debug_mode),
            },
            FieldInfoData {
                name: "HeavySynchronization",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, heavy_synchronization),
            },
            FieldInfoData {
                name: "MaxLinearAllocatedTransientBufferSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, max_linear_allocated_transient_buffer_size),
            },
            FieldInfoData {
                name: "MipsStatsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, mips_stats_enable),
            },
            FieldInfoData {
                name: "MipsStatsStart",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, mips_stats_start),
            },
            FieldInfoData {
                name: "MipsStatsStop",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, mips_stats_stop),
            },
            FieldInfoData {
                name: "MipsStatsClearStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, mips_stats_clear_stats),
            },
            FieldInfoData {
                name: "MipsStatsReportDump",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, mips_stats_report_dump),
            },
            FieldInfoData {
                name: "MipsStatsFreq",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, mips_stats_freq),
            },
            FieldInfoData {
                name: "MipsStatsTextureSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, mips_stats_texture_size),
            },
            FieldInfoData {
                name: "MipsStatsSingleCapture",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4DisplaySettings, mips_stats_single_capture),
            },
        ],
    }),
    array_type: Some(PS4DISPLAYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Ps4DisplaySettings {
    fn type_info() -> &'static TypeInfo {
        PS4DISPLAYSETTINGS_TYPE_INFO
    }
}


pub const PS4DISPLAYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4DisplaySettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Ps4DisplaySettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Dx12DisplaySettings {
    pub debug_report_leak_summary_enable: bool,
    pub debug_report_leak_details_enable: bool,
    pub check_device_removed_enable: bool,
    pub memory_pools_enable: bool,
    pub stable_power_state_enable: StablePowerState,
    pub draw_stats: bool,
    pub draw_transient_texture_pool_stats: bool,
    pub draw_placed_resource_manager_stats: bool,
    pub descriptor_table_frame_reuse_enable: bool,
    pub compute_queue_enable: i32,
    pub copy_queue_enable: i32,
    pub submit_job_enable: bool,
    pub latency_limit_ms: f32,
    pub recovery_time_max_ms: f32,
    pub recovery_time_ramp_ms: f32,
    pub pix_markers_enable: bool,
    pub pipeline_caching_enable: bool,
    pub optimized_compute_sync_enable: bool,
    pub max_multisample_count: u32,
    pub scorpio4k_enable: bool,
    pub memory_manager_enable: bool,
    pub memory_manager_verbose: bool,
    pub memory_manager_age_to_evict_in_frames: u32,
    pub descriptor_allocator_size: u32,
    pub dx12_transient_texture_pool_enable: bool,
    pub dx12_frame_resource_manager_enable: bool,
    pub xb1_transient_esram_enable: bool,
    pub xb1_transient_dram_pool_initial_size_mb: u32,
    pub xb1_transient_dram_pool_maximum_size_mb: u32,
    pub xb1_transient_dram_pool_dynamic_trim_enable: bool,
    pub xb1_dma_engine_pipe_index: u32,
    pub v_sync_between_frames_enable: bool,
    pub clear_unused_descriptors_to_null: bool,
    pub trigger_gpu_hang_frame: i32,
    pub prevent_reboot_on_gpu_hang_enable: bool,
    pub gpu_crash_analysis_enabled: bool,
    pub gpu_crash_analysis_vs_enabled: i32,
    pub gpu_crash_analysis_debug_verbosity: u32,
    pub gpu_crash_analysis_buffer_size_bytes: u32,
    pub gpu_crash_analysis_page_size_bytes: u32,
    pub gpu_crash_analysis_prior_report_count: u32,
    pub gpu_crash_analysis_post_report_count: u32,
    pub cpu_cbv_srv_uav_descriptor_heap_size: u32,
    pub cpu_sampler_descriptor_heap_size: u32,
    pub gpu_sampler_descriptor_heap_size: u32,
    pub cpu_rtv_descriptor_heap_size: u32,
    pub cpu_dsv_descriptor_heap_size: u32,
    pub cbv_srv_uav_temporal_descriptor_heap_size: u32,
    pub cbv_srv_uav_persistent_descriptor_heap_size: u32,
    pub preload_pso_initial_load: bool,
    pub preload_pso_enable: bool,
    pub preload_pso_only_used: bool,
}

pub const DX12DISPLAYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12DisplaySettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DXDISPLAYSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DebugReportLeakSummaryEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, debug_report_leak_summary_enable),
            },
            FieldInfoData {
                name: "DebugReportLeakDetailsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, debug_report_leak_details_enable),
            },
            FieldInfoData {
                name: "CheckDeviceRemovedEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, check_device_removed_enable),
            },
            FieldInfoData {
                name: "MemoryPoolsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, memory_pools_enable),
            },
            FieldInfoData {
                name: "StablePowerStateEnable",
                flags: MemberInfoFlags::new(0),
                field_type: STABLEPOWERSTATE_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, stable_power_state_enable),
            },
            FieldInfoData {
                name: "DrawStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, draw_stats),
            },
            FieldInfoData {
                name: "DrawTransientTexturePoolStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, draw_transient_texture_pool_stats),
            },
            FieldInfoData {
                name: "DrawPlacedResourceManagerStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, draw_placed_resource_manager_stats),
            },
            FieldInfoData {
                name: "DescriptorTableFrameReuseEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, descriptor_table_frame_reuse_enable),
            },
            FieldInfoData {
                name: "ComputeQueueEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, compute_queue_enable),
            },
            FieldInfoData {
                name: "CopyQueueEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, copy_queue_enable),
            },
            FieldInfoData {
                name: "SubmitJobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, submit_job_enable),
            },
            FieldInfoData {
                name: "LatencyLimitMs",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, latency_limit_ms),
            },
            FieldInfoData {
                name: "RecoveryTimeMaxMs",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, recovery_time_max_ms),
            },
            FieldInfoData {
                name: "RecoveryTimeRampMs",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, recovery_time_ramp_ms),
            },
            FieldInfoData {
                name: "PixMarkersEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, pix_markers_enable),
            },
            FieldInfoData {
                name: "PipelineCachingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, pipeline_caching_enable),
            },
            FieldInfoData {
                name: "OptimizedComputeSyncEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, optimized_compute_sync_enable),
            },
            FieldInfoData {
                name: "MaxMultisampleCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, max_multisample_count),
            },
            FieldInfoData {
                name: "Scorpio4kEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, scorpio4k_enable),
            },
            FieldInfoData {
                name: "MemoryManagerEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, memory_manager_enable),
            },
            FieldInfoData {
                name: "MemoryManagerVerbose",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, memory_manager_verbose),
            },
            FieldInfoData {
                name: "MemoryManagerAgeToEvictInFrames",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, memory_manager_age_to_evict_in_frames),
            },
            FieldInfoData {
                name: "DescriptorAllocatorSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, descriptor_allocator_size),
            },
            FieldInfoData {
                name: "Dx12TransientTexturePoolEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, dx12_transient_texture_pool_enable),
            },
            FieldInfoData {
                name: "Dx12FrameResourceManagerEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, dx12_frame_resource_manager_enable),
            },
            FieldInfoData {
                name: "Xb1TransientEsramEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, xb1_transient_esram_enable),
            },
            FieldInfoData {
                name: "Xb1TransientDramPoolInitialSizeMb",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, xb1_transient_dram_pool_initial_size_mb),
            },
            FieldInfoData {
                name: "Xb1TransientDramPoolMaximumSizeMb",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, xb1_transient_dram_pool_maximum_size_mb),
            },
            FieldInfoData {
                name: "Xb1TransientDramPoolDynamicTrimEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, xb1_transient_dram_pool_dynamic_trim_enable),
            },
            FieldInfoData {
                name: "Xb1DmaEnginePipeIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, xb1_dma_engine_pipe_index),
            },
            FieldInfoData {
                name: "VSyncBetweenFramesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, v_sync_between_frames_enable),
            },
            FieldInfoData {
                name: "ClearUnusedDescriptorsToNull",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, clear_unused_descriptors_to_null),
            },
            FieldInfoData {
                name: "TriggerGpuHangFrame",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, trigger_gpu_hang_frame),
            },
            FieldInfoData {
                name: "PreventRebootOnGpuHangEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, prevent_reboot_on_gpu_hang_enable),
            },
            FieldInfoData {
                name: "GpuCrashAnalysisEnabled",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, gpu_crash_analysis_enabled),
            },
            FieldInfoData {
                name: "GpuCrashAnalysisVsEnabled",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, gpu_crash_analysis_vs_enabled),
            },
            FieldInfoData {
                name: "GpuCrashAnalysisDebugVerbosity",
                flags: MemberInfoFlags::new(8192),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, gpu_crash_analysis_debug_verbosity),
            },
            FieldInfoData {
                name: "GpuCrashAnalysisBufferSizeBytes",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, gpu_crash_analysis_buffer_size_bytes),
            },
            FieldInfoData {
                name: "GpuCrashAnalysisPageSizeBytes",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, gpu_crash_analysis_page_size_bytes),
            },
            FieldInfoData {
                name: "GpuCrashAnalysisPriorReportCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, gpu_crash_analysis_prior_report_count),
            },
            FieldInfoData {
                name: "GpuCrashAnalysisPostReportCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, gpu_crash_analysis_post_report_count),
            },
            FieldInfoData {
                name: "CpuCbvSrvUavDescriptorHeapSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, cpu_cbv_srv_uav_descriptor_heap_size),
            },
            FieldInfoData {
                name: "CpuSamplerDescriptorHeapSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, cpu_sampler_descriptor_heap_size),
            },
            FieldInfoData {
                name: "GpuSamplerDescriptorHeapSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, gpu_sampler_descriptor_heap_size),
            },
            FieldInfoData {
                name: "CpuRtvDescriptorHeapSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, cpu_rtv_descriptor_heap_size),
            },
            FieldInfoData {
                name: "CpuDsvDescriptorHeapSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, cpu_dsv_descriptor_heap_size),
            },
            FieldInfoData {
                name: "CbvSrvUavTemporalDescriptorHeapSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, cbv_srv_uav_temporal_descriptor_heap_size),
            },
            FieldInfoData {
                name: "CbvSrvUavPersistentDescriptorHeapSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, cbv_srv_uav_persistent_descriptor_heap_size),
            },
            FieldInfoData {
                name: "PreloadPsoInitialLoad",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, preload_pso_initial_load),
            },
            FieldInfoData {
                name: "PreloadPsoEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, preload_pso_enable),
            },
            FieldInfoData {
                name: "PreloadPsoOnlyUsed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx12DisplaySettings, preload_pso_only_used),
            },
        ],
    }),
    array_type: Some(DX12DISPLAYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx12DisplaySettings {
    fn type_info() -> &'static TypeInfo {
        DX12DISPLAYSETTINGS_TYPE_INFO
    }
}


pub const DX12DISPLAYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx12DisplaySettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Dx12DisplaySettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum StablePowerState {
    #[default]
    StablePowerStateOff = 0,
    StablePowerStateOn = 1,
    StablePowerStateAuto = 2,
}

pub const STABLEPOWERSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StablePowerState",
    flags: MemberInfoFlags::new(49429),
    module: "Render",
    data: TypeInfoData::Enum,
    array_type: Some(STABLEPOWERSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StablePowerState {
    fn type_info() -> &'static TypeInfo {
        STABLEPOWERSTATE_TYPE_INFO
    }
}


pub const STABLEPOWERSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StablePowerState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("StablePowerState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Dx11DisplaySettings {
    pub ref_driver_enable: bool,
    pub driver_internal_threading_enable: bool,
    pub amd_driver_optimizations_enable: bool,
    pub get_device_removed_reason_enable: bool,
    pub memory_pools_enable: bool,
    pub nv_perf_hud_enable: bool,
    pub annotation_interface_enable: bool,
    pub amd_generate_mips_workaround_enable: bool,
    pub dx11_dot1_enable: bool,
    pub dx11_dot1_runtime_enable: bool,
    pub async_cmd_list_execution_enable: bool,
    pub end_frame_job_enable: bool,
    pub depth_stencil_extension_enable: bool,
    pub force_render_target_in_esram_test: bool,
    pub re_z_enable: bool,
    pub compute_shader_cache_enable: bool,
}

pub const DX11DISPLAYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11DisplaySettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DXDISPLAYSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RefDriverEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx11DisplaySettings, ref_driver_enable),
            },
            FieldInfoData {
                name: "DriverInternalThreadingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx11DisplaySettings, driver_internal_threading_enable),
            },
            FieldInfoData {
                name: "AmdDriverOptimizationsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx11DisplaySettings, amd_driver_optimizations_enable),
            },
            FieldInfoData {
                name: "GetDeviceRemovedReasonEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx11DisplaySettings, get_device_removed_reason_enable),
            },
            FieldInfoData {
                name: "MemoryPoolsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx11DisplaySettings, memory_pools_enable),
            },
            FieldInfoData {
                name: "NvPerfHudEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx11DisplaySettings, nv_perf_hud_enable),
            },
            FieldInfoData {
                name: "AnnotationInterfaceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx11DisplaySettings, annotation_interface_enable),
            },
            FieldInfoData {
                name: "AmdGenerateMipsWorkaroundEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx11DisplaySettings, amd_generate_mips_workaround_enable),
            },
            FieldInfoData {
                name: "Dx11Dot1Enable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx11DisplaySettings, dx11_dot1_enable),
            },
            FieldInfoData {
                name: "Dx11Dot1RuntimeEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx11DisplaySettings, dx11_dot1_runtime_enable),
            },
            FieldInfoData {
                name: "AsyncCmdListExecutionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx11DisplaySettings, async_cmd_list_execution_enable),
            },
            FieldInfoData {
                name: "EndFrameJobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx11DisplaySettings, end_frame_job_enable),
            },
            FieldInfoData {
                name: "DepthStencilExtensionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx11DisplaySettings, depth_stencil_extension_enable),
            },
            FieldInfoData {
                name: "ForceRenderTargetInEsramTest",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx11DisplaySettings, force_render_target_in_esram_test),
            },
            FieldInfoData {
                name: "ReZEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx11DisplaySettings, re_z_enable),
            },
            FieldInfoData {
                name: "ComputeShaderCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Dx11DisplaySettings, compute_shader_cache_enable),
            },
        ],
    }),
    array_type: Some(DX11DISPLAYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Dx11DisplaySettings {
    fn type_info() -> &'static TypeInfo {
        DX11DISPLAYSETTINGS_TYPE_INFO
    }
}


pub const DX11DISPLAYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Dx11DisplaySettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("Dx11DisplaySettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DxDisplaySettings {
    pub warp_driver_enable: bool,
    pub debug_info_enable: bool,
    pub debug_info_gp_based_validation_enable: u32,
    pub debug_info_output_enable: bool,
    pub debug_info_xb1_barrier_validation_enable: bool,
    pub debug_info_xb1_transition_barrier_log_enable: bool,
    pub debug_break_on_error_enable: bool,
    pub debug_break_on_warning_enable: bool,
    pub debug_break_on_info_enable: bool,
    pub debug_info_mute_severity: u32,
    pub debug_break_ignored_i_ds: Vec<i32>,
    pub multi_gpu_validation_enable: bool,
    pub dx_diag_driver_detection_enable: bool,
    pub nv_api_enable: bool,
    pub nv_aftermath_enable: bool,
    pub nv_hlsl_intrinsics_enable: bool,
    pub amd_ags_enable: bool,
    pub amd_quad_primitive_enable: bool,
    pub amd_rect_primitive_enable: bool,
    pub uav_overlap_extension_enable: bool,
    pub depth_bounds_extension_enable: bool,
    pub min_driver_required: bool,
    pub nvidia_min_driver_version: u32,
    pub nvidia_min_dx12_driver_version: u32,
    pub amd_min_driver_version: String,
    pub amd_min_internal_driver_version: String,
    pub amd_min_windows8_driver_version: String,
    pub amd_min_windows8_internal_driver_version: String,
    pub intel_min_driver_version: String,
    pub capture_enable: bool,
    pub capture_output_path: String,
    pub pix_profiling_enable: bool,
    pub present_join_jobs_enable: bool,
    pub present_join_job_stall_threshold_ms: f32,
    pub present_join_job_stall_safety_ms: f32,
    pub draw_memory_stats: bool,
    pub draw_memory_graph: bool,
    pub memory_stats_start_x: u32,
    pub memory_stats_start_y: u32,
    pub memory_stats_start_step: u32,
}

pub const DXDISPLAYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DxDisplaySettings",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASEDISPLAYSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "WarpDriverEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, warp_driver_enable),
            },
            FieldInfoData {
                name: "DebugInfoEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, debug_info_enable),
            },
            FieldInfoData {
                name: "DebugInfoGpBasedValidationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, debug_info_gp_based_validation_enable),
            },
            FieldInfoData {
                name: "DebugInfoOutputEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, debug_info_output_enable),
            },
            FieldInfoData {
                name: "DebugInfoXb1BarrierValidationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, debug_info_xb1_barrier_validation_enable),
            },
            FieldInfoData {
                name: "DebugInfoXb1TransitionBarrierLogEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, debug_info_xb1_transition_barrier_log_enable),
            },
            FieldInfoData {
                name: "DebugBreakOnErrorEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, debug_break_on_error_enable),
            },
            FieldInfoData {
                name: "DebugBreakOnWarningEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, debug_break_on_warning_enable),
            },
            FieldInfoData {
                name: "DebugBreakOnInfoEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, debug_break_on_info_enable),
            },
            FieldInfoData {
                name: "DebugInfoMuteSeverity",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, debug_info_mute_severity),
            },
            FieldInfoData {
                name: "DebugBreakIgnoredIDs",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, debug_break_ignored_i_ds),
            },
            FieldInfoData {
                name: "MultiGpuValidationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, multi_gpu_validation_enable),
            },
            FieldInfoData {
                name: "DxDiagDriverDetectionEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, dx_diag_driver_detection_enable),
            },
            FieldInfoData {
                name: "NvApiEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, nv_api_enable),
            },
            FieldInfoData {
                name: "NvAftermathEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, nv_aftermath_enable),
            },
            FieldInfoData {
                name: "NvHlslIntrinsicsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, nv_hlsl_intrinsics_enable),
            },
            FieldInfoData {
                name: "AmdAgsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, amd_ags_enable),
            },
            FieldInfoData {
                name: "AmdQuadPrimitiveEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, amd_quad_primitive_enable),
            },
            FieldInfoData {
                name: "AmdRectPrimitiveEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, amd_rect_primitive_enable),
            },
            FieldInfoData {
                name: "UavOverlapExtensionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, uav_overlap_extension_enable),
            },
            FieldInfoData {
                name: "DepthBoundsExtensionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, depth_bounds_extension_enable),
            },
            FieldInfoData {
                name: "MinDriverRequired",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, min_driver_required),
            },
            FieldInfoData {
                name: "NvidiaMinDriverVersion",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, nvidia_min_driver_version),
            },
            FieldInfoData {
                name: "NvidiaMinDx12DriverVersion",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, nvidia_min_dx12_driver_version),
            },
            FieldInfoData {
                name: "AmdMinDriverVersion",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, amd_min_driver_version),
            },
            FieldInfoData {
                name: "AmdMinInternalDriverVersion",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, amd_min_internal_driver_version),
            },
            FieldInfoData {
                name: "AmdMinWindows8DriverVersion",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, amd_min_windows8_driver_version),
            },
            FieldInfoData {
                name: "AmdMinWindows8InternalDriverVersion",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, amd_min_windows8_internal_driver_version),
            },
            FieldInfoData {
                name: "IntelMinDriverVersion",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, intel_min_driver_version),
            },
            FieldInfoData {
                name: "CaptureEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, capture_enable),
            },
            FieldInfoData {
                name: "CaptureOutputPath",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, capture_output_path),
            },
            FieldInfoData {
                name: "PixProfilingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, pix_profiling_enable),
            },
            FieldInfoData {
                name: "PresentJoinJobsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, present_join_jobs_enable),
            },
            FieldInfoData {
                name: "PresentJoinJobStallThresholdMs",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, present_join_job_stall_threshold_ms),
            },
            FieldInfoData {
                name: "PresentJoinJobStallSafetyMs",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, present_join_job_stall_safety_ms),
            },
            FieldInfoData {
                name: "DrawMemoryStats",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, draw_memory_stats),
            },
            FieldInfoData {
                name: "DrawMemoryGraph",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, draw_memory_graph),
            },
            FieldInfoData {
                name: "MemoryStatsStartX",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, memory_stats_start_x),
            },
            FieldInfoData {
                name: "MemoryStatsStartY",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, memory_stats_start_y),
            },
            FieldInfoData {
                name: "MemoryStatsStartStep",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DxDisplaySettings, memory_stats_start_step),
            },
        ],
    }),
    array_type: Some(DXDISPLAYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DxDisplaySettings {
    fn type_info() -> &'static TypeInfo {
        DXDISPLAYSETTINGS_TYPE_INFO
    }
}


pub const DXDISPLAYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DxDisplaySettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("DxDisplaySettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ITexture {
}

pub const ITEXTURE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ITexture",
    flags: MemberInfoFlags::new(101),
    module: "Render",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IRENDERRESOURCE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ITEXTURE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ITexture {
    fn type_info() -> &'static TypeInfo {
        ITEXTURE_TYPE_INFO
    }
}


pub const ITEXTURE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ITexture-Array",
    flags: MemberInfoFlags::new(145),
    module: "Render",
    data: TypeInfoData::Array("ITexture-Array"),
    array_type: None,
    alignment: 8,
};


