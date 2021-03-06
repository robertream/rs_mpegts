
use mpegts::stream_type::*;

pub fn get_stream_type(stream_type: u8) -> StreamType {
  match stream_type {
    0x00 => StreamType::Reserved,
    0x01 => StreamType::Reserved,
    0x02 => StreamType::VideoStreamHeaderParametersForItuTRecH262IsoIec138182AndIsoIec111722,
    0x03 => StreamType::AudioStreamHeaderParametersForIsoIec138183AndIsoIec111723,
    0x04 => StreamType::HierarchyForStreamSelection,
    0x05 => StreamType::RegistrationOfPrivateFormats,
    0x06 => StreamType::DataStreamAlignmentForPacketizedVideoAndAudioSyncPoint,
    0x07 => StreamType::TargetBackgroundGridDefinesTotalDisplayAreaSize,
    0x08 => StreamType::VideoWindowDefinesPositionInDisplayArea,
    0x09 => StreamType::ConditionalAccessSystemAndEmmEcmPid,
    0x0A => StreamType::Iso639LanguageAndAudioType,
    0x0B => StreamType::SystemClockExternalReference,
    0x0C => StreamType::MultiplexBufferUtilizationBounds,
    0x0D => StreamType::CopyrightIdentificationSystemAndReference,
    0x0E => StreamType::MaximumBitRate,
    0x0F => StreamType::PrivateDataIndicator,
    0x10 => StreamType::SmoothingBuffer,
    0x11 => StreamType::StdVideoBufferLeakControl,
    0x12 => StreamType::IbpVideoIFrameIndicator,
    0x13 => StreamType::IsiIec138186DsmCcCarouselIdentifier,
    0x14 => StreamType::IsiIec138186DsmCcAssociationTag,
    0x15 => StreamType::IsiIec138186DsmCcDeferredAssociationTag,
    0x16 => StreamType::IsiIec138186DsmCcReserved,
    0x17 => StreamType::DsmCcNptReference,
    0x18 => StreamType::DsmCcNptEndpoint,
    0x19 => StreamType::DsmCcStreamMode,
    0x1A => StreamType::DsmCcStreamEvent,
    0x1B => StreamType::VideoStreamHeaderParametersForIsoIec144962,
    0x1C => StreamType::AudioStreamHeaderParametersForIsoIec144963,
    0x1D => StreamType::IodParametersForIsoIec144961,
    0x1E => StreamType::SlParametersForIsoIec144961,
    0x1F => StreamType::FmcParametersForIsoIec144961,
    0x20 => StreamType::ExternalEsIdentifierForIsoIec144961,
    0x21 => StreamType::MuxCodeForIsoIec144961,
    0x22 => StreamType::FmxBufferSizeForIsoIec144961,
    0x23 => StreamType::MultiplexBufferForIsoIec144961,
    0x24 => StreamType::ContentLabelingForIsoIec144961,
    0x25 => StreamType::MetadataPointer,
    0x26 => StreamType::Metadata,
    0x27 => StreamType::MetadataStd,
    0x28 => StreamType::VideoStreamHeaderParametersForItuTRecH264AndIsoIec1449610,
    0x29 => StreamType::IsoIec1381811Ipmp,
    0x2A => StreamType::TimingAndHrdForItuTRecH264AndIsoIec1449610,
    0x2B => StreamType::AudioStreamHeaderParametersForIsoIec138187AdtsAac,
    0x2C => StreamType::FlexMuxTimingForIsoIec144961,
    0x2D => StreamType::TextStreamHeaderParametersForIsoIec14496,
    0x2E => StreamType::AudioExtensionStreamHeaderParametersForIsoIec144963,
    0x2F => StreamType::VideoAuxiliaryStreamHeaderParameters,
    0x30 => StreamType::VideoScalableStreamHeaderParameters,
    0x31 => StreamType::VideoMultiStreamHeaderParameters,
    0x32 => StreamType::VideoStreamHeaderParametersForItuTRecT802AndIsoIec154443,
    0x33 => StreamType::VideoMultiOperationPointStreamHeaderParameters,
    0x34 => StreamType::VideoStereoscopic3DStreamHeaderParametersForItuTRecH262IsoIec138182AndIsoIec111722,
    0x35 => StreamType::ProgramStereoscopic3DInformation,
    0x36 => StreamType::VideoStereoscopic3DInformation,
    0xA0 => StreamType::VideoLanFourCc,
    0xFF => StreamType::Forbidden,
       _ => {
      if (stream_type >= 0x37) && (stream_type <= 0x3F) {
        StreamType::Reserved
      } else {
        StreamType::Other
      }
    }
  }
}
