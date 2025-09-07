// Compatibility layer for API differences between protobuf versions
// This file provides fallback implementations when protobuf generation fails

#[cfg(not(feature = "protobuf_compat"))]
use crate::message_proto::*;

#[cfg(feature = "protobuf_compat")]
pub mod fallback {
    use super::*;
    
    // Fallback CliprdrFile if not generated
    #[derive(Clone, Debug, Default)]
    pub struct CliprdrFile {
        pub name: String,
        pub size: u64,
    }
    
    // Fallback CliprdrFiles if not generated  
    #[derive(Clone, Debug, Default)]
    pub struct CliprdrFiles {
        pub files: Vec<CliprdrFile>,
    }
    
    // Add Files variant to cliprdr::Union if missing
    pub mod cliprdr {
        use super::*;
        
        #[derive(Clone, Debug)]
        pub enum Union {
            Ready(crate::message_proto::CliprdrMonitorReady),
            FormatList(crate::message_proto::CliprdrServerFormatList),
            FormatListResponse(crate::message_proto::CliprdrServerFormatListResponse),
            FormatDataRequest(crate::message_proto::CliprdrServerFormatDataRequest),
            FormatDataResponse(crate::message_proto::CliprdrServerFormatDataResponse),
            FileContentsRequest(crate::message_proto::CliprdrFileContentsRequest),
            FileContentsResponse(crate::message_proto::CliprdrFileContentsResponse),
            TryEmpty(crate::message_proto::CliprdrTryEmpty),
            Files(CliprdrFiles),
        }
    }
}

// Re-export fallback implementations if needed
#[cfg(feature = "protobuf_compat")]
pub use fallback::*;
