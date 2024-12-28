struct BundleEbxEntry {
    be u32 nameOffset;
    be u32 originalSize;
    
    char name[] @ 4 + parent.stringsOffset + nameOffset;
};

struct BundleManifest {
    be u32 dataOffset;
    be u32 magic;
    
    if (magic != 0x9D798ED5) {
        return;
    }
    
    be u32 totalCount;
    be u32 ebxCount;
    be u32 resCount;
    be u32 chunkCount;
    
    be u32 stringsOffset;
    
    be u32 metaOffset;
    be u32 metaSize;
    
    u8 hashes[totalCount * 20];
    u8 meta[metaSize] @ metaOffset;
    
    BundleEbxEntry ebxEntries[ebxCount];
};

BundleManifest a @ 0x00;
